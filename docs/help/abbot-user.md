# User

Manage user accounts, invites, machine keys, impersonation, and sudo flows.

This branch covers the current identity, invite creation, user lists,
machine-key management, trusted bearer-token introspection, and elevated operations.

Common uses:

- `abbot user me`
- `abbot user introspect`
- `abbot user machine-keys list`
- `abbot user invite --username alice --invite-type human`
- `abbot user list`
- `abbot user sudo`

Use `abbot keys` for durable global bearer API keys. Use
`abbot user machine-keys` only when you need tenant-bound public-key management
and rotation for machine identities.

Use `--help` on a subcommand for the next level down.
