# -----------------------------------------------------------------------------
# install_view()
# Installs the current Forge crate as a binary safely.
# -----------------------------------------------------------------------------
install_view() {
  set +e  # prevent SSH exit on failure

  # ---------------------------------------------
  # Argument parsing
  # ---------------------------------------------
  local force=0
  local report_format=""
  local report_path=""

  zparseopts -D -E \
    {f,-force}=force \
    {-report}:=report_format \
    {-out}:=report_path

  [[ -n "$report_format" ]] && report_format="${report_format[2]}"
  [[ -n "$report_path" ]] && report_path="${report_path[2]}"
  [[ -n "$force" ]] && force=1

  # ---------------------------------------------
  # Pre-flight
  # ---------------------------------------------
  forge_log "INFO" "Starting install_view..."
  if [[ -z "$FORGE_CURRENT_CRATE" ]]; then
    forge_log "ERROR" "No current crate set. Run 'forge_view <crate>' first."
    set -e
    return 1
  fi

  local crate="$FORGE_CURRENT_CRATE"
  forge_log "INFO" "Installing crate: $crate"
  local start_time=$(date '+%s')

  # ---------------------------------------------
  # Run cargo install safely
  # ---------------------------------------------
  local cargo_args=(install --path "$FORGE_ROOT/crates/$crate")
  (( force )) && cargo_args+=(--force)

  local _status="failure"
  if cargo "${cargo_args[@]}"; then
    forge_log "SUCCESS" "Installed crate '$crate' successfully."
    _status="success"
  else
    forge_log "ERROR" "Installation failed for '$crate' (SSH preserved)."
  fi

  # ---------------------------------------------
  # Reporting (optional)
  # ---------------------------------------------
  local end_time=$(date '+%s')
  local duration=$((end_time - start_time))
  local timestamp="$(date -Iseconds)"
  local outfile="${report_path:-$FORGE_ROOT/install_reports/$crate.${report_format:-txt}}"

  if [[ -n "$report_format" ]]; then
    mkdir -p "$(dirname "$outfile")"
    case "$report_format" in
      json)
        cat > "$outfile" <<EOF
{
  "crate": "$crate",
  "status": "$_status",
  "force": $force,
  "duration_sec": $duration,
  "timestamp": "$timestamp"
}
EOF
        ;;
      yaml)
        cat > "$outfile" <<EOF
crate: "$crate"
status: "$_status"
force: $force
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

  forge_log "DETAIL" "Binary installed to: \$HOME/.cargo/bin"
  forge_log "SUCCESS" "Done."

  set -e
  return 0
}
