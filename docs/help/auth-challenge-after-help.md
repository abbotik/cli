# Auth Challenge

Request a short-lived single-use challenge for an existing tenant-bound machine
key.

Common use:

- `monk auth challenge --tenant acme --fingerprint fp_1234abcd`
- `monk auth challenge --tenant acme --key-id <uuid>`

Use `monk auth verify` after signing the returned nonce.
