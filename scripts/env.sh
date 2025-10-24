#!/usr/bin/env sh
set -eu

# Resolve and source project_root.sh
. "$(dirname "$0")/project_root.sh"

export RUST_LOG="${RUST_LOG:-info}"
export CARGO_TERM_COLOR=always
export PATH="$FORGE_ROOT/bin:$PATH"
export WASM_TARGET="wasm32-unknown-unknown"

printf "Environment loaded. FORGE_ROOT=%s\n" "$FORGE_ROOT"
