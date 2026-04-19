# LLM

Use this branch for the tenant-scoped `/llm/*` surfaces.

The API docs split this surface into:

- `/llm/providers` and `/llm/providers/models` for rentable model discovery
- `/llm/skills` for skill discovery
- `/llm/room/*` for bounded live execution
- `/llm/factory/*` for durable delivery orchestration state

Common uses:

- `abbot llm providers`
- `abbot llm models`
- `abbot llm skills`
- `abbot llm room list`
- `abbot llm factory list`

For exact route semantics, read `abbot docs path /docs/llm/room` and
`abbot docs path /docs/llm/factory`.

Body-bearing commands read JSON from stdin unless stated otherwise.
