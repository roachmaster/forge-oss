# -----------------------------------------------------------------------------
# build_view()
# Builds the current Forge crate safely, with full argument parsing.
#
# Usage:
#   build_view
#   build_view --release
#   build_view --report json --out /tmp/build.json
#   build_view --report yaml --out /tmp/build.yaml
# -----------------------------------------------------------------------------
build_view() {
  set +e  # Disable "exit-on-error" in this function

  # ---------------------------------------------
  # Argument parsing (robust, expandable)
  # ---------------------------------------------
  local release=0
  local report_format=""
  local report_path=""

  zparseopts -D -E \
    {r,-release}=release \
    {-report}:=report_format \
    {-out}:=report_path

  # Extract actual values
  [[ -n "$report_format" ]] && report_format="${report_format[2]}"
  [[ -n "$report_path" ]] && report_path="${report_path[2]}"
  [[ -n "$release" ]] && release=1

  # ---------------------------------------------
  # Pre-flight
  # ---------------------------------------------
  forge_log "INFO" "Starting build_view..."
  if [[ -z "$FORGE_CURRENT_CRATE" ]]; then
    forge_log "ERROR" "No current crate set. Run 'forge_view <crate>' first."
    set -e
    return 1
  fi

  forge_log "INFO" "Building crate: $FORGE_CURRENT_CRATE"
  local start_time=$(date '+%s')

  # ---------------------------------------------
  # Cargo build invocation
  # ---------------------------------------------
  local cargo_args=(-p "$FORGE_CURRENT_CRATE")
  (( release )) && cargo_args+=(--release)

  cargo build "${cargo_args[@]}"
  local exit_code=$?

  local end_time=$(date '+%s')
  local duration=$((end_time - start_time))
  local build_status="$([[ $exit_code -eq 0 ]] && echo success || echo failure)"

  # ---------------------------------------------
  # Logging summary
  # ---------------------------------------------
  if (( exit_code == 0 )); then
    forge_log "SUCCESS" "Build completed successfully for $FORGE_CURRENT_CRATE."
  else
    forge_log "ERROR" "Build failed for $FORGE_CURRENT_CRATE (SSH session preserved)."
  fi

  # ---------------------------------------------
  # Optional report output
  # ---------------------------------------------
  if [[ -n "$report_format" ]]; then
    local timestamp="$(date -Iseconds)"
    local outfile="${report_path:-$FORGE_ROOT/build_reports/$FORGE_CURRENT_CRATE.$report_format}"
    mkdir -p "$(dirname "$outfile")"

    case "$report_format" in
      json)
        cat > "$outfile" <<EOF
{
  "crate": "$FORGE_CURRENT_CRATE",
  "status": "$build_status",
  "release": $release,
  "duration_sec": $duration,
  "timestamp": "$timestamp"
}
EOF
        ;;
      yaml)
        cat > "$outfile" <<EOF
crate: "$FORGE_CURRENT_CRATE"
status: "$build_status"
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

  set -e  # Restore shell safety
  return 0
}
