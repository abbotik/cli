# Abbot CLI

`abbot` is the command-line client for the Abbotik API.
By default it talks to the public API at `https://api.abbotik.com`.

It is organized as a command tree with a small set of root surfaces and a
larger set of resource-specific branches:

- `public` for unauthenticated discovery documents
- `auth` for human login, machine auth bootstrap, token refresh, and tenant selection
- `health` for a quick service check
- `docs` for direct API documentation access
- `describe`, `data`, `find`, `aggregate`, and `bulk` for model and record work
- `acls`, `stat`, `tracked`, and `trashed` for record metadata and lifecycle
- `user` for account and sudo workflows
- `keys` for tenant-bound machine credentials
- `cron` for scheduled process management
- `fs` for tenant filesystem access
- `app` for dynamic application paths

For first-time use, the shortest onboarding path is usually:

1. `abbot auth register --tenant <tenant> --username <user> --email <email> --password <password>`
2. `abbot auth login --tenant <tenant> --username <user> --password <password>` on later runs or other machines
3. `abbot public llms` or `abbot docs root`
4. `abbot describe list` or `abbot data list <model>`
5. `abbot health`

For existing tenants, the invite path is:

1. `abbot user invite --username <user> --invite-type human|machine`
2. `abbot auth register --tenant <tenant> --username <user> --invite-code <code> --email <email> --password <password>`
3. or `abbot auth provision --tenant <tenant> --username <user> --invite-code <code> --public-key @machine.pub`

The CLI prefers structured output and explicit subcommands so it can be driven by
scripts or agents without guessing hidden state.
