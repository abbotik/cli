# TUI

`abbot tui` opens the operator console for room and factory workflows.

It reuses the saved CLI config for:

- base URL
- bearer token
- default tenant context

The TUI is a thin client over the API:

- rooms list from `/llm/room`
- room history from `/llm/room/:id/history`
- room send/create via `/v1/responses`
- factory views from `/llm/factory/runs/*`

Current controls:

- `q` quit
- `Ctrl-N` start a draft room
- `Ctrl-R` refresh rooms, factories, and the active detail pane
- `Tab` switch between sidebar and composer
- `1`-`6` switch factory subviews
- `p` toggle room pinning locally

For v1, login still happens outside the TUI:

```bash
abbot auth login --tenant <tenant> --username <user> --password <password>
abbot tui
```
