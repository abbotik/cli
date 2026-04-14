# Auth Dissolve

Request a short-lived confirmation token for the two-step tenant dissolution flow.

Use this when you want to verify the credentials first, then pass the returned
token into `abbot auth dissolve confirm`.

Common uses:

- `abbot auth dissolve request --tenant acme --username alice --password @secret.txt`
- `printf 'secret-pass' | abbot auth dissolve request --tenant acme --username alice --password -`

Use `--help` on `request` or `confirm` for the next level down.
