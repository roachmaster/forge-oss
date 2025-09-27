#!/usr/bin/env bash
# Build the whole workspace or a single crate.
# Usage:
#   ./scripts/build.sh [--debug] [--no-wasm] [--crate CRATE_NAME]

set -euo pipefail

BUILD_MODE="--release"
BUILD_WASM=1
TARGET_CRATE=""

while [[ $# -gt 0 ]]; do
  case "$1" in
    --debug)
      BUILD_MODE=""
      shift
      ;;
    --no-wasm)
      BUILD_WASM=0
      shift
      ;;
    --crate)
      if [[ $# -lt 2 ]]; then
        echo "ERROR: --crate requires a crate name" >&2
        exit 2
      fi
      TARGET_CRATE="$2"
      shift 2
      ;;
    *)
      echo "Unknown option: $1" >&2
      exit 2
      ;;
  esac
done

# --- Locate repo root ---
if ROOT=$(git rev-parse --show-toplevel 2>/dev/null); then
  :
else
  echo "ERROR: could not locate repo root (git repo missing)." >&2
  exit 1
fi
export FORGE_ROOT="$ROOT"

echo "==> Forge build starting"
echo "    FORGE_ROOT: $FORGE_ROOT"
echo "    Mode:      ${BUILD_MODE:---release}"
if [[ -n "$TARGET_CRATE" ]]; then
  echo "    Crate:     $TARGET_CRATE"
else
  echo "    Crate:     (workspace)"
fi
echo "    WASM:      $([ $BUILD_WASM -eq 1 ] && echo yes || echo no)"
echo

# --- Cargo build ---
if [[ -n "$TARGET_CRATE" ]]; then
  echo "==> cargo build $BUILD_MODE -p $TARGET_CRATE"
  ( cd "$FORGE_ROOT" && cargo build $BUILD_MODE -p "$TARGET_CRATE" )
else
  echo "==> cargo build $BUILD_MODE --workspace"
  ( cd "$FORGE_ROOT" && cargo build $BUILD_MODE --workspace )
fi

# --- Web UI (wasm) ---
if [[ $BUILD_WASM -eq 1 && -z "$TARGET_CRATE" ]]; then
  if command -v trunk >/dev/null 2>&1; then
    echo
    echo "==> trunk build $BUILD_MODE (forge-web-ui)"
    ( cd "$FORGE_ROOT/crates/forge-web-ui" && trunk build $BUILD_MODE )
    echo "    dist: $FORGE_ROOT/crates/forge-web-ui/dist"
  else
    echo
    echo "!! trunk not found; skipping wasm build."
    echo "   Install: cargo install trunk && rustup target add wasm32-unknown-unknown"
  fi
fi

echo
echo "==> Build complete."
