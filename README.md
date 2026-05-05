# abbot

`abbot` is the Rust operator CLI for Abbotik.
By default it talks to the public API at `https://api.abbotik.com`.

The CLI stores credentials per API host under `~/.config/abbot/cli/hosts/`.
`abbot auth login [host]` logs in to one server and makes that server the
default for later commands. Omit `[host]` to use `https://api.abbotik.com`.

Core human-first commands:

- `abbot auth login [host]` to authenticate to a public, local, or internal API host
- `abbot auth list` and `abbot auth use <host>` to inspect and switch saved hosts
- `abbot doctor` to check the live server connection, health, and auth state
- `abbot update` to refresh the current CLI binary using the install method it detects
- `abbot guide <path...>` to print the embedded markdown doc for a command path
- `abbot api <name>` for route-shaped `/api/<name>` families
- `abbot mcp list` and `abbot mcp call` for MCP tool workflows

## New tenant bootstrap

For a brand-new tenant, the CLI supports the machine-auth bootstrap flow
directly. The normal sequence is:

1. Create or choose a tenant slug that matches the server rules:
   - lowercase letters, numbers, and underscores only
   - must start with a lowercase letter
2. Generate a private/public keypair for the machine user.
3. Run `abbot auth machine connect` against the target host:

```bash
abbot --host http://127.0.0.1:9100 auth machine connect \
  --tenant my_new_tenant \
  --username machine_root \
  --key /path/to/machine.key
```

What this does:

- provisions the machine user when the tenant is new
- requests the signing challenge
- signs the challenge with the private key
- verifies the signature
- saves the resulting bearer token and machine-auth metadata in the local host
  config

For an existing machine user, the same command can reconnect using saved
metadata or an invite code:

```bash
abbot --host http://127.0.0.1:9100 auth machine connect \
  --tenant existing_tenant \
  --username machine_root \
  --key /path/to/machine.key

abbot --host http://127.0.0.1:9100 auth machine connect \
  --tenant existing_tenant \
  --username builder_2 \
  --invite-code <code> \
  --key /path/to/builder_2.key
```

Useful follow-up checks:

- `abbot auth list`
- `abbot api user me`
- `abbot auth refresh` for later token refreshes

## Release and install

The release process is tag-based and publishes GitHub Release assets for:

- `x86_64-unknown-linux-gnu`
- `aarch64-unknown-linux-gnu`
- `x86_64-apple-darwin`
- `aarch64-apple-darwin`
- Homebrew artifacts mirrored to `abbotik/homebrew-releases`
- Homebrew formula published in `abbotik/tap`

### Curl install

Install from the latest GitHub release:

```bash
curl -fsSL https://raw.githubusercontent.com/abbotik/cli/main/scripts/install.sh | bash
```

To pin a version:

```bash
ABBOTIK_CLI_VERSION=v2.0.0 \
  curl -fsSL https://raw.githubusercontent.com/abbotik/cli/main/scripts/install.sh | bash
```

### Homebrew install

```bash
brew install abbotik/tap/abbot
```

Optional install directory:

```bash
ABBOTIK_CLI_INSTALL_DIR="$HOME/bin" \
  curl -fsSL https://raw.githubusercontent.com/abbotik/cli/main/scripts/install.sh | bash
```

Once installed, `abbot update` distinguishes the current binary's install path:

- Homebrew installs run `brew upgrade abbotik/tap/abbot`
- curl installs pull the latest GitHub release asset and replace the current binary in place

Extra update modes:

- `abbot update --version-list` shows published release versions without installing
- `abbot update --version v1.7.0` installs that exact published release for curl-installed binaries

Exact version installs are not supported for Homebrew-managed binaries because the
tap formula tracks the current release.

## Current state

The v2 CLI has a clean top-level surface. Product workflows stay at the root,
while route-shaped API families live under `abbot api`.

Registered `/api/<name>` families:

```text
acls aggregate bulk cron data describe find keys stat tracked trashed user
```

For a new human user, the intended first steps are:

```bash
abbot auth login --tenant acme --username alice --password secret
abbot api data list rooms
abbot tui
```

For multiple API servers, log in to each host once:

```bash
abbot auth login http://localhost:3000 --tenant acme --username alice --password secret
abbot auth login http://192.168.1.50:3000 --tenant acme --username alice --password secret
abbot auth list
abbot auth use http://localhost:3000
abbot --host http://192.168.1.50:3000 api data list rooms
```

Each host keeps its own saved token, output format, and machine-auth metadata.
The older `--config <name>` profile path remains available for scripting and
debugging, but host credentials are the normal user surface.

For a new user, the intended first steps are:

1. `abbot auth register --tenant <tenant> --username <user> --email <email> --password <password>`
2. `abbot auth login --tenant <tenant> --username <user> --password <password>` if you need a fresh session later
3. `abbot docs path /llms.txt` or `abbot docs root`
4. `abbot doctor`

`abbot auth register` now follows the API contract change by registering first and
then immediately completing `/auth/login` so the CLI still lands with a saved JWT.

For users joining an existing tenant, the invite flow is:

1. Existing root or full user runs `abbot api user invite --username <user> --invite-type human|machine`
2. Human invitees run `abbot auth register --tenant <tenant> --username <user> --invite-code <code> --email <email> --password <password>`
3. Machine invitees run `abbot auth provision --tenant <tenant> --username <user> --invite-code <code> --public-key @~/.config/abbot/abbotik.pub`
4. Invited humans can later use `abbot auth login`; invited machines finish with `abbot auth verify` or `abbot auth machine connect`

Machine clients should use:

1. `abbot auth machine connect --tenant <tenant> --username <user> --key ~/.config/abbot/abbotik.key`
2. `abbot auth refresh` when the saved machine token expires; `abbot` auto-detects public-key auth and runs challenge→sign→verify using the saved key path
3. `abbot api user machine-keys list`

User-scoped provider secrets should use:

```bash
abbot api user secrets create \
  --name openrouter_primary \
  --value @~/.config/secrets/openrouter.key \
  --kind api_key \
  --metadata '{"provider":"openrouter"}'

abbot api user secrets list
abbot api user secrets update openrouter_primary --value @~/.config/secrets/openrouter.key
abbot api user secrets delete openrouter_primary
```

The API encrypts secret values at rest and CLI list/delete responses only show
metadata returned by `/api/user/secrets`; plaintext is sent only on create or
update.

## API and MCP

Use `abbot api` for exact `/api/<name>` route families:

```bash
abbot api describe list
abbot api data list users
abbot api keys create --name ci-runner
abbot api user introspect
```

Use `abbot mcp` for MCP concepts:

```bash
abbot mcp list
abbot mcp call abbot_data --arguments '{"action":"list","model":"rooms"}'
```

## Factory

Use `abbot factory` for high-level durable workflow operations:

```bash
abbot factory submit --prompt "create me a marketing plan for an iPhone app"
abbot factory submit --plan ./plan.md --workflow software.delivery --subject repo:abbotik/api
abbot factory status <run_id>
abbot factory watch <run_id>
abbot factory watch <run_id> --timeout 1800 --until completed
abbot factory review <run_id>
```

The lower-level `abbot llm factory` branch remains available for route-shaped
debugging and manual state authoring.

## TUI

`abbot tui` opens a terminal operator console over the real Abbotik room and
factory APIs.

Current v2 behavior:

- reuses the saved CLI base URL and bearer token
- shows grouped room and factory rails
- loads room history from `/llm/room/:id/history`
- sends room turns through `/v1/responses`
- shows read-only factory overview, stage, issue, checkpoint, artifact, and review views

Typical flow:

```bash
abbot auth login --tenant acme --username alice --password secret-pass
abbot tui
```

Low-level machine-auth commands still exist for manual control:

1. `abbot auth provision --tenant <tenant> --username <user> --public-key @~/.config/abbot/abbotik.pub --save-private-key-path ~/.config/abbot/abbotik.key`
2. Sign the returned nonce with the matching private key
3. `abbot auth verify --tenant <tenant> --challenge-id <id> --signature @signature.txt`
