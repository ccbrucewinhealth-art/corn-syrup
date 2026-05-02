#!/usr/bin/env bash
set -euo pipefail
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
WORKSPACE="$(cd "${SCRIPT_DIR}/../.." && pwd)"
FRONTEND_DIR="${WORKSPACE}/src/frontend"
CACHE_DIR="${SCRIPT_DIR}/npm-cache"
mkdir -p "${CACHE_DIR}"
cd "${FRONTEND_DIR}"
npm install --package-lock-only --cache "${CACHE_DIR}"
npm ci --cache "${CACHE_DIR}" --prefer-offline
tar -czf "${SCRIPT_DIR}/corn-syrup-frontend-npm-cache.tar.gz" -C "${CACHE_DIR}" .
