# Auth

Authenticate, register, bootstrap machine keys, and inspect tenant state.

This branch covers both the explicit bootstrap flows and the normal login path.

For new users, start with `monk auth register`. The CLI follows the updated API
contract by calling `/auth/register` and then `/auth/login` so the initial run
still ends with a saved local session.

Common uses:

- `monk auth register --tenant acme --username alice --email alice@example.com --password @auth-password.txt`
- `monk auth provision --tenant acme --username machine_root --public-key @machine.pub`
- `monk auth challenge --tenant acme --fingerprint fp_1234abcd`
- `monk auth verify --tenant acme --challenge-id <id> --signature @signature.txt`
- `printf 'secret-pass' | monk auth login --tenant acme --username alice --password -`
- `monk auth refresh`
- `monk auth dissolve request --tenant acme --username alice --password @secret.txt`
- `monk auth dissolve confirm --confirmation-token <token>`
- `monk auth token get`
- `monk auth token clear`
- `monk auth tenants`

Use `--help` on `login`, `register`, `provision`, `challenge`, `verify`,
`refresh`, `dissolve`, `token`, or `tenants` for the next level down.
