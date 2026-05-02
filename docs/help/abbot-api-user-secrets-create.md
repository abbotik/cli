# User Secrets Create

Create one encrypted user-scoped secret.

```bash
abbot api user secrets create \
  --name openrouter_primary \
  --value @~/.config/secrets/openrouter.key \
  --kind api_key \
  --description "Primary OpenRouter key" \
  --metadata '{"provider":"openrouter"}'
```

`--value` accepts inline text, `-` for stdin, or `@<path>` for a file. `--metadata`
must be a JSON object and also accepts `-` or `@<path>`.

Use `--body '{"name":"openrouter_primary","value":"..."}'` when you need to send
the exact API body.
