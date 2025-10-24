#!/usr/bin/env zsh
# -----------------------------------------------------------------------------
# test_view()
# Runs cargo test for the currently selected Forge crate safely.
#
# Usage:
#   test_view
#   test_view --release
#   test_view --report json --out /tmp/test.json
#   test_view --report yaml --out /tmp/test.yaml
# -----------------------------------------------------------------------------
test_view() {
  set +e  # prevent SSH exit on failure

  # ---------------------------------------------
  # Argument parsing
  # ---------------------------------------------
  local release=0
  local report_format=""
  local report_path=""

  zparseopts -D -E \
    {r,-release}=release \
    {-report}:=report_format \
    {-out}:=report_path

  [[ -n "$report_format" ]] && report_format="${report_format[2]}"
  [[ -n "$report_path" ]] && report_path="${report_path[2]}"
  [[ -n "$release" ]] && release=1

  # ---------------------------------------------
  # Pre-flight validation
  # ---------------------------------------------
  forge_log "INFO" "Starting test_view..."
  if [[ -z "$FORGE_CURRENT_CRATE" ]]; then
    forge_log "ERROR" "No current crate set. Run 'forge_view <crate>' first."
    set -e
    return 1
  fi

  forge_log "INFO" "Testing crate: $FORGE_CURRENT_CRATE"
  local start_time=$(date '+%s')

  # ---------------------------------------------
  # Cargo test execution
  # ---------------------------------------------
  local cargo_args=(-p "$FORGE_CURRENT_CRATE" -- --nocapture)
  (( release )) && cargo_args=(--release "${cargo_args[@]}")

  cargo test "${cargo_args[@]}"
  local exit_code=$?

  local end_time=$(date '+%s')
  local duration=$((end_time - start_time))
  local test_status="$([[ $exit_code -eq 0 ]] && echo success || echo failure)"

  # ---------------------------------------------
  # Summary
  # ---------------------------------------------
  if (( exit_code == 0 )); then
    forge_log "SUCCESS" "Tests passed for $FORGE_CURRENT_CRATE."
  else
    forge_log "ERROR" "Some tests failed in $FORGE_CURRENT_CRATE (SSH preserved)."
  fi

  # ---------------------------------------------
  # Optional report output
  # ---------------------------------------------
  if [[ -n "$report_format" ]]; then
    local timestamp="$(date -Iseconds)"
    local outfile="${report_path:-$FORGE_ROOT/test_reports/$FORGE_CURRENT_CRATE.$report_format}"
    mkdir -p "$(dirname "$outfile")"

    case "$report_format" in
      json)
        cat > "$outfile" <<EOF
{
  "crate": "$FORGE_CURRENT_CRATE",
  "status": "$test_status",
  "release": $release,
  "duration_sec": $duration,
  "timestamp": "$timestamp"
}
EOF
        ;;
      yaml)
        cat > "$outfile" <<EOF
crate: "$FORGE_CURRENT_CRATE"
status: "$test_status"
release: $release
duration_sec: $duration
timestamp: "$timestamp"
EOF
        ;;
      *)
        forge_log "WARN" "Unknown report format '$report_format' (use json|yaml)"
        ;;
    esac

    forge_log "DETAIL" "Report written to: $outfile"
  fi

  forge_log "DETAIL" "Output: target/${release:+release}${release==0:+debug}"
  forge_log "SUCCESS" "Done."

  set -e
  return 0
}
