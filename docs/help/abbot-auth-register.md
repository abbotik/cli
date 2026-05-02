# Auth Register

Create a new tenant identity, or redeem a tenant invite, then complete the
required follow-up login.

Use this for initial bootstrap when you do not already have an account or saved
state on the current machine.

Typical onboarding flow:

1. `abbot auth register --tenant <tenant> --username <user> --email <email> --password <password>`
2. `abbot auth login --tenant <tenant> --username <user> --password <password>` on later runs or other machines
3. continue with `abbot docs path /llms.txt`, `abbot api describe list`, or `abbot api data list <model>`

For existing tenants, redeem an invite instead:

1. `abbot api user invite --username alice --invite-type human`
2. `abbot auth register --tenant acme --username alice --invite-code <code> --email alice@example.com --password @auth-password.txt`

The server no longer returns a JWT directly from `/auth/register`, so this
command immediately performs `/auth/login` after successful registration or
invite redemption and saves the returned token.

For `--password`, use `-` to read from stdin or `@<path>` to read from a file.

This branch is intentionally concise; use the command itself for the full flag list.
