# create_test_mod_file <crate> <modname> <file>
# Example: create_test_mod_file forge-fs glue build_a_tree_from_mixed_files_and_dirs
create_test_mod_file() {
  emulate -L zsh
  set -e

  if [[ $# -lt 3 ]]; then
    echo "usage: create_test_mod_file <crate> <modname> <file>" >&2
    return 2
  fi

  local CRATE="$1"
  local MODNAME="$2"
  local FILE="$3"

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

  local BASE_DIR="$ROOT/crates/$CRATE/tests/$MODNAME"
  local MOD_FILE="$BASE_DIR/mod.rs"
  local NEW_FILE="$BASE_DIR/${FILE}.rs"

  # --- ensure base mod exists ---
  if [[ ! -d "$BASE_DIR" ]]; then
    echo "Base module '$MODNAME' missing — creating..."
    create_tests_mod "$CRATE" "$MODNAME"
  fi

  # --- create new file if missing ---
  if [[ ! -f "$NEW_FILE" ]]; then
    echo "// Scenario: $FILE" > "$NEW_FILE"
    echo "Created $NEW_FILE"
  else
    echo "$NEW_FILE already exists"
  fi

  # --- update mod.rs ---
  if ! grep -q "pub mod $FILE;" "$MOD_FILE"; then
    echo "pub mod $FILE;" >> "$MOD_FILE"
    echo "Added pub mod $FILE; to $MOD_FILE"
  else
    echo "mod.rs already references $FILE"
  fi
}
