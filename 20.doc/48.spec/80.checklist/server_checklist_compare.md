# 參考系統 server 目錄完整檔案清單

## 根目錄檔案 (server/)

| 序號 | 檔案 | 說明 | Check List 狀態 |
|------|------|------|------------------|
| 1 | server.js | 主伺服器入口 (69KB) | Phase1-1 已列入 |
| 2 | database.js | 資料庫管理 (35KB) | Phase1-2 已列入 |
| 3 | util-server.js | 工具函數 (34KB) | Phase1-3 已列入 |
| 4 | settings.js | 系統設定 | Phase1-5 已列入 |
| 5 | config.js | 設定配置 | Phase1-4 已列入 |
| 6 | auth.js | 認證模組 (5KB) | Phase4-27 已列入 |
| 7 | 2fa.js | 2FA 驗證 | Phase4-28 已列入 |
| 8 | client.js | 客戶端 Socket | ⚠️ 漏列 |
| 9 | notification.js | 通知管理 (12KB) | Phase2-12 已列入 |
| 10 | server-kuma-server.js | 伺服器核心 (20KB) | ⚠️ 漏列 |
| 11 | uptime-calculator.js | 正常運行時間計算 | ⚠️ 漏列 |
| 12 | prometheus.js | Prometheus 導出器 | Phase4-已列入 |
| 13 | proxy.js | 代理伺服器 | ⚠️ 漏列 |
| 14 | remote-browser.js | 遠端瀏覽器 | ⚠️ 漏列 |
| 15 | docker.js | Docker 監控 | Phase5-已列入 |
| 16 | embedded-mariadb.js | 內嵌 MariaDB | ⚠️ 漏列 |
| 17 | image-data-uri.js | 圖片處理 | ⚠️ 漏列 |
| 18 | radius-client.js | RADIUS 客戶端 | ⚠️ 漏列 |
| 19 | rate-limiter.js | 速率限制 | ⚠️ 漏列 |
| 20 | check-version.js | 版本檢查 | ⚠️ 漏列 |
| 21 | jobs.js | 定時任務 | ⚠️ 漏列 |
| 22 | setup-database.js |資料庫初始化 | Phase3-已列入 |
| 23 | password-hash.js | 密碼雜湊 | ⚠️ 漏列 |
| 24 | translatable-error.js | 可翻譯錯誤 | ⚠️ 漏列 |

---

## analytics/ 目錄

| 序號 | 檔案 | 說明 | Check List 狀態 |
|------|------|------|------------------|
| 25 | analytics.js | Analytics 主模組 | ⚠️ 漏列 |
| 26 | google-analytics.js | Google Analytics | ⚠️ 漏列 |
| 27 | matomo-analytics.js | Matomo | ⚠️ 漏列 |
| 28 | plausible-analytics.js | Plausible | ⚠️ 漏列 |
| 29 | umami-analytics.js | Umami | ⚠️ 漏列 |

---

## jobs/ 目錄

| 序號 | 檔案 | 說明 | Check List 狀態 |
|------|------|------|------------------|
| 30 | clear-old-data.js | 清除舊資料 | ⚠️ 漏列 |
| 31 | incremental-vacuum.js | 資料庫優化 | ⚠️ 漏列 |

---

## model/ 目錄

| 序號 | 檔案 | 說明 | Check List 狀態 |
|------|------|------|------------------|
| 32 | monitor.js | 監控模型 (84KB) | Phase3-18 已列入 |
| 33 | heartbeat.js | 心跳模型 | Phase3-19 已列入 |
| 34 | user.js | 使用者模型 | Phase3-20 已列入 |
| 35 | group.js | 群組模型 | Phase3-21 已列入 |
| 36 | tag.js | 標籤模型 | Phase3-22 已列入 |
| 37 | status_page.js | 狀態頁面模型 | Phase3-23 已列入 |
| 38 | maintenance.js | 維護模型 (17KB) | Phase3-24 已列入 |
| 39 | domain_expiry.js | 網域過期模型 | Phase3-25 已列入 |
| 40 | api_key.js | API Key 模型 | ⚠️ 漏列 |
| 41 | proxy.js | 代理模型 | ⚠️ 漏列 |
| 42 | remote_browser.js | 遠端瀏覽器模型 | ⚠️ 漏列 |
| 43 | docker_host.js | Docker 主機模型 | ⚠️ 漏列 |
| 44 | incident.js | 事件模型 | ⚠️ 漏列 |

---

## monitor-types/ 目錄

| 序號 | 檔案 | 說明 | Check List 狀態 |
|------|------|------|------------------|
| 45 | http.js | HTTP 監控 | Phase1-6 已列入 |
| 46 | tcp.js | TCP 監控 | Phase1-7 已列入 |
| 47 | dns.js | DNS 監控 | Phase1-8 已列入 |
| 48 | ping.js | Ping 監控 | Phase1-9 不需轉換 |
| 49 | manual.js | Push 監控 | Phase1-10 已列入 |
| 50 | websocket-upgrade.js | WebSocket 監控 | Phase1-11 已列入 |
| 51 | mysql.js | MySQL 監控 | Phase5-29 已列入 |
| 52 | postgres.js | PostgreSQL 監控 | Phase5-30 已列入 |
| 53 | mongodb.js | MongoDB 監控 | Phase5-31 已列入 |
| 54 | redis.js | Redis 監控 | Phase5-32 已列入 |
| 55 | mssql.js | MSSQL 監控 | Phase5-33 已列入 |
| 56 | rabbitmq.js | RabbitMQ 監控 | Phase5-34 已列入 |
| 57 | mqtt.js | MQTT 監控 | Phase5-35 已列入 |
| 58 | grpc.js | gRPC 監控 | Phase5-36 已列入 |
| 59 | snmp.js | SNMP 監控 | Phase5-37 已列入 |
| 60 | smtp.js | SMTP 監控 | ⚠️ 漏列 |
| 61 | oracledb.js | OracleDB 監控 | ⚠️ 漏列 |
| 62 | gamedig.js | 遊戲伺服器監控 | ⚠️ 漏列 |
| 63 | globalping.js | GlobalPing 監控 | ⚠️ 漏列 |
| 64 | group.js | 群組監控 | ⚠️ 漏列 |
| 65 | monitor-type.js | 監控類型基底 | ⚠️ 漏列 |
| 66 | system-service.js | 系統服務監控 | ⚠️ 漏列 |
| 67 | tailscale-ping.js | Tailscale Ping | ⚠️ 漏列 |
| 68 | real-browser-monitor-type.js | 瀏覽器監控 | ⚠️ 漏列 |
| 69 | sip-options.js | SIP OPTIONS | ⚠️ 漏列 |

---

## notification-providers/ 目錄

| 序號 | 檔案 | 說明 | Check List 狀態 |
|------|------|------|------------------|
| 70 | notification-provider.js | 通知基底 | ⚠️ 漏列 |
| 71 | smtp.js | SMTP 通知 | Phase2-13 已列入 |
| 72 | telegram.js | Telegram 通知 | Phase2-14 已列入 |
| 73 | discord.js | Discord 通知 | Phase2-15 已列入 |
| 74 | slack.js | Slack 通知 | Phase2-16 已列入 |
| 75 | webhook.js | Webhook 通知 | Phase2-17 已列入 |
| 76 | gotify.js | Gotify | ⚠️ 漏列 |
| 77 | pushover.js | Pushover | ⚠️ 漏列 |
| 78 | teams.js | Microsoft Teams | ⚠️ 漏列 |
| 79 | line.js | LINE Notify | ⚠️ 漏列 |
| 80 | email.js | Email | ⚠️ 漏列 |
| 81 |-alertnow.js | AlertNow | ⚠️ 漏列 |
| 82 | aliyun-sms.js | Aliyun SMS | ⚠️ 漏列 |
| 83 | bark.js | Bark | ⚠️ 漏列 |
| 84 | call-me-bot.js | CallMeBot | ⚠️ 漏列 |
| 85 | cellsynt.js | Cellsynt | ⚠️ 漏列 |
| 86 | clicksendsms.js | ClickSend SMS | ⚠️ 漏列 |
| 87 | dingding.js | 釘釘 | ⚠️ 漏列 |
| 88 | feishu.js | 飛書 | ⚠️ 漏列 |
| 89 | fluxer.js | Fluxer | ⚠️ 漏列 |
| 90 | free-mobile.js | Free Mobile | ⚠️ 漏列 |
| 91 | gotify.js | Gotify | ⚠️ 漏列 |
| 92 | gorush.js | Gorush | ⚠️ 漏列 |
| 93 | grafana-oncall.js | Grafana OnCall | ⚠️ 漏列 |
| 94 | heii-oncall.js | Heii OnCall | ⚠️ 漏列 |
| 95 | home-assistant.js | Home Assistant | ⚠️ 漏列 |
| 96 | jira-service-management.js | Jira SM | ⚠️ 漏列 |
| 97 | kook.js | KOOK | ⚠️ 漏列 |
| 98 | lunasea.js | LunaSea | ⚠️ 漏列 |
| 99 | matrix.js | Matrix | ⚠️ 漏列 |
| 100 | mattermost.js | Mattermost | ⚠️ 漏列 |
| 101 | nostr.js | Nostr | ⚠️ 漏列 |
| 102 | ntfy.js | Ntfy | ⚠️ 漏列 |
| 103 | octopush.js | Octopush | ⚠️ 漏列 |
| 104 | onebot.js | OneBot | ⚠️ 漏列 |
| 105 | opsgenie.js | Opsgenie | ⚠️ 漏列 |
| 106 | pagerduty.js | PagerDuty | ⚠️ 漏列 |
| 107 | pagertree.js | PagerTree | ⚠️ 漏列 |
| 108 | promosms.js | PromoSMS | ⚠️ 漏列 |
| 109 | pushbullet.js | Pushbullet | ⚠️ 漏列 |
| 110 | pushdeer.js | PushDeer | ⚠️ 漏列 |
| 111 | pushplus.js | PushPlus | ⚠️ 漏列 |
| 112 | pushy.js | Pushy | ⚠️ 漏列 |
| 113 | rocket-chat.js | Rocket.Chat | ⚠️ 漏列 |
| 114 | serverchan.js | 伺服器chan | ⚠️ 漏列 |
| 115 | signal.js | Signal | ⚠️ 漏列 |
| 116 | smseagle.js | SMSEagle | ⚠️ 漏列 |
| 117 | smsir.js | SMS.ir | ⚠️ 漏列 |
| 118 | telegram.js | Telegram | ⚠️ 漏列 |
| 119 | twilio.js | Twilio | ⚠️ 漏列 |
| 120 | wecom.js | 企業微信 | ⚠️ 漏列 |
| 121 | yzj.js | 雲之訊 | ⚠️ 漏列 |

---

## router/ 目錄

| 序號 | 檔案 | 說明 | Check List 狀態 |
|------|------|------|------------------|
| 122 | api-router.js | API 路由 | Phase3-26 已列入 |
| 123 | status-page-router.js | 狀態頁面路由 | ⚠️ 漏列 |

---

## socket-handlers/ 目錄

| 序號 | 檔案 | 說明 | Check List 狀態 |
|------|------|------|------------------|
| 124 | general-socket-handler.js | 一般 Socket | ⚠️ 漏列 |
| 125 | api-key-socket-handler.js | API Key Socket | ⚠️ 漏列 |
| 126 | chart-socket-handler.js | 圖表 Socket | ⚠️ 漏列 |
| 127 | cloudflared-socket-handler.js | Cloudflare Tunnel | ⚠️ 漏列 |
| 128 | database-socket-handler.js | 資料庫 Socket | ⚠️ 漏列 |
| 129 | docker-socket-handler.js | Docker Socket | ⚠️ 漏列 |
| 130 | maintenance-socket-handler.js | 維護 Socket | ⚠️ 漏列 |
| 131 | proxy-socket-handler.js | 代理 Socket | ⚠️ 漏列 |
| 132 | remote-browser-socket-handler.js | 遠端瀏覽器 | ⚠️ 漏列 |
| 133 | status-page-socket-handler.js | 狀態頁面 Socket | ⚠️ 漏列 |

---

## monitor-conditions/ 目錄

| 序號 | 檔案 | 說明 | Check List 狀態 |
|------|------|------|------------------|
| 134 | evaluator.js | 條件評估 | ⚠️ 漏列 |
| 135 | expression.js | 表達式 | ⚠️ 漏列 |
| 136 | operators.js | 運算子 | ⚠️ 漏列 |
| 137 | variables.js | 變數 | ⚠️ 漏列 |

---

## 總統計

| 類別 | 總數 | 已列入 | 漏列 |
|------|------|--------|------|
| 根目錄 | 24 | 12 | 12 |
| analytics/ | 5 | 0 | 5 |
| jobs/ | 2 | 0 | 2 |
| model/ | 13 | 9 | 4 |
| monitor-types/ | 25 | 11 | 14 |
| notification-providers/ | 52+ | 6 | 46+ |
| router/ | 2 | 1 | 1 |
| socket-handlers/ | 10 | 0 | 10 |
| monitor-conditions/ | 4 | 0 | 4 |
| **合計** | **137+** | **~39** | **~98** |