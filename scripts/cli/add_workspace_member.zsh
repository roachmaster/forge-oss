# add_workspace_member <root_dir> <member_path>
add_workspace_member() {
  emulate -L zsh
  set -euo pipefail
  local ROOT="$1" MEMBER="$2"
  local ROOT_CARGO="$ROOT/Cargo.toml"

  python3 - "$ROOT_CARGO" "$MEMBER" <<'PY'
import sys, re, pathlib

cargo_path = pathlib.Path(sys.argv[1])
member = sys.argv[2]

def write(p: pathlib.Path, text: str):
    tmp = p.with_suffix(p.suffix + ".tmp")
    tmp.write_text(text, encoding="utf-8")
    tmp.replace(p)

# If Cargo.toml doesn't exist, create minimal workspace
if not cargo_path.exists():
    text = (
        "[workspace]\n"
        "members = [\n"
        f'    "{member}",\n'
        "]\n"
    )
    write(cargo_path, text)
    sys.exit(0)

text = cargo_path.read_text(encoding="utf-8")

# If already present anywhere, do nothing (idempotent)
quoted = f'"{member}"'
if quoted in text:
    sys.exit(0)

# Ensure a [workspace] section exists
if "[workspace]" not in text:
    text = text.rstrip() + (
        "\n\n[workspace]\n"
        "members = [\n"
        f'    "{member}",\n'
        "]\n"
    )
    write(cargo_path, text)
    sys.exit(0)

# Try to find an existing members array inside [workspace]
# Capture [workspace] block (until next [section] or end)
ws_block_pattern = re.compile(
    r"(\[workspace\]\s*)([\s\S]*?)(?=(?:\n\[[^\]]+\])|\Z)",
    re.M
)
m = ws_block_pattern.search(text)
if not m:
    # Fallback: append a fresh [workspace] with members
    text = text.rstrip() + (
        "\n\n[workspace]\n"
        "members = [\n"
        f'    "{member}",\n'
        "]\n"
    )
    write(cargo_path, text)
    sys.exit(0)

ws_head, ws_body = m.group(1), m.group(2)

# Look for members = [ ... ] in the workspace body
members_pattern = re.compile(r"(members\s*=\s*\[\s*)([\s\S]*?)(\])", re.M)
mm = members_pattern.search(ws_body)

if mm:
    # Insert before closing bracket
    pre, inner, close = mm.groups()
    # If inner is empty or whitespace-only, just add our member
    inner_stripped = inner.strip()
    if member in inner_stripped:
        # Shouldn't happen (we checked above), but be safe
        new_inner = inner
    else:
        prefix = "" if inner_stripped == "" else inner
        # Ensure inner ends with newline if not empty
        if prefix and not prefix.endswith("\n"):
            prefix += "\n"
        new_inner = f'{prefix}    "{member}",\n'
    new_ws_body = ws_body[:mm.start()] + pre + new_inner + close + ws_body[mm.end():]
else:
    # No members array; add one at the end of the workspace body
    suffix = "" if ws_body.endswith("\n") else "\n"
    new_ws_body = ws_body + suffix + (
        "members = [\n"
        f'    "{member}",\n'
        "]\n"
    )

# Rebuild file
new_text = text[:m.start()] + ws_head + new_ws_body + text[m.end():]
write(cargo_path, new_text)
PY
}
