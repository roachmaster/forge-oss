#!/usr/bin/env zsh
# scaffold_bdd_from_yaml.zsh <path/to/yaml>
# When sourced (by forge-scripts.sh), only define the function.
# When executed directly, run it with the provided args.

emulate -L zsh
set -euo pipefail

scaffold_bdd_from_yaml() {
  emulate -L zsh
  set -euo pipefail

  echo "[DEBUG] Raw args: $*"
  echo "[DEBUG] Arg count: $#"

  if [[ $# -ne 1 ]]; then
    echo "usage: scaffold_bdd_from_yaml.zsh <path/to/yaml>" >&2
    return 2
  fi

  local YAML_PATH="$1"
  echo "[DEBUG] YAML_PATH: $YAML_PATH"

  if [[ ! -f "$YAML_PATH" ]]; then
    echo "YAML not found: $YAML_PATH" >&2
    return 2
  fi

  # Ensure repo root
  local ROOT
  if ROOT=$(git rev-parse --show-toplevel 2>/dev/null); then
    :
  elif [[ -f .forge-root ]]; then
    ROOT="$PWD"
  else
    echo "Run inside a git repo or add .forge-root at repo root." >&2
    return 1
  fi

  # Load helper functions
  source "$ROOT/forge-scripts.sh"

  # Need yq
  if ! command -v yq >/dev/null 2>&1; then
    echo "yq not found. Install: brew install yq" >&2
    return 1
  fi

  local crate test_mod count
  crate=$(yq -r '.crate // "forge-fs"' "$YAML_PATH")
  test_mod=$(yq -r '.test_mod // "glue"' "$YAML_PATH")
  count=$(yq -r '.scenarios | length' "$YAML_PATH")

  echo "[DEBUG] crate=$crate"
  echo "[DEBUG] test_mod=$test_mod"
  echo "[DEBUG] scenario count=$count"

  local i name
  for i in $(seq 0 $((count - 1))); do
    name=$(yq -r ".scenarios[$i].name" "$YAML_PATH")
    echo "==> Scaffolding scenario: $name"

    create_test_mod_file "$crate" "$test_mod" "$name"

    # collect steps
    local steps=()
    while IFS= read -r step; do
      [[ -z "$step" || "$step" == "null" ]] && continue
      steps+=("$step")
    done < <(yq -r ".scenarios[$i].steps[]?" "$YAML_PATH")

    echo "[DEBUG] steps for $name: ${steps[*]}"
    scaffold_bdd_scenario "$name" "${steps[@]}"
  done

  echo "All scenarios scaffolded from $YAML_PATH"
}

# --- Sourcing guard ---
# If this file is *sourced* (e.g., by forge-scripts.sh), do NOT execute.
# In zsh, $ZSH_EVAL_CONTEXT contains ":file" when being sourced.
if [[ "${ZSH_EVAL_CONTEXT:-}" == *":file"* ]]; then
  return 0
fi

# If executed directly, run the function with CLI args.
scaffold_bdd_from_yaml "$@"
