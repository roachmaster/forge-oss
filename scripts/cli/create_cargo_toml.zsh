# create_cargo_toml <crate_dir> <crate_name>
create_cargo_toml() {
  emulate -L zsh
  set -euo pipefail
  local CRATE_DIR="$1" NAME="$2"

  cat > "$CRATE_DIR/Cargo.toml" <<TOML
[package]
name = "${NAME}"
version = "0.1.0"
edition = "2021"

[dependencies]
TOML
}
