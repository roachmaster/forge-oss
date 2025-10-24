# Forge crate creation utility
# Automatically loaded by forge-scripts.sh
# Usage:
#   forge_create <crate_name> [--lib|--bin]
# Example:
#   forge_create forge-network --lib

forge_create() {
  local name="$1"
  local kind="${2:---lib}"   # default to --lib if no second arg
  local crate_dir="crates/$name"

  if [[ -z "$name" ]]; then
    echo "❌ Usage: forge_create <crate_name> [--lib|--bin]"
    return 1
  fi

  if [[ -d "$crate_dir" ]]; then
    echo "⚠️  Crate '$name' already exists at $crate_dir"
    return 1
  fi

  echo "📦 Creating new Forge crate: $name ($kind)"
  cargo new "$crate_dir" "$kind" || return 1

  echo "🧩 Adding to workspace Cargo.toml..."
  if ! grep -q "crates/$name" Cargo.toml; then
    # Append to the members array before the closing bracket
    perl -i -pe "s|(members = \\[)|\\1\n    \"crates/$name\",|" Cargo.toml
  fi

  echo "✅ Crate '$name' created successfully."
  echo "📁 Location: $crate_dir"
}
