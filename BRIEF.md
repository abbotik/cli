# abbot Brief

## Interpreted Problem
Build a **new Rust CLI** in `cli/` using `clap` that can interface with the **full Abbotik API surface** exposed by `api/`.

The CLI should start from a clean slate. The empty `cli/` directory is the target workspace.

## Normalized Spec

### In scope
- Create a Rust command-line application with `clap`.
- Treat `api/` as the API source of truth.
- Use `cephalopodic/cli/` as the structural reference for command layout, config, output, and API-client patterns.
- Cover Abbotik API endpoints across:
  - auth
  - health / root / docs
  - describe
  - data
  - find
  - aggregate
  - bulk
  - acls
  - stat
  - tracked
  - trashed
  - user
  - cron
  - filesystem
  - optional app-package access if needed
- Plan for machine-friendly output and human-friendly TTY output.

### Out of scope for planning pass
- Implementation details beyond a recommended structure.
- Any destructive changes in `api/`.
- Any assumptions about hidden endpoints not present in the repo evidence.

## Repo-Aware Baseline

### `api/`
- TypeScript/Bun/Hono backend.
- Main server entrypoint: `src/index.ts`.
- HTTP route registration: `src/servers/http.ts`.
- Docs and route contracts live in `README.md`, `AGENTS.md`, `src/routes/docs/PUBLIC.md`, and per-route docs under `src/routes/**`.
- Full exposed surface includes `/auth/*`, `/api/*`, `/fs/*`, `/docs/*`, `/llms.txt`, `/app/*`, and `/health`.

### `cephalopodic/cli/`
- Rust CLI reference repo.
- Demonstrates:
  - `clap`-driven nested command tree
  - config/session persistence
  - transport helpers that apply auth centrally
  - centralized output policy with JSON/TTY modes
  - surface-by-surface command modules

### `cli/`
- Now populated with a Rust CLI crate.
- Git repository is initialized.
- Best treated as the live Abbot CLI workspace.

## Proposed Product Shape

`abbotik-cli` should be a **shell-native Abbotik API client** rather than a local admin tool.

### Likely top-level command groups
- `abbot auth`
- `abbot health`
- `abbot docs`
- `abbot describe`
- `abbot data`
- `abbot find`
- `abbot aggregate`
- `abbot bulk`
- `abbot acls`
- `abbot stat`
- `abbot tracked`
- `abbot trashed`
- `abbot user`
- `abbot cron`
- `abbot fs`
- `abbot app`

### Likely cross-cutting CLI behavior
- `--format json` for explicit machine output mode.
- TTY-aware human output by default.
- Persistent config for API base URL and auth token.
- Central request helper for attaching headers and parsing errors.
- Clear support for tenant-aware / authenticated workflows.

## Stage Graph

### Stage 1 — Scaffold the crate
**Input:** empty `cli/` directory

**Work:**
- initialize a Rust binary crate
- add `Cargo.toml`
- create `src/main.rs`
- establish module layout for CLI, config, client, and output

**Output:** buildable empty CLI skeleton

**Validation:** `cargo check`

---

### Stage 2 — Define API coverage map
**Input:** Abbotik API route surface from `api/src/servers/http.ts` and docs

**Work:**
- enumerate endpoint groups
- decide command names and subcommand shapes
- identify which routes are public vs auth-required
- note any special parameter conventions (`:model`, `:id`, `:field`, etc.)

**Output:** command tree and endpoint coverage matrix

**Validation:** internal review against route registration and docs

---

### Stage 3 — Build shared client infrastructure
**Input:** command tree + output policy decisions

**Work:**
- implement config loading/saving
- implement API client wrapper
- implement auth header handling
- implement output rendering policy
- define error model

**Output:** reusable transport/config/output foundation

**Validation:** unit tests for config/output/client helpers

---

### Stage 4 — Implement core public commands first
**Input:** client infrastructure

**Work:**
- `health`
- `docs`
- `auth` login/register/refresh/tenants
- maybe `root`/`llms` read-only fetch helpers if needed

**Output:** first user-visible CLI slice

**Validation:** small integration smoke tests against Abbotik API

---

### Stage 5 — Implement protected API domains
**Input:** auth-capable client + command shell

**Work:**
- `describe`
- `data`
- `find`
- `aggregate`
- `bulk`
- `acls`
- `stat`
- `tracked`
- `trashed`
- `user`
- `cron`
- `fs`
- optionally `app`

**Output:** full Abbotik API client coverage

**Validation:** targeted endpoint tests and command help checks

---

### Stage 6 — Polish UX and docs
**Input:** working CLI

**Work:**
- top-level help text
- command docs / examples
- error messages
- output formatting polish
- README or usage guide

**Output:** usable operator-facing CLI

**Validation:** `cargo test`, `cargo clippy` if enabled, help output review

## Design Decisions to Make Next

1. **Repository layout**
   - single binary crate in `cli/`
   - or workspace with library + binary split

2. **Auth model**
   - simple bearer-token config
   - or richer login/session management if Abbotik API requires it

3. **Output policy**
   - plain JSON only
   - or TTY-aware human summaries plus JSON escape hatch

4. **Command granularity**
   - one command per API surface
   - or deeper nesting that mirrors route families exactly

5. **App-package support**
   - support `/app/*` now
   - or defer until core API coverage is complete

## Risk Notes

- Abbotik API has a broad surface; trying to implement every endpoint at once would be too much.
- Some endpoints may have route-specific request/response conventions that should be confirmed from the route docs before coding.
- If the CLI is meant to be broadly useful to humans and agents, output consistency matters as much as route coverage.

## Recommended Next Step

Initialize `cli/` as a Rust crate, then implement the **shared client/config/output layer first**, followed by **auth + health + docs** as the first working slice.

## Full API Shape Pass Addendum

This second discovery pass confirmed the live Abbotik API shape from `src/servers/http.ts` and the route docs under `src/routes/**`.

### Live route families to cover
- `GET /`, `GET /llms.txt`, `GET /health`
- `/auth/*`
- `/docs/*`
- `/api/describe/*`
- `/api/data/*`
- `/api/find/*`
- `/api/aggregate/*`
- `/api/bulk`, `/api/bulk/export`, `/api/bulk/import`
- `/api/acls/*`
- `/api/stat/*`
- `/api/tracked/*`
- `/api/trashed/*`
- `/api/user/*`
- `/api/cron/*`
- `/fs/*`
- `/app/:appName/*`

### Route/doc mismatches to remember
- The live canonical self-profile route is `GET /api/user/me`.
- Cron creation is implemented in the CLI and maps to the live `/api/cron` surface.
- Some docs use older or alternate path aliases; the live Hono route tree is the source of truth.

### Command-shape consequence
The CLI should likely be organized as a surface-by-surface client with:
- `auth`
- `health`
- `docs`
- `describe`
- `data`
- `find`
- `aggregate`
- `bulk`
- `acls`
- `stat`
- `tracked`
- `trashed`
- `user`
- `cron`
- `fs`
- optional `app`

Each surface will need consistent handling for:
- auth-bearing requests
- JSON bodies
- query parameters like `where`, `select`, `unwrap`, `format`
- path parameter expansion for `:model`, `:id`, `:field`, `:pid`, and similar placeholders

### Output implications
Abbotik API supports multiple response formats and projection controls, so the CLI should plan for:
- TTY-friendly summaries by default
- explicit JSON mode for machines
- query options for `format`, `select`, `unwrap`, and related response modifiers

### Implementation caution
Do not mirror stale docs blindly. Build the CLI from the live route tree and treat the markdown docs as guidance and examples, not as the final contract when they disagree with `src/servers/http.ts`.
