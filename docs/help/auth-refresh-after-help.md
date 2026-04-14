# Auth Refresh

Exchange the saved token for a new access token.

Use this when the current session is stale but the saved config still has enough
state to renew it.

`abbot` now auto-detects the refresh path:

- human login tokens use `/auth/refresh`
- machine public-key tokens use `/auth/challenge` + local nonce signing + `/auth/verify`

Machine refresh requires a saved private key path in local config. You can save
machine key paths during `abbot auth provision` or `abbot auth verify` with the
`--save-public-key-path` and `--save-private-key-path` flags.
