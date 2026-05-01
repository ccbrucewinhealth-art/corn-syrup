#!/bin/bash

set -e

OUTPUT_DIR="package/modules"
TIMESTAMP=$(date '+%Y%m%d_%H%M%S')
TAR_FILE="package_modules_${TIMESTAMP}.tar.gz"

echo "=== 建立離線套件包 ==="

mkdir -p "${OUTPUT_DIR}"

echo "Step 1: 下載所有依賴..."
cd src && cargo fetch 2>/dev/null || true
cd ..

echo "Step 2: 建立 vendor 目錄..."
mkdir -p "${OUTPUT_DIR}/vendor"

echo "Step 3: 複製 Cargo registry..."
CARGO_HOME="${CARGO_HOME:-$HOME/.cargo}"
if [ -d "${CARGO_HOME}/registry/cache" ]; then
    cp -r "${CARGO_HOME}/registry/cache" "${OUTPUT_DIR}/vendor/"
fi
if [ -d "${CARGO_HOME}/registry/index" ]; then
    cp -r "${CARGO_HOME}/registry/index" "${OUTPUT_DIR}/vendor/"
fi

if [ -d "${CARGO_HOME}/git" ]; then
    cp -r "${CARGO_HOME}/git" "${OUTPUT_DIR}/vendor/"
fi

echo "Step 4: 複製 Cargo.lock..."
if [ -f "src/Cargo.lock" ]; then
    cp src/Cargo.lock "${OUTPUT_DIR}/"
fi

echo "Step 5: 紀錄依賴清單..."
cd src && cargo metadata --format-version=1 --no-deps > "../${OUTPUT_DIR}/cargo_metadata.json" 2>/dev/null || echo "{}" > "../${OUTPUT_DIR}/cargo_metadata.json"
cd ..

echo "Step 6: 建立壓縮包..."
cd ${OUTPUT_DIR} && tar -czf ../${TAR_FILE} vendor/ cargo_metadata.json Cargo.lock 2>/dev/null || true
cd ..

echo "=== 完成 ==="
echo "離線套件: ${TAR_FILE}"
ls -lh ${TAR_FILE}
