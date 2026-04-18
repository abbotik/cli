# Abbot CLI

`abbot` is the command-line client for the Abbotik API.
By default it talks to the public API at `https://api.abbotik.com`.

It is organized as a command tree with a small set of root surfaces and a
larger set of resource-specific branches:

- `public` for unauthenticated discovery documents
- `auth` for machine-first bootstrap, human login, token refresh, and tenant selection
- `health` for a quick service check
- `docs` for direct router-shaped API documentation access
- `describe`, `data`, `find`, `aggregate`, and `bulk` for model and record work
- `acls`, `stat`, `tracked`, and `trashed` for record metadata and lifecycle
- `user` for account, machine-key, introspection, and sudo workflows
- `keys` for durable self-service bearer API keys
- `cron` for scheduled process management
- `fs` for tenant filesystem access
- `app` for dynamic application paths

If you are an agent or long-running client, the shortest reliable path is:

1. `abbot public llms`
2. `abbot docs path /docs/auth`
3. `abbot auth machine connect --tenant <tenant> --username <user> --key @~/.config/secrets/machine.key`
4. `abbot keys create --name agent-token`
5. `abbot docs path /docs/api/keys`

The API docs recommend this machine-first path because bearer token minting has
to work before the protected surfaces are reliable.

For first-time human bootstrap, use:

1. `abbot auth register --tenant <tenant> --username <user> --email <email> --password <password>`
2. `abbot auth login --tenant <tenant> --username <user> --password <password>` on later runs or other machines
3. `abbot public llms` or `abbot docs root`
4. `abbot describe list` or `abbot data list <model>`

For existing tenants, the invite path is:

1. `abbot user invite --username <user> --invite-type human|machine`
2. human: `abbot auth register --tenant <tenant> --username <user> --invite-code <code> --email <email> --password <password>`
3. machine: `abbot auth machine connect --tenant <tenant> --username <user> --invite-code <code> --key @~/.config/secrets/<user>.key`

Use `abbot docs path /docs/api/keys`, `abbot docs path /docs/api/user/machine-keys`,
`abbot docs path /docs/llm/room`, and `abbot docs path /docs/llm/factory` when
you need the exact HTTP contract behind a CLI branch.

The CLI prefers explicit subcommands and documented route parity so it can be
driven by scripts or agents without guessing hidden state.
