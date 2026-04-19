# Config

Manage named CLI config profiles and inspect local config state.

Common commands:

- `abbot config create <name> [url]`
- `abbot config use <name>`
- `abbot config list`
- `abbot config show <name>`
- `abbot config set <name> <key> <value>`
- `abbot config set <name> <key> --unset`
- `abbot config get <name> <key>`
- `abbot config delete <name>`
- `abbot config doctor`

Use `abbot config doctor` for local config integrity checks only.

Use top-level `abbot doctor` when you want live connection, health, and auth
status against the currently configured server.
