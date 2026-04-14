# Auth

Authenticate, register, bootstrap machine keys, and inspect tenant state.

This branch covers both the explicit bootstrap flows and the normal login path.

For new users, start with `abbot auth register`. The CLI follows the updated API
contract by calling `/auth/register` and then `/auth/login` so the initial run
still ends with a saved local session.

Common uses:

- `abbot auth register --tenant acme --username alice --email alice@example.com --password @auth-password.txt`
- `abbot auth provision --tenant acme --username machine_root --public-key @machine.pub`
- `abbot auth challenge --tenant acme --fingerprint fp_1234abcd`
- `abbot auth verify --tenant acme --challenge-id <id> --signature @signature.txt`
- `printf 'secret-pass' | abbot auth login --tenant acme --username alice --password -`
- `abbot auth refresh`
- `abbot auth dissolve request --tenant acme --username alice --password @secret.txt`
- `abbot auth dissolve confirm --confirmation-token <token>`
- `abbot auth token get`
- `abbot auth token clear`
- `abbot auth tenants`

Use `--help` on `login`, `register`, `provision`, `challenge`, `verify`,
`refresh`, `dissolve`, `token`, or `tenants` for the next level down.
