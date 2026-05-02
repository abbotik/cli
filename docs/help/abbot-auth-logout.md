# Auth Logout

Remove the saved token and machine-auth metadata for an API host.

```bash
abbot auth logout
abbot auth logout http://localhost:3000
```

Without a host argument, logout targets the active host. Logout does not delete
the host record or change the active host.
