# Root-level usage notes

By default, `monk` talks to the public Monk API at `https://monk-api.com`.

Examples:

```bash
monk auth register --tenant acme --username alice --email alice@example.com --password secret-pass
monk auth login --tenant acme --username alice --password secret-pass
monk public llms
monk describe list
monk data list users
monk data get users 123
monk auth provision --tenant acme --username machine_root --public-key @machine.pub
monk auth verify --tenant acme --challenge-id <id> --signature @signature.txt
monk keys list
monk find query users --where '{"active":true}'
monk aggregate run users --count
monk bulk export
monk fs get /docs/README.md
```

Useful onboarding sequence for new users:

```bash
monk auth register --tenant acme --username alice --email alice@example.com --password secret-pass
monk public llms
monk health
monk describe list
monk data list <model>
```

Machine-readable output is available with `--format json`:

```bash
monk --format json auth login --tenant acme --username alice --password secret-pass
monk --format json describe list
```

If you are automating against Monk, start from the root command tree and then
move down into the resource branch you need.
