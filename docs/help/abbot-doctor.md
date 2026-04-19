# Doctor

Run a human-oriented live sanity check for the active profile and server.

This command:

- probes the configured server health route
- shows the active config path and base URL
- inspects the saved token locally
- probes `/api/user/introspect` when a token exists
- probes `/auth/refresh` for saved username/password tokens without writing config
- returns exact next steps instead of sending you into the auth tree blindly

Use `abbot doctor` when `abbot tui`, `abbot auth refresh`, or authenticated API
calls are behaving unexpectedly.
