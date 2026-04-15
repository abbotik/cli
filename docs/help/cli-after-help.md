# Root-level usage notes

By default, `abbot` talks to the public Abbotik API at `https://api.abbotik.com`.

Examples:

```bash
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
abbot find query users --where '{"active":true}'
abbot aggregate run users --count
abbot bulk export
abbot fs get /docs/README.md
```

Useful onboarding sequence for new users:

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

Machine-readable output is available with `--format json`:

```bash
abbot --format json auth login --tenant acme --username alice --password secret-pass
abbot --format json describe list
```

If you are automating against Abbotik, start from the root command tree and then
move down into the resource branch you need.
