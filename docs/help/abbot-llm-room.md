# LLM Room

Manage bounded room execution through `/llm/room`.

This is the live execution primitive in the API docs: rooms accept messages,
emit durable events, and can be woken, interrupted, or released explicitly.

Common uses:

- `abbot llm room list`
- `abbot llm room create --name math --model openai/gpt-5.4 --provider openrouter --purpose "math scratch"`
- `abbot llm room run math "Calculate 42 * 19" --stream`
- `abbot llm room run math "Now do 42 / 7"`
- `abbot llm room release math`
- `abbot llm room create < body.json`
- `abbot llm room message math < body.json`
- `abbot llm room events math`

`create` accepts flags for the common one-agent room case. It stores `--name`
in room metadata so later commands can use the name as their room target.

`run` appends one task message to an existing room, waits for the next agent
output, and prints only that output on stdout. Use `--stream` to stream the
assistant text on stdout as it arrives. Add `--debug` to print room event
diagnostics on stderr while waiting.

For the full HTTP contract and lifecycle notes, read
`abbot docs path /docs/llm/room`.
