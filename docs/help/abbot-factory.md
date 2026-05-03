# abbot factory

High-level durable factory workflow operations.

Use this branch for operator workflows:

- `abbot factory submit --prompt "create me a marketing plan"`
- `abbot factory submit "create me a marketing plan"`
- `abbot factory run "create me a marketing plan"`
- `abbot factory submit --prompt-file ./plan.md --workflow software.delivery --subject repo:abbotik/api`
- `abbot factory run --prompt-file ./plan.md --timeout 1800`
- `abbot factory submit --prompt-file -`
- `abbot factory status <run_id>`
- `abbot factory watch <run_id>`
- `abbot factory watch <run_id> --timeout 1800 --until completed`
- `abbot factory review <run_id>`

`abbot factory watch` attaches to a run and may block for minutes or hours.
Press Ctrl-C to detach; the factory run continues on the server.

`abbot factory run` is `submit` plus `watch`: it creates and wakes a run, then
waits until completion, failure, timeout, or attention.

`--prompt-file -` reads prompt text from stdin.

`abbot factory create` remains accepted as a compatibility alias for
`abbot factory submit`.

`abbot llm factory` remains available for low-level route-shaped debugging.
