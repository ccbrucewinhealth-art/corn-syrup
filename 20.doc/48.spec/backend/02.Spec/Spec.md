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