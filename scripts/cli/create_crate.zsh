# create_crate <name> [--lib|--bin] [--dir DIR]
create_crate() {
  emulate -L zsh
  setopt LOCAL_OPTIONS ERR_RETURN NO_UNSET PIPE_FAIL

  if [[ $# -lt 1 ]]; then
    print -u2 -- "usage: create_crate <name> [--lib|--bin] [--dir DIR]"
    return 2
  fi

  local NAME="$1"; shift
  local KIND="lib"   # "lib" | "bin"
  local DIR="crates"
  while [[ $# -gt 0 ]]; do
    case "$1" in
      --lib) KIND="lib"; shift;;
      --bin) KIND="bin"; shift;;
      --dir) DIR="${2:-crates}"; shift 2;;
      *) print -u2 -- "unknown option: $1"; return 2;;
    esac
  done

  local ROOT; ROOT=$(find_repo_root) || { print -u2 -- "cannot find repo root"; return 1; }
  local CRATE_DIR="$ROOT/${DIR}/${NAME}"
  mkdir -p "$CRATE_DIR/src"

  # Create minimal files only if missing (idempotent)
  if [[ ! -f "$CRATE_DIR/Cargo.toml" ]]; then
    create_cargo_toml "$CRATE_DIR" "$NAME"
  fi
  if [[ "$KIND" == "bin" ]]; then
    [[ -f "$CRATE_DIR/src/main.rs" ]] || create_bin_main "$CRATE_DIR" "$NAME"
  else
    [[ -f "$CRATE_DIR/src/lib.rs"  ]] || create_lib_skeleton "$CRATE_DIR" "$NAME"
  fi

  # Always ensure workspace membership; Python helper is idempotent
  add_workspace_member "$ROOT" "${DIR}/${NAME}"

  print -- "Crate ready: ${CRATE_DIR}"
  return 0
}
