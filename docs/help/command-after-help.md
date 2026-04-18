# Command tree notes

The root command splits into focused branches instead of flattening everything
into one long list.

If you are an agent or long-running client, start with `abbot public llms` and
`abbot docs path /docs/auth`, then prefer `abbot auth machine connect` and
`abbot keys create` before exploring protected routes.

If you are doing human bootstrap, `abbot auth register` still creates a tenant
and local session. The CLI follows registration with login because the server no
longer mints a JWT directly from `/auth/register`.

If you are joining an existing tenant, have a root or full user mint a one-time
invite with `abbot user invite`, then redeem it with `abbot auth register` or
`abbot auth provision` using `--invite-code`.

Use this rough map when navigating the CLI:

- `public` for root documents and agent-facing discovery
- `auth` for machine-first bootstrap, human login, refresh, and tenant state
- `docs` for exact router-shaped API docs lookups
- `describe` for schema and field metadata
- `data` for CRUD and relationship traversal
- `find`, `aggregate`, and `bulk` for queries and batch work
- `acls`, `stat`, `tracked`, and `trashed` for record state and lifecycle views
- `user` for account operations, machine-key management, introspection, and elevated actions
- `keys` for durable self-service bearer API keys
- `llm` for rooms, factories, providers, and skills
- `cron` for scheduled workflows
- `fs` for file content and metadata
- `app` for app-specific path forwarding

Each branch usually accepts either a collection-style command such as `list` or a
resource-oriented command such as `get`, `create`, `update`, or `delete`.
