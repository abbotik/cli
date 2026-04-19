# Data

Work with model records and nested relationships.

This is the main CRUD branch, and it maps directly onto `/api/data/:model` and
related record paths.

Common uses:

- `abbot data list users`
- `abbot data get users 123`
- `abbot data patch users`
- `abbot data relationship users 123 posts get`

Use `--help` on a subcommand for the next level down.
