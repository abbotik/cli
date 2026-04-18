# LLM Room

Manage bounded room execution through `/llm/room`.

This is the live execution primitive in the API docs: rooms accept messages,
emit durable events, and can be woken, interrupted, or released explicitly.

Common uses:

- `abbot llm room list`
- `abbot llm room create < body.json`
- `abbot llm room message room_123 < body.json`
- `abbot llm room events room_123`

For the full HTTP contract and lifecycle notes, read
`abbot docs path /docs/llm/room`.
