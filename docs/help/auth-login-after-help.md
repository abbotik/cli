# Auth Login

Log in against an existing tenant.

Use this when the tenant already exists and you just need a token-backed local
session. If you are new, run `monk auth register` first to create the tenant;
the CLI will handle the required follow-up login and save the initial session.

Pass `--username` and `--password` to authenticate. For `--password`, use `-`
to read from stdin or `@<path>` to read from a file.

This branch is intentionally concise; use the command itself for the full flag list.
