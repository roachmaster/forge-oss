# scaffold_bdd_scenario <scenario> <statement1> <statement2> ...
# Example:
#   scaffold_bdd_scenario de_duplicate_repeated_paths \
#     set_up_and_get_paths_with_duplicates \
#     execute_and_return_build_build_simple_tree \
#     verify_no_duplicate_dirs \
#     and_verify_no_duplicate_files
scaffold_bdd_scenario() {
  emulate -L zsh
  set -e

  if [[ $# -lt 2 ]]; then
    echo "usage: scaffold_bdd_scenario <scenario> <statement1> [statement2 ...]" >&2
    return 2
  fi

  local CRATE="forge-fs"
  local MODNAME="glue"
  local SCENARIO="$1"; shift

  # repo root
  local ROOT
  if ROOT=$(git rev-parse --show-toplevel 2>/dev/null); then
    :
  elif [[ -f .forge-root ]]; then
    ROOT="$PWD"
  else
    echo "Run inside a git repo or add .forge-root at the root." >&2
    return 1
  fi

  local TESTS_DIR="$ROOT/crates/$CRATE/tests"
  local MOD_DIR="$TESTS_DIR/$MODNAME"
  local MOD_RS="$MOD_DIR/mod.rs"
  local GLUE_RS="$MOD_DIR/${SCENARIO}.rs"
  local SCENARIO_RS="$TESTS_DIR/scenario_${SCENARIO}.rs"

  # ensure base mod exists
  if [[ ! -d "$MOD_DIR" ]]; then
    echo "Base test module '$MODNAME' missing — creating..."
    create_tests_mod "$CRATE" "$MODNAME"
  fi

  # ensure glue file & add any missing step stubs
  if [[ ! -f "$GLUE_RS" ]]; then
    echo "Creating $GLUE_RS"
    {
      echo "//! Glue: steps for scenario \"$SCENARIO\""
      echo
      echo "// NOTE: Fill in real signatures/returns later. Placeholders created for BDD scaffolding."
      echo
      for fn in "$@"; do
        echo "pub fn ${fn}() {"
        echo "    // TODO: implement step: ${fn}"
        echo "}"
        echo
      done
    } > "$GLUE_RS"
  else
    for fn in "$@"; do
      # fixed-string match to avoid regex issues
      if ! grep -Fq "pub fn ${fn}(" "$GLUE_RS"; then
        {
          echo
          echo "pub fn ${fn}() {"
          echo "    // TODO: implement step: ${fn}"
          echo "}"
        } >> "$GLUE_RS"
        echo "Added missing step function ${fn}() to $GLUE_RS"
      fi
    done
  fi

  # ensure mod.rs references this scenario module
  if [[ ! -f "$MOD_RS" ]]; then
    echo "pub mod ${SCENARIO};" > "$MOD_RS"
  elif ! grep -Fq "pub mod ${SCENARIO};" "$MOD_RS"; then
    echo "pub mod ${SCENARIO};" >> "$MOD_RS"
    echo "Updated $MOD_RS with: pub mod ${SCENARIO};"
  fi

  # write/overwrite scenario facade
  {
    echo "//! Scenario facade: ${SCENARIO}"
    echo
    echo "mod ${MODNAME};"
    echo
    echo "use ${MODNAME}::${SCENARIO}::{"
    local first=1
    for fn in "$@"; do
      if [[ $first -eq 1 ]]; then
        printf "    %s" "$fn"
        first=0
      else
        printf ",\n    %s" "$fn"
      fi
    done
    echo
    echo "};"
    echo
    echo "#[test]"
    echo "fn scenario_${SCENARIO}() {"
    for fn in "$@"; do
      echo "    ${fn}();"
    done
    echo "}"
  } > "$SCENARIO_RS"

  echo "Scaffold complete:"
  echo "  - $GLUE_RS"
  echo "  - $MOD_RS (ensured)"
  echo "  - $SCENARIO_RS"
}
