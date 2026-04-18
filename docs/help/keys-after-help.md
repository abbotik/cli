# Keys

Manage self-service bearer API keys through `/api/keys`.

Use this branch for durable user-owned service tokens.

The API docs treat `/api/keys` as the preferred bearer-token surface for
long-running clients once initial auth is working.

Common uses:

- `abbot keys list`
- `abbot keys create --name ci-runner --expires-at 2026-12-31T23:59:59Z`
- `abbot keys delete <key_id>`
- `abbot keys revoke-all`

For the exact HTTP contract, read `abbot docs path /docs/api/keys`.
