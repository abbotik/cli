# Auth

Authenticate to API hosts, switch saved hosts, register tenants, bootstrap
machine keys, and inspect tenant state.

This branch covers both the explicit bootstrap flows and the normal login path.
Credentials are stored per API host. `abbot auth login [host]` uses
`https://api.abbotik.com` when `[host]` is omitted, and a successful login makes
that host the default for later commands.

If you are an agent or long-running client, start with machine auth or bearer
API keys, not the username/password path:

- `abbot auth machine connect --tenant acme --username machine_root --key @~/.config/secrets/machine.key`
- `abbot auth provision --tenant acme --username machine_root --public-key @machine.pub`
- `abbot auth verify --tenant acme --challenge-id <id> --signature @signature.txt`
- `abbot api keys create --name ci-runner`

For human bootstrap, start with `abbot auth register`. The CLI follows the
updated API contract by calling `/auth/register` and then `/auth/login` so the
initial run still ends with a saved local session.

For existing tenants, root or full users can mint one-time invite codes with
`abbot api user invite`, then the invited human or machine can redeem with
`abbot auth register --invite-code ...`, `abbot auth provision --invite-code ...`,
or `abbot auth machine connect --invite-code ...`.

Common uses:

- `abbot docs path /llms.txt`
- `abbot docs path /docs/auth`
- `abbot auth machine connect --tenant acme --username machine_root --key @~/.config/secrets/machine.key`
- `abbot auth register --tenant acme --username alice --email alice@example.com --password @auth-password.txt`
- `abbot auth login http://localhost:3000 --tenant acme --username alice --password @auth-password.txt`
- `abbot auth list`
- `abbot auth use http://localhost:3000`
- `abbot auth logout http://localhost:3000`
- `abbot api user invite --username alice --invite-type human`
- `abbot auth register --tenant acme --username alice --invite-code <code> --email alice@example.com --password @auth-password.txt`
- `abbot auth provision --tenant acme --username machine_root --public-key @machine.pub`
- `abbot auth provision --tenant acme --username builder_2 --invite-code <code> --public-key @machine.pub`
- `abbot auth challenge --tenant acme --fingerprint fp_1234abcd`
- `abbot auth verify --tenant acme --challenge-id <id> --signature @signature.txt`
- `printf 'secret-pass' | abbot auth login --tenant acme --username alice --password -`
- `abbot auth refresh`
- `abbot auth dissolve request --tenant acme --username alice --password @secret.txt`
- `abbot auth dissolve confirm --confirmation-token <token>`
- `abbot auth token get`
- `abbot auth token clear`
- `abbot auth tenants`

Use `--help` on `login`, `use`, `list`, `logout`, `register`, `machine`,
`provision`, `challenge`, `verify`, `refresh`, `dissolve`, `token`, or
`tenants` for the next level down.
