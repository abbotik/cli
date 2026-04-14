# Docs

Fetch Abbotik API documentation by path.

Useful entry points from the API docs surface:

- `/docs` for API discovery
- `/docs/auth` for authentication and tenant provisioning
- `/docs/api/data` for CRUD operations
- `/docs/api/describe` for model and field metadata
- `/docs/api/find`, `/docs/api/aggregate`, and `/docs/api/bulk` for query and batch work
- `/docs/api/tracked` and `/docs/api/trashed` for change tracking and lifecycle flows
- `/docs/api/cron` and `/docs/fs` for scheduled jobs and filesystem access

Common uses:

- `abbot docs root`
- `abbot docs path /docs`
- `abbot docs path /docs/api/data`

Use `--help` on `root` or `path` for the next level down.
