#!/usr/bin/env zsh
# =====================================================================
# 🔥 Forge IDE Module Renderer (Direct YAML → Rust)
# =====================================================================

set -e

# ---------------------------------------------------------------------
# 🎨 Colors
# ---------------------------------------------------------------------
BOLD=$(tput bold)
RESET=$(tput sgr0)
CYAN=$(tput setaf 6)
GREEN=$(tput setaf 2)
YELLOW=$(tput setaf 3)
RED=$(tput setaf 1)

# ---------------------------------------------------------------------
# 📂 Paths & Globals
# ---------------------------------------------------------------------
root_dir=$(dirname "$0")/..
cd "$root_dir"

modules_dir="templates/forge_ide/modules"
templates_dir="templates/forge_ide"
rust_dir="crates/forge-ide/src"

# List of modules to process
MODULES=(
  schema
  command
  provider
  router
)

# ---------------------------------------------------------------------
# 🧽 Format Rust file
# ---------------------------------------------------------------------
format_rust() {
  local file=$1

  if command -v rustfmt >/dev/null 2>&1; then
    echo "   ${CYAN}↳ Formatting (edition 2021):${RESET} $file"
    if ! rustfmt --edition 2021 "$file"; then
      echo "   ${RED}⚠ rustfmt failed for:${RESET} $file"
    fi
  else
    echo "   ${YELLOW}⚠ rustfmt not found; skipping format${RESET}"
  fi
}
# ---------------------------------------------------------------------
# 🧩 Render module helper
# ---------------------------------------------------------------------
render_module() {
  local name=$1
  local yaml_src="${modules_dir}/${name}.yaml"
  local mustache="${templates_dir}/${name}.mustache"
  local rust_out="${rust_dir}/${name}.rs"

  if [[ -f "$yaml_src" && -f "$mustache" ]]; then
    echo "${YELLOW}→ Rendering ${name}.rs...${RESET}"
    forge-template render "$yaml_src" "$mustache" > "$rust_out"
    echo "   ${GREEN}✓ Rendered:${RESET} $rust_out"

    format_rust "$rust_out"
  else
    echo "${RED}✗ Missing file(s) for ${name}:${RESET}"
    [[ ! -f "$yaml_src" ]] && echo "   - Missing YAML:      $yaml_src"
    [[ ! -f "$mustache" ]] && echo "   - Missing template:  $mustache"
  fi

  echo ""
}

# ---------------------------------------------------------------------
# 🚀 Main entrypoint for script
# ---------------------------------------------------------------------
main() {
  echo ""
  echo "${CYAN}${BOLD}⚙️  [Forge IDE Renderer]${RESET}"
  echo "Modules folder:   $modules_dir"
  echo "Templates folder: $templates_dir"
  echo "Rust output:      $rust_dir"
  echo ""

  mkdir -p "$rust_dir"

  for module in $MODULES; do
    render_module "$module"
  done

  echo "${GREEN}${BOLD}✅ All Forge IDE modules rendered & formatted successfully!${RESET}"
  echo ""
}

# ---------------------------------------------------------------------
# 🔥 Execute main
# ---------------------------------------------------------------------
main
