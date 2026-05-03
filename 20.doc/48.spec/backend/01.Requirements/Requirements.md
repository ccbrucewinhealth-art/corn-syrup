# Corn Syrup 系統需求規格

## 系統概述

Corn Syrup 是基於 Uptime Kuma 重新實作的監控系統，採用 Rust 後端 + React 前端架構。

| 項目 | 內容 |
|------|------|
| 後端技術 | Rust with Axum, SeaORM, tokio |
| 前端技術 | React + TypeScript + Vite + i18next |
| 資料庫 | SQLite (預設) / PostgreSQL (可選) |
| 預設語言 | 繁體中文 (zh-TW) |
| 支援語言 | 繁體中文, English, 日本語 |

---

# Phase 1: 核心監控功能 (Core Monitoring)

## R1.1 HTTP/HTTPS 監控

### 需求內容
- 支援 HTTP(s) 端點監控
- 支援 HTTP(s) 關鍵字檢測
- 支援 HTTP(s) JSON Query 檢測
- 支援自定義 HTTP 方法 (GET, POST, PUT, DELETE, PATCH, HEAD, OPTIONS)
- 支援自定義 Headers
- 支援自定義 Request Body

### 描述
使用者可設定 URL 進行 HTTP 監控，可指定檢測關鍵字是否存在於回應中，或使用 JSON Path 提取回應值進行驗證。

### 邏輯流程
1. 發送 HTTP 請求至目標 URL
2. 檢查 HTTP 狀態碼是否為 2xx/3xx
3. 若有設定關鍵字，比對回應內容
4. 若有設定 JSON Query，提取並驗證值
5. 記錄心跳結果 (上線/離線)

### 確認 [ ]
- [ ] HTTP 請求可自定義 method
- [ ] 可設定自定義 headers
- [ ] 可設定 request body
- [ ] 支援 HTTPS 憑證驗證
- [ ] 支援關鍵字檢測
- [ ] 支援 JSON Query 檢測

---

## R1.2 TCP 監控

### 需求內容
- 支援 TCP 連接監控
- 支援自定義 TCP 端口
- 支援 TCP 回應內容驗證

### 描述
透過 TCP 連線測試目標主機的特定端口是否可連線，可進一步驗證回應內容。

### 邏輯流程
1. 建立 TCP 連線至目標主機:端口
2. 檢查連線是否成功
3. 若有設定回應內容，比對回應
4. 記錄心跳結果

### 確認 [ ]
- [ ] 可指定目標主機與端口
- [ ] 連線失敗記錄為離線
- [ ] 支援回應內容驗證

---

## R1.3 DNS 監控

### 需求內容
- 支援 DNS 記錄類型: A, AAAA, CNAME, MX, TXT, NS, SOA
- 支援 DNS 解析驗證
- 支援 DNS 過期監控 (domain_expiry)

### 描述
查詢 DNS 記錄類型並驗證解析結果，可設定過期監控提醒。

### 邏輯流程
1. 解析目標域名
2. 根據記錄類型比對預期值
3. 檢查憑證過期日期 (若為 SSL)
4. 記錄心跳結果

### 確認 [ ]
- [ ] 支援 A/AAAA/CNAME/MX/TXT/NS/SOA 記錄
- [ ] 可設定預期解析值
- [ ] 支援域名過期提醒

---

## R1.4 Ping 監控

### 需求內容
- 支援 ICMP Ping 監控
- 支援回應時間測量

### 描述
使用 ICMP 協議測試主機是否可達，並記錄回應時間。

### 邏輯流程
1. 發送 ICMP Echo Request
2. 等待 Echo Reply
3. 記錄 RTT (往返時間)
4. 記錄心跳結果

### 確認 [ ]
- [ ] 可設定目標 IP/主機
- [ ] 記錄回應時間
- [ ] 超時記錄為離線

---

## R1.5 Push 監控

### 需求內容
- 支援 Push 方式監控 (客戶端主動上報)
- 支援 REST API Push 端點

### 描述
提供 API 端點供客戶端主動上報狀態，適用於無法主動監控的場景。

### 邏輯流程
1. 建立 Push API 端點
2. 客戶端定時上報心跳
3. 記錄最後收到心跳的時間
4. 逾時未收到視為離線

### 確認 [ ]
- [ ] 提供 Push API 端點
- [ ] 驗證客戶端 ID
- [ ] 逾時判斷離線

---

## R1.6 WebSocket 監控

### 需求內容
- 支援 WebSocket 連接監控
- 支援 WebSocket 升級監控

### 描述
測試 WebSocket 連接是否可建立。

### 邏輯流程
1. 發起 WebSocket 連線
2. 完成 HTTP 升級
3. 關閉連線
4. 記錄心跳結果

### 確認 [ ]
- [ ] 支援 ws:// 和 wss://
- [ ] 升級失敗記錄為離線

---

## R1.7 監控間隔

### 需求內容
- 支援 20 秒監控間隔
- 支援自定義監控間隔
- 支援心跳歷史記錄 (heartbeat)

### 描述
設定監控檢查頻率，儲存歷史心跳資料供分析。

### 邏輯流程
1. 根據監控間隔排程檢查
2. 執行監控類型對應的檢查邏輯
3. 儲存心跳結果至資料庫
4. 清理過期心跳資料

### 確認 [ ]
- [ ] 支援 20 秒預設間隔
- [ ] 可自定義間隔 (最小 10 秒)
- [ ] 儲存心跳歷史
- [ ] 支援歷史資料查詢

---

# Phase 2: 通知系統 (Notification System)

## R2.1 通知渠道

### 需求內容
- 支援 Email (SMTP)
- 支援 Telegram
- 支援 Discord
- 支援 Slack
- 支援 Gotify
- 支援 Pushover
- 支援自定義 Webhook

### 描述
提供多種通知渠道，使用者可設定多個通知規則。

### 邏輯流程
1. 使用者設定通知供應商
2. 觸發通知條件時呼叫對應 API
3. 發送通知至目標
4. 記錄通知發送結果

### UI 操作需求
- [ ] 新增通知供應商
- [ ] 編輯通知供應商設定
- [ ] 刪除通知供應商
- [ ] 測試通知發送
- [ ] 設定通知順序

### 確認 [ ]
- [ ] Email (SMTP) 支援
- [ ] Telegram Bot API 支援
- [ ] Discord Webhook 支援
- [ ] Slack Webhook 支援
- [ ] Gotify 支援
- [ ] Pushover 支援
- [ ] 自定義 Webhook 支援

---

## R2.2 通知條件

### 需求內容
- 狀態變更時通知
- 離線時通知
- 恢復上線時通知

### 描述
設定觸發通知的條件，可針對不同狀態設定不同通知規則。

### 邏輯流程
1. 監控狀態變更
2. 比對通知規則條件
3. 觸發符合的通知
4. 記錄通知發送日誌

### 確認 [ ]
- [ ] 狀態變更通知
- [ ] 離線通知
- [ ] 恢復通知

---

# Phase 3: 使用者介面與資料管理 (UI & Data Management)

## R3.1 使用者認證

### 需求內容
- 支援 2FA (兩步驟驗證)
- 支援 JWT Token 認證
- 支援密碼登入

### 描述
使用者認證系統，支援密碼登入與選用 2FA。

### 邏輯流程
1. 使用者輸入帳號密碼
2. 驗證密碼雜湊
3. 若啟用 2FA，驗證 TOTP
4. 核發 JWT Token
5. 後續請求攜帶 Token 驗證

### UI 操作需求
- [ ] 登入頁面 (Login)
- [ ] 註冊頁面 (Register)
- [ ] 設定 2FA
- [ ] 啟用/停用 2FA

### 確認 [ ]
- [ ] 密碼登入功能
- [ ] JWT Token 核發
- [ ] Token 驗證 Middleware
- [ ] TOTP 2FA 支援

---

## R3.2 監控群組

### 需求內容
- 支援監控項目群組管理
- 支援群組標籤

### 描述
將監控項目分組管理，可使用標籤標記。

### 邏輯流程
1. 建立群組/標籤
2. 將監控項目關聯至群組
3. 按群組篩選監控

### UI 操作需求
- [ ] 新增群組
- [ ] 編輯群組
- [ ] 刪除群組
- [ ] 設定監控所屬群組

### 確認 [ ]
- [ ] 群組 CRUD 功能
- [ ] 標籤 CRUD 功能

---

## R3.3 監控管理

### 需求內容
- 新增/編輯/刪除監控項目
- 監控項目啟用/停用
- 監控項目排序

### 描述
管理監控項目的生命週期。

### UI 操作需求
- [ ] 監控列表頁面
- [ ] 新增監控表單
- [ ] 編輯監控表單
- [ ] 刪除確認對話框
- [ ] 啟用/停用開關
- [ ] 拖曳排序

### 確認 [ ]
- [ ] 新增監控功能
- [ ] 編輯監控功能
- [ ] 刪除監控功能
- [ ] 啟用/停用功能

---

## R3.4 資料儲存

### 需求內容
- SQLite 資料庫儲存
- 心跳記錄儲存
- 歷史資料查詢

### 描述
資料持久化與查詢功能。

### 確認 [ ]
- [ ] SQLite 連線與操作
- [ ] 心跳資料寫入
- [ ] 歷史資料查詢 API

---

# Phase 4: 進階功能 (Advanced Features)

## R4.1 狀態頁面 (Status Page)

### 需求內容
- 公開狀態頁面
- 自定義狀態頁面樣式
- 支援多狀態頁面

### 描述
提供公開頁面顯示監控狀態。

### UI 操作需求
- [ ] 狀態頁面列表
- [ ] 新增狀態頁面
- [ ] 編輯狀態頁面
- [ ] 發布/下架狀態頁面
- [ ] 公開頁面檢視

### 確認 [ ]
- [ ] 公開 URL 訪問
- [ ] 自定義標題/描述
- [ ] 顯示監控狀態列表
- [ ] 顯示事件記錄

---

## R4.2 維護模式

### 需求內容
- 排程維護時間
- 維護期間暫停通知

### 描述
設定維護時間區間，期間不發送通知。

### UI 操作需求
- [ ] 維護列表
- [ ] 新增維護排程
- [ ] 編輯維護時間
- [ ] 刪除維護

### 確認 [ ]
- [ ] 設定維護時間
- [ ] 維護期間不發通知
- [ ] 維護期間狀態顯示

---

## R4.3 證書資訊

### 需求內容
- SSL 證書過期監控
- 證書資訊顯示

### 描述
監控 SSL 憑證過期時間。

### 確認 [ ]
- [ ] 自動取得 SSL 資訊
- [ ] 顯示過期日期
- [ ] 過期前提醒

---

## R4.4 代理伺服器支援

### 需求內容
- HTTP 代理支援
- SOCKS 代理支援

### 描述
透過代理伺服器執行監控。

### UI 操作需求
- [ ] 代理設定頁面
- [ ] 新增代理
- [ ] 選擇監控使用代理

### 確認 [ ]
- [ ] HTTP 代理支援
- [ ] SOCKS 代理支援

---

## R4.5 Prometheus 支援

### 需求內容
- Prometheus 導出器
- Prometheus metrics 端點

### 描述
提供 Prometheus 相容的 metrics 端點。

### 確認 [ ]
- [ ] /api/prometheus metrics 端點
- [ ] 支援 Prometheus scrap

---

# Phase 5: 額外監控與系統整合 (Extra Monitoring & Integration)

## R5.1 資料庫監控

### 需求內容
- MySQL/MariaDB 監控
- PostgreSQL 監控
- MongoDB 監控
- Redis 監控
- MSSQL 監控

### 確認 [ ]
- [ ] MySQL 連線測試
- [ ] PostgreSQL 連線測試
- [ ] MongoDB 連線測試
- [ ] Redis 連線測試
- [ ] MSSQL 連線測試

---

## R5.2 訊息隊列監控

### 需求內容
- RabbitMQ 監控
- MQTT 監控

### 確認 [ ]
- [ ] RabbitMQ 連線測試
- [ ] MQTT 連線測試

---

## R5.3 Docker 監控

### 需求內容
- Docker 容器監控
- Docker 主機監控

### UI 操作需求
- [ ] Docker 主機設定
- [ ] 容器列表顯示

### 確認 [ ]
- [ ] Docker API 連線
- [ ] 容器列表取得

---

## R5.4 遊戲伺服器監控

### 需求內容
- Steam 遊戲伺服器監控
- gamedig 支援

### 確認 [ ]
- [ ] Steam API 整合
- [ ] gamedig 支援

---

## R5.5 其他監控

### 需求內容
- gRPC 監控
- SNMP 監控
- SIP OPTIONS 監控
- Real Browser 監控

### 確認 [ ]
- [ ] gRPC 健康檢查
- [ ] SNMP 輪詢
- [ ] SIP OPTIONS 請求
- [ ] 瀏覽器自動化檢查

---

# Phase 6: 前端介面 (Frontend UI - React)

## R6.1 選單系統 (Sidebar/Navigation)

### R6.1.1 側邊欄展開/收合

#### 需求內容
- 預設展開狀態 (280px 寬度)
- 點擊 kero-menu-button 切換收合/展開
- 收合狀態寬度 60px
- 收合時隱藏選單文字，僅顯示圖示
- 收合時隱藏 MENU/SETTINGS 標題
- 收合時隱藏品牌文字 "Corn-Syrup"
- CSS transition 平滑動畫效果
- 收合時 kero-menu-button 仍可見且可點擊

#### 描述
左側側邊欄支援展開/收合功能，收合時保留toggle按鈕供展開使用。

#### 邏輯流程
1. 初始渲染側邊欄，寬度 280px
2. 使用者點擊 kero-menu-button
3. 切換 sidebarCollapsed 狀態
4. CSS grid-template-columns 改變
5. CSS transition 產生動畫

#### UI 操作需求
- [ ] 點擊按鈕展開/收合側邊欄
- [ ] 收合時隱藏文字，顯示圖示
- [ ] 收合時隱藏標題
- [ ] 收合時隱藏品牌文字
- [ ] 收合時保持 toggle 按鈕可見

#### 確認 [ ]
- [ ] 預設 280px 寬度
- [ ] 收合 60px 寬度
- [ ] 隱藏文字僅顯示圖示
- [ ] CSS transition 動畫
- [ ] toggle 按鈕持續可見

---

### R6.1.2 導航項目

#### 需求內容
- MENU 群組: Dashboard, Status Pages, Maintenance (3項)
- SETTINGS 群組: General, Appearance, Notifications, Reverse Proxy, Tags, Monitor History, Docker Hosts, Remote Browsers, Security, API Keys, Proxies, About (12項)
- 導航連結正確導航至對應路由
- 支援 Font Awesome 圖示

#### 描述
側邊欄顯示導航項目，分為 MENU 和 SETTINGS 兩群組。

#### 邏輯流程
1. 定義 keroSidebarItems 資料陣列
2. 使用 filter 依 group 分類
3. 渲染 NavLink 元件
4. 使用 react-router-dom 導航

#### UI 操作需求
- [ ] 顯示 MENU 群組 (3項)
- [ ] 顯示 SETTINGS 群組 (12項)
- [ ] 顯示各項目圖示
- [ ] 點擊導航至正確路由

#### 確認 [ ]
- [ ] Dashboard 連結 /dashboard
- [ ] Status Pages 連結 /manage-status-page
- [ ] Maintenance 連結 /maintenance
- [ ] General 連結 /settings/general
- [ ] About 連結 /settings/about
- [ ] 其餘 10 項設定連結

---

### R6.1.3 Active 狀態

#### 需求內容
- 根據目前路由顯示 active 樣式
- active 樣式: 背景粉色 (#fceeed)，文字紅色 (#e45d4f)
- 使用 React Router NavLink isActive 判斷

#### 描述
目前路由對應的選單項目顯示 active 樣式。

#### 邏輯流程
1. 使用 NavLink 元件
2. react-router-dom 提供 isActive 函式
3. 動態串接 className

#### UI 操作需求
- [ ] 瀏覽 /dashboard 時 Dashboard 顯示 active
- [ ] 瀏覽 /settings/general 時 General 顯示 active

#### 確認 [ ]
- [ ] 路由匹配時 active 樣式套用
- [ ] active 背景色 #fceeed
- [ ] active 文字色 #e45d4f

---

### R6.1.4 多國語言

#### 需求內容
- 預設繁體中文 (zh-TW): MENU=選單, SETTINGS=設定
- 英文 (en): MENU=Menu, SETTINGS=Settings
- 日文 (ja): MENU=メニュー, SETTINGS=設定
- 所有導航項目文字同步翻譯

#### 描述
選單標題與項目文字支援多國語言切換。

#### 邏輯流程
1. 使用 useTranslation hook
2. 呼叫 t(group) 和 t(item.label)
3. 從 i18n JSON 取得翻譯
4. profile dropdown 語言選擇器切換

#### UI 操作需求
- [ ] 預設顯示繁體中文
- [ ] 可切換至英文
- [ ] 可切換至日文
- [ ] 切換後所有選單文字更新

#### 確認 [ ]
- [ ] zh-TW: MENU=選單, SETTINGS=設定
- [ ] en: MENU=Menu, SETTINGS=Settings
- [ ] ja: MENU=メニュー, SETTINGS=設定
- [ ] 語言選擇器功能正常

---

### R6.1.5 品牌區域

#### 需求內容
- 顯示 SVG Logo (/icon.svg)
- 顯示品牌文字 "Corn-Syrup"
- 點擊 Logo 導航至 /dashboard

#### 描述
側邊欄頂部品牌區域，包含 Logo 與名稱。

#### 邏輯流程
1. 渲染 object 標籤顯示 SVG
2. 渲染 span 顯示文字 (條件隱藏)
3. NavLink 包覆整個區域

#### UI 操作需求
- [ ] 顯示 Logo 圖示
- [ ] 顯示 "Corn-Syrup" 文字
- [ ] 點擊導航至首頁

#### 確認 [ ]
- [ ] /icon.svg 存在並顯示
- [ ] "Corn-Syrup" 文字顯示
- [ ] 點擊導航至 /dashboard

---

### R6.1.6 無障礙 (Accessibility)

#### 需求內容
- 側邊欄 aria-label="Main Navigation"
- 選單按鈕 aria-label
- 圖示 aria-hidden="true"

#### 描述
支援螢幕閱讀器的無障礙屬性。

#### UI 操作需求
- [ ] aside 標籤有 aria-label
- [ ] button 標籤有 aria-label
- [ ] i 標籤有 aria-hidden

#### 確認 [ ]
- [ ] aside aria-label="Main Navigation"
- [ ] kero-menu-button aria-label
- [ ] kero-nav-icon aria-hidden="true"

---

## R6.2 儀表板 (Dashboard)

### R6.2.1 Dashboard 首頁

#### 需求內容
- 顯示監控項目列表
- 顯示 Portfolio Performance 指標區塊
- 指標: Monitors (數量), Incidents (數量), Uptime (可用率%)
- 支援監控項目篩選

#### UI 操作需求
- [ ] 顯示監控項目列表
- [ ] 顯示指標區塊 (3項)
- [ ] 可用率顯示為 % 非 $
- [ ] 支援篩選功能

#### 確認 [ ]
- [ ] DashboardHome.tsx 渲染
- [ ] 指標區塊顯示正確
- [ ] 可用率格式為 99.9%

---

### R6.2.2 監控項目列表

#### 需求內容
- 顯示監控名稱與狀態
- 顯示 Uptime 可用率
- 支援排序功能

#### UI 操作需求
- [ ] 列表顯示監控名稱
- [ ] 列表顯示狀態 (Up/Down/Pending/Maintenance)
- [ ] 列表顯示可用率百分比

#### 確認 [ ]
- [ ] List.tsx 渲染監控列表
- [ ] 狀態顏色正確
- [ ] 可用率計算正確

---

### R6.2.3 Uptime 組件

#### 需求內容
- 顯示可用率百分比 (%)
- 支援不同時間範圍: 24小時, 30天, 1年
- 狀態顏色: Up (綠), Down (紅), Maintenance (紫), Pending (黃)

#### UI 操作需求
- [ ] 顯示百分比格式
- [ ] 支援時間範圍切換
- [ ] 狀態顏色正確

#### 確認 [ ]
- [ ] Uptime.tsx 百分比顯示
- [ ] 24h/30d/1y 時間範圍
- [ ] 狀態顏色對應

---

## R6.3 監控管理

### R6.3.1 監控列表

#### 需求內容
- 新增監控項目
- 編輯監控項目
- 刪除監控項目
- 啟用/停用監控

#### UI 操作需求
- [ ] 顯示新增按鈕
- [ ] 顯示編輯按鈕
- [ ] 顯示刪除按鈕
- [ ] 顯示啟用/停用開關

#### 確認 [ ]
- [ ] List.tsx 提供 CRUD 操作
- [ ] 確認刪除對話框
- [ ] 啟用/停用狀態更新

---

### R6.3.2 監控編輯頁面

#### 需求內容
- 監控類型選擇 (HTTP, TCP, DNS, Ping, etc.)
- 監控名稱與 URL
- 心跳間隔設定
- 進階選項 (Headers, Body, etc.)

#### UI 操作需求
- [ ] 監控類型下拉選單
- [ ] 名稱與 URL 輸入欄位
- [ ] 間隔設定
- [ ] 進階選項展開/收合

#### 確認 [ ]
- [ ] EditMonitor.tsx 渲染
- [ ] 類型選擇功能
- [ ] 儲存功能

---

## R6.4 設定頁面 (Settings)

### R6.4.1 一般設定 (General)

#### 需求內容
- Display Timezone 選擇
- Server Timezone 選擇
- Search Engine Visibility 設定
- Entry Page 選擇
- Primary Base URL 設定
- Steam API Key
- Globalping API Token
- NSCD 啟用/停用
- Chrome/Chromium Executable 路徑
- 支援 Load/Save 按鈕 API 連結

#### UI 操作需求
- [ ] 顯示 Timezone 下拉選單
- [ ] 顯示 Visibility 選項
- [ ] 顯示 Entry Page 選項
- [ ] 顯示 API Key 輸入 (password 類型)
- [ ] Load 按鈕呼叫 getSettings API
- [ ] Save 按鈕呼叫 updateSettings API

#### 確認 [ ]
- [ ] General.tsx 渲染
- [ ] Load 功能呼叫 API
- [ ] Save 功能呼叫 API
- [ ] 顯示 loading/saving 狀態

---

### R6.4.2 外觀設定 (Appearance)

#### 需求內容
- Language 選擇 (繁體中文, English, 日本語)
- Theme 選擇 (Auto, Light, Dark)
- Heartbeat Bar 樣式選擇

#### UI 操作需求
- [ ] Language 下拉選單
- [ ] Theme 下拉選單
- [ ] Heartbeat Bar 下拉選單
- [ ] Load/Save 功能

#### 確認 [ ]
- [ ] Appearance.tsx 渲染
- [ ] Load/Save API 連結

---

### R6.4.3 通知設定 (Notifications)

#### 需求內容
- 通知供應商設定
- Error notification timeout
- Success notification timeout
- TLS Expiry Notify Days

#### UI 操作需求
- [ ] 顯示供應商列表
- [ ] 新增供應商按鈕
- [ ] 顯示 timeout 輸入欄位
- [ ] Load/Save 功能

#### 確認 [ ]
- [ ] Notifications.tsx 渲染
- [ ] Load/Save API 連結

---

### R6.4.4 反向代理設定 (Reverse Proxy)

#### 需求內容
- Trusted Proxies 設定

#### UI 操作需求
- [ ] Trusted Proxies 文字區域
- [ ] Load/Save 功能

#### 確認 [ ]
- [ ] ReverseProxy.tsx 渲染
- [ ] Load/Save API 連結

---

### R6.4.5 標籤管理 (Tags)

#### 需求內容
- 新增標籤
- 編輯標籤
- 標籤顏色選擇

#### UI 操作需求
- [ ] 新增標籤按鈕
- [ ] 標籤列表顯示
- [ ] 顏色選擇器

#### 確認 [ ]
- [ ] Tags.tsx 渲染
- [ ] Load/Save API 連結

---

### R6.4.6 監控歷史 (Monitor History)

#### 需求內容
- Keep Monitor History 天數設定
- Data Retention 啟用/停用

#### UI 操作需求
- [ ] 天數輸入欄位
- [ ] 啟用/停用開關
- [ ] Load/Save 功能

#### 確認 [ ]
- [ ] MonitorHistory.tsx 渲染
- [ ] Load/Save API 連結

---

### R6.4.7 Docker 設定 (Docker Hosts)

#### 需求內容
- Docker Daemon 路徑設定

#### UI 操作需求
- [ ] Daemon 路徑輸入
- [ ] 新增 Docker 主機按鈕
- [ ] Load/Save 功能

#### 確認 [ ]
- [ ] Docker.tsx 渲染
- [ ] Load/Save API 連結

---

### R6.4.8 遠端瀏覽器 (Remote Browsers)

#### 需求內容
- Remote Browser URL 設定

#### UI 操作需求
- [ ] URL 輸入欄位
- [ ] 新增瀏覽器按鈕
- [ ] Load/Save 功能

#### 確認 [ ]
- [ ] RemoteBrowsers.tsx 渲染
- [ ] Load/Save API 連結

---

### R6.4.9 安全設定 (Security)

#### 需求內容
- 密碼變更
- 2FA 設定

#### UI 操作需求
- [ ] 目前密碼輸入
- [ ] 新密碼輸入
- [ ] 確認密碼輸入
- [ ] 2FA 設定連結

#### 確認 [ ]
- [ ] Security.tsx 渲染 (無 Save 按鈕)

---

### R6.4.10 API 金鑰 (API Keys)

#### 需求內容
- 新增 API 金鑰
- 管理 API 金鑰列表

#### UI 操作需求
- [ ] 新增 API Key 按鈕
- [ ] API Key 列表顯示
- [ ] 複製/刪除功能

#### 確認 [ ]
- [ ] APIKeys.tsx 渲染 (無 Save 按鈕)

---

### R6.4.11 代理設定 (Proxies)

#### 需求內容
- Proxy Protocol 選擇
- Proxy Server 設定
- Proxy Authentication

#### UI 操作需求
- [ ] Protocol 下拉選單
- [ ] Server 輸入欄位
- [ ] Authentication checkbox
- [ ] Load/Save 功能

#### 確認 [ ]
- [ ] Proxies.tsx 渲染
- [ ] Load/Save API 連結

---

### R6.4.12 關於 (About)

#### 需求內容
- 系統資訊
- 連結至 Wiki/Repository

#### UI 操作需求
- [ ] 顯示版本資訊
- [ ] 顯示 Wiki 連結
- [ ] 顯示 GitHub 連結

#### 確認 [ ]
- [ ] About.tsx 渲染
- [ ] 連結正確

---

### R6.4.13 Settings 通用功能

#### 需求內容
- SettingsSection 共用元件
- 支援 Load/Save API 呼叫
- 顯示 loading/saving 狀態
- 顯示成功/錯誤訊息

#### 描述
所有 Settings 子頁面共用 SettingsSection 元件。

#### 邏輯流程
1. 子元件傳入 onLoad/onSave callback
2. SettingsSection 呼叫 callback
3. 顯示 loading/saving 狀態
4. 顯示成功/錯誤訊息

#### UI 操作需求
- [ ] loading 訊息顯示
- [ ] saving 訊息顯示
- [ ] 成功訊息顯示
- [ ] 錯誤訊息顯示

#### 確認 [ ]
- [ ] SettingsSection.tsx onLoad prop
- [ ] SettingsSection.tsx onSave prop
- [ ] 狀態與訊息顯示

---

## R6.5 狀態頁面 (Status Page)

### R6.5.1 公開狀態頁面

#### 需求內容
- 顯示監控狀態
- 顯示事件列表

#### UI 操作需求
- [ ] 公開 URL 訪問
- [ ] 顯示狀態總覽
- [ ] 顯示事件時間線

#### 確認 [ ]
- [ ] StatusPage.tsx 渲染
- [ ] 公開訪問正常

---

### R6.5.2 狀態頁面管理

#### 需求內容
- 新增狀態頁面
- 編輯狀態頁面
- 刪除狀態頁面

#### UI 操作需求
- [ ] 管理列表顯示
- [ ] 新增表單
- [ ] 編輯表單
- [ ] 刪除確認

#### 確認 [ ]
- [ ] ManageStatusPage.tsx 渲染
- [ ] CRUD 功能正常

---

## R6.6 維護管理 (Maintenance)

### R6.6.1 維護列表

#### 需求內容
- 顯示維護項目
- 新增維護

#### UI 操作需求
- [ ] 維護列表顯示
- [ ] 新增維護按鈕

#### 確認 [ ]
- [ ] ManageMaintenance.tsx 渲染

---

### R6.6.2 維護編輯

#### 需求內容
- 維護時間設定
- 影響範圍設定

#### UI 操作需求
- [ ] 開始/結束時間選擇
- [ ] 影響監控選擇
- [ ] 儲存功能

#### 確認 [ ]
- [ ] EditMaintenance.tsx 渲染

---

## R6.7 登入與認證

### R6.7.1 登入頁面

#### 需求內容
- 使用者名稱/密碼登入
- JWT Token 驗證

#### UI 操作需求
- [ ] 帳號輸入欄位
- [ ] 密碼輸入欄位
- [ ] 登入按鈕

#### 確認 [ ]
- [ ] 登入表單渲染
- [ ] API 呼叫正確

---

### R6.7.2 初始設定 (Setup)

#### 需求內容
- 首次啟動設定

#### UI 操作需求
- [ ] 建立管理員帳號
- [ ] 設定密碼

#### 確認 [ ]
- [ ] Setup.tsx 渲染

---

### R6.7.3 資料庫設定 (SetupDatabase)

#### 需求內容
- 資料庫初始化

#### UI 操作需求
- [ ] 資料庫選擇
- [ ] 連線測試

#### 確認 [ ]
- [ ] SetupDatabase.tsx 渲染

---

## R6.8 多國語言 (i18n)

### R6.8.1 語系支援

#### 需求內容
- 繁體中文 (zh-TW) - 預設
- 英文 (en)
- 日文 (ja)

#### UI 操作需求
- [ ] 語言選擇器 (profile dropdown)
- [ ] 切換後即時更新

#### 確認 [ ]
- [ ] i18n.ts 配置正確
- [ ] 三個語系檔案存在

---

### R6.8.2 翻譯內容

#### 需求內容
- 選單標題 (MENU, SETTINGS)
- 導航項目名稱
- 設定頁面欄位
- 系統訊息

#### 確認 [ ]
- [ ] zh-TW.json 完整
- [ ] en.json 完整
- [ ] ja.json 完整

---

# Phase 7: 後端 API (Backend API)

## R7.1 REST API

### R7.1.1 認證 API

| API | 方法 | 描述 |
|-----|------|------|
| /api/login | POST | 登入 |
| /api/register | POST | 註冊 |
| /api/logout | POST | 登出 |

### R7.1.2 監控 API

| API | 方法 | 描述 |
|-----|------|------|
| /api/monitors | GET | 取得監控列表 |
| /api/monitor/:id | GET | 取得監控詳情 |
| /api/monitor | POST | 新增監控 |
| /api/monitor/:id | PUT | 更新監控 |
| /api/monitor/:id | DELETE | 刪除監控 |
| /api/monitor/:id/heartbeat | GET | 取得心跳歷史 |

### R7.1.3 設定 API

| API | 方法 | 描述 |
|-----|------|------|
| /api/settings | GET | 取得設定 |
| /api/settings | PUT | 更新設定 |

### R7.1.4 狀態頁面 API

| API | 方法 | 描述 |
|-----|------|------|
| /api/status-page | GET | 取得狀態頁面列表 |
| /api/status-page/:slug | GET | 取得狀態頁面 |
| /api/status-page | POST | 新增狀態頁面 |
| /api/status-page/:id | PUT | 更新狀態頁面 |
| /api/status-page/:id | DELETE | 刪除狀態頁面 |

### 確認 [ ]
- [ ] /api/login 功能正常
- [ ] /api/monitors 功能正常
- [ ] /api/settings 功能正常
- [ ] /api/status-page 功能正常
- [ ] JWT Middleware 驗證

---

## R7.2 WebSocket API

### R7.2.1 即時更新

#### 需求內容
- 心跳狀態即時推送
- 監控狀態變更通知

#### 確認 [ ]
- [ ] Socket.IO 整合
- [ ] 心跳推送功能

---

# Phase 8: 資料模型 (Data Model)

## R8.1 核心資料表

### R8.1.1 監控 (Monitor)

| 欄位 | 類型 | 描述 |
|------|------|------|
| id | INTEGER | 主鍵 |
| name | TEXT | 名稱 |
| url | TEXT | 目標 URL |
| type | TEXT | 監控類型 |
| interval | INTEGER | 監控間隔 |
| active | BOOLEAN | 啟用狀態 |

### R8.1.2 心跳 (Heartbeat)

| 欄位 | 類型 | 描述 |
|------|------|------|
| id | INTEGER | 主鍵 |
| monitor_id | INTEGER | 監控 ID |
| status | INTEGER | 狀態碼 |
| time | DATETIME | 時間 |
| msg | TEXT | 訊息 |

### R8.1.3 使用者 (User)

| 欄位 | 類型 | 描述 |
|------|------|------|
| id | INTEGER | 主鍵 |
| username | TEXT | 帳號 |
| password | TEXT | 密碼雜湊 |
| email | TEXT |  email |
| 2fa_secret | TEXT | 2FA 密鑰 |

### R8.1.4 設定 (Setting)

| 欄位 | 類型 | 描述 |
|------|------|------|
| key | TEXT | 鍵 |
| value | TEXT | 值 |
| type | TEXT | 類型 |

### R8.1.5 標籤 (Tag)

| 欄位 | 類型 | 描述 |
|------|------|------|
| id | INTEGER | 主鍵 |
| name | TEXT | 名稱 |
| color | TEXT | 顏色 |

### R8.1.6 通知 (Notification)

| 欄位 | 類型 | 描述 |
|------|------|------|
| id | INTEGER | 主鍵 |
| name | TEXT | 名稱 |
| type | TEXT | 類型 |
| config | TEXT | 設定 JSON |

### 確認 [ ]
- [ ] Monitor 模型存在
- [ ] Heartbeat 模型存在
- [ ] User 模型存在
- [ ] Setting 模型存在

---

# 測試需求

## T1 選單功能測試案例

### T1.1 側邊欄展開/收合 (8 項)

| 編號 | 案例名稱 | 預期結果 |
|------|----------|----------|
| TC-001 | 預設展開狀態 | 側邊欄寬度 280px |
| TC-002 | 點擊收合按鈕 | 收合至 60px，圖示切換 |
| TC-003 | 點擊展開按鈕 | 展開至 280px |
| TC-004 | 收合時隱藏文字 | 僅顯示圖示 |
| TC-005 | 收合時隱藏標題 | 標題隱藏 |
| TC-006 | 收合時隱藏品牌文字 | 文字隱藏 |
| TC-007 | 收合動畫效果 | 平滑動畫 |
| TC-008 | 收合時按鈕可見 | 按鈕仍可點擊 |

### T1.2 導航項目 (8 項)

| 編號 | 案例名稱 | 預期結果 |
|------|----------|----------|
| TC-010 | MENU 群組顯示 | 顯示 3 項 |
| TC-011 | SETTINGS 群組顯示 | 顯示 12 項 |
| TC-012 | 導航連結正確 | 路由正確 |
| TC-013 | Dashboard 連結 | /dashboard |
| TC-014 | Status Pages 連結 | /manage-status-page |
| TC-015 | Maintenance 連結 | /maintenance |
| TC-016 | General 設定連結 | /settings/general |
| TC-017 | About 設定連結 | /settings/about |

### T1.3 Active 狀態 (4 項)

| 編號 | 案例名稱 | 預期結果 |
|------|----------|----------|
| TC-020 | Dashboard active | active 樣式 |
| TC-021 | Settings 內頁 active | active 樣式 |
| TC-022 | 非選單路由無 active | 無 active |
| TC-023 | active 樣式正確 | 粉紅背景紅字 |

### T1.4 多國語言 (4 項)

| 編號 | 案例名稱 | 預期結果 |
|------|----------|----------|
| TC-030 | 預設繁中 | 選單/設定 |
| TC-031 | 切換英文 | Menu/Settings |
| TC-032 | 切換日文 | メニュー/設定 |
| TC-033 | 項目文字翻譯 | 同步更新 |

### T1.5 無障礙 (3 項)

| 編號 | 案例名稱 | 預期結果 |
|------|----------|----------|
| TC-040 | 側邊欄 aria-label | Main Navigation |
| TC-041 | 選單按鈕 aria-label | 存在 |
| TC-042 | 圖示 aria-hidden | true |

### T1.6 品牌區域 (3 項)

| 編號 | 案例名稱 | 預期結果 |
|------|----------|----------|
| TC-050 | Logo 顯示 | SVG 顯示 |
| TC-051 | 品牌文字顯示 | Corn-Syrup |
| TC-052 | 品牌連結 | /dashboard |

### 確認 [ ]
- [ ] 30 個測試案例已建立
- [ ] 測試腳本可執行
- [ ] 覆蓋主要功能

---

# 附錄: 技術スタック

| 層面 | 技術 |
|------|------|
| 後端框架 | Rust, Axum, SeaORM, tokio |
| 前端框架 | React 18, TypeScript, Vite |
| 路由 | react-router-dom |
| 國際化 | i18next |
| HTTP 客戶端 | axios |
| 資料庫 | SQLite, PostgreSQL |
| 樣式 | CSS, Bootstrap, Font Awesome |
| 圖表 | Chart.js |
| 日誌 | log, env_logger (後端), loglevel (前端) |

---

# 版本資訊

| 版本 | 日期 | 描述 |
|------|------|------|
| 1.0.0 | 2026-05-04 | 初始版本，含完整需求與測試案例 |