#!/bin/sh

set -eu

SCRIPT_DIR=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
. "$SCRIPT_DIR/lib.sh"

ABBOT_IT_HOST=$(integration_host_arg "$@")
ABBOT_BIN=$(integration_abbot_bin)
export ABBOT_IT_HOST ABBOT_BIN

integration_require_tools
integration_new_context "api"
trap integration_cleanup EXIT INT TERM

integration_bootstrap_machine_tenant

MODEL=$(integration_slug "cli_widgets_${ABBOT_IT_TENANT}")
MODEL_BODY='{
  "status": "active",
  "description": "CLI integration disposable model",
  "fields": {
    "name": { "type": "text", "required": true, "tracked": true, "searchable": true },
    "status": { "type": "text", "tracked": true },
    "count": { "type": "integer" }
  }
}'

path=$(integration_run describe-list-before api describe list | tail -n 1)
integration_assert_success "$path"

path=$(integration_run_stdin describe-create "$MODEL_BODY" api describe create "$MODEL" | tail -n 1)
integration_assert_success "$path"

path=$(integration_run describe-get api describe get "$MODEL" | tail -n 1)
integration_assert_success "$path"

path=$(integration_run describe-fields-list api describe fields list "$MODEL" | tail -n 1)
integration_assert_success "$path"

CREATE_BODY='[
  { "name": "alpha", "status": "new", "count": 1 },
  { "name": "beta", "status": "new", "count": 2 }
]'
path=$(integration_run_stdin data-create "$CREATE_BODY" api data create "$MODEL" | tail -n 1)
integration_assert_success "$path"
RECORD_ID=$(integration_json_first_id "$path")

path=$(integration_run data-list api data list --limit 10 "$MODEL" | tail -n 1)
integration_assert_success "$path"

path=$(integration_run data-get api data get "$MODEL" "$RECORD_ID" | tail -n 1)
integration_assert_success "$path"

UPDATE_BODY='{ "name": "alpha", "status": "updated", "count": 3 }'
path=$(integration_run_stdin data-put "$UPDATE_BODY" api data put "$MODEL" "$RECORD_ID" | tail -n 1)
integration_assert_success "$path"

path=$(integration_run find-query api find --limit 10 query "$MODEL" | tail -n 1)
integration_assert_success "$path"

path=$(integration_run tracked-list api tracked list "$MODEL" "$RECORD_ID" | tail -n 1)
integration_assert_success "$path"

path=$(integration_run data-delete-record api data delete-record "$MODEL" "$RECORD_ID" | tail -n 1)
integration_assert_success "$path"

path=$(integration_run trashed-list api trashed list | tail -n 1)
integration_assert_success "$path"

path=$(integration_run trashed-model api trashed model "$MODEL" | tail -n 1)
integration_assert_success "$path"

path=$(integration_run describe-delete api describe delete "$MODEL" | tail -n 1)
integration_assert_success "$path"

echo "api-basic passed for $ABBOT_IT_HOST using tenant $ABBOT_IT_TENANT and model $MODEL"
