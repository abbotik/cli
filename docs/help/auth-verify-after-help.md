# Auth Verify

Verify a signed machine-auth challenge and mint an Abbotik bearer token.

Common use:

- `abbot auth verify --tenant acme --challenge-id <uuid> --signature @signature.txt`
- `abbot auth verify --tenant acme --challenge-id <uuid> --signature @signature.txt --save-private-key-path ~/.config/secrets/machine.key`

Successful verification saves the returned token into the local config.
When `--save-public-key-path` or `--save-private-key-path` is provided, `abbot`
also persists those key paths for future machine refresh.
