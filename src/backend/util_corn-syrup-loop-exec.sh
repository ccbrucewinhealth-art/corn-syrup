#!/bin/bash

set -e

PROJECT_NAME="corn-syrup-backend"
BUILD_DIR="target/release"
BINARY_NAME="${PROJECT_NAME}"

echo "=== [${PROJECT_NAME}] 無限迴圈執行中 ==="

check_and_build() {
    if [ ! -f "${BUILD_DIR}/${BINARY_NAME}" ]; then
        echo "找不到執行檔，正在編譯..."
        bash util_compile.sh
    fi
}

while true; do
    check_and_build
    
    echo "===================================="
    echo "執行中 PID: $$"
    echo "時間: $(date '+%Y-%m-%d %H:%M:%S')"
    echo "===================================="
    
    ./${BUILD_DIR}/${BINARY_NAME} || {
        echo "執行失敗，重新啟動..."
        sleep 5
        continue
    }
    
    echo "程式已結束，5秒後重新啟動..."
    sleep 5
done
