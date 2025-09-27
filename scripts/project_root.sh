#!/usr/bin/env sh
set -eu

# Walk upward from current directory to find a .forge-root marker
_CUR="$PWD"
_LIMIT=15
_FOUND=""

while [ "$_LIMIT" -gt 0 ]; do
  if [ -f "$_CUR/.forge-root" ]; then
    _FOUND="$_CUR"
    break
  fi
  _PARENT=$(cd "$_CUR/.." && pwd)
  [ "$_PARENT" = "$_CUR" ] && break
  _CUR="$_PARENT"
  _LIMIT=$(( _LIMIT - 1 ))
done

if [ -z "$_FOUND" ]; then
  printf "%s\n" "ERROR: Could not locate .forge-root (are you inside the repo?)" >&2
  (return 1) 2>/dev/null || exit 1
fi

export FORGE_ROOT="$_FOUND"
printf "FORGE_ROOT=%s\n" "$FORGE_ROOT"
