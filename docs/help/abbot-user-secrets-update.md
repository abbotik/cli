# User Secrets Update

Replace one encrypted user-scoped secret by name.

```bash
abbot user secrets update openrouter_primary \
  --value @~/.config/secrets/openrouter.key \
  --metadata '{"provider":"openrouter"}'
```

The API requires a replacement `value`. Use `--body` or stdin when you need to
clear optional fields with JSON `null`.
