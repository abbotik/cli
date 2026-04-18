use std::collections::{HashMap, HashSet};

use super::api::{FactoryDetail, FactoryRun, RoomHistory, RoomRecord};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Focus {
    Sidebar,
    Composer,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FactorySubview {
    Overview,
    Stages,
    Issues,
    Checkpoints,
    Artifacts,
    Review,
}

impl FactorySubview {
    pub const ALL: [FactorySubview; 6] = [
        FactorySubview::Overview,
        FactorySubview::Stages,
        FactorySubview::Issues,
        FactorySubview::Checkpoints,
        FactorySubview::Artifacts,
        FactorySubview::Review,
    ];

    pub fn title(self) -> &'static str {
        match self {
            FactorySubview::Overview => "Overview",
            FactorySubview::Stages => "Stages",
            FactorySubview::Issues => "Issues",
            FactorySubview::Checkpoints => "Checkpoints",
            FactorySubview::Artifacts => "Artifacts",
            FactorySubview::Review => "Review",
        }
    }

    pub fn from_digit(digit: char) -> Option<Self> {
        match digit {
            '1' => Some(Self::Overview),
            '2' => Some(Self::Stages),
            '3' => Some(Self::Issues),
            '4' => Some(Self::Checkpoints),
            '5' => Some(Self::Artifacts),
            '6' => Some(Self::Review),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Selection {
    DraftRoom,
    Room(String),
    Factory(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SidebarEntry {
    DraftRoom,
    Room(String),
    Factory(String),
}

#[derive(Debug, Clone)]
pub struct DraftRoom {
    pub purpose: String,
    pub provider: String,
    pub model: String,
    pub adapter: String,
    pub input: String,
    pub active: bool,
}

impl Default for DraftRoom {
    fn default() -> Self {
        Self {
            purpose: String::new(),
            provider: "openrouter".to_string(),
            model: "openai/gpt-5.4".to_string(),
            adapter: "pi".to_string(),
            input: String::new(),
            active: false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct AppState {
    pub focus: Focus,
    pub rooms: Vec<RoomRecord>,
    pub factories: Vec<FactoryRun>,
    pub selection: Option<Selection>,
    pub draft: DraftRoom,
    pub pinned_rooms: HashSet<String>,
    pub unread_counts: HashMap<String, usize>,
    pub room_inputs: HashMap<String, String>,
    pub room_history: Option<RoomHistory>,
    pub factory_detail: Option<FactoryDetail>,
    pub factory_view: FactorySubview,
    pub status_line: String,
    pub last_refresh_label: Option<String>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            focus: Focus::Sidebar,
            rooms: Vec::new(),
            factories: Vec::new(),
            selection: None,
            draft: DraftRoom::default(),
            pinned_rooms: HashSet::new(),
            unread_counts: HashMap::new(),
            room_inputs: HashMap::new(),
            room_history: None,
            factory_detail: None,
            factory_view: FactorySubview::Overview,
            status_line: "Press Ctrl-N to start a draft room.".to_string(),
            last_refresh_label: None,
        }
    }

    pub fn sidebar_entries(&self) -> Vec<SidebarEntry> {
        let mut entries = Vec::new();
        if self.draft.active {
            entries.push(SidebarEntry::DraftRoom);
        }

        let mut pinned = Vec::new();
        let mut recent = Vec::new();

        for room in &self.rooms {
            if self.pinned_rooms.contains(&room.id) {
                pinned.push(SidebarEntry::Room(room.id.clone()));
            } else {
                recent.push(SidebarEntry::Room(room.id.clone()));
            }
        }

        entries.extend(pinned);
        entries.extend(
            self.factories
                .iter()
                .map(|factory| SidebarEntry::Factory(factory.id.clone())),
        );
        entries.extend(recent);
        entries
    }

    pub fn move_selection(&mut self, delta: isize) {
        let entries = self.sidebar_entries();
        if entries.is_empty() {
            self.selection = None;
            return;
        }

        let current_index = self
            .selection
            .as_ref()
            .and_then(|selection| entries.iter().position(|entry| selection.matches(entry)))
            .unwrap_or(0) as isize;

        let next_index = (current_index + delta).clamp(0, entries.len().saturating_sub(1) as isize);
        self.selection = Some(entries[next_index as usize].selection());
    }

    pub fn reconcile_selection(&mut self) {
        let entries = self.sidebar_entries();
        if entries.is_empty() {
            self.selection = None;
            return;
        }

        if self
            .selection
            .as_ref()
            .and_then(|selection| entries.iter().position(|entry| selection.matches(entry)))
            .is_none()
        {
            self.selection = Some(entries[0].selection());
        }
    }

    pub fn activate_draft(&mut self) {
        self.draft.active = true;
        self.selection = Some(Selection::DraftRoom);
        self.focus = Focus::Composer;
        self.status_line =
            "Draft room ready. Type in the composer and press Enter to send.".to_string();
    }

    pub fn clear_draft_if_empty(&mut self) {
        if self.draft.active
            && self.draft.input.trim().is_empty()
            && self.draft.purpose.trim().is_empty()
        {
            self.draft.active = false;
            if matches!(self.selection, Some(Selection::DraftRoom)) {
                self.selection = None;
            }
        }
    }

    pub fn current_input_mut(&mut self) -> Option<&mut String> {
        match self.selection.clone() {
            Some(Selection::DraftRoom) => Some(&mut self.draft.input),
            Some(Selection::Room(room_id)) => Some(self.room_inputs.entry(room_id).or_default()),
            Some(Selection::Factory(_)) | None => None,
        }
    }

    pub fn current_input(&self) -> Option<&str> {
        match self.selection.as_ref() {
            Some(Selection::DraftRoom) => Some(self.draft.input.as_str()),
            Some(Selection::Room(room_id)) => self.room_inputs.get(room_id).map(String::as_str),
            Some(Selection::Factory(_)) | None => None,
        }
    }

    pub fn current_room_id(&self) -> Option<&str> {
        match self.selection.as_ref() {
            Some(Selection::Room(room_id)) => Some(room_id.as_str()),
            _ => None,
        }
    }

    pub fn is_room_selected(&self) -> bool {
        matches!(
            self.selection,
            Some(Selection::Room(_)) | Some(Selection::DraftRoom)
        )
    }

    pub fn toggle_pin_for_selected_room(&mut self) {
        let Some(room_id) = self.current_room_id().map(ToOwned::to_owned) else {
            return;
        };

        if !self.pinned_rooms.insert(room_id.clone()) {
            self.pinned_rooms.remove(&room_id);
        }
    }
}

impl Selection {
    fn matches(&self, entry: &SidebarEntry) -> bool {
        match (self, entry) {
            (Selection::DraftRoom, SidebarEntry::DraftRoom) => true,
            (Selection::Room(left), SidebarEntry::Room(right)) => left == right,
            (Selection::Factory(left), SidebarEntry::Factory(right)) => left == right,
            _ => false,
        }
    }
}

impl SidebarEntry {
    fn selection(&self) -> Selection {
        match self {
            SidebarEntry::DraftRoom => Selection::DraftRoom,
            SidebarEntry::Room(room_id) => Selection::Room(room_id.clone()),
            SidebarEntry::Factory(run_id) => Selection::Factory(run_id.clone()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn draft_room_appears_first_when_active() {
        let mut state = AppState::new();
        state.activate_draft();
        state.rooms.push(RoomRecord {
            id: "room-1".to_string(),
            purpose: "Investigate bug".to_string(),
            status: "idle".to_string(),
            rented_at: "2026-04-18T00:00:00Z".to_string(),
            last_active_at: None,
            actors: Vec::new(),
            summary_text: None,
            last_error: None,
        });

        let entries = state.sidebar_entries();
        assert_eq!(entries.first(), Some(&SidebarEntry::DraftRoom));
    }

    #[test]
    fn moving_selection_uses_sidebar_order() {
        let mut state = AppState::new();
        state.rooms.push(RoomRecord {
            id: "room-1".to_string(),
            purpose: "Investigate bug".to_string(),
            status: "idle".to_string(),
            rented_at: "2026-04-18T00:00:00Z".to_string(),
            last_active_at: None,
            actors: Vec::new(),
            summary_text: None,
            last_error: None,
        });
        state.factories.push(FactoryRun {
            id: "run-1".to_string(),
            status: "planning".to_string(),
            updated_at: "2026-04-18T00:00:00Z".to_string(),
            source_brief: None,
            current_stage: Some("planning".to_string()),
            active_room_ids: Vec::new(),
        });
        state.reconcile_selection();
        assert_eq!(
            state.selection,
            Some(Selection::Factory("run-1".to_string()))
        );
        state.move_selection(1);

        assert_eq!(state.selection, Some(Selection::Room("room-1".to_string())));
    }
}
