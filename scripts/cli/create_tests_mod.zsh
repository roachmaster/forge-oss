# create_tests_mod <crate> <modname>
# Ensure crates/<crate>/tests/<modname>/mod.rs exists (creates dirs if missing)
create_tests_mod() {
  emulate -L zsh
  set -e

  if [[ $# -lt 2 ]]; then
    echo "usage: create_tests_mod <crate> <modname>" >&2
    return 2
  fi

  local CRATE="$1"
  local MODNAME="$2"

  # --- locate repo root ---
  local ROOT
  if ROOT=$(git rev-parse --show-toplevel 2>/dev/null); then
    :
  elif [[ -f .forge-root ]]; then
    ROOT="$PWD"
  else
    echo "Run inside a git repo or add .forge-root at the root." >&2
    return 1
  fi

  # --- ensure folder + mod.rs ---
  local MOD_DIR="$ROOT/crates/$CRATE/tests/$MODNAME"
  mkdir -p "$MOD_DIR"

  local MOD_FILE="$MOD_DIR/mod.rs"
  if [[ ! -f "$MOD_FILE" ]]; then
    echo "// $MODNAME module for integration test scenarios" > "$MOD_FILE"
    echo "Created $MOD_FILE"
  else
    echo "$MOD_FILE already exists"
  fi
}
