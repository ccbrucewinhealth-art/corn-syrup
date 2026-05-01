# corn-syrup

以 Rust 實作的自架監控工具，源自 Uptime Kuma 的轉換專案。

## 功能

- 支援 HTTP(s) / TCP / HTTP(s) 關鍵字 / HTTP(s) JSON Query / Websocket / Ping / DNS Record / Push / Steam Game Server / Docker Containers 監控
- 支援 Telegram、Discord、Gotify、Slack、Pushover、SMTP 等通知
- 20 秒檢測間隔
- 多狀態頁面
- 憑證 / TLS 資訊檢查
- Proxy 支援與反向代理整合
- 內建多語系支援：zh-TW、en-US、ja-JP
- Service Worker 快取與離線前端資源支援

## 架構

- **後端**：Rust 伺服器程式碼，位於 `src/backend/`
- **前端**：Rust/Yew 風格前端程式碼，位於 `src/frontend-rust/`
- **資料庫**：預設 SQLite，後端模組也支援 PostgreSQL
- **建置**：使用 `Makefile`，由專案根目錄呼叫 `cargo`（在 `src/` 目錄下）

## 目錄說明

- `src/backend/` — 伺服器入口、設定、資料庫、認證、API、Socket、監控、通知、代理、Metrics、排程、工具
- `src/frontend-rust/` — 應用入口、路由、頁面、元件、佈局、Service Worker、i18n、前端工具
- `src/frontend-rust/components/notifications/` — 通知提供者表單與 payload 模組

## 建置與執行

在專案根目錄執行：

```bash
make build
make dev
make test
make clean
```

`Makefile` 會將工作委派給 `src/` 底下的 Rust 編譯系統。

## 備註

本專案為受 Uptime Kuma 啟發的 Rust 翻譯實作。文件與程式碼已調整為目前的 Rust 原始碼樹狀結構，而非原始 Vue 前端。

- 上游專案：Uptime Kuma
- 上游儲存庫：https://github.com/louislam/uptime-kuma
- 上游授權：MIT License

完整授權條文與原始聲明請以官方上游儲存庫為準。
