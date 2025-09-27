# add_mod_to_lib <crate_dir> <mod_name>
add_mod_to_lib() {
  emulate -L zsh
  set -euo pipefail
  local CRATE_DIR="$1" MOD="$2"
  local LIB="$CRATE_DIR/src/lib.rs"
  [[ -f "$LIB" ]] || { print -u2 -- "Missing $LIB"; return 1; }

  # already present?
  grep -qE "^\\s*pub\\s+mod\\s+${MOD}\\s*;" "$LIB" && return 0

  printf '\npub mod %s;\n' "$MOD" >> "$LIB"
}
