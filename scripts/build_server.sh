#!/usr/bin/env sh
set -eu

. "$(dirname "$0")/env.sh"

echo "[forge] Building server (release)…"
cd "$FORGE_ROOT"
cargo build --release -p forge-workbenchd

BIN="$FORGE_ROOT/target/release/forge-workbenchd"
if [ ! -x "$BIN" ]; then
  echo "Build failed, missing binary at $BIN"
  exit 1
fi

echo "[forge] Server built: $BIN"
