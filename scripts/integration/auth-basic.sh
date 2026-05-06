#!/bin/sh

set -eu

SCRIPT_DIR=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
. "$SCRIPT_DIR/lib.sh"

ABBOT_IT_HOST=$(integration_host_arg "$@")
ABBOT_BIN=$(integration_abbot_bin)
export ABBOT_IT_HOST ABBOT_BIN

integration_require_tools
integration_new_context "auth"
trap integration_cleanup EXIT INT TERM

integration_bootstrap_machine_tenant

path=$(integration_run auth-list auth list | tail -n 1)
integration_assert_success "$path"

path=$(integration_run doctor doctor | tail -n 1)
integration_assert_success "$path"

path=$(integration_run user-me api user me | tail -n 1)
integration_assert_success "$path"

path=$(integration_run auth-refresh auth refresh | tail -n 1)
integration_assert_success "$path"

echo "auth-basic passed for $ABBOT_IT_HOST using tenant $ABBOT_IT_TENANT"
