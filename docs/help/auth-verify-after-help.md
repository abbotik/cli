# Auth Verify

Verify a signed machine-auth challenge and mint a Monk bearer token.

Common use:

- `monk auth verify --tenant acme --challenge-id <uuid> --signature @signature.txt`

Successful verification saves the returned token into the local config.
