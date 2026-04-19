# abbot command

Print the embedded markdown doc for a command path.

Use this when you want the long-form operator or agent guidance without routing
through clap help formatting.

Examples:

```bash
abbot command
abbot command auth
abbot command auth machine
abbot command auth machine connect
abbot command data relationship child patch
abbot command update
```

The resolver uses the exact command path when a dedicated markdown file exists.
If a leaf command has no dedicated file yet, it falls back to the nearest parent
command doc and tells you which parent doc it used.
