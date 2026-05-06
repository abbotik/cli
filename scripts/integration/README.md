# CLI Integration Scripts

These scripts validate the compiled `abbot` CLI against a live Abbotik API host.
They intentionally use the CLI binary for all API interactions; they do not use
`curl` or direct HTTP clients.

Each suite creates a fresh disposable machine-auth tenant and runs with a
temporary `HOME`, so it does not read or write the operator's normal
`~/.config/abbot/cli` state.

Run everything:

```sh
scripts/integration/run-all.sh http://127.0.0.1:9001
scripts/integration/run-all.sh https://abbotik-web.ianzepp.workers.dev
```

Run one suite with an already-built binary:

```sh
cargo build --bin abbot
ABBOT_BIN="$PWD/target/debug/abbot" scripts/integration/api-basic.sh https://abbotik-web.ianzepp.workers.dev
```

Set `KEEP_ABBOT_ITMP=1` to keep the temporary test directory after a failure.
