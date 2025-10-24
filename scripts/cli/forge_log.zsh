# -----------------------------------------------------------------------------
# forge_log — universal Forge logger
# -----------------------------------------------------------------------------
# Usage:
#   forge_log "INFO" "Starting build..."
#   forge_log "ERROR" "Something failed."
#   forge_log "SUCCESS" "Done!"
#
# Features:
#   - Zsh-safe (no parameter-unset errors)
#   - Color-coded by level
#   - Timestamps for each message
# -----------------------------------------------------------------------------
forge_log() {
  # capture arg count before locals (zsh safe)
  local argc=$#
  local level="${1:-INFO}"

  # shift only if there’s more than one argument
  if (( argc > 1 )); then
    shift
  fi

  local msg="$*"
  local ts="$(date '+%H:%M:%S')"

  # ANSI color palette
  local reset="\033[0m"
  local blue="\033[34m"
  local green="\033[32m"
  local yellow="\033[33m"
  local red="\033[31m"
  local magenta="\033[35m"
  local cyan="\033[36m"

  local color="$reset"
  case "$level" in
    INFO)    color="$blue" ;;
    DEBUG)   color="$cyan" ;;
    WARN)    color="$yellow" ;;
    ERROR)   color="$red" ;;
    SUCCESS) color="$green" ;;
    DETAIL)  color="$magenta" ;;
  esac

  echo -e "${color}[${ts}] [${level}]${reset} ${msg}"
}
