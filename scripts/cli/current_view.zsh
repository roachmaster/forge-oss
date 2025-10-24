# -----------------------------------------------------------------------------
# current_view — display the currently active Forge crate view
# -----------------------------------------------------------------------------
current_view() {
  # Ensure FORGE_ROOT is set
  if [[ -z "$FORGE_ROOT" ]]; then
    echo "❌ FORGE_ROOT is not set. Please export it before using current_view."
    return 1
  fi

  # Show the current crate
  if [[ -n "$FORGE_CURRENT_CRATE" ]]; then
    echo "👁️  Current Forge crate view: $FORGE_CURRENT_CRATE"
    echo "📦 Path: $FORGE_ROOT/crates/$FORGE_CURRENT_CRATE"
  else
    echo "⚠️  No current crate view is set."
  fi
}
