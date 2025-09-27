# create_bin_main <crate_dir> <crate_name>
create_bin_main() {
  emulate -L zsh
  set -euo pipefail
  local CRATE_DIR="$1" NAME="$2"

  cat > "$CRATE_DIR/src/main.rs" <<'RS'
fn main() {
    println!("ready");
}
RS
}
