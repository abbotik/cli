# Root-level usage notes

By default, `abbot` talks to the public Abbotik API at `https://api.abbotik.com`.

Examples:

```bash
abbot public llms
abbot config
abbot doctor
abbot update
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
abbot public llms
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

Useful onboarding sequence for agents and long-running clients:

```bash
abbot public llms
abbot docs path /docs/auth
abbot auth machine connect --tenant acme --username machine_root --key @~/.config/secrets/machine.key
abbot tui
abbot keys create --name ci-runner
abbot docs path /docs/api/keys
```

Useful onboarding sequence for new human users:

```bash
abbot auth register --tenant acme --username alice --email alice@example.com --password secret-pass
abbot public llms
abbot health
abbot describe list
abbot data list <model>
```

Useful onboarding sequence for invited tenant users:

```bash
abbot user invite --username alice --invite-type human
abbot auth register --tenant acme --username alice --invite-code <code> --email alice@example.com --password secret-pass
abbot auth login --tenant acme --username alice --password secret-pass
```

The global `--format` flag currently accepts `json` only.

Machine-readable output is available with `--format json`:

```bash
abbot --format json auth login --tenant acme --username alice --password secret-pass
abbot --format json describe list
```

If you are automating against Abbotik, start from `public llms` and the `docs`
branch, then move down into the resource branch you need.
