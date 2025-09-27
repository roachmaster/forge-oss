# create_lib_skeleton <crate_dir> <crate_name>
create_lib_skeleton() {
  emulate -L zsh
  set -euo pipefail
  local CRATE_DIR="$1" NAME="$2"

  cat > "$CRATE_DIR/src/lib.rs" <<RS
//! ${NAME} library

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
RS
}
