# Factory Runtime Phase 6 Delivery Spec: CLI Operator Workflow

Date: 2026-05-02

## Interpreted Phase Problem

The API now has higher-level Factory creation and start/wake behavior, but the
CLI still exposes Factory primarily through `abbot llm factory`, which mirrors
low-level route verbs. Operators need a top-level `abbot factory` workflow for
prompt or plan based runs.

## Normalized Phase Spec

Deliver a high-level CLI surface:

- add top-level `abbot factory`
- support `create --prompt`
- support `create --plan`
- support optional `--workflow`, `--subject`, and `--title`
- support `start`, `status`, `watch`, `artifacts`, and `review`
- keep low-level `abbot llm factory` commands
- add low-level `abbot llm factory start` for route coverage

## Repo-Aware Baseline

Touched surfaces:

- `src/cli/factory.rs`
- `src/commands/factory.rs`
- `src/cli/mod.rs`
- `src/commands/mod.rs`
- `src/cli/llm.rs`
- `src/commands/llm.rs`
- `src/commands.rs.test.rs`
- `docs/help/abbot-factory.md`
- `README.md`

Verification:

- `cargo fmt`
- `cargo test`

## Checkpoint And Gate Plan

Checkpoint: CLI Operator Workflow

Gate criteria:

- `abbot factory create --prompt ...` parses
- `abbot factory create --plan ...` is implemented
- `abbot factory start/status/watch/artifacts/review` are implemented
- `abbot llm factory start` remains available as a low-level route wrapper
- tests pass

## Open Questions

`watch` currently returns one status snapshot because the API event feed is not
implemented yet.
