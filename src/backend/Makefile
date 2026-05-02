.PHONY: all build run clean test install check help dev docs

PROJECT_NAME = corn-syrup-backend
BINARY_NAME = $(PROJECT_NAME)
SRC_DIR = .
BUILD_DIR = target/release

all: build

check:
	@echo "=== 檢查程式碼 ==="
	cargo check

build: $(BUILD_DIR)/$(BINARY_NAME)
	@echo "=== 編譯完成 ==="

$(BUILD_DIR)/$(BINARY_NAME):
	@echo "=== 編譯中 ==="
	cargo build --release --bin $(BINARY_NAME)

run: build
	@echo "=== 執行中 ==="
	./$(BUILD_DIR)/$(BINARY_NAME)

dev:
	@echo "=== 開發模式 ==="
	cargo run --bin $(BINARY_NAME)

test:
	@echo "=== 測試中 ==="
	cargo test

install: build
	@echo "=== 安裝 ==="
	install -m 755 $(BUILD_DIR)/$(BINARY_NAME) /usr/local/bin/

clean:
	@echo "=== 清理 ==="
	cargo clean

docs:
	@echo "=== 產生文件 ==="
	cargo doc --no-deps

help:
	@echo "可用目標:"
	@echo "  make build   - 編譯專案"
	@echo "  make run     - 編譯並執行"
	@echo "  make dev    - 開發模式執行"
	@echo "  make test    - 執行測試"
	@echo "  make clean   - 清理編譯檔"
	@echo "  make install - 安裝到系統"
	@echo "  make docs    - 產生文件"
	@echo "  make help    - 顯示說明"
