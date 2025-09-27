# current_crate_view [PATH]
# Show basic info (Cargo.toml path + src tree) for a crate directory (default: .)
current_crate_view() {
  emulate -L zsh
  set -euo pipefail
  local DIR="${1:-.}"

  if [[ -f "$DIR/Cargo.toml" ]]; then
    echo "==> $DIR/Cargo.toml"
    sed -n '1,40p' "$DIR/Cargo.toml"
  else
    echo "No Cargo.toml in $DIR" >&2
  fi

  echo
  echo "==> $DIR/src"
  if command -v tree >/dev/null; then
    tree -a "$DIR/src" 2>/dev/null || ls -la "$DIR/src"
  else
    ls -la "$DIR/src"
  fi
}
