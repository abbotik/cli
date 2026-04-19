# Doctor

Run a human-oriented auth and config sanity check for the active profile.

This command:

- shows the active config path and base URL
- inspects the saved token locally
- probes `/api/user/introspect` when a token exists
- probes `/auth/refresh` for saved username/password tokens without writing config
- returns exact next steps instead of sending you into the auth tree blindly

Use `abbot doctor` when `abbot tui`, `abbot auth refresh`, or authenticated API
calls are behaving unexpectedly.
