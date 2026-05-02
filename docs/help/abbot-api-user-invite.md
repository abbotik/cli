# User Invite

Mint a one-time invite code for a future tenant-local user.

This command calls `POST /api/user/invite` and requires a saved bearer token
with `root` or `full` access.

Common uses:

- `abbot api user invite --username alice --invite-type human`
- `abbot api user invite --username builder_2 --invite-type machine --access edit`
- `abbot api user invite --username analyst --invite-type either --expires-in 3600`

The returned code is only shown once. Pass it to the invited user for:

- `abbot auth register --tenant <tenant> --username <user> --invite-code <code> --email <email> --password <password>`
- `abbot auth provision --tenant <tenant> --username <user> --invite-code <code> --public-key @machine.pub`
