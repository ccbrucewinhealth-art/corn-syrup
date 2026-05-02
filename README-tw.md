# corn-syrup

`corn-syrup` 是以 Rust 後端與 React/TypeScript 前端重新實作 Uptime Kuma 的轉換專案。目標是在目前 Rust 與 React 架構下，盡可能保留上游監控行為、API/Socket 相容性、狀態頁流程與使用者體驗。

## 功能

- 由 Uptime Kuma 轉換而來的監控行為，包含 HTTP(s)、TCP、關鍵字、JSON Query、WebSocket、Ping、DNS Record、Push、遊戲伺服器、Docker 與相關監控類型（依實作進度補齊）。
- 相容 Uptime Kuma 概念的後端 API 路由、Socket.IO 風格事件、認證、設定、狀態頁、監控歷史、排程、Metrics、Proxy 處理與通知 payload。
- Rust 後端模組涵蓋設定、Logging、認證、資料庫/模型轉換、監控、通知、Docker 整合、Remote Browser 支援、工具與錯誤處理。
- React/TypeScript 前端由原 Vue UI 轉換而來，使用 React Router、i18next/react-i18next、Socket.IO client、Axios、Bootstrap 相容樣式、Font Awesome、Chart.js 與 Vite。
- 前端 i18n 資源包含 `zh-TW`、`en`、`ja`，並以 `zh-TW` 為預設語系。
- Service Worker 與靜態資產保留於根目錄 public 資源與前端 workspace 中。

## 架構

- **後端**：Rust crate `corn-syrup-backend`，位於 `src/backend/`；包含 binary `corn-syrup`，以及 server core、config、settings、logging、auth、API、socket、database/model adaptation、monitor、notification、jobs、metrics、proxy、Docker、remote browser、utilities 與 errors 等模組。
- **前端**：React/TypeScript workspace `corn-syrup-frontend`，位於 `src/frontend/`；以 Vite 建置，並以 `App.tsx`、`main.tsx`、router、layouts、pages、components、mixins、shared libraries、assets、styles 與 i18n resources 組織應用程式。
- **文件**：轉換需求、規格、計畫、任務與檢查清單位於 `20.doc/48.spec/backend/` 與 `20.doc/48.spec/frontend/`。
- **測試與參考**：上游風格的相容性測試、手動測試與 e2e 資產位於 `test/`，用於行為比對與轉換參考。
- **離線工具**：套件下載輔助工具位於 `package/modules/`。

## 目錄說明

- `src/backend/` — Rust 後端 package、binary、server core、config、settings、logging、auth、API、socket handlers、database/model adaptation、monitor、notification、jobs、metrics、proxy、Docker、remote、utilities 與 error 模組。
- `src/frontend/` — React/TypeScript 前端 package、Vite 設定、Makefile、建置腳本、public assets 與應用程式原始碼。
- `src/frontend/src/` — 前端應用入口、router、styles、assets、components、layouts、pages、mixins、shared libraries 與 i18n 設定。
- `src/frontend/src/i18n/locales/` — `zh-TW`、`en`、`ja` 語系 JSON 檔。
- `public/` — 根目錄靜態 icons、manifest 與 service worker 資產，用於相容性保留。
- `20.doc/48.spec/backend/` — 後端需求、規格、計畫、任務清單與轉換檢查清單。
- `20.doc/48.spec/frontend/` — 前端需求、規格、計畫、任務清單與轉換檢查清單。
- `test/` — 後端、e2e 與手動測試資產，保留或模擬上游行為。

## 建置與執行

目前 repository root 不提供主要建置用 `Makefile`。後端與前端指令請分別在各自 workspace 中執行。

### 後端

```bash
cd src/backend
make check
make build
make dev
make test
make clean
```

需要時也可以在 `src/backend/` 直接使用 Cargo 指令。

### 前端

```bash
cd src/frontend
make install
make build
make dev
make loop
make all-in-one
make clean
```

前端 npm scripts 包含 `dev`、`build`、`preview` 與 `lint`。

## 開發備註

- 後端應盡可能維持與 Uptime Kuma API、socket event semantics、monitor behavior、notification payloads、settings 與 status page expectations 的相容性。
- 前端 UI flow、route structure、translations、reusable components 與 runtime integrations 應貼近原 Vue 實作，同時採用慣用 React/TypeScript 寫法。
- 請保持 `zh-TW`、`en`、`ja` i18n 資源同步；`zh-TW` 為預設語系。
- 不要重新引入已淘汰的 `src/frontend-rust/` 假設；目前有效前端 workspace 是 `src/frontend/`。
- 相容性取捨與架構差異需記錄於 `20.doc/48.spec/` 底下相關文件。

## 上游

本專案為受 Uptime Kuma 啟發的轉換實作。

- 上游專案：Uptime Kuma
- 上游儲存庫：https://github.com/louislam/uptime-kuma
- 上游授權：MIT License

## 參考範本

- Kero：https://github.com/iuap-design/kero

完整授權條文與原始聲明請以官方上游儲存庫為準。
