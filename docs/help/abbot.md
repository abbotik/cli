# abbot

`abbot` is the command-line client for the Abbotik API.
By default it talks to the public API at `https://api.abbotik.com`.

It is organized as a command tree with a small core set of root surfaces and a
larger set of secondary branches.

Core surfaces:

- `auth` for machine-first bootstrap, human login, token refresh, and tenant selection
- `config` for local profile management and switching
- `data` for model record operations
- `describe` for model metadata and schema management

Other root branches:

- `acls` for record ACL management
- `aggregate` for aggregate operations
- `app` for dynamic application paths
- `bulk` for multi-operation transactions
- `command` for embedded markdown docs about a command path
- `cron` for scheduled process management
- `docs` for direct router-shaped API documentation access
- `doctor` for active-profile diagnostics and connection checks
- `find` for advanced query operations
- `fs` for tenant filesystem access
- `factory` for high-level durable workflow operations
- `health` for a quick service check
- `keys` for durable self-service bearer API keys
- `llm` for rooms, factory runs, and provider discovery
- `public` for unauthenticated discovery documents
- `stat`, `tracked`, and `trashed` for record metadata and lifecycle
- `tui` for the terminal operator console
- `update` for CLI self-update flows
- `user` for account, machine-key, introspection, and sudo workflows

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

Examples:

```bash
abbot public llms
abbot config
abbot doctor
abbot update
abbot command auth machine
abbot docs path /docs/auth
abbot tui
abbot auth machine connect --tenant acme --username machine_root --key @~/.config/secrets/machine.key
abbot keys create --name ci-runner
abbot docs path /docs/api/keys
abbot docs path /docs/api/user/machine-keys
abbot docs path /docs/llm/room
abbot docs path /docs/llm/factory
abbot auth register --tenant acme --username alice --email alice@example.com --password secret-pass
abbot auth login --tenant acme --username alice --password secret-pass
abbot user invite --username alice --invite-type human
abbot auth register --tenant acme --username alice --invite-code <code> --email alice@example.com --password secret-pass
abbot describe list
abbot data list users
abbot data get users 123
abbot auth provision --tenant acme --username machine_root --public-key @machine.pub
abbot auth provision --tenant acme --username builder_2 --invite-code <code> --public-key @machine.pub
abbot auth verify --tenant acme --challenge-id <id> --signature @signature.txt
abbot keys list
abbot user machine-keys list
abbot find query users --where '{"active":true}'
abbot aggregate run users --count
abbot bulk export
abbot fs get /docs/README.md
```

The global `--format` flag currently accepts `json` only.

Machine-readable output is available with `--format json`:

```bash
abbot --format json auth login --tenant acme --username alice --password secret-pass
abbot --format json describe list
```

If you are automating against Abbotik, start from `public llms` and the `docs`
branch, then move down into the resource branch you need.
