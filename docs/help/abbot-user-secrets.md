# User Secrets

Manage encrypted user-scoped secrets through `/api/user/secrets`.

Secrets are owned by the authenticated user inside the current tenant. Create
and update send plaintext values to the API; list and delete responses return
only metadata.

Common uses:

- `abbot user secrets list`
- `abbot user secrets create --name openrouter_primary --value @~/.config/secrets/openrouter.key --kind api_key --metadata '{"provider":"openrouter"}'`
- `abbot user secrets update openrouter_primary --value @rotated.key`
- `abbot user secrets delete openrouter_primary`

Use `--body` or stdin when you need the exact API JSON body.
