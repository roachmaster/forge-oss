#!/usr/bin/env sh
set -eu

. "$(dirname "$0")/env.sh"

BIN="$FORGE_ROOT/target/release/forge-workbenchd"
BIND_ADDR="${FORGE_BIND_ADDR:-0.0.0.0:8787}"
ROOT_PATH="${FORGE_ROOT_PATH:-$FORGE_ROOT}"  # default to repo root
TOKEN_OPT=""
[ -n "${FORGE_TOKEN:-}" ] && TOKEN_OPT="--token $FORGE_TOKEN"

if [ ! -x "$BIN" ]; then
  echo "Server binary not found at $BIN. Run scripts/build_server.sh first."
  exit 1
fi

echo "[forge] Running server: $BIN --bind $BIND_ADDR --root $ROOT_PATH $TOKEN_OPT"
exec "$BIN" --bind "$BIND_ADDR" --root "$ROOT_PATH" $TOKEN_OPT
