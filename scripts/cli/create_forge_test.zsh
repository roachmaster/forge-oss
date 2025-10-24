# -----------------------------------------------------------------------------
# create_forge_test()
# Creates a Forge-style integration test file in the current crate.
#
# Usage:
#   create_forge_test --name expand_tests
# -----------------------------------------------------------------------------
create_forge_test() {
  set +e

  # ---------------------------------------------
  # Argument parsing
  # ---------------------------------------------
  local test_name=""
  while [[ $# -gt 0 ]]; do
    case "$1" in
      --name)
        shift
        test_name="$1"
        ;;
      *)
        forge_log "WARN" "Unknown argument: $1"
        ;;
    esac
    shift
  done

  # ---------------------------------------------
  # Validation
  # ---------------------------------------------
  if [[ -z "$test_name" ]]; then
    forge_log "ERROR" "Missing required argument: --name <test_name>"
    echo "Usage: create_forge_test --name <test_name>"
    return 1
  fi

  if [[ -z "$FORGE_CURRENT_CRATE" ]]; then
    forge_log "ERROR" "No current crate selected. Run 'forge_view <crate>' first."
    return 1
  fi

  # ---------------------------------------------
  # Paths
  # ---------------------------------------------
  local crate_path="$FORGE_ROOT/crates/$FORGE_CURRENT_CRATE"
  local tests_dir="$crate_path/tests"
  local test_file="$tests_dir/${test_name}.rs"

  mkdir -p "$tests_dir"

  # ---------------------------------------------
  # File generation
  # ---------------------------------------------
  cat > "$test_file" <<EOF
// ============================================================================
// Forge Integration Test
// File: tests/${test_name}.rs
// Crate: $FORGE_CURRENT_CRATE
// Description: Integration test scaffold generated via create_forge_test.
// ============================================================================

use serde_json::json;
use $FORGE_CURRENT_CRATE::*;

#[test]
fn ${test_name}_basic_sanity() {
    // Placeholder Forge test
    let input = json!({"key": "value"});
    assert_eq!(input["key"], "value");
    println!("✅ ${test_name}_basic_sanity executed successfully.");
}
EOF

  # ---------------------------------------------
  # Confirmation
  # ---------------------------------------------
  forge_log "SUCCESS" "Created new Forge integration test: ${test_name}.rs"
  forge_log "DETAIL"  "Location: $test_file"

  set -e
}
