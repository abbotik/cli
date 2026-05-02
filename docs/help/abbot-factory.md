# abbot factory

High-level durable factory workflow operations.

Use this branch for operator workflows:

- `abbot factory submit --prompt "create me a marketing plan"`
- `abbot factory submit --plan ./plan.md --workflow software.delivery --subject repo:abbotik/api`
- `abbot factory status <run_id>`
- `abbot factory watch <run_id>`
- `abbot factory watch <run_id> --timeout 1800 --until completed`
- `abbot factory review <run_id>`

`abbot factory watch` attaches to a run and may block for minutes or hours.
Press Ctrl-C to detach; the factory run continues on the server.

`abbot factory create` remains accepted as a compatibility alias for
`abbot factory submit`.

`abbot llm factory` remains available for low-level route-shaped debugging.
