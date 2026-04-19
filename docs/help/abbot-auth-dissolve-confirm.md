# Auth Dissolve Confirm

Consume a confirmation token and permanently dissolve the tenant.

This completes the two-step auth dissolve flow. The confirmation token must come
from `abbot auth dissolve request`.

Common uses:

- `abbot auth dissolve confirm --confirmation-token <token>`

Use `--help` on `confirm` for the full flag list.
