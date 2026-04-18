# LLM Factory

Manage durable factory runs through `/llm/factory/runs`.

Factory is the durable orchestration sibling to rooms. Use it when the work
needs run status, stages, issues, checkpoints, verification, gates, or review
state across a delivery run.

Common uses:

- `abbot llm factory list`
- `abbot llm factory create < body.json`
- `abbot llm factory status run_123`
- `abbot llm factory verify run_123 < body.json`

For the full HTTP contract, read `abbot docs path /docs/llm/factory`.
