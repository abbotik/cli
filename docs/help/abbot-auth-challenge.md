# Auth Challenge

Request a short-lived single-use challenge for an existing tenant-bound machine
key.

Common use:

- `abbot auth challenge --tenant acme --fingerprint fp_1234abcd`
- `abbot auth challenge --tenant acme --key-id <uuid>`

Use `abbot auth verify` after signing the returned nonce.
