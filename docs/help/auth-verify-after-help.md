# Auth Verify

Verify a signed machine-auth challenge and mint an Abbotik bearer token.

Common use:

- `abbot auth verify --tenant acme --challenge-id <uuid> --signature @signature.txt`

Successful verification saves the returned token into the local config.
