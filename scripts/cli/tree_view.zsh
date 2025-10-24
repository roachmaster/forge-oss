# -----------------------------------------------------------------------------
# tree_view — display a clean directory tree of the current crate
# -----------------------------------------------------------------------------
tree_view() {
  # Ensure FORGE_ROOT is set
  if [[ -z "$FORGE_ROOT" ]]; then
    echo "❌ FORGE_ROOT is not set. Please export it before using tree_view."
    return 1
  fi

  # Ensure a current crate is selected
  if [[ -z "$FORGE_CURRENT_CRATE" ]]; then
    echo "⚠️  No current crate view is set. Use 'forge_view <crate>' first."
    return 1
  fi

  local crate_dir="$FORGE_ROOT/crates/$FORGE_CURRENT_CRATE"

  # Verify directory exists
  if [[ ! -d "$crate_dir" ]]; then
    echo "❌ Crate directory not found: $crate_dir"
    return 1
  fi

  echo "🌳 Tree view for crate: $FORGE_CURRENT_CRATE"
  echo "📁 Location: $crate_dir"
  echo "------------------------------------------"

  # Try to use the 'tree' command if available
  if command -v tree >/dev/null 2>&1; then
    tree -I 'target|out|tmp|temp|.git|node_modules' "$crate_dir"
  else
    # Fallback: manual find-based tree display
    find "$crate_dir" \
      -type d \( -name target -o -name out -o -name tmp -o -name temp -o -name .git -o -name node_modules \) -prune -o -print |
      sed -e "s|$crate_dir|.|" -e 's|[^/]*/|  |g'
  fi

  echo "------------------------------------------"
}
