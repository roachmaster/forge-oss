# Forge clean & build utilities
# Automatically loaded by forge-scripts.sh

forge_clean() {
  echo "🧹 Cleaning Forge workspace..."
  cargo clean && echo "✅ Clean complete."
}

forge_build() {
  echo "⚙️  Building Forge workspace..."
  cargo build --workspace && echo "✅ Build complete."
}

forge_rebuild() {
  echo "🔁 Cleaning and rebuilding Forge workspace..."
  cargo clean && cargo build --workspace && echo "✅ Rebuild complete."
}

# Forge refresh utility
# Automatically loaded by forge-scripts.sh

forge_refresh() {
  echo "🔁 Refreshing Forge workspace..."
  forge_clean
  forge_build
  echo "✅ Refresh complete."
}
