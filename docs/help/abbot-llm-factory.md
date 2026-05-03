# LLM Factory

Manage durable factory runs through `/llm/factory/runs`.

Factory is the durable orchestration sibling to rooms. Use it when the work
needs run status, stages, issues, checkpoints, verification, gates, or review
state across a delivery run.

Common uses:

- `abbot llm factory list`
- `abbot llm factory create < body.json`
- `abbot llm factory status <run-id>`
- `abbot llm factory cancel <run-id> --reason "operator requested"`
- `abbot llm factory stop <run-id>`
- `abbot llm factory dispatch-issue <run-id> <issue-id> < body.json`
- `abbot llm factory create-artifact <run-id> < body.json`
- `abbot llm factory create-gate <run-id> < body.json`
- `abbot llm factory verify <run-id> < body.json`

For the full HTTP contract, read `abbot docs path /docs/llm/factory`.
