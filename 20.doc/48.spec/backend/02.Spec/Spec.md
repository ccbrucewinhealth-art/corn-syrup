# Phase 1: 核心監控功能 (Core Monitoring)

## S1.1 HTTP/HTTPS 監控
| 需求 | 說明 |
|------|------|
| R1.1 | 對應 monitor-types/http.js, monitor-types/https.js |
| R1.2 | 對應 monitor-types/http.js 中的 keyword 檢測 |
| R1.3 | 對應 monitor-types/http.js 中的 jsonQuery 檢測 |
| R1.4 | 對應 util-server.js 中的 HTTP request 方法 |
| R1.5 | 對應 Axios 請求自定義 Headers |
| R1.6 | 對應 Axios 請求 Request Body |

## S1.2 TCP 監控
| 需求 | 說明 |
|------|------|
| R1.7 | 對應 monitor-types/tcp.js |
| R1.8 | TCP 端口連接檢測 |
| R1.9 | TCP 回應驗證 |

## S1.3 DNS 監控
| 需求 | 說明 |
|------|------|
| R1.10 | 對應 monitor-types/dns.js, model/domain_expiry.js |
| R1.11 | DNS 記錄類型查詢 |
| R1.12 | DNS 過期檢測 |

## S1.4 Ping 監控
| 需求 | 說明 |
|------|------|
| R1.13 | 對應 @louislam/ping 套件 |
| R1.14 | ICMP Ping |

## S1.5 Push 監控
| 需求 | 說明 |
|------|------|
| R1.15 | 對應 monitor-types/manual.js Push API |
| R1.16 | /api/push 端點 |

## S1.6 WebSocket 監控
| 需求 | 說明 |
|------|------|
| R1.17 | 對應 monitor-types/websocket-upgrade.js |
| R1.18 | WS 連接測試 |

## S1.7 監控間隔
| 需求 | 說明 |
|------|------|
| R1.19 | Croner job 排程 |
| R1.20 | model/heartbeat.js 心跳記錄 |

---

# Phase 2: 通知系統 (Notification System)

## S2.1 通知渠道
| 需求 | 說明 |
|------|------|
| R2.1 | notification-providers/smtp.js |
| R2.2 | notification-providers/telegram.js |
| R2.3 | notification-providers/discord.js |
| R2.4 | notification-providers/slack.js |
| R2.5 | notification-providers/gotify.js |
| R2.6 | notification-providers/webhook.js |

## S2.2 通知條件
| 需求 | 說明 |
|------|------|
| R2.7 | server/notification.js 狀態變更邏輯 |
| R2.8 | heartbeat 離線偵測 |

---

# Phase 3: 使用者介面與資料管理 (UI & Data Management)

## S3.1 使用者認證
| 需求 | 說明 |
|------|------|
| R3.1 | server/2fa.js 2FA |
| R3.2 | server/auth.js JWT |
| R3.3 | model/user.js 使用者 |

## S3.2 監控群組
| 需求 | 說明 |
|------|------|
| R3.4 | model/group.js 群組 |
| R3.5 | model/tag.js 標籤 |

## S3.3 監控管理
| 需求 | 說明 |
|------|------|
| R3.6 | model/monitor.js CRUD |
| R3.7 | routers/api-router.js |

## S3.4 資料儲存
| 需求 | 說明 |
|------|------|
| R3.8 | server/database.js SQLite |
| R3.9 | setup-database.js 初始化 |
| R3.10 | 後端資料存取層選用 SeaORM 作為 Rust ORM framework，統一 SQLite/MySQL/MariaDB 連線設定與後續 entity/repository 實作入口 |
| R3.11 | `.env` 支援 `DATABASE_TYPE`、`DATABASE_URL`、`DATABASE_HOST`、`DATABASE_PORT`、`DATABASE_NAME`、`DATABASE_USER`、`DATABASE_PASSWORD` 與 SeaORM pool 設定 |

## S3.5 Settings API 規格（對齊前端 Settings 12 類場景）

### R3.12 API 路由與方法

| Route | Method | 用途 |
|------|--------|------|
| `/api/settings` | GET | 載入全部 settings（供所有 settings 頁籤初始化） |
| `/api/settings` | PUT | 批次更新 settings key-value |
| `/api/settings/reset` | POST | 重設指定 keys（可選） |
| `/api/settings/schema` | GET | 回傳欄位 schema（可選，供前端動態表單） |

### R3.13 前端對應 12 類 settings 分群（邏輯分群）

1. General
2. Appearance
3. Notifications
4. Reverse Proxy
5. Tags
6. Monitor History
7. Docker Hosts
8. Remote Browsers
9. Security
10. API Keys
11. Proxies
12. About（唯讀資訊，不寫入 settings）

> 說明：目前前端 [`apiClient.getSettings()`](src/frontend/src/lib/api.ts:80) 與 [`apiClient.updateSettings()`](src/frontend/src/lib/api.ts:85) 皆走單一路由 `/api/settings`，後端需依 keys 進行分群驗證與持久化。

### R3.14 Request / Response 合約

- GET `/api/settings`
  - Response
    - `ok: boolean`
    - `settings: Record<string, string | number | boolean | null>`
    - `meta: { version: number, updatedAt: string }`

- PUT `/api/settings`
  - Request
    - `section?: string`（可選，例：`general`、`appearance`）
    - `settings: Record<string, unknown>`（只傳變更欄位）
  - Response
    - `ok: boolean`
    - `appliedKeys: string[]`
    - `rejected: Array<{ key: string, reason: string }>`
    - `settings: Record<string, string | number | boolean | null>`（更新後快照）

### R3.15 Settings Model / ORM 規劃

- 新增 Entity：`settings`
  - `id` (PK)
  - `key` (unique)
  - `value_json` (text/json)
  - `value_type` (string: `string|number|boolean|null|json`)
  - `group_name` (string)
  - `is_secret` (bool)
  - `updated_by` (nullable user id)
  - `created_at`, `updated_at`

- Rust model 建議：
  - `SettingsKey`（enum，管理白名單 key）
  - `SettingsValue`（enum：String/Number/Bool/Json/Null）
  - `SettingsRecord`（domain struct）

### R3.16 驗證與安全

- 採白名單 key 驗證（未知 key 直接拒絕）
- 型別驗證：
  - `keepMonitorHistory`: number
  - `dataRetentionEnabled`: boolean
  - `tlsExpiryNotifyDays`: string（CSV）或 number[]（規格化後）
  - `theme/language/timezone`: enum 範圍檢查
- 敏感欄位遮罩與分流：
  - `steamApiKey`, `globalpingApiToken`, `currentPassword`, `newPassword`
- debug log 只記錄 key 與結果，不輸出 secret value

### R3.17 Service 邊界（建議模組）

- `src/backend/settings/schema.rs`
  - 定義 key 白名單、型別與分群
- `src/backend/settings/service.rs`
  - `load_all()`, `update_partial()`, `reset_keys()`
- `src/backend/settings/repository.rs`
  - ORM CRUD、交易處理
- `src/backend/rest/mod.rs`
  - route handler 僅處理 request parse / response 組裝

### R3.18 實作順序

1. 建立 `settings` table migration + entity
2. 實作 schema/validator（先覆蓋前端已出現 keys）
3. 實作 repository + service
4. 將 `/api/settings` GET/PUT 從 placeholder 換成 service 呼叫
5. 補上錯誤碼與整合測試（invalid key/type/permission）

---

# Phase 4: 進階功能 (Advanced Features)

## S4.1 狀態頁面
| 需求 | 說明 |
|------|------|
| R4.1 | model/status_page.js |
| R4.2 | routers/status-page-router.js |

## S4.2 維護模式
| 需求 | 說明 |
|------|------|
| R4.3 | model/maintenance.js |

## S4.3 證書資訊
| 需求 | 說明 |
|------|------|
| R4.4 | util-server.js SSL cert info |

## S4.4 代理支援
| 需求 | 說明 |
|------|------|
| R4.5 | axios http-proxy-agent |

## S4.5 Prometheus
| 需求 | 說明 |
|------|------|
| R4.6 | server/prometheus.js |

---

# Phase 5: 額外監控與系統整合 (Extra Monitoring & Integration)

## S5.1 資料庫監控
| 需求 | 說明 |
|------|------|
| R5.1 | monitor-types/mysql.js |
| R5.2 | monitor-types/postgres.js |
| R5.3 | monitor-types/mongodb.js |
| R5.4 | monitor-types/redis.js |
| R5.5 | monitor-types/mssql.js |

## S5.2 訊息隊列監控
| 需求 | 說明 |
|------|------|
| R5.6 | monitor-types/rabbitmq.js |
| R5.7 | monitor-types/mqtt.js |

## S5.3 Docker 監控
| 需求 | 說明 |
|------|------|
| R5.8 | server/docker.js |
| R5.9 | model/docker_host.js |

## S5.4 遊戲伺服器監控
| 需求 | 說��� |
|------|------|
| R5.10 | monitor-types/gamedig.js |

## S5.5 其他監控
| 需求 | 說明 |
|------|------|
| R5.11 | monitor-types/grpc.js |
| R5.12 | monitor-types/snmp.js |
| R5.13 | monitor-types/real-browser-monitor-type.js |

---

# Phase 6: 前端介面 (Frontend UI - Yew)

## S6.1 儀表板
| 需求 | 來源 | 目標 (Rust/Yew) |
|------|------|-----------------|
| R6.1 | pages/DashboardHome.vue | src/web/pages/dashboard_home.rs |
| R6.2 | pages/Dashboard.vue | src/web/pages/dashboard.rs |
| R6.3 | components/MonitorList.vue | src/web/components/monitor_list.rs |
| R6.4 | components/MonitorListFilter.vue | src/web/components/monitor_list_filter.rs |
| R6.5 | components/MonitorListItem.vue | src/web/components/monitor_list_item.rs |

## S6.2 監控編輯
| 需求 | 來源 | 目標 (Rust/Yew) |
|------|------|-----------------|
| R6.6 | pages/EditMonitor.vue | src/web/pages/edit_monitor.rs |
| R6.7 | components/EditMonitorCondition.vue | src/web/components/edit_monitor_condition.rs |
| R6.8 | components/EditMonitorConditions.vue | src/web/components/edit_monitor_conditions.rs |

## S6.3 通知
| 需求 | 來源 | 目標 (Rust/Yew) |
|------|------|-----------------|
| R6.9 | components/NotificationDialog.vue | src/web/components/notification_dialog.rs |
| R6.10 | components/notifications/*.vue | src/web/components/notifications/*.rs |

## S6.4 狀態頁面
| 需求 | 來源 | 目標 (Rust/Yew) |
|------|------|-----------------|
| R6.11 | pages/StatusPage.vue | src/web/pages/status_page.rs |
| R6.12 | components/PublicGroupList.vue | src/web/components/public_group_list.rs |

## S6.5 設定
| 需求 | 來源 | 目標 (Rust/Yew) |
|------|------|-----------------|
| R6.13 | pages/Settings.vue | src/web/pages/settings.rs |
| R6.14 | components/APIKeyDialog.vue | src/web/components/api_key_dialog.rs |
| R6.15 | components/TwoFADialog.vue | src/web/components/two_fa_dialog.rs |
| R6.16 | components/ProxyDialog.vue | src/web/components/proxy_dialog.rs |
| R6.17 | components/settings/*.vue | src/web/components/settings/*.rs |

## S6.6 維護管理
| 需求 | 來源 | 目標 (Rust/Yew) |
|------|------|-----------------|
| R6.18 | pages/ManageMaintenance.vue | src/web/pages/manage_maintenance.rs |
| R6.19 | pages/EditMaintenance.vue | src/web/pages/edit_maintenance.rs |

## S6.7 圖表與數據
| 需求 | 來源 | 目標 (Rust/Yew) |
|------|------|-----------------|
| R6.20 | components/PingChart.vue | src/web/components/ping_chart.rs |
| R6.21 | components/HeartbeatBar.vue | src/web/components/heartbeat_bar.rs |
| R6.22 | components/Uptime.vue | src/web/components/uptime.rs |

## S6.8 登入與認證
| 需求 | 來源 | 目標 (Rust/Yew) |
|------|------|-----------------|
| R6.23 | components/Login.vue | src/web/components/login.rs |
| R6.24 | pages/Setup.vue | src/web/pages/setup.rs |
| R6.25 | pages/SetupDatabase.vue | src/web/components/setup_database.rs |

## S6.9 共用元件
| 需求 | 來源 | 目標 (Rust/Yew) |
|------|------|-----------------|
| R6.26 | components/Confirm.vue | src/web/components/confirm.rs |
| R6.27 | components/Datetime.vue | src/web/components/datetime.rs |
| R6.28 | components/Tag.vue | src/web/components/tag.rs |
| R6.29 | src/router.js | src/web/router.rs |
| R6.30 | src/util.ts | src/web/util.rs |
| R6.31 | src/i18n.js | src/web/i18n/mod.rs |
