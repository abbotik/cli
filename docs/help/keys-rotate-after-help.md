# Keys Rotate

Rotate a machine key by providing a replacement public key and a grace period
before the old key is revoked.

Common use:

- `monk keys rotate --key-id <uuid> --new-public-key @next.pub --revoke-old-after-seconds 300`
