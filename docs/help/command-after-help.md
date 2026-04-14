# Command tree notes

The root command splits into focused branches instead of flattening everything
into one long list.

If you are new to Monk, start with `monk auth register` to create a tenant and
local session. The CLI now follows registration with login because the server no
longer mints a JWT directly from `/auth/register`.

Use this rough map when navigating the CLI:

- `public` for root documents and agent-facing discovery
- `auth` for human login, machine bootstrap, refresh, and tenant state
- `docs` for API docs lookups
- `describe` for schema and field metadata
- `data` for CRUD and relationship traversal
- `find`, `aggregate`, and `bulk` for queries and batch work
- `acls`, `stat`, `tracked`, and `trashed` for record state and audit-like views
- `user` for account operations and elevated actions
- `keys` for tenant-bound machine credentials
- `cron` for scheduled workflows
- `fs` for file content and metadata
- `app` for app-specific path forwarding

Each branch usually accepts either a collection-style command such as `list` or a
resource-oriented command such as `get`, `create`, `update`, or `delete`.
