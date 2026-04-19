# Auth Machine

Machine-auth happy-path commands.

Use this branch when you want `abbot` to handle the provision or reconnect
flow instead of manually calling `challenge` and `verify`.

Common use:

- `abbot auth machine connect --tenant acme --username machine_root --key @~/.config/secrets/machine.key`

Use `--help` on `connect` for the next level down.
