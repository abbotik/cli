# LLM Response Delivery Plan

## Purpose

This document lowers the proposed `abbot llm response` command tree into a
repo-aware delivery plan for `abbotik/cli`.

Proposed command family:

- `abbot llm response create`
- `abbot llm response stream`
- `abbot llm response wait`
- `abbot llm response continue --room <id>`

This plan treats the current API truth as authoritative:

- the durable execution primitive is a room
- `POST /v1/responses` is a thin ingress over room-backed turns
- the CLI should not invent a fake detached job system in v1

## Stage 1: Interpreted Problem

### Claimed Problem

Add a shell-friendly remote-agent execution flow to the CLI so a primary LLM
or operator can submit one bounded response turn, stream it, wait for it, and
continue an existing room.

### Inferred Actual Problem

The repo already has most of the execution substrate:

- `abbot llm room *` for direct room lifecycle control
- `POST /v1/responses` in `api` for simpler room-backed turn submission
- room history and event feeds for durable execution state

The actual gap is not remote execution itself. The gap is CLI shape and naming.
The current `llm room` tree exposes the low-level primitive, but there is no
CLI subtree that cleanly models "submit one bounded turn and watch it finish."

The important constraint is that the API does not yet provide a durable
`GET /v1/responses/:id` lookup surface. So `wait` cannot honestly be specified
as a pure response-id lookup without adding more API truth.

### Confidence

High.

The command tree is additive and the current repo already exposes the required
room and responses seams. The only materially underspecified part is `wait`.

### Planning Assumptions

- `abbot llm` remains the truthful top-level namespace for this work.
- `agent` stays out of scope for this wave.
- `abbot llm response` is a CLI wrapper over `/v1/responses`, not a new API
  orchestration layer.
- The room runtime remains single-actor and one queued message at a time.

## Stage 2: Normalized Spec

## Project Frame

Add a new `response` subtree under `abbot llm` that models one bounded remote
turn over the existing `/v1/responses` API.

## Problem Statement

The CLI currently exposes:

- provider/model/skill discovery
- low-level room lifecycle
- low-level factory orchestration

It does not expose the higher-level "submit one response turn" workflow even
though the API already ships that ingress.

The missing work is to add a response-oriented CLI surface that is honest about
its underlying room semantics and does not overstate the durability of the
current response id.

## Functional Requirements

1. Add `abbot llm response create`.
2. Add `abbot llm response stream`.
3. Add `abbot llm response wait`.
4. Add `abbot llm response continue --room <id>`.
5. Reuse one request-building implementation path so `create` and `continue`
   do not drift.
6. Keep the command tree clearly layered over `/v1/responses` and room-backed
   execution.
7. Update CLI docs and command parsing tests.

## Technical Constraints

1. Do not invent a new detached job substrate in `cli`.
2. Do not rename or destabilize existing `abbot llm room` commands.
3. Do not claim that response ids are durable lookup handles unless the API
   contract is expanded to support that.
4. Keep stdin/body handling consistent with the existing CLI command style.
5. Preserve the current command-doc embedding flow under `cli/docs/help`.

## Open Question

How should `wait` resolve a response handle in v1?

Options:

- client-side projection from locally known `room_id + request_message_id`
- explicit flags like `--room` and `--message`
- API expansion later for true response lookup

## Stage 3: Repo-Aware Baseline

### Repo Profile

- Language: Rust
- CLI parser: Clap derive
- Command definitions: `src/cli/*`
- Command execution: `src/commands/*`
- HTTP client: `src/api.rs`
- Embedded help docs: `docs/help/*`
- Command-path doc resolver: `src/command_docs.rs`

### Confirmed Current Seams

1. Command tree root
   - `src/cli/mod.rs`

2. `llm` command definitions
   - `src/cli/llm.rs`

3. `llm` command execution
   - `src/commands/llm.rs`

4. Request transport helpers
   - `src/api.rs`
   - shared stdin/body helpers already used in `src/commands/mod.rs`

5. Parsing coverage
   - `src/commands.rs.test.rs`

6. Embedded markdown help
   - `docs/help/abbot-llm.md`
   - `docs/help/abbot-llm-room.md`

### Upstream API Truth

The CLI plan depends on these current API facts:

1. `POST /v1/responses` returns a queued response envelope immediately when
   `stream=false`.
2. `POST /v1/responses` streams one bounded turn when `stream=true`.
3. The response envelope includes:
   - `id`
   - `room_id`
   - `request_message_id`
4. Room events and history remain the durable execution record.
5. There is no confirmed `GET /v1/responses/:id` lookup route today.

### Constraint Decisions

#### Decision

Put `response` under `llm`, not at the top level.

#### Why

That matches the current truthful architecture and avoids inventing a second
conceptual hierarchy beside `room`.

#### Tradeoff

The command path is slightly longer than `abbot agent ...`, but the contract is
more honest.

#### Decision

Treat `continue` as a request-construction mode of `create`.

#### Why

The underlying API call is still `POST /v1/responses`; only `room_id`
requirements differ.

#### Tradeoff

The CLI may expose two verbs backed by one internal request builder, so docs
must explain the distinction clearly.

#### Decision

Delay any detached response lookup abstraction until the API actually ships it.

#### Why

Otherwise `wait` becomes a misleading promise over a non-durable id.

#### Tradeoff

The v1 `wait` contract will need to stay explicit about how it resolves state.

### Hard Gates

- Do not introduce a fake top-level job model in `cli`.
- Do not break `abbot llm room`.
- Do not require API changes just to land `create`, `stream`, and `continue`.
- Do not overclaim `wait` semantics in docs or help text.

## Stage 4: Stage Graph

### Stage A: Command Tree Admission

Scope:

- add `response` to `src/cli/llm.rs`
- add execution branch to `src/commands/llm.rs`
- add parse tests

Output:

- compiling command tree
- parsing coverage for the four new verbs

### Stage B: Response Request Builder

Scope:

- define CLI args for `create`, `stream`, `wait`, and `continue`
- map `create` and `continue` onto `/v1/responses`
- make `stream` force streaming mode

Output:

- one shared request builder for response submissions
- consistent JSON/stdin behavior

### Stage C: Wait Contract Lock

Scope:

- choose the honest v1 handle contract
- implement event/history-based waiting if kept client-side
- document the exact limitations

Output:

- explicit `wait` semantics
- no hidden detached-job claims

### Stage D: Docs And Operator Guidance

Scope:

- add embedded help docs for `abbot llm response`
- update parent `abbot llm` help text
- point users to `/docs/v1/responses` and room docs where appropriate

Output:

- discoverable CLI help
- consistent terminology between CLI and API docs

### Stage E: Verification

Scope:

- parse tests
- focused command execution tests if available
- smoke run against a local or integration API if needed

Output:

- verified command shape
- verified request/response mapping

## Stage 5: Parallel Workstreams

### Workstream 1: CLI Tree And Parsing

Files:

- `src/cli/llm.rs`
- `src/commands.rs.test.rs`

Goal:

Add the new subtree and lock parsing behavior.

### Workstream 2: Execution And Transport

Files:

- `src/commands/llm.rs`
- `src/api.rs`

Goal:

Implement request construction and route mapping for the new verbs.

### Workstream 3: Docs

Files:

- `docs/help/abbot-llm.md`
- new `docs/help/abbot-llm-response*.md`

Goal:

Make the new tree discoverable and honest about room-backed semantics.

These three slices can proceed mostly independently once the `wait` contract is
settled.

## Stage 6: Checkpoints And Gates

### Checkpoint 1: Command Contract Freeze

Must be true:

- the four response verbs are named and scoped exactly as agreed
- argument names are settled
- `wait` semantics are written down precisely

Gate:

- no implementation starts before the `wait` contract is explicit

### Checkpoint 2: Compile And Parse Green

Must be true:

- CLI compiles
- parse tests cover the new subtree
- no existing `llm room` parsing regresses

Gate:

- proceed to docs and runtime smoke only after parse stability

### Checkpoint 3: Route Mapping Verified

Must be true:

- `create` and `continue` hit `POST /v1/responses` in non-streaming mode
- `stream` hits `POST /v1/responses` in streaming mode
- `wait` behavior matches the documented handle contract

Gate:

- no release or merge if `wait` silently depends on unavailable API lookup

### Checkpoint 4: Help Surface Synchronized

Must be true:

- embedded help docs exist for the new subtree
- parent `llm` help mentions the new response branch
- wording stays consistent with room-backed execution truth

## Required Follow-On Work

1. Finalize the v1 `wait` handle contract.
2. Decide whether `continue` accepts stdin exactly like `create` or mixes flags
   plus stdin.
3. Add the `response` subtree to Clap definitions and command execution.
4. Add embedded help docs for:
   - `abbot llm response`
   - `abbot llm response create`
   - `abbot llm response stream`
   - `abbot llm response wait`
   - `abbot llm response continue`
5. Add parse tests for each new command path.

## Risks And Gaps

- The only real design risk is `wait`. Everything else is straightforward
  command-surface work over an existing API route.
- If `wait` is specified loosely now, the CLI will teach users the wrong mental
  model and create pressure for an API contract that does not exist yet.
- If `response` docs are too abstract, they will overlap confusingly with
  `room`; if they are too low-level, the new subtree loses its value.

## Recommendation

Proceed with the `abbot llm response` subtree, but treat `wait` as the gating
decision for implementation. `create`, `stream`, and `continue` can land
immediately against current API truth. `wait` should only land once its handle
contract is locked in language that is honest about the current absence of a
durable response lookup route.
