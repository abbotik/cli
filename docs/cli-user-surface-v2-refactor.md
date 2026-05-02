# Abbot CLI v2 User Surface Refactor

## Project Frame

`abbot` is currently a broad route/client surface for Abbotik. The top-level
help exposes many low-level API families beside operator workflows, which makes
first use unclear.

This refactor is a clean break for a major version release. Backwards-compatible
aliases are intentionally out of scope unless a route or workflow explicitly
needs them.

## Problem Statement

The CLI root should expose product and protocol concepts, not every route family.
Route-shaped `/api/<name>` commands should live under a single `abbot api`
parent. Stale or compatibility-only backend routes should not appear in the
normal CLI surface.

## Route Inventory

The live API router in `../api/apps/api/src/servers/http.ts` registers these
`/api/<name>` families:

- `/api/acls`
- `/api/aggregate`
- `/api/bulk`
- `/api/cron`
- `/api/data`
- `/api/describe`
- `/api/find`
- `/api/keys`
- `/api/stat`
- `/api/tracked`
- `/api/trashed`
- `/api/user`

The same router also registers these non-`/api` roots:

- `/auth`
- `/oauth`
- `/docs`
- `/mcp`
- `/llm`
- `/v1`
- `/fs`
- `/health`
- `/`

The current CLI `app` command targets `/app/<name>`, but the live router no
longer registers `/app/*` or `/api/app/*`. Treat it as stale.

## Command Surface Decisions

### Top-Level Product And Local Commands

Keep these root commands:

- `auth` for login, registration, machine auth, token, tenant, and dissolve flows.
- `config` for local profile state.
- `doctor` for local and remote readiness checks.
- `factory` for high-level durable Factory workflows.
- `tui` for the terminal operator console.
- `update` for CLI self-update.

### Top-Level Route Or Protocol Commands

Keep or add these root commands:

- `api` for exact `/api/<name>` route families only.
- `llm` for `/llm/...` route-shaped LLM room, factory, provider, model, and skill surfaces.
- `mcp` for MCP-semantic operations, not raw HTTP methods.
- `docs` if direct API documentation lookup remains useful as a discovery surface.

### `abbot api` Children

`abbot api <name>` must correspond to an actual `/api/<name>` route family.
Add these children:

- `abbot api acls`
- `abbot api aggregate`
- `abbot api bulk`
- `abbot api cron`
- `abbot api data`
- `abbot api describe`
- `abbot api find`
- `abbot api keys`
- `abbot api stat`
- `abbot api tracked`
- `abbot api trashed`
- `abbot api user`

Remove these as top-level commands.

### MCP Surface

`abbot mcp` should be protocol-shaped:

- `abbot mcp list`
- `abbot mcp call <tool> [--arguments <json|@file|->]`

Do not expose route-shaped commands such as `get`, `post`, `sse`, or
`messages`.

### Hide Or Remove

Remove from the normal CLI command tree:

- `app`, because its backend route appears stale.
- `fs`, because there is no current named CLI workflow for tenant filesystem access.
- `v1`, because `/v1/responses` is a direct HTTP compatibility surface for SDKs and model clients.
- `public`, because root discovery documents are not an operator command family.
- `health`, because readiness belongs under `doctor` and should not clutter the root.
- `oauth`, because it is protocol plumbing under auth, not a user-facing CLI family.

## Implementation Stages

1. Add the delivery artifact and freeze the v2 command contract.
2. Add an `api` command parent and move all current `/api/<name>` command structs
   and dispatch paths under it.
3. Remove stale or hidden root command variants from the CLI tree.
4. Add the semantic `mcp` command.
5. Refresh root help, command docs, README examples, and parser tests.
6. Bump the crate to `2.0.0`.
7. Run validation and fix tests.
8. Commit, tag `v2.0.0`, push, and watch the release workflow.

## Acceptance Criteria

- `abbot --help` shows a smaller root surface centered on product, protocol,
  and local commands.
- `abbot api --help` lists exactly the registered `/api/<name>` families above.
- Old top-level route-family commands such as `abbot data`, `abbot user`, and
  `abbot keys` no longer parse.
- New route-family paths such as `abbot api data`, `abbot api user`, and
  `abbot api keys` parse.
- `abbot mcp --help` exposes MCP concepts rather than HTTP methods.
- `abbot --version` works.
- `cargo test` passes.
- `cargo clippy --bin abbot -- -D warnings` passes or any pre-existing blocker is documented.
- The release workflow publishes `v2.0.0` artifacts after the tag is pushed.

## Open Questions

None blocking. `docs` remains top-level for this refactor because it is the
current direct discovery surface and is not an `/api/<name>` route family.
