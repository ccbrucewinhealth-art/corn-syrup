# Phase 1: 核心監控功能 (Core Monitoring)

## R1.1 HTTP/HTTPS 監控
- 支援 HTTP(s) 端點監控
- 支援 HTTP(s) 關鍵字檢測
- 支援 HTTP(s) JSON Query 檢測
- 支援自定義 HTTP 方法 (GET, POST, PUT, DELETE 等)
- 支援自定義 Headers
- 支援自定義 Request Body

## R1.2 TCP 監控
- 支援 TCP 連接監控
- 支援自定義 TCP 端口
- 支援 TCP 回應內容驗證

## R1.3 DNS 監控
- 支援 DNS 記錄類型: A, AAAA, CNAME, MX, TXT, NS, SOA
- 支援 DNS 解析驗證
- 支援 DNS 過期監控 (domain_expiry)

## R1.4 Ping 監控
- 支援 ICMP Ping 監控
- 支援回應時間測量

## R1.5 Push 監控
- 支援 Push 方式監控 (客戶端主動上報)
- 支援 REST API Push 端點

## R1.6 WebSocket 監控
- 支援 WebSocket 連接監控
- 支援 WebSocket升級監控

## R1.7 監控間隔
- 支援 20 秒監控間隔
- 支援自定義監控間隔
- 支援心跳歷史記錄 (heartbeat)

---

# Phase 2: 通知系統 (Notification System)

## R2.1 通知渠道
- 支援 Email (SMTP)
- 支援 Telegram
- 支援 Discord
- 支援 Slack
- 支援 Gotify
- 支援 Pushover
- 支援自定義 Webhook

## R2.2 通知條件
- 狀態變更時通知
- 離線時通知
- 恢復上線時通知

---

# Phase 3: 使用者介面與資料管理 (UI & Data Management)

## R3.1 使用者認證
- 支援 2FA (兩步驟驗證)
- 支援 JWT Token 認證
- 支援密碼登入

## R3.2 監控群組
- 支援監控項目群組管理
- 支援群組標籤

## R3.3 監控管理
- 新增/編輯/刪除監控項目
- 監控項目啟用/停用
- 監控項目排序

## R3.4 資料儲存
- SQLite 資料庫儲存
- 心跳記錄儲存
- 歷史資料查詢

---

# Phase 4: 進階功能 (Advanced Features)

## R4.1 狀態頁面 (Status Page)
- 公開狀態頁面
- 自定義狀態頁面樣式
- 支援多狀態頁面

## R4.2 維護模式
- 排程維護時間
- 維護期間暫停通知

## R4.3 證書資訊
- SSL 證書過期監控
- 證書資訊顯示

## R4.4 代理伺服器支援
- HTTP 代理支援
- SOCKS 代理支援

## R4.5 Prometheus 支援
- Prometheus 導出器
- Prometheus metrics 端點

---

# Phase 5: 額外監控與系統整合 (Extra Monitoring & Integration)

## R5.1 資料庫監控
- MySQL/MariaDB 監控
- PostgreSQL 監控
- MongoDB 監控
- Redis 監控
- MSSQL 監控

## R5.2 訊息隊列監控
- RabbitMQ 監控
- MQTT 監控

## R5.3 Docker 監控
- Docker 容器監控
- Docker 主機監控

## R5.4 遊戲伺服器監控
- Steam 遊戲伺服器監控
- gamedig 支援

## R5.5 其他監控
- gRPC 監控
- SNMP 監控
- SIP OPTIONS 監控
- Real Browser 監控

---

# Phase 6: 前端介面 (Frontend UI - Yew)

## R6.1 儀表板 (Dashboard)
- Dashboard 首頁 (DashboardHome.vue)
- 監控項目列表 (MonitorList.vue)
- 監控項目篩選 (MonitorListFilter.vue)

## R6.2 監控編輯 (Monitor Edit)
- 監控項目編輯頁面 (EditMonitor.vue)
- 監控條件編輯 (EditMonitorCondition.vue)

## R6.3 通知管理
- 通知設定對話框 (NotificationDialog.vue)
- 通知設定元件 (settings/*.vue)

## R6.4 狀態頁面 (Status Page)
- 公開狀態頁面 (StatusPage.vue)
- 狀態群組列表 (PublicGroupList.vue)

## R6.5 設定頁面 (Settings)
- 一般設定 (Settings.vue)
- API Key 管理 (APIKeyDialog.vue)
- 2FA 設定 (TwoFADialog.vue)
- 代理設定 (ProxyDialog.vue)

## R6.6 維護管理
- 維護列表 (ManageMaintenance.vue)
- 維護編輯 (EditMaintenance.vue)

## R6.7 圖表與數據
- Ping 圖表 (PingChart.vue)
- 心跳條顯示 (HeartbeatBar.vue)
- 正常運行時間 (Uptime.vue)

## R6.8 登入與認證
- 登入頁面 (Login.vue)
- 初始設定 (Setup.vue)
- 資料庫設定 (SetupDatabase.vue)