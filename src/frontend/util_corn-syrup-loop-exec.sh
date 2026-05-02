#!/usr/bin/env bash
set -euo pipefail
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
while true; do
  bash "${SCRIPT_DIR}/util_corn-syrup.sh" || true
  sleep "${CORN_SYRUP_RESTART_DELAY:-5}"
done
