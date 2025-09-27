#!/usr/bin/env sh
set -eu

# Load environment
. "$(dirname "$0")/env.sh"

# Ensure wasm target
if ! rustup target list --installed | grep -q "$WASM_TARGET"; then
  echo "Installing target: $WASM_TARGET"
  rustup target add "$WASM_TARGET"
fi

# Ensure trunk (for Rust+Wasm web dev)
if ! command -v trunk >/dev/null 2>&1; then
  echo "Installing trunk..."
  cargo install trunk
fi

cd "$FORGE_ROOT/crates/forge-web-ui"
trunk serve --address 0.0.0.0 --port 8080
