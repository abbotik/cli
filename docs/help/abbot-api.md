# API

Route-shaped access to registered `/api/<name>` families.

Each `abbot api <name>` child corresponds to a live backend route family:

- `acls`
- `aggregate`
- `bulk`
- `cron`
- `data`
- `describe`
- `find`
- `keys`
- `stat`
- `tracked`
- `trashed`
- `user`

Examples:

```bash
abbot api describe list
abbot api data list users
abbot api keys create --name ci-runner
abbot api user introspect
abbot api cron list
```

Use `--help` on a child command for the next level down.
