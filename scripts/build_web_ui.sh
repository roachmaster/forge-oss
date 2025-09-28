#!/usr/bin/env sh
set -eu

# Load environment (expects FORGE_ROOT and WASM_TARGET at least)
. "$(dirname "$0")/env.sh"

# Ensure wasm target
if ! rustup target list --installed | grep -q "$WASM_TARGET"; then
  echo "Installing target: $WASM_TARGET"
  rustup target add "$WASM_TARGET"
fi

# Ensure trunk (Rust+Wasm bundler)
if ! command -v trunk >/dev/null 2>&1; then
  echo "Installing trunk..."
  cargo install trunk
fi

echo "[forge] Building web UI (release) with trunk…"
cd "$FORGE_ROOT/crates/forge-web-ui"
trunk build --release

echo "[forge] Web UI built: $FORGE_ROOT/crates/forge-web-ui/dist/"
