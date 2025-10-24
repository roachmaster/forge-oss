#!/usr/bin/env sh
# Determine the absolute path to the Forge repo root.
# This allows other scripts to source it and get FORGE_ROOT defined.

set -eu

# Resolve the directory of this script (resilient even if run via symlink)
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
FORGE_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

export FORGE_ROOT
