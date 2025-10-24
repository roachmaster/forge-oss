#!/usr/bin/env zsh
# Auto-load all CLI functions from scripts/cli/*.zsh without changing shell opts.

SCRIPT_DIR="${0:A:h}"
echo $SCRIPT_DIR
CLI_DIR="$SCRIPT_DIR/scripts/cli"


if [[ -d "$CLI_DIR" ]]; then
  for f in "$CLI_DIR"/*.zsh(.N); do
    # Source each file in a subshell so local setopts don't leak
    () { source "$f" }   # subshell source
  done
fi
refresh_source(){
    source $SCRIPT_DIR/forge-scripts.sh
}
forge_help() {
  echo "Forge CLI functions:"
  for f in "$CLI_DIR"/*.zsh(.N); do
    print "  - ${(r:24:)${f:t:r}}"
  done
  echo "Example: create_crate my-crate --lib"
}
