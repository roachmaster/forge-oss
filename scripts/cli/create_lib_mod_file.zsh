# create_lib_mod_file <crate_dir> <mod_name>
create_lib_mod_file() {
  emulate -L zsh
  set -euo pipefail
  local CRATE_DIR="$1" MOD="$2"
  local FILE="$CRATE_DIR/src/${MOD}.rs"
  [[ -f "$FILE" ]] && return 0

  cat > "$FILE" <<RS
//! ${MOD} module

// add API for ${MOD} here

#[cfg(test)]
mod tests {
    #[test]
    fn compiles() {
        assert!(true);
    }
}
RS
}
