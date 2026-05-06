#!/bin/sh

set -eu

SCRIPT_DIR=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
REPO_ROOT=$(cd "$SCRIPT_DIR/../.." && pwd)

if [ "$#" -ne 1 ]; then
    cat >&2 <<EOF
Usage: $0 <host>

Builds the CLI once, then runs all CLI integration suites against <host>.
EOF
    exit 2
fi

HOST="$1"

cd "$REPO_ROOT"
cargo build --bin abbot

export ABBOT_BIN="${ABBOT_BIN:-$REPO_ROOT/target/debug/abbot}"

"$SCRIPT_DIR/auth-basic.sh" "$HOST"
"$SCRIPT_DIR/api-basic.sh" "$HOST"

echo "all CLI integration suites passed for $HOST"
