use std::{
    cmp::min,
    io::{self, Stdout},
    time::Duration,
};

use anyhow::{bail, Context};
use chrono::{DateTime, Local};
use crossterm::{
    event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    prelude::{Alignment, Line, Span, Style, Text},
    widgets::{Block, Borders, Cell, Clear, Paragraph, Row, Table, Tabs, Wrap},
    Frame, Terminal,
};

use super::{
    api::{
        FactoryCheckpoint, FactoryDetail, FactoryRun, RoomHistory, RoomRecord, SendMessageRequest,
        TuiApi,
    },
    layout,
    state::{AppState, FactorySubview, Focus, Selection, SidebarEntry},
    theme::Theme,
};

type Backend = CrosstermBackend<Stdout>;

pub async fn run(client: crate::api::ApiClient) -> anyhow::Result<()> {
    let api = TuiApi::new(client);
    if !api.token_present() {
        bail!("abbot tui requires an existing saved or overridden bearer token; run `abbot auth login` first");
    }

    let mut terminal = TerminalGuard::enter()?;
    let mut app = TuiApp::new(api);
    app.bootstrap().await?;
    let result = app.run_loop(terminal.terminal_mut()).await;
    drop(terminal);
    result
}

struct TuiApp {
    api: TuiApi,
    state: AppState,
}

impl TuiApp {
    fn new(api: TuiApi) -> Self {
        Self {
            api,
            state: AppState::new(),
        }
    }

    async fn bootstrap(&mut self) -> anyhow::Result<()> {
        self.refresh_all().await
    }

    async fn run_loop(&mut self, terminal: &mut Terminal<Backend>) -> anyhow::Result<()> {
        loop {
            terminal.draw(|frame| self.render(frame))?;

            if !event::poll(Duration::from_millis(200))? {
                continue;
            }

            match event::read()? {
                Event::Key(key) if key.kind == KeyEventKind::Press => {
                    if self.handle_key(key).await? {
                        break;
                    }
                }
                Event::Resize(_, _) => {}
                _ => {}
            }
        }

        Ok(())
    }

    async fn handle_key(&mut self, key: KeyEvent) -> anyhow::Result<bool> {
        if key.modifiers.contains(KeyModifiers::CONTROL) {
            match key.code {
                KeyCode::Char('c') | KeyCode::Char('q') => return Ok(true),
                KeyCode::Char('n') => {
                    self.state.activate_draft();
                    return Ok(false);
                }
                KeyCode::Char('r') => {
                    self.refresh_all().await?;
                    return Ok(false);
                }
                _ => {}
            }
        }

        if let KeyCode::Char('q') = key.code {
            return Ok(true);
        }

        if let KeyCode::Tab = key.code {
            self.toggle_focus();
            return Ok(false);
        }

        match self.state.focus {
            Focus::Sidebar => self.handle_sidebar_key(key).await?,
            Focus::Composer => self.handle_composer_key(key).await?,
        }

        Ok(false)
    }

    async fn handle_sidebar_key(&mut self, key: KeyEvent) -> anyhow::Result<()> {
        match key.code {
            KeyCode::Up | KeyCode::Char('k') => self.state.move_selection(-1),
            KeyCode::Down | KeyCode::Char('j') => self.state.move_selection(1),
            KeyCode::Enter => self.refresh_selected_detail().await?,
            KeyCode::Esc => {
                self.state.selection = None;
                self.state.focus = Focus::Sidebar;
            }
            KeyCode::Char('p') => {
                self.state.toggle_pin_for_selected_room();
                self.state.reconcile_selection();
            }
            KeyCode::Char(digit) => {
                if let Some(view) = FactorySubview::from_digit(digit) {
                    self.state.factory_view = view;
                }
            }
            _ => {}
        }

        if matches!(key.code, KeyCode::Up | KeyCode::Down) {
            self.refresh_selected_detail().await?;
        }

        Ok(())
    }

    async fn handle_composer_key(&mut self, key: KeyEvent) -> anyhow::Result<()> {
        match key.code {
            KeyCode::Esc => {
                self.state.focus = Focus::Sidebar;
                self.state.clear_draft_if_empty();
            }
            KeyCode::Backspace => {
                if let Some(input) = self.state.current_input_mut() {
                    input.pop();
                }
            }
            KeyCode::Enter => {
                self.submit_current_message().await?;
            }
            KeyCode::Char(ch) if !key.modifiers.contains(KeyModifiers::CONTROL) => {
                if let Some(input) = self.state.current_input_mut() {
                    input.push(ch);
                }
            }
            KeyCode::Tab => {
                self.state.focus = Focus::Sidebar;
            }
            _ => {}
        }

        Ok(())
    }

    fn toggle_focus(&mut self) {
        self.state.focus = match self.state.focus {
            Focus::Sidebar if self.state.is_room_selected() => Focus::Composer,
            Focus::Sidebar => Focus::Sidebar,
            Focus::Composer => Focus::Sidebar,
        };
    }

    async fn refresh_all(&mut self) -> anyhow::Result<()> {
        let rooms = self.api.list_rooms().await?;
        let factories = self.api.list_factory_runs().await?;
        self.state.rooms = rooms;
        self.state.factories = factories;
        self.state.reconcile_selection();
        self.refresh_selected_detail().await?;
        self.state.last_refresh_label = Some(now_label());
        self.state.status_line = "Refreshed rooms and factory runs.".to_string();
        Ok(())
    }

    async fn refresh_selected_detail(&mut self) -> anyhow::Result<()> {
        self.state.room_history = None;
        self.state.factory_detail = None;

        match self.state.selection.clone() {
            Some(Selection::Room(room_id)) => {
                let history = self.api.load_room(&room_id).await?;
                self.state.room_history = Some(history);
                self.state.status_line = format!("Loaded room {room_id}.");
            }
            Some(Selection::Factory(run_id)) => {
                let detail = self.api.load_factory(&run_id).await?;
                self.state.factory_detail = Some(detail);
                self.state.status_line = format!("Loaded factory run {run_id}.");
            }
            Some(Selection::DraftRoom) => {
                self.state.status_line = "Draft room is local until the first send.".to_string();
            }
            None => {
                self.state.status_line = "No room or factory selected.".to_string();
            }
        }

        Ok(())
    }

    async fn submit_current_message(&mut self) -> anyhow::Result<()> {
        let input = self
            .state
            .current_input()
            .map(str::trim)
            .unwrap_or_default()
            .to_string();

        if input.is_empty() {
            self.state.status_line = "Composer is empty.".to_string();
            return Ok(());
        }

        match self.state.selection.clone() {
            Some(Selection::DraftRoom) => {
                let response = self
                    .api
                    .send_message(SendMessageRequest {
                        room_id: None,
                        input,
                        purpose: if self.state.draft.purpose.trim().is_empty() {
                            None
                        } else {
                            Some(self.state.draft.purpose.trim().to_string())
                        },
                        provider: self.state.draft.provider.clone(),
                        model: self.state.draft.model.clone(),
                        adapter: self.state.draft.adapter.clone(),
                    })
                    .await?;
                self.state.draft.input.clear();
                self.state.draft.active = false;
                self.state.selection = Some(Selection::Room(response.room_id.clone()));
                self.refresh_all().await?;
                self.state.status_line = format!(
                    "Queued room-backed response for {}{}.",
                    response.room_id,
                    if response.room_created {
                        " (new room)"
                    } else {
                        ""
                    }
                );
            }
            Some(Selection::Room(room_id)) => {
                let response = self
                    .api
                    .send_message(SendMessageRequest {
                        room_id: Some(room_id.clone()),
                        input,
                        purpose: None,
                        provider: "openrouter".to_string(),
                        model: "openai/gpt-5.4".to_string(),
                        adapter: "pi".to_string(),
                    })
                    .await?;
                self.state
                    .room_inputs
                    .insert(room_id.clone(), String::new());
                self.refresh_selected_detail().await?;
                self.state.last_refresh_label = Some(now_label());
                self.state.status_line = format!(
                    "Queued response in room {} ({})",
                    room_id,
                    response.status.unwrap_or_else(|| "queued".to_string())
                );
            }
            Some(Selection::Factory(_)) | None => {
                self.state.status_line = "Factory views are read-only in v1.".to_string();
            }
        }

        Ok(())
    }

    fn render(&self, frame: &mut Frame) {
        frame.render_widget(Clear, frame.area());
        let layout = layout::split(frame.area());

        self.render_shortcuts(frame, layout.top_shortcuts);
        self.render_header(frame, layout.header);
        self.render_sidebar(frame, layout.main_sidebar);
        self.render_content(frame, layout.main_content);
        self.render_footer(frame, layout.footer);
        self.render_composer(frame, layout.composer);
    }

    fn render_shortcuts(&self, frame: &mut Frame, area: Rect) {
        let left = Line::from(vec![
            Span::styled("q", Theme::accent()),
            Span::styled(" quit  ", Theme::dim()),
            Span::styled("Ctrl-N", Theme::accent()),
            Span::styled(" draft  ", Theme::dim()),
            Span::styled("Ctrl-R", Theme::accent()),
            Span::styled(" refresh  ", Theme::dim()),
            Span::styled("Tab", Theme::accent()),
            Span::styled(" focus  ", Theme::dim()),
            Span::styled("1-6", Theme::accent()),
            Span::styled(" factory views", Theme::dim()),
        ]);
        frame.render_widget(Paragraph::new(left).style(Theme::chrome()), area);
    }

    fn render_header(&self, frame: &mut Frame, area: Rect) {
        let title = match self.state.selection.as_ref() {
            Some(Selection::DraftRoom) => "Draft Room",
            Some(Selection::Room(room_id)) => room_id.as_str(),
            Some(Selection::Factory(run_id)) => run_id.as_str(),
            None => "Abbot TUI",
        };

        let status = match self.state.selection.as_ref() {
            Some(Selection::DraftRoom) => "local draft",
            Some(Selection::Room(_)) => self
                .state
                .room_history
                .as_ref()
                .map(|history| history.room.status.as_str())
                .unwrap_or("room"),
            Some(Selection::Factory(_)) => self
                .state
                .factory_detail
                .as_ref()
                .map(|detail| detail.status.status.as_str())
                .unwrap_or("factory"),
            None => "idle",
        };

        let text = vec![
            Line::from(vec![
                Span::styled("Abbot Operator Console", Theme::accent()),
                Span::styled("  ", Theme::chrome()),
                Span::raw("● "),
                Span::styled(status, status_style(status)),
            ]),
            Line::from(vec![
                Span::styled(title, Theme::text()),
                Span::styled("  ", Theme::chrome()),
                Span::styled(self.focus_label(), Theme::dim()),
            ]),
        ];
        frame.render_widget(Paragraph::new(text).style(Theme::text()), area);
    }

    fn render_sidebar(&self, frame: &mut Frame, area: Rect) {
        let block = Block::default()
            .title(Span::styled("Rooms and Factory", Theme::accent()))
            .borders(Borders::RIGHT);
        let inner = block.inner(area);
        frame.render_widget(block, area);

        let render = self.sidebar_render();
        let viewport_height = inner.height as usize;
        let scroll = render
            .selected_row
            .map(|row| row.saturating_sub(viewport_height.saturating_sub(1) / 2))
            .unwrap_or(0);

        frame.render_widget(
            Paragraph::new(Text::from(render.lines))
                .style(Theme::text())
                .scroll((scroll as u16, 0)),
            inner,
        );
    }

    fn render_content(&self, frame: &mut Frame, area: Rect) {
        match self.state.selection.as_ref() {
            Some(Selection::DraftRoom) | Some(Selection::Room(_)) => {
                self.render_room_content(frame, area)
            }
            Some(Selection::Factory(_)) => self.render_factory_content(frame, area),
            None => self.render_empty_content(frame, area),
        }
    }

    fn render_room_content(&self, frame: &mut Frame, area: Rect) {
        let vertical = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(2), Constraint::Min(4)])
            .split(area);

        let header = match self.state.selection.as_ref() {
            Some(Selection::DraftRoom) => vec![
                Line::from(vec![
                    Span::styled("Draft room", Theme::accent()),
                    Span::styled("  create on first send", Theme::dim()),
                ]),
                Line::from(vec![
                    Span::styled(self.draft_model_line(), Theme::dim()),
                    Span::styled("  ", Theme::dim()),
                    Span::styled(self.draft_purpose_line(), Theme::text()),
                ]),
            ],
            Some(Selection::Room(_)) => {
                let Some(history) = self.state.room_history.as_ref() else {
                    self.render_empty_content(frame, area);
                    return;
                };
                vec![
                    Line::from(vec![
                        Span::styled(history.room.purpose.as_str(), Theme::accent()),
                        Span::styled("  ", Theme::chrome()),
                        Span::styled(
                            history.room.status.as_str(),
                            status_style(&history.room.status),
                        ),
                        Span::styled("  ", Theme::chrome()),
                        Span::styled(actor_label(&history.room), Theme::dim()),
                    ]),
                    Line::from(vec![
                        Span::styled(
                            format!(
                                "updated {}",
                                format_timestamp(
                                    history
                                        .room
                                        .last_active_at
                                        .as_deref()
                                        .unwrap_or(&history.room.rented_at)
                                )
                            ),
                            Theme::dim(),
                        ),
                        Span::styled("  ", Theme::chrome()),
                        Span::styled(
                            history
                                .room
                                .summary_text
                                .as_deref()
                                .unwrap_or("no summary yet"),
                            Theme::text(),
                        ),
                    ]),
                ]
            }
            _ => unreachable!(),
        };
        frame.render_widget(Paragraph::new(header).style(Theme::text()), vertical[0]);

        let lines = match self.state.selection.as_ref() {
            Some(Selection::DraftRoom) => vec![
                Line::from(Span::styled("No room exists yet.", Theme::dim())),
                Line::from(Span::styled(
                    "Type in the composer below. The first send will call /v1/responses and attach the draft to a real room.",
                    Theme::text(),
                )),
            ],
            Some(Selection::Room(_)) => room_transcript_lines(self.state.room_history.as_ref()),
            _ => Vec::new(),
        };

        frame.render_widget(
            Paragraph::new(Text::from(lines))
                .style(Theme::text())
                .wrap(Wrap { trim: false }),
            vertical[1],
        );
    }

    fn render_factory_content(&self, frame: &mut Frame, area: Rect) {
        let Some(detail) = self.state.factory_detail.as_ref() else {
            self.render_empty_content(frame, area);
            return;
        };

        let vertical = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(2),
                Constraint::Length(2),
                Constraint::Min(6),
            ])
            .split(area);

        let header = vec![
            Line::from(vec![
                Span::styled(
                    detail
                        .run
                        .source_brief
                        .as_deref()
                        .unwrap_or(detail.run.id.as_str()),
                    Theme::accent(),
                ),
                Span::styled("  ", Theme::chrome()),
                Span::styled(
                    detail.status.status.as_str(),
                    status_style(&detail.status.status),
                ),
                Span::styled("  ", Theme::chrome()),
                Span::styled(
                    format!(
                        "stage {}",
                        detail.status.current_stage.as_deref().unwrap_or("n/a")
                    ),
                    Theme::dim(),
                ),
            ]),
            Line::from(vec![
                Span::styled(
                    format!("updated {}", format_timestamp(&detail.run.updated_at)),
                    Theme::dim(),
                ),
                Span::styled("  ", Theme::chrome()),
                Span::styled(
                    format!("{} active rooms", detail.run.active_room_ids.len()),
                    Theme::dim(),
                ),
            ]),
        ];
        frame.render_widget(Paragraph::new(header).style(Theme::text()), vertical[0]);

        let tabs = Tabs::new(
            FactorySubview::ALL
                .iter()
                .enumerate()
                .map(|(index, view)| {
                    Line::from(vec![
                        Span::styled(format!("{} ", index + 1), Theme::accent()),
                        Span::raw(view.title()),
                    ])
                })
                .collect::<Vec<_>>(),
        )
        .select(
            FactorySubview::ALL
                .iter()
                .position(|view| *view == self.state.factory_view)
                .unwrap_or(0),
        )
        .style(Theme::dim())
        .highlight_style(Theme::selected())
        .divider("  ");
        frame.render_widget(tabs, vertical[1]);

        match self.state.factory_view {
            FactorySubview::Overview => self.render_factory_overview(frame, vertical[2], detail),
            FactorySubview::Stages => {
                self.render_stage_table(frame, vertical[2], detail);
            }
            FactorySubview::Issues => {
                self.render_issue_table(frame, vertical[2], detail);
            }
            FactorySubview::Checkpoints => {
                self.render_checkpoint_table(frame, vertical[2], detail);
            }
            FactorySubview::Artifacts => {
                self.render_artifact_table(frame, vertical[2], detail);
            }
            FactorySubview::Review => {
                self.render_factory_review(frame, vertical[2], detail);
            }
        }
    }

    fn render_factory_overview(&self, frame: &mut Frame, area: Rect, detail: &FactoryDetail) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(8), Constraint::Min(6)])
            .split(area);

        let summary = vec![
            Line::from(vec![
                Span::styled("verification ", Theme::dim()),
                Span::styled(
                    match detail.status.latest_verification_success {
                        Some(true) => "✓ pass",
                        Some(false) => "✗ fail",
                        None => "⏱ pending",
                    },
                    match detail.status.latest_verification_success {
                        Some(true) => Theme::success(),
                        Some(false) => Theme::error(),
                        None => Theme::warning(),
                    },
                ),
            ]),
            Line::from(vec![
                Span::styled("checkpoint ", Theme::dim()),
                Span::styled(
                    detail
                        .status
                        .latest_checkpoint_status
                        .as_deref()
                        .unwrap_or("none"),
                    Theme::text(),
                ),
            ]),
            Line::from(vec![
                Span::styled("gates ", Theme::dim()),
                Span::styled(
                    render_map(&detail.status.latest_gate_verdicts),
                    Theme::text(),
                ),
            ]),
            Line::from(vec![
                Span::styled("stage counts ", Theme::dim()),
                Span::styled(render_map_u64(&detail.status.stage_counts), Theme::text()),
            ]),
            Line::from(vec![
                Span::styled("issue counts ", Theme::dim()),
                Span::styled(render_map_u64(&detail.status.issue_counts), Theme::text()),
            ]),
        ];
        frame.render_widget(Paragraph::new(summary).style(Theme::text()), chunks[0]);

        let details = vec![
            section_lines("Blockers", &detail.status.blockers, Theme::error()),
            section_lines("Diagnostics", &detail.status.diagnostics, Theme::warning()),
            section_lines("Next Actions", &detail.status.next_actions, Theme::accent()),
        ]
        .into_iter()
        .flatten()
        .collect::<Vec<_>>();
        frame.render_widget(
            Paragraph::new(Text::from(details))
                .style(Theme::text())
                .wrap(Wrap { trim: false }),
            chunks[1],
        );
    }

    fn render_stage_table(&self, frame: &mut Frame, area: Rect, detail: &FactoryDetail) {
        let rows = detail.stages.iter().map(|stage| {
            Row::new(vec![
                Cell::from(stage.stage_name.clone()),
                Cell::from(stage.status.clone()),
                Cell::from(
                    stage
                        .assigned_room_id
                        .clone()
                        .unwrap_or_else(|| "-".to_string()),
                ),
                Cell::from(
                    stage
                        .verification_summary
                        .clone()
                        .unwrap_or_else(|| stage.summary.clone().unwrap_or_default()),
                ),
            ])
        });
        let table = Table::new(
            rows,
            [
                Constraint::Percentage(28),
                Constraint::Length(12),
                Constraint::Length(16),
                Constraint::Percentage(44),
            ],
        )
        .header(Row::new(vec!["Stage", "Status", "Room", "Summary"]).style(Theme::accent()))
        .column_spacing(1);
        frame.render_widget(table, area);
    }

    fn render_issue_table(&self, frame: &mut Frame, area: Rect, detail: &FactoryDetail) {
        let rows = detail.issues.iter().map(|issue| {
            Row::new(vec![
                Cell::from(issue.title.clone()),
                Cell::from(issue.kind.clone()),
                Cell::from(issue.status.clone()),
                Cell::from(
                    issue
                        .assigned_room_id
                        .clone()
                        .unwrap_or_else(|| "-".to_string()),
                ),
            ])
        });
        let table = Table::new(
            rows,
            [
                Constraint::Percentage(44),
                Constraint::Length(14),
                Constraint::Length(12),
                Constraint::Length(16),
            ],
        )
        .header(Row::new(vec!["Issue", "Kind", "Status", "Room"]).style(Theme::accent()))
        .column_spacing(1);
        frame.render_widget(table, area);
    }

    fn render_checkpoint_table(&self, frame: &mut Frame, area: Rect, detail: &FactoryDetail) {
        checkpoint_table(frame, area, &detail.checkpoints);
    }

    fn render_artifact_table(&self, frame: &mut Frame, area: Rect, detail: &FactoryDetail) {
        let rows = detail.artifacts.iter().map(|artifact| {
            Row::new(vec![
                Cell::from(artifact.artifact_type.clone()),
                Cell::from(format!("v{}", artifact.version)),
                Cell::from(
                    artifact
                        .producer_stage_id
                        .clone()
                        .unwrap_or_else(|| "-".to_string()),
                ),
                Cell::from(compact_json(&artifact.payload)),
            ])
        });
        let table = Table::new(
            rows,
            [
                Constraint::Length(22),
                Constraint::Length(6),
                Constraint::Length(18),
                Constraint::Percentage(100),
            ],
        )
        .header(Row::new(vec!["Artifact", "Ver", "Stage", "Payload"]).style(Theme::accent()))
        .column_spacing(1);
        frame.render_widget(table, area);
    }

    fn render_factory_review(&self, frame: &mut Frame, area: Rect, detail: &FactoryDetail) {
        let review = &detail.review;
        let lines = vec![
            Line::from(Span::styled(review.plan_summary.clone(), Theme::text())),
            Line::default(),
        ]
        .into_iter()
        .chain(section_lines(
            "Artifacts",
            &review.artifact_summary,
            Theme::text(),
        ))
        .chain(section_lines(
            "Checkpoints",
            &review.checkpoint_summary,
            Theme::text(),
        ))
        .chain(section_lines(
            "Stages",
            &review.stage_summary,
            Theme::text(),
        ))
        .chain(section_lines(
            "Issues",
            &review.issue_summary,
            Theme::text(),
        ))
        .chain(section_lines("Gates", &review.gate_summary, Theme::text()))
        .chain(section_lines("Blockers", &review.blockers, Theme::error()))
        .chain(section_lines(
            "Diagnostics",
            &review.diagnostics,
            Theme::warning(),
        ))
        .chain(section_lines(
            "Next Actions",
            &review.next_actions,
            Theme::accent(),
        ))
        .chain(section_lines(
            "Open Risks",
            &review.open_risks,
            Theme::warning(),
        ))
        .collect::<Vec<_>>();
        frame.render_widget(
            Paragraph::new(Text::from(lines))
                .style(Theme::text())
                .wrap(Wrap { trim: false }),
            area,
        );
    }

    fn render_empty_content(&self, frame: &mut Frame, area: Rect) {
        let lines = vec![
            Line::from(Span::styled("No selection yet.", Theme::accent())),
            Line::from(Span::styled(
                "Use the sidebar to inspect a room or factory run. Press Ctrl-N to start a draft room.",
                Theme::text(),
            )),
        ];
        frame.render_widget(
            Paragraph::new(lines)
                .style(Theme::text())
                .alignment(Alignment::Left),
            area,
        );
    }

    fn render_footer(&self, frame: &mut Frame, area: Rect) {
        let left = self
            .state
            .last_refresh_label
            .as_deref()
            .map(|label| format!("last refresh {label}"))
            .unwrap_or_else(|| "not refreshed yet".to_string());
        let mode = match self.state.focus {
            Focus::Sidebar => "NORMAL",
            Focus::Composer => "INSERT",
        };
        let text = Line::from(vec![
            Span::styled(left, Theme::dim()),
            Span::styled("  ", Theme::chrome()),
            Span::styled(mode, Theme::accent()),
            Span::styled("  ", Theme::chrome()),
            Span::styled(self.state.status_line.as_str(), Theme::text()),
        ]);
        frame.render_widget(Paragraph::new(text).style(Theme::text()), area);
    }

    fn render_composer(&self, frame: &mut Frame, area: Rect) {
        let block = Block::default()
            .title(Span::styled("Composer", Theme::accent()))
            .borders(Borders::TOP);
        let inner = block.inner(area);
        frame.render_widget(block, area);

        let input = self.state.current_input().unwrap_or_default();
        let mut lines = vec![Line::from(vec![
            Span::styled(
                if self.state.focus == Focus::Composer {
                    "▌ "
                } else {
                    "  "
                },
                Theme::accent(),
            ),
            Span::styled(
                if input.is_empty() {
                    "Type a message and press Enter."
                } else {
                    input
                },
                if input.is_empty() {
                    Theme::dim()
                } else {
                    Theme::text()
                },
            ),
        ])];

        if matches!(self.state.selection, Some(Selection::DraftRoom)) {
            lines.push(Line::from(vec![
                Span::styled("purpose ", Theme::dim()),
                Span::styled(self.draft_purpose_line(), Theme::text()),
            ]));
        } else if matches!(self.state.selection, Some(Selection::Factory(_))) {
            lines = vec![Line::from(Span::styled(
                "Factory views are read-only in v1. Use the CLI verb surface for mutations.",
                Theme::dim(),
            ))];
        }

        frame.render_widget(
            Paragraph::new(lines)
                .style(Theme::text())
                .wrap(Wrap { trim: false }),
            inner,
        );
    }

    fn sidebar_render(&self) -> SidebarRender {
        let entries = self.state.sidebar_entries();
        let mut lines = Vec::new();
        let mut selected_row = None;
        let push_section = |label: &str, lines: &mut Vec<Line<'static>>| {
            if !lines.is_empty() {
                lines.push(Line::default());
            }
            lines.push(Line::from(Span::styled(label.to_string(), Theme::dim())));
        };

        let is_selected = |entry: &SidebarEntry| {
            self.state
                .selection
                .as_ref()
                .map(|selection| match (selection, entry) {
                    (Selection::DraftRoom, SidebarEntry::DraftRoom) => true,
                    (Selection::Room(left), SidebarEntry::Room(right)) => left == right,
                    (Selection::Factory(left), SidebarEntry::Factory(right)) => left == right,
                    _ => false,
                })
                .unwrap_or(false)
        };

        let draft_active = entries
            .iter()
            .any(|entry| matches!(entry, SidebarEntry::DraftRoom));
        if draft_active {
            push_section("Draft", &mut lines);
            let row_index = lines.len();
            let style = if matches!(self.state.selection, Some(Selection::DraftRoom)) {
                selected_row = Some(row_index);
                Theme::selected()
            } else {
                Theme::text()
            };
            lines.push(Line::from(Span::styled("✎ New room draft", style)));
        }

        let pinned_rooms = self
            .state
            .rooms
            .iter()
            .filter(|room| self.state.pinned_rooms.contains(&room.id))
            .collect::<Vec<_>>();
        if !pinned_rooms.is_empty() {
            push_section("Pinned Rooms", &mut lines);
            for room in pinned_rooms {
                let row_index = lines.len();
                let style = if is_selected(&SidebarEntry::Room(room.id.clone())) {
                    selected_row = Some(row_index);
                    Theme::selected()
                } else {
                    Theme::text()
                };
                lines.push(Line::from(Span::styled(
                    sidebar_room_label(room, &self.state),
                    style,
                )));
            }
        }

        push_section("Factory Runs", &mut lines);
        for factory in &self.state.factories {
            let row_index = lines.len();
            let style = if is_selected(&SidebarEntry::Factory(factory.id.clone())) {
                selected_row = Some(row_index);
                Theme::selected()
            } else {
                Theme::text()
            };
            lines.push(Line::from(Span::styled(
                sidebar_factory_label(factory),
                style,
            )));
        }

        push_section("Recent Rooms", &mut lines);
        for room in self
            .state
            .rooms
            .iter()
            .filter(|room| !self.state.pinned_rooms.contains(&room.id))
        {
            let row_index = lines.len();
            let style = if is_selected(&SidebarEntry::Room(room.id.clone())) {
                selected_row = Some(row_index);
                Theme::selected()
            } else {
                Theme::text()
            };
            lines.push(Line::from(Span::styled(
                sidebar_room_label(room, &self.state),
                style,
            )));
        }

        SidebarRender {
            lines,
            selected_row,
        }
    }

    fn draft_model_line(&self) -> String {
        format!(
            "{} / {} / {}",
            self.state.draft.adapter, self.state.draft.provider, self.state.draft.model
        )
    }

    fn draft_purpose_line(&self) -> String {
        if self.state.draft.purpose.trim().is_empty() {
            "purpose will be inferred from the first message".to_string()
        } else {
            self.state.draft.purpose.clone()
        }
    }

    fn focus_label(&self) -> &'static str {
        match self.state.focus {
            Focus::Sidebar => "focus SIDEBAR",
            Focus::Composer => "focus COMPOSER",
        }
    }
}

struct SidebarRender {
    lines: Vec<Line<'static>>,
    selected_row: Option<usize>,
}

struct TerminalGuard {
    terminal: Terminal<Backend>,
}

impl TerminalGuard {
    fn enter() -> anyhow::Result<Self> {
        enable_raw_mode().context("failed to enable raw mode")?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen).context("failed to enter alternate screen")?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend).context("failed to initialize terminal")?;
        terminal.clear().context("failed to clear terminal")?;
        Ok(Self { terminal })
    }

    fn terminal_mut(&mut self) -> &mut Terminal<Backend> {
        &mut self.terminal
    }
}

impl Drop for TerminalGuard {
    fn drop(&mut self) {
        let _ = disable_raw_mode();
        let _ = execute!(self.terminal.backend_mut(), LeaveAlternateScreen);
        let _ = self.terminal.show_cursor();
    }
}

fn room_transcript_lines(history: Option<&RoomHistory>) -> Vec<Line<'static>> {
    let Some(history) = history else {
        return vec![Line::from(Span::styled(
            "Room history not loaded.",
            Theme::dim(),
        ))];
    };

    let mut lines = history
        .messages
        .iter()
        .map(|message| {
            let prefix = match message.author_kind.as_str() {
                "user" => "U",
                "assistant" => "A",
                "system" => "S",
                other => other,
            };
            Line::from(vec![
                Span::styled(format!("{prefix:>2} "), Theme::accent()),
                Span::styled(format_timestamp(&message.created_at), Theme::dim()),
                Span::styled("  ", Theme::chrome()),
                Span::styled(message.content.clone(), Theme::text()),
            ])
        })
        .collect::<Vec<_>>();

    if let Some(last_event) = history.events.last() {
        lines.push(Line::default());
        lines.push(Line::from(vec![
            Span::styled("last event ", Theme::dim()),
            Span::styled(last_event.event_type.clone(), Theme::warning()),
            Span::styled("  ", Theme::chrome()),
            Span::styled(format_timestamp(&last_event.created_at), Theme::dim()),
        ]));
    }

    let keep = min(lines.len(), 64);
    lines
        .into_iter()
        .rev()
        .take(keep)
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .collect()
}

fn checkpoint_table(frame: &mut Frame, area: Rect, checkpoints: &[FactoryCheckpoint]) {
    let rows = checkpoints.iter().map(|checkpoint| {
        Row::new(vec![
            Cell::from(checkpoint.branch_name.clone()),
            Cell::from(checkpoint.status.clone()),
            Cell::from(checkpoint.base_ref.clone()),
            Cell::from(
                checkpoint
                    .head_sha
                    .clone()
                    .unwrap_or_else(|| "-".to_string()),
            ),
        ])
    });
    let table = Table::new(
        rows,
        [
            Constraint::Percentage(34),
            Constraint::Length(12),
            Constraint::Percentage(26),
            Constraint::Percentage(28),
        ],
    )
    .header(Row::new(vec!["Branch", "Status", "Base", "Head"]).style(Theme::accent()))
    .column_spacing(1);
    frame.render_widget(table, area);
}

fn format_timestamp(raw: &str) -> String {
    DateTime::parse_from_rfc3339(raw)
        .map(|value| {
            value
                .with_timezone(&Local)
                .format("%b %d %H:%M")
                .to_string()
        })
        .unwrap_or_else(|_| raw.to_string())
}

fn now_label() -> String {
    Local::now().format("%H:%M:%S").to_string()
}

fn actor_label(room: &RoomRecord) -> String {
    room.actors
        .first()
        .map(|actor| {
            let adapter = actor.adapter.as_deref().unwrap_or("adapter");
            let provider = actor.provider.as_deref().unwrap_or("provider");
            let model = actor.model.as_deref().unwrap_or("model");
            format!("{adapter} / {provider} / {model}")
        })
        .unwrap_or_else(|| "no actor".to_string())
}

fn status_style(status: &str) -> Style {
    match status {
        "completed" | "passed" | "idle" | "active" => Theme::success(),
        "failed" | "error" | "blocked" => Theme::error(),
        "planning" | "verifying" | "gated" | "ready" | "running" | "pending" => Theme::warning(),
        _ => Theme::text(),
    }
}

fn sidebar_room_label(room: &RoomRecord, state: &AppState) -> String {
    let unread = state
        .unread_counts
        .get(&room.id)
        .copied()
        .unwrap_or_default();
    let unread_label = if unread > 0 {
        format!(" •{unread}")
    } else {
        String::new()
    };
    let summary = room
        .summary_text
        .as_deref()
        .filter(|value| !value.trim().is_empty())
        .unwrap_or(room.purpose.as_str());
    format!(
        "◌ {} [{}]{}",
        truncate(summary, 24),
        room.status,
        unread_label
    )
}

fn sidebar_factory_label(factory: &FactoryRun) -> String {
    let label = factory
        .source_brief
        .as_deref()
        .filter(|value| !value.trim().is_empty())
        .unwrap_or(
            factory
                .current_stage
                .as_deref()
                .unwrap_or(factory.id.as_str()),
        );
    format!("ƒ {} [{}]", truncate(label, 24), factory.status)
}

fn truncate(value: &str, max: usize) -> String {
    let mut chars = value.chars();
    let visible = chars.by_ref().take(max).collect::<String>();
    if chars.next().is_some() {
        format!("{visible}…")
    } else {
        visible
    }
}

fn render_map(map: &std::collections::BTreeMap<String, String>) -> String {
    if map.is_empty() {
        return "none".to_string();
    }
    map.iter()
        .map(|(key, value)| format!("{key}={value}"))
        .collect::<Vec<_>>()
        .join(", ")
}

fn render_map_u64(map: &std::collections::BTreeMap<String, u64>) -> String {
    if map.is_empty() {
        return "none".to_string();
    }
    map.iter()
        .map(|(key, value)| format!("{key}={value}"))
        .collect::<Vec<_>>()
        .join(", ")
}

fn section_lines(title: &str, items: &[String], title_style: Style) -> Vec<Line<'static>> {
    let mut lines = vec![
        Line::default(),
        Line::from(Span::styled(title.to_string(), title_style)),
    ];
    if items.is_empty() {
        lines.push(Line::from(Span::styled("  none", Theme::dim())));
        return lines;
    }

    lines.extend(
        items
            .iter()
            .map(|item| Line::from(Span::styled(format!("  • {item}"), Theme::text()))),
    );
    lines
}

fn compact_json(value: &serde_json::Value) -> String {
    let text = serde_json::to_string(value).unwrap_or_else(|_| "{}".to_string());
    truncate(&text, 56)
}
