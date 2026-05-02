#!/bin/bash

set -e

PROJECT_NAME="corn-syrup-backend"
BUILD_DIR="target/release"
BINARY_NAME="${PROJECT_NAME}"

echo "=== [${PROJECT_NAME}] 編譯中... ==="

if [ ! -f "Cargo.toml" ]; then
    echo "錯誤: 找不到 Cargo.toml，請確認專案目錄"
    exit 1
fi

echo "Step 1: 檢查依賴..."
cargo check || cargo build

echo "Step 2: 編譯中..."
cargo build --release --bin "${BINARY_NAME}"

if [ -f "target/release/${BINARY_NAME}" ]; then
    echo "=== 編譯成功 ==="
    echo "執行檔: target/release/${BINARY_NAME}"
    ls -lh "target/release/${BINARY_NAME}"
else
    echo "錯誤: 編譯失敗"
    exit 1
fi
