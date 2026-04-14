# Auth Provision

Bootstrap machine auth for a brand-new tenant by sending the first bound public
key to Monk. This returns a challenge, not a bearer token.

Common use:

- `monk auth provision --tenant acme --username machine_root --public-key @machine.pub`

Use `monk auth verify` after signing the returned nonce.
