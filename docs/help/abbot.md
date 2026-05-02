# abbot

`abbot` is the operator CLI for Abbotik.
By default it talks to `https://api.abbotik.com`.

The v2 command surface is intentionally split into product workflows, protocol
surfaces, and route-shaped API access.

Top-level commands:

- `auth` for host login, host switching, registration, machine auth, token, tenant, and dissolve flows
- `config` for legacy/profile-level local config management
- `doctor` for active-host diagnostics and connection checks
- `factory` for high-level durable Factory workflows
- `tui` for the terminal operator console
- `update` for CLI self-update
- `api` for `/api/<name>` route families
- `llm` for `/llm/...` route-shaped LLM room, factory, provider, model, and skill surfaces
- `mcp` for MCP tool listing and tool calls
- `docs` for live API documentation lookup
- `guide` for embedded CLI guide pages

If you are an agent or long-running client, the shortest reliable path is:

1. `abbot docs path /llms.txt`
2. `abbot docs path /docs/auth`
3. `abbot auth machine connect --tenant <tenant> --username <user> --key @~/.config/secrets/machine.key`
4. `abbot api keys create --name agent-token`
5. `abbot docs path /docs/api/keys`

For first-time human bootstrap, use:

1. `abbot auth register --tenant <tenant> --username <user> --email <email> --password <password>`
2. `abbot auth login --tenant <tenant> --username <user> --password <password>` on later runs or other machines
3. `abbot doctor`
4. `abbot tui` or `abbot factory submit --prompt "..."`

To work with multiple API servers, log in per host:

```bash
abbot auth login http://localhost:3000 --tenant acme --username alice --password secret-pass
abbot auth login http://192.168.1.50:3000 --tenant acme --username alice --password secret-pass
abbot auth list
abbot auth use http://localhost:3000
abbot --host http://192.168.1.50:3000 api data list rooms
```

For existing tenants, the invite path is:

1. `abbot api user invite --username <user> --invite-type human|machine`
2. human: `abbot auth register --tenant <tenant> --username <user> --invite-code <code> --email <email> --password <password>`
3. machine: `abbot auth machine connect --tenant <tenant> --username <user> --invite-code <code> --key @~/.config/secrets/<user>.key`

Route-shaped API commands live under `abbot api`:

```bash
abbot api describe list
abbot api data list users
abbot api keys list
abbot api user machine-keys list
abbot api find query users --where '{"active":true}'
abbot api aggregate run users --count
abbot api bulk export
```

MCP commands use MCP concepts:

```bash
abbot mcp list
abbot mcp call abbot_data --arguments '{"action":"list","model":"rooms"}'
```

Machine-readable output is available with `--format json`:

```bash
abbot --format json auth login --tenant acme --username alice --password secret-pass
abbot --format json api describe list
```
