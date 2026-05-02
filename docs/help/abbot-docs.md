# Docs

Fetch Abbotik API documentation by path.

Useful entry points from the API docs surface:

- `/docs` for API discovery
- `/docs/api/keys` for durable bearer API keys
- `/docs/api/user/machine-keys` for tenant-bound public-key rotation
- `/docs/auth` for authentication and tenant provisioning
- `/docs/api/data` for CRUD operations
- `/docs/api/describe` for model and field metadata
- `/docs/api/find`, `/docs/api/aggregate`, and `/docs/api/bulk` for query and batch work
- `/docs/llm/room` and `/docs/llm/factory` for the `/llm/*` protocol contract
- `/docs/api/tracked` and `/docs/api/trashed` for change tracking and lifecycle flows
- `/docs/api/cron` for scheduled jobs

Common uses:

- `abbot docs root`
- `abbot docs path /docs`
- `abbot docs path /docs/auth`
- `abbot docs path /docs/api/keys`
- `abbot docs path /docs/api/user/machine-keys`
- `abbot docs path /docs/api/data`
- `abbot docs path /docs/llm/room`

Use `--help` on `root` or `path` for the next level down.
