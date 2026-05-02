# abbot factory

High-level durable factory workflow operations.

Use this branch for operator workflows:

- `abbot factory create --prompt "create me a marketing plan"`
- `abbot factory create --plan ./plan.md --workflow software.delivery --subject repo:abbotik/api`
- `abbot factory start <run_id>`
- `abbot factory status <run_id>`
- `abbot factory watch <run_id>`
- `abbot factory artifacts <run_id>`
- `abbot factory review <run_id>`

`abbot llm factory` remains available for low-level route-shaped debugging.
