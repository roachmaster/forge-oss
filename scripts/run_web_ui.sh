#!/usr/bin/env sh
set -eu

. "$(dirname "$0")/env.sh"

DIST_DIR="$FORGE_ROOT/crates/forge-web-ui/dist"
PORT="${FORGE_WEB_PORT:-8080}"

if [ ! -d "$DIST_DIR" ]; then
  echo "Dist not found at $DIST_DIR. Run scripts/build_web_ui.sh first."
  exit 1
fi

echo "[forge] Serving $DIST_DIR on http://0.0.0.0:$PORT"
cd "$DIST_DIR"

# Use the system Python to serve static assets (simple & portable).
# If you prefer another static server, swap it here.
exec python3 -m http.server "$PORT"
