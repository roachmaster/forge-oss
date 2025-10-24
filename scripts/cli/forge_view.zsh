forge_view() {
  # ---------------------------------------------------------------------------
  # If no argument -> show current crate immediately and return
  # ---------------------------------------------------------------------------
  if [[ -z "$1" ]]; then
    if [[ -n "$FORGE_CURRENT_CRATE" ]]; then
      forge_log "INFO" "Current Forge crate view: $FORGE_CURRENT_CRATE"
      forge_log "INFO" "Path: $FORGE_ROOT/crates/$FORGE_CURRENT_CRATE"
    else
      forge_log "WARN" "No current crate view is set."
    fi
    return 0
  fi

  forge_log "INFO" "Starting forge_view command..."

  # Ensure FORGE_ROOT is set
  if [[ -z "$FORGE_ROOT" ]]; then
    forge_log "ERROR" "FORGE_ROOT is not set. Please export it before using forge_view."
    return 1
  fi

  local env_file="$FORGE_ROOT/.forge_env"
  local crate="$1"
  local crate_dir="$FORGE_ROOT/crates/$crate"

  forge_log "DEBUG" "FORGE_ROOT = $FORGE_ROOT"
  forge_log "DEBUG" "ENV file = $env_file"
  forge_log "DEBUG" "Arg = $crate"

  # Clear
  if [[ "$crate" == "--clear" ]]; then
    forge_log "INFO" "Clearing current crate view..."
    unset FORGE_CURRENT_CRATE
    if [[ -f "$env_file" ]]; then
      grep -v '^export FORGE_CURRENT_CRATE=' "$env_file" > "$env_file.tmp" || true
      mv "$env_file.tmp" "$env_file"
    fi
    forge_log "SUCCESS" "Cleared current crate view."
    return 0
  fi

  # Validate crate exists
  forge_log "INFO" "Validating crate existence..."
  if [[ ! -d "$crate_dir" ]]; then
    forge_log "ERROR" "Crate '$crate' not found under $FORGE_ROOT/crates/"
    return 1
  fi

  # Persist
  forge_log "INFO" "Setting current crate to '$crate'..."
  export FORGE_CURRENT_CRATE="$crate"
  mkdir -p "$FORGE_ROOT"

  # Safely rewrite env file
  if [[ -f "$env_file" ]]; then
    grep -v '^export FORGE_CURRENT_CRATE=' "$env_file" > "$env_file.tmp" || true
    mv "$env_file.tmp" "$env_file"
  fi
  echo "export FORGE_CURRENT_CRATE=\"$crate\"" >> "$env_file"

  forge_log "SUCCESS" "Forge crate view set successfully."
  forge_log "DETAIL" "Crate: $crate"
  forge_log "DETAIL" "Path: $crate_dir"
  forge_log "DETAIL" "Persisted to: $env_file"
}
