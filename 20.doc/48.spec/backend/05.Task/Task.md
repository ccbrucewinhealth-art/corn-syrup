# Phase 1: 核心監控功能工作項目

## T1.1 專案初始化
| 工作 | 說明 |
|------|------|
| T1.1.1 | 執行 `cargo new corn-syrup` 初始化 Rust 專案 |
| T1.1.2 | 配置 Cargo.toml 依賴 |
| T1.1.3 | 建立 src/main.rs 入口 |

## T1.2 HTTP 監控
| 工作 | 說明 |
|------|------|
| T1.2.1 | 建立 src/monitor/http.rs |
| T1.2.2 | 實作 HTTP 請求與回應處理 |
| T1.2.3 | 實作關鍵字檢測 |
| T1.2.4 | 實作 JSON Query 解析 |

## T1.3 TCP 監控
| 工作 | 說明 |
|------|------|
| T1.3.1 | 建立 src/monitor/tcp.rs |
| T1.3.2 | 實作 TCP 連接測試 |
| T1.3.3 | 實作回應驗證 |

## T1.4 DNS 監控
| 工作 | 說明 |
|------|------|
| T1.4.1 | 建立 src/monitor/dns.rs |
| T1.4.2 | 實作 DNS 查詢 |
| T1.4.3 | 實作 DNS 過期檢測 |

## T1.5 Ping 監控
| 工作 | 說明 |
|------|------|
| T1.5.1 | 建立 src/monitor/ping.rs |
| T1.5.2 | 實作 ICMP ping |

## T1.6 Push API
| 工作 | 說明 |
|------|------|
| T1.6.1 | 建立 src/api/push.rs |
| T1.6.2 | 實作 Push 端點 |

## T1.7 WebSocket 監控
| 工作 | 說明 |
|------|------|
| T1.7.1 | 建立 src/monitor/websocket.rs |
| T1.7.2 | 實作 WS 連接測試 |

---

# Phase 2: 通知系統工作項目

## T2.1 HTTP 通知
| 工作 | 說明 |
|------|------|
| T2.1.1 | 建立 src/notification/http.rs |
| T2.1.2 | 實作 Webhook 通知 |

## T2.2 SMTP 通知
| 工作 | 說明 |
|------|------|
| T2.2.1 | 建立 src/notification/smtp.rs |
| T2.2.2 | 實作 Email 發送 |

## T2.3 即時通訊通知
| 工作 | 說明 |
|------|------|
| T2.3.1 | 建立 src/notification/telegram.rs |
| T2.3.2 | 建立 src/notification/discord.rs |
| T2.3.3 | 建立 src/notification/slack.rs |

---

# Phase 3: 資料管理與 API 工作項目

## T3.1 SQLite 資料庫
| 工作 | 說明 |
|------|------|
| T3.1.1 | 建立 src/database/mod.rs |
| T3.1.2 | 配置 sqlx 或 rusqlite |
| T3.1.3 | 實作資料庫連線池 |

## T3.2 資料模型
| 工作 | 說明 |
|------|------|
| T3.2.1 | 建立 src/model/monitor.rs |
| T3.2.2 | 建立 src/model/heartbeat.rs |
| T3.2.3 | 建立 src/model/user.rs |
| T3.2.4 | 建立 src/model/group.rs |

## T3.3 REST API
| 工作 | 說明 |
|------|------|
| T3.3.1 | 建立 src/api/mod.rs |
| T3.3.2 | 實作 CRUD endpoints |
| T3.3.3 | 建立 src/websocket/handler.rs |

---

# Phase 4: 進階功能工作項目

## T4.1 狀態頁面
| 工作 | 說明 |
|------|------|
| T4.1.1 | 建立 src/model/status_page.rs |
| T4.1.2 | 實作公開 API |

## T4.2 維護模式
| 工作 | 說明 |
|------|------|
| T4.2.1 | 建立 src/model/maintenance.rs |
| T4.2.2 | 實作排程處理 |

## T4.3 Prometheus
| 工作 | 說明 |
|------|------|
| T4.3.1 | 建立 src/metrics/mod.rs |
| T4.3.2 | 實作 metrics 端點 |

---

# Phase 5: 額外監控工作項目

## T5.1 資料庫監控
| 工作 | 說明 |
|------|------|
| T5.1.1 | 建立 src/monitor/mysql.rs |
| T5.1.2 | 建立 src/monitor/postgres.rs |
| T5.1.3 | 建立 src/monitor/mongodb.rs |
| T5.1.4 | 建立 src/monitor/redis.rs |
| T5.1.5 | 建立 src/monitor/mssql.rs |

## T5.2 訊息隊列
| 工作 | 說明 |
|------|------|
| T5.2.1 | 建立 src/monitor/rabbitmq.rs |
| T5.2.2 | 建立 src/monitor/mqtt.rs |

## T5.3 Docker
| 工作 | 說明 |
|------|------|
| T5.3.1 | 建立 src/monitor/docker.rs |
| T5.3.2 | 實作 Docker API 查詢 |

---

# Phase 6: 前端工作項目 (Yew)

## T6.1 專案初始化
| 工作 | 說明 |
|------|------|
| T6.1.1 | 初始化 Yew 前端專案 |
| T6.1.2 | 配置 Cargo.toml (frontend) |
| T6.1.3 | 建立 index.html |
| T6.1.4 | 設定 trunk.config.toml |

## T6.2 ���礎架構
| 工作 | 說明 |
|------|------|
| T6.2.1 | 建立 src/lib.rs |
| T6.2.2 | 建立 src/main.rs |
| T6.2.3 | 建立 src/app.rs |

## T6.3 路由
| 工作 | 說明 |
|------|------|
| T6.3.1 | 建立 src/router.rs |
| T6.3.2 | 設定路由表 |
| T6.3.3 | 實作認證保護 |

## T6.4 狀態管理
| 工作 | 說明 |
|------|------|
| T6.4.1 | 建立 src/store/mod.rs |
| T6.4.2 | 建立 src/store/auth.rs |
| T6.4.3 | 建立 src/store/monitor.rs |

## T6.5 API 客戶端
| 工作 | 說明 |
|------|------|
| T6.5.1 | 建立 src/api/mod.rs |
| T6.5.2 | 建立 src/api/client.rs |
| T6.5.3 | 實作 HTTP 請求 |

## T6.6 登入頁面
| 工作 | 說明 |
|------|------|
| T6.6.1 | 建立 src/pages/login.rs |
| T6.6.2 | 實作登入表單 |
| T6.6.3 | 實作 2FA 輸入 |

## T6.7 儀表板
| 工作 | 說明 |
|------|------|
| T7.7.1 | 建立 src/pages/dashboard.rs |
| T7.7.2 | 建立 src/pages/dashboard_home.rs |
| T7.7.3 | 實作監控列表 |
| T7.7.4 | 實作監控篩選 |

## T6.8 監控編輯
| 工作 | 說明 |
|------|------|
| T6.8.1 | 建立 src/pages/edit_monitor.rs |
| T6.8.2 | 實作監控類型選擇 |
| T6.8.3 | 實作監控條件 |

## T6.9 設定頁面
| 工作 | 說明 |
|------|------|
| T6.9.1 | 建立 src/pages/settings.rs |
| T6.9.2 | 建立 src/components/settings/*.rs |
| T6.9.3 | 實作 API Key 管理 |

## T6.10 狀態頁面
| 工作 | 說明 |
|------|------|
| T6.10.1 | 建立 src/pages/status_page.rs |
| T6.10.2 | 實作公開狀態顯示 |

## T6.11 維護管理
| 工作 | 說明 |
|------|------|
| T6.11.1 | 建立 src/pages/maintenance.rs |
| T6.11.2 | 建立 src/pages/edit_maintenance.rs |

## T6.12 圖表
| 工作 | 說明 |
|------|------|
| T6.12.1 | 建立 src/components/chart.rs |
| T6.12.2 | 實作 Ping 圖表 |

## T6.13 國際化
| 工作 | 說明 |
|------|------|
| T6.13.1 | 建立 src/i18n/mod.rs |
| T6.13.2 | 加入翻譯檔 |

## T6.14 UI 元件
| 工作 | 說明 |
|------|------|
| T6.14.1 | 建立 src/components/button.rs |
| T6.14.2 | 建立 src/components/input.rs |
| T6.14.3 | 建立 src/components/modal.rs |
| T6.14.4 | 建立 src/components/table.rs |
| T6.14.5 | 建立 src/components/badge.rs |