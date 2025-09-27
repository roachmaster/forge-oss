# find_repo_root -> prints absolute path to repo root and exits non-zero on failure
find_repo_root() {
  emulate -L zsh
  set -euo pipefail

  local ROOT=""
  if ROOT=$(git rev-parse --show-toplevel 2>/dev/null); then
    print -r -- "$ROOT"
    return 0
  fi
  if [[ -f .forge-root ]]; then
    print -r -- "$PWD"
    return 0
  fi

  # Walk up to find .forge-root
  local CUR="$PWD"
  for _ in {1..10}; do
    if [[ -f "$CUR/.forge-root" ]]; then
      print -r -- "$CUR"
      return 0
    fi
    CUR="${CUR:h}"
  done

  echo "Cannot determine repo root (git or .forge-root required)" >&2
  return 1
}
