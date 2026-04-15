# Auth Provision

Bootstrap machine auth for a brand-new tenant, or redeem a machine invite for
an existing tenant, by sending the first bound public key to Abbotik. This
returns a challenge, not a bearer token.

Common use:

- `abbot auth provision --tenant acme --username machine_root --public-key @machine.pub`
- `abbot auth provision --tenant acme --username machine_root --public-key @machine.pub --save-private-key-path ~/.config/secrets/machine.key`
- `abbot auth provision --tenant acme --username builder_2 --invite-code <code> --public-key @machine.pub`

If `--public-key` uses `@<path>`, `abbot` can infer and save that public-key path.
Use `--save-public-key-path` when the key source did not come from a file path.

Use `abbot auth verify` after signing the returned nonce.
