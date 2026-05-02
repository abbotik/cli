# User

Manage user accounts, invites, machine keys, secrets, impersonation, and sudo flows.

This branch covers the current identity, invite creation, user lists,
machine-key management, user-scoped encrypted secrets, trusted bearer-token
introspection, and elevated operations.

Common uses:

- `abbot api user me`
- `abbot api user introspect`
- `abbot api user machine-keys list`
- `abbot api user secrets list`
- `abbot api user invite --username alice --invite-type human`
- `abbot api user list`
- `abbot api user sudo`

Use `abbot api keys` for durable global bearer API keys. Use
`abbot api user machine-keys` only when you need tenant-bound public-key management
and rotation for machine identities.

Use `--help` on a subcommand for the next level down.
