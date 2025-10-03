# ──────────────────────────────────────────────────────────────────────────────
# Forge OSS: create `forgery-macros` proc-macro crate (SSH-safe, error-handled)
# Usage (foreground):
#   source ./scripts/env.sh
#   source /dev/stdin <<<"$(cat <<'EOS'
#   [paste this whole block here]
#   EOS
#   )"
#   forge_create_forgery_macros
#
# Usage (background/SSH-safe):
#   source ./scripts/env.sh
#   LOG="$FORGE_ROOT/.logs/forgery-macros.$(date +%Y%m%d-%H%M%S).log"
#   ( forge_create_forgery_macros |& tee -a "$LOG" ) & disown
#   echo "Tail logs: tail -f $LOG"
# ──────────────────────────────────────────────────────────────────────────────

# Strict mode
set -euo pipefail

# ── tiny logger
_forge_log() { print -r -- "[$(date +'%Y-%m-%d %H:%M:%S')] $*" }
_forge_die() { _forge_log "ERROR: $*"; return 1 }
_forge_cmd() { _forge_log "RUN: $*"; eval "$*"; }

# ── trap useful diagnostics
_forge_on_err() {
  local ec=$?
  _forge_log "❌ Aborted with exit code $ec"
  return $ec
}
trap _forge_on_err ERR

# ── require a command exists
_forge_require() {
  local bin="$1"
  command -v "$bin" >/dev/null 2>&1 || _forge_die "Missing required command: $bin"
}

# ── append member to workspace using helper or fallback
_forge_add_workspace_member() {
  local member="$1"
  local root="${FORGE_ROOT:-${REPO_ROOT:-$PWD}}"
  local ws="$root/Cargo.toml"

  if [[ ! -f "$ws" ]]; then
    _forge_die "Workspace Cargo.toml not found at $ws"
  fi

  if [[ -x "$root/scripts/cli/add_workspace_member.zsh" ]]; then
    _forge_log "Adding workspace member via helper: $member"
    "$root/scripts/cli/add_workspace_member.zsh" "$member"
    return
  fi

  # Fallback: idempotent insert into [workspace].members
  _forge_log "Helper missing; using fallback to add member: $member"
  if grep -qF "$member" "$ws"; then
    _forge_log "Member already present: $member"
    return
  fi

  # macOS-safe awk edit
  awk -v member="$member" '
    BEGIN{added=0; inws=0}
    /^\[workspace\]/ {print; inws=1; next}
    inws && /^\s*members\s*=\s*\[/ {
      print; print "  \"" member "\","; inws=0; added=1; next
    }
    {print}
    END{
      if(added==0){
        print "\n[workspace]\nmembers = [\n  \"" member "\",\n]\n"
      }
    }
  ' "$ws" > "$ws.tmp" && mv "$ws.tmp" "$ws"
}

# ── main action: create the proc-macro crate (ceremonial stub)
forge_create_forgery_macros() {
  _forge_require cargo
  _forge_require awk

  local root="${FORGE_ROOT:-${REPO_ROOT:-$PWD}}"
  local name="forgery-macros"
  local dir="$root/crates/$name"

  _forge_log "FORGE_ROOT = $root"
  _forge_log "Creating proc-macro crate: $name"

  # Create dirs
  _forge_cmd "mkdir -p '$dir/src'"

  # Cargo.toml (idempotent write if missing)
  if [[ ! -f "$dir/Cargo.toml" ]]; then
    _forge_log "Writing $dir/Cargo.toml"
    cat > "$dir/Cargo.toml" <<'EOF'
[package]
name = "forgery-macros"
version = "0.1.0"
edition = "2021"
description = "Single macro for ceremonial front-end; logs and returns empty output."

[lib]
proc-macro = true

[dependencies]
EOF
  else
    _forge_log "Skipping Cargo.toml (exists)"
  fi

  # src/lib.rs (minimal macro: logs and returns empty)
  if [[ ! -f "$dir/src/lib.rs" ]]; then
    _forge_log "Writing $dir/src/lib.rs"
    cat > "$dir/src/lib.rs" <<'EOF'
use proc_macro::TokenStream;

/// MVP macro: `render!(template_rel, values_yaml)`
/// For now: log invocation at compile time and return empty expansion.
#[proc_macro]
pub fn render(input: TokenStream) -> TokenStream {
    eprintln!("[forgery-macros] render! invoked with: {}", input.to_string());
    "".parse().unwrap()
}
EOF
  else
    _forge_log "Skipping src/lib.rs (exists)"
  fi

  # Add to workspace
  _forge_add_workspace_member "crates/$name"

  # Build to verify
  _forge_log "Building crate…"
  _forge_cmd "cargo build -p $name"

  _forge_log "✅ Done: crates/$name"
  _forge_log "Tip: call render!(\"path.mustache\", \"---\\nheader: {}\\n\") somewhere and run cargo check to see the compile-time log."
}
