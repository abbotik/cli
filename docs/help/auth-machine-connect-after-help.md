# Auth Machine Connect

Connect a machine key with the shortest viable flow.

`abbot` will:

- reuse saved machine key metadata and run challenge -> sign -> verify when it can
- otherwise provision the machine key, sign the returned nonce, and verify it
- save the resulting bearer token and machine-auth config locally

Common uses:

- `abbot auth machine connect --tenant acme --username machine_root --key @~/.config/secrets/machine.key`
- `abbot auth machine connect --tenant acme --key @~/.config/secrets/machine.key`

Notes:

- `--username` is required the first time, when no saved machine key metadata exists yet
- `--key` should point at a private key PEM path; plain paths and `@<path>` both work
- `--public-key` is optional; if omitted, `abbot` derives it from the private key
