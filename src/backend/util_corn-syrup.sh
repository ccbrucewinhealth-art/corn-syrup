#!/bin/bash

set -e

PROJECT_NAME="corn-syrup"
BUILD_DIR="target/release"
BINARY_NAME="${PROJECT_NAME}"

echo "=== [${PROJECT_NAME}] 執行中 ==="

if [ ! -f "${BUILD_DIR}/${BINARY_NAME}" ]; then
    echo "找不到執行檔，正在編譯..."
    bash util_compile.sh
fi

./${BUILD_DIR}/${BINARY_NAME} "$@"
