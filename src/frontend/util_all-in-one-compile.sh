#!/usr/bin/env bash
set -euo pipefail
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "${SCRIPT_DIR}"
rm -rf node_modules dist
npm install --ignore-scripts=false
npm run build
tar -czf corn-syrup-frontend-dist.tar.gz dist package.json package-lock.json 2>/dev/null || tar -czf corn-syrup-frontend-dist.tar.gz dist package.json
