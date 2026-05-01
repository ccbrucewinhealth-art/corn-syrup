#!/bin/bash

set -e

PROJECT_NAME="corn-syrup"
BUILD_DIR=""
BINARY_NAME="${PROJECT_NAME}"

echo "=== [${PROJECT_NAME}] All-in-One 編譯中 ==="

if [ ! -f "Cargo.toml" ]; then
    echo "錯誤: 找不到 Cargo.toml"
    exit 1
fi

echo "Step 1: 清理之前的編譯..."
cargo clean -p ${PROJECT_NAME} 2>/dev/null || true

echo "Step 2: 檢查依賴..."
cargo check

echo "Step 3: 單一檔案編譯..."
cargo build --release

if [ -f "target/release/${BINARY_NAME}" ]; then
    echo "=== 編譯成功 ==="
    echo "執行檔: target/release/${BINARY_NAME}"
    ls -lh "target/release/${BINARY_NAME}"
else
    echo "錯誤: 編譯失敗"
    exit 1
fi
