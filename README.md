# monk-cli

`monk` is the Rust CLI for the Monk API.
By default it talks to the public API at `https://monk-api.com`.

## Release and install

The release process is tag-based and publishes GitHub Release assets for:

- `x86_64-unknown-linux-gnu`
- `x86_64-apple-darwin`
- `aarch64-apple-darwin`

Homebrew is intentionally excluded for now.

### Curl install

Install from the latest GitHub release:

```bash
curl -fsSL https://raw.githubusercontent.com/ianzepp/monk-cli/main/scripts/install.sh | bash
```

To pin a version:

```bash
MONK_CLI_VERSION=v0.3.0 \
  curl -fsSL https://raw.githubusercontent.com/ianzepp/monk-cli/main/scripts/install.sh | bash
```

Optional install directory:

```bash
MONK_CLI_INSTALL_DIR="$HOME/bin" \
  curl -fsSL https://raw.githubusercontent.com/ianzepp/monk-cli/main/scripts/install.sh | bash
```

## Current state

The CLI now has a shared API helper layer plus command-family dispatch wired up to Monk routes. The `data` family is aligned to Monk's model, record, relationship, and nested-child route shapes, with query flags threaded through the request helpers.

For a new user, the intended first steps are:

1. `monk auth register --tenant <tenant> --username <user> --email <email> --password <password>`
2. `monk auth login --tenant <tenant> --username <user> --password <password>` if you need a fresh session later
3. `monk public llms` or `monk docs root`
4. `monk health`

`monk auth register` now follows the API contract change by registering first and
then immediately completing `/auth/login` so the CLI still lands with a saved JWT.

Machine clients should use:

1. `monk auth provision --tenant <tenant> --username <user> --public-key @machine.pub`
2. Sign the returned nonce with the matching private key
3. `monk auth verify --tenant <tenant> --challenge-id <id> --signature @signature.txt`
4. `monk keys list`
