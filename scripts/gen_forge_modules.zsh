#!/usr/bin/env zsh
# =====================================================================
# 🔥 Forge IDE Module Renderer (Direct YAML → Rust)
# Description:
#   Renders Rust source files directly from YAML modules under
#   templates/forge_ide/modules/ using their matching Mustache templates.
#   Outputs go to crates/forge-ide/src/.
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
# 📂 Paths
# ---------------------------------------------------------------------
root_dir=$(dirname "$0")/..
cd "$root_dir"

modules_dir="templates/forge_ide/modules"
templates_dir="templates/forge_ide"
rust_dir="crates/forge-ide/src"

echo ""
echo "${CYAN}${BOLD}⚙️  [Forge IDE Renderer]${RESET}"
echo "Modules folder:   $modules_dir"
echo "Templates folder: $templates_dir"
echo "Rust output:      $rust_dir"
echo ""

mkdir -p "$rust_dir"

# ---------------------------------------------------------------------
# 🧩 Render helper
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
  else
    echo "${RED}✗ Missing file(s) for ${name}:${RESET}"
    [[ ! -f "$yaml_src" ]] && echo "   - Missing YAML: $yaml_src"
    [[ ! -f "$mustache" ]] && echo "   - Missing template: $mustache"
  fi

  echo ""
}

# ---------------------------------------------------------------------
# 🚀 Render All Forge IDE Modules
# ---------------------------------------------------------------------
render_module "schema"
render_module "command"
render_module "provider"
render_module "router"

echo "${GREEN}${BOLD}✅ All Forge IDE modules rendered successfully!${RESET}"
echo ""
