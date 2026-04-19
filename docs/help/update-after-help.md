# Update

Update the current `abbot` binary to the latest published release.

This command inspects the running binary path before it does anything:

- Homebrew installs delegate to `brew upgrade abbotik/tap/abbot`
- curl installs download the latest GitHub release asset and replace the current binary in place
- `--version-list` shows published GitHub release tags without installing
- `--version <v>` installs that exact GitHub release for curl-installed binaries

Because Homebrew tracks the current formula, `--version <v>` is not supported for
Homebrew-managed binaries.

Use `abbot update` from the binary you actually want to refresh.
