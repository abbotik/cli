# User Machine Keys Rotate

Rotate a tenant machine key by providing a replacement public key and a grace
period before the old key is revoked.

Common use:

- `abbot api user machine-keys rotate --key-id <uuid> --new-public-key @next.pub --revoke-old-after-seconds 300`
