# abbot

`abbot` is the Rust CLI for the Abbotik API.
By default it talks to the public API at `https://api.abbotik.com`.

The CLI stores its default config at `~/.config/abbot/cli/config.toml`. Pass `--config <name>` to use an isolated profile at `~/.config/abbot/cli/configs/<name>.toml` instead.

## Release and install

The release process is tag-based and publishes GitHub Release assets for:

- `x86_64-unknown-linux-gnu`
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
ABBOTIK_CLI_VERSION=v1.5.0 \
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

## Current state

The CLI now has a shared API helper layer plus command-family dispatch wired up to Abbotik routes. The `data` family is aligned to Abbotik's model, record, relationship, and nested-child route shapes, with query flags threaded through the request helpers.

For a new user, the intended first steps are:

```bash
abbot --config staging auth login --tenant acme --username alice --password secret
abbot --config staging data list rooms
```

Each named config keeps its own saved token, base URL overrides, output format, and machine-auth metadata.

For scripting, `ABBOTIK_CONFIG=<name>` selects the same profile as `--config <name>`, and the CLI flag wins when both are present.

For a new user, the intended first steps are:

1. `abbot auth register --tenant <tenant> --username <user> --email <email> --password <password>`
2. `abbot auth login --tenant <tenant> --username <user> --password <password>` if you need a fresh session later
3. `abbot public llms` or `abbot docs root`
4. `abbot health`

`abbot auth register` now follows the API contract change by registering first and
then immediately completing `/auth/login` so the CLI still lands with a saved JWT.

For users joining an existing tenant, the invite flow is:

1. Existing root or full user runs `abbot user invite --username <user> --invite-type human|machine`
2. Human invitees run `abbot auth register --tenant <tenant> --username <user> --invite-code <code> --email <email> --password <password>`
3. Machine invitees run `abbot auth provision --tenant <tenant> --username <user> --invite-code <code> --public-key @~/.config/abbot/abbotik.pub`
4. Invited humans can later use `abbot auth login`; invited machines finish with `abbot auth verify` or `abbot auth machine connect`

Machine clients should use:

1. `abbot auth machine connect --tenant <tenant> --username <user> --key ~/.config/abbot/abbotik.key`
2. `abbot auth refresh` when the saved machine token expires; `abbot` auto-detects public-key auth and runs challenge→sign→verify using the saved key path
3. `abbot keys list`

Low-level machine-auth commands still exist for manual control:

1. `abbot auth provision --tenant <tenant> --username <user> --public-key @~/.config/abbot/abbotik.pub --save-private-key-path ~/.config/abbot/abbotik.key`
2. Sign the returned nonce with the matching private key
3. `abbot auth verify --tenant <tenant> --challenge-id <id> --signature @signature.txt`
