#!/bin/sh

set -eu

integration_repo_root() {
    script_dir=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
    (cd "$script_dir/../.." && pwd)
}

integration_usage() {
    cat >&2 <<EOF
Usage: $0 <host>

Examples:
  $0 http://127.0.0.1:9001
  $0 https://abbotik-web.ianzepp.workers.dev

Environment:
  ABBOT_BIN       Path to a compiled abbot binary.
                  Defaults to <cli repo>/target/debug/abbot.
  ABBOT_TEST_RUN  Optional run prefix for tenant names.
  KEEP_ABBOT_ITMP Set to 1 to keep the temporary HOME after failure.
EOF
}

integration_host_arg() {
    if [ "$#" -ne 1 ]; then
        integration_usage
        exit 2
    fi
    printf '%s\n' "$1"
}

integration_require_tools() {
    for tool in openssl python3; do
        if ! command -v "$tool" >/dev/null 2>&1; then
            echo "Missing required tool: $tool" >&2
            exit 1
        fi
    done
}

integration_abbot_bin() {
    repo_root=$(integration_repo_root)
    bin=${ABBOT_BIN:-"$repo_root/target/debug/abbot"}
    if [ ! -x "$bin" ]; then
        echo "Missing compiled CLI binary: $bin" >&2
        echo "Build it first with: cargo build --bin abbot" >&2
        exit 1
    fi
    printf '%s\n' "$bin"
}

integration_slug() {
    python3 - "$1" <<'PY'
import re
import sys

raw = sys.argv[1].lower()
slug = re.sub(r"[^a-z0-9_]", "_", raw)
slug = re.sub(r"_+", "_", slug).strip("_")
if not slug or not slug[0].isalpha():
    slug = "t_" + slug
print(slug[:48])
PY
}

integration_new_context() {
    suite="$1"
    stamp=$(date -u +%Y%m%d_%H%M%S)
    suffix=$(python3 - <<'PY'
import random
import string
print("".join(random.choice(string.ascii_lowercase + string.digits) for _ in range(8)))
PY
)
    run_prefix=${ABBOT_TEST_RUN:-cli_it}
    tenant=$(integration_slug "${run_prefix}_${suite}_${stamp}_${suffix}")
    username=$(integration_slug "machine_${suite}_${suffix}")
    tmp_root=$(mktemp -d "${TMPDIR:-/tmp}/abbot-cli-it-${suite}.XXXXXX")
    key_path="$tmp_root/machine.key"
    home_dir="$tmp_root/home"
    mkdir -p "$home_dir"
    (umask 077 && openssl genpkey -algorithm ED25519 -out "$key_path" >/dev/null 2>&1)

    ABBOT_IT_TMP="$tmp_root"
    ABBOT_IT_HOME="$home_dir"
    ABBOT_IT_TENANT="$tenant"
    ABBOT_IT_USERNAME="$username"
    ABBOT_IT_KEY="$key_path"
    export ABBOT_IT_TMP ABBOT_IT_HOME ABBOT_IT_TENANT ABBOT_IT_USERNAME ABBOT_IT_KEY
}

integration_cleanup() {
    status=$?
    if [ "${KEEP_ABBOT_ITMP:-}" = "1" ] && [ -n "${ABBOT_IT_TMP:-}" ]; then
        echo "Keeping temporary integration directory: $ABBOT_IT_TMP" >&2
        exit "$status"
    fi
    if [ -n "${ABBOT_IT_TMP:-}" ] && [ -d "$ABBOT_IT_TMP" ]; then
        rm -rf "$ABBOT_IT_TMP"
    fi
    exit "$status"
}

integration_run() {
    label="$1"
    shift
    out="$ABBOT_IT_TMP/$label.json"
    echo "==> $label" >&2
    set +e
    HOME="$ABBOT_IT_HOME" "$ABBOT_BIN" --host "$ABBOT_IT_HOST" "$@" >"$out"
    status=$?
    set -e
    if [ "$status" -ne 0 ]; then
        if [ -s "$out" ]; then
            cat "$out" >&2
            printf '\n' >&2
        fi
        return "$status"
    fi
    cat "$out" >&2
    printf '\n' >&2
    printf '%s\n' "$out"
}

integration_run_stdin() {
    label="$1"
    input="$2"
    shift 2
    out="$ABBOT_IT_TMP/$label.json"
    echo "==> $label" >&2
    set +e
    printf '%s' "$input" | HOME="$ABBOT_IT_HOME" "$ABBOT_BIN" --host "$ABBOT_IT_HOST" "$@" >"$out"
    status=$?
    set -e
    if [ "$status" -ne 0 ]; then
        if [ -s "$out" ]; then
            cat "$out" >&2
            printf '\n' >&2
        fi
        return "$status"
    fi
    cat "$out" >&2
    printf '\n' >&2
    printf '%s\n' "$out"
}

integration_json_get() {
    file="$1"
    path="$2"
    python3 - "$file" "$path" <<'PY'
import json
import sys

with open(sys.argv[1], "r", encoding="utf-8") as handle:
    value = json.load(handle)

for part in sys.argv[2].split("."):
    if isinstance(value, list):
        value = value[int(part)]
    else:
        value = value[part]

if value is None:
    sys.exit(1)
if isinstance(value, (dict, list)):
    print(json.dumps(value))
else:
    print(value)
PY
}

integration_json_first_id() {
    file="$1"
    python3 - "$file" <<'PY'
import json
import sys

with open(sys.argv[1], "r", encoding="utf-8") as handle:
    payload = json.load(handle)

def walk(value):
    if isinstance(value, dict):
        ident = value.get("id")
        if isinstance(ident, str) and ident:
            return ident
        for child in value.values():
            found = walk(child)
            if found:
                return found
    elif isinstance(value, list):
        for child in value:
            found = walk(child)
            if found:
                return found
    return None

found = walk(payload.get("data", payload) if isinstance(payload, dict) else payload)
if not found:
    raise SystemExit("no id field found in JSON payload")
print(found)
PY
}

integration_assert_success() {
    file="$1"
    python3 - "$file" <<'PY'
import json
import sys

with open(sys.argv[1], "r", encoding="utf-8") as handle:
    payload = json.load(handle)

if isinstance(payload, dict) and payload.get("success") is False:
    raise SystemExit(f"CLI response success=false: {sys.argv[1]}")
PY
}

integration_bootstrap_machine_tenant() {
    path=$(integration_run auth-machine-connect \
        auth machine connect \
        --tenant "$ABBOT_IT_TENANT" \
        --username "$ABBOT_IT_USERNAME" \
        --key "$ABBOT_IT_KEY" \
        --key-name "$ABBOT_IT_USERNAME")
    integration_assert_success "$path"
    echo "Tenant: $ABBOT_IT_TENANT"
    echo "Temp HOME: $ABBOT_IT_HOME"
}
