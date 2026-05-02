# Auth Login

Log in against an existing tenant on an API host.

Use this when the tenant already exists and you just need a token-backed local
session. If you are new, run `abbot auth register` first to create the tenant;
the CLI will handle the required follow-up login and save the initial session.

The optional positional host selects the API server:

```bash
abbot auth login
abbot auth login http://localhost:3000
abbot auth login http://192.168.1.50:3000
```

If the host is omitted, login targets `https://api.abbotik.com`. Bare local
hosts such as `localhost:3000` are accepted and normalized to `http://`.

Successful login saves credentials for that host and makes it the default for
later commands. Use `abbot auth list` to inspect saved hosts, `abbot auth use
<host>` to switch the default, and top-level `--host <host>` for a one-command
override.

Pass `--username` and `--password` to authenticate. For `--password`, use `-`
to read from stdin or `@<path>` to read from a file.

This branch is intentionally concise; use the command itself for the full flag list.
