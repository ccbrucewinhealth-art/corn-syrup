# 參考系統 server 目錄完整檔案清單 (v3)

## 一、server/ 根目錄 (25 個)

| 序號 | 檔案 | 已在 Check List |
|------|------|---------------|
| 1 | server.js | ✅ Phase1-1 |
| 2 | database.js | ✅ Phase1-2 |
| 3 | util-server.js | ✅ Phase1-3 |
| 4 | settings.js | ✅ Phase1-5 |
| 5 | config.js | ✅ Phase1-4 |
| 6 | auth.js | ✅ Phase4-27 |
| 7 | 2fa.js | ✅ Phase4-28 |
| 8 | client.js | ✅ Phase1-5.3 |
| 9 | server-kuma-server.js | ✅ Phase1-5.1 |
| 10 | uptime-calculator.js | ✅ Phase1-5.2 |
| 11 | prometheus.js | ✅ Phase4-已列入 |
| 12 | notification.js | ✅ Phase2-12 |
| 13 | proxy.js | ✅ Phase4-28.1 |
| 14 | remote-browser.js | ✅ Phase4-28.2 |
| 15 | docker.js | ✅ Phase4-28.4 |
| 16 | embedded-mariadb.js | ✅ Phase4-28.5 |
| 17 | image-data-uri.js | ✅ Phase4-28.6 |
| 18 | radius-client.js | ✅ Phase4-28.3 |
| 19 | rate-limiter.js | ✅ Phase1-5.4 |
| 20 | check-version.js | ✅ Phase1-5.5 |
| 21 | jobs.js | ✅ Phase4-28.7 |
| 22 | setup-database.js | ✅ Phase3-26.2 |
| 23 | password-hash.js | ✅ Phase1-5.6 |
| 24 | translatable-error.js | ✅ Phase1-5.7 |

**合計: 24 項全數列入**

---

## 二、analytics/ (5 個)

| 序號 | 檔案 | 已在 Check List |
|------|------|---------------|
| 1 | analytics.js | ✅ Phase6-38 |
| 2 | google-analytics.js | ✅ Phase6-38.1 |
| 3 | matomo-analytics.js | ✅ Phase6-38.2 |
| 4 | plausible-analytics.js | ✅ Phase6-38.3 |
| 5 | umami-analytics.js | ✅ Phase6-38.4 |

**合計: 5 項全數列入**

---

## 三、jobs/ (2 個)

| 序號 | 檔案 | 已在 Check List |
|------|------|---------------|
| 1 | clear-old-data.js | ✅ Phase7-39 |
| 2 | incremental-vacuum.js | ✅ Phase7-39.1 |

**合計: 2 項全數列入**

---

## 四、model/ (13 個)

| 序號 | 檔案 | 已在 Check List |
|------|------|---------------|
| 1 | monitor.js | ✅ Phase3-18 |
| 2 | heartbeat.js | ✅ Phase3-19 |
| 3 | user.js | ✅ Phase3-20 |
| 4 | group.js | ✅ Phase3-21 |
| 5 | tag.js | ✅ Phase3-22 |
| 6 | status_page.js | ✅ Phase3-23 |
| 7 | maintenance.js | ✅ Phase3-24 |
| 8 | domain_expiry.js | ✅ Phase3-25 |
| 9 | api_key.js | ✅ Phase3-25.1 |
| 10 | proxy.js | ✅ Phase3-25.2 |
| 11 | remote_browser.js | ✅ Phase3-25.3 |
| 12 | docker_host.js | ✅ Phase3-25.4 |
| 13 | incident.js | ✅ Phase3-25.5 |

**合計: 13 項全數列入**

---

## 五、monitor-types/ (25 個)

| 序號 | 檔案 | 已在 Check List |
|------|------|---------------|
| 1 | http.js | ✅ Phase1-6 |
| 2 | tcp.js | ✅ Phase1-7 |
| 3 | dns.js | ✅ Phase1-8 |
| 4 | ping.js | ⚠️ 外部套件 |
| 5 | manual.js | ✅ Phase1-10 |
| 6 | websocket-upgrade.js | ✅ Phase1-11 |
| 7 | smtp.js | ✅ Phase1-11.1 |
| 8 | oracledb.js | ✅ Phase1-11.2 |
| 9 | gamedig.js | ✅ Phase1-11.3 |
| 10 | globalping.js | ✅ Phase1-11.4 |
| 11 | group.js | ✅ Phase1-11.5 |
| 12 | monitor-type.js | ✅ Phase1-11.10 |
| 13 | mqtt.js | ✅ Phase5-35 |
| 14 | mssql.js | ✅ Phase5-33 |
| 15 | mysql.js | ✅ Phase5-29 |
| 16 | postgres.js | ✅ Phase5-30 |
| 17 | rabbitmq.js | ✅ Phase5-34 |
| 18 | redis.js | ✅ Phase5-32 |
| 19 | grpc.js | ✅ Phase5-36 |
| 20 | snmp.js | ✅ Phase5-37 |
| 21 | system-service.js | ✅ Phase1-11.6 |
| 22 | tailscale-ping.js | ✅ Phase1-11.7 |
| 23 | real-browser-monitor-type.js | ✅ Phase1-11.8 |
| 24 | sip-options.js | ✅ Phase1-11.9 |
| 25 | mongodb.js | ✅ Phase5-31 |

**合計: 24 項列入 (1 項外部套件)**

---

## 六、monitor-conditions/ (4 個)

| 序號 | 檔案 | 已在 Check List |
|------|------|---------------|
| 1 | evaluator.js | ✅ Phase8-40 |
| 2 | expression.js | ✅ Phase8-40.1 |
| 3 | operators.js | ✅ Phase8-40.2 |
| 4 | variables.js | ✅ Phase8-40.3 |

**合計: 4 項全數列入**

---

## 七、notification-providers/ (93 個)

### 完整清單：

| 序號 | 檔案 | Check List 編號 | 狀態 |
|------|------|---------------|------|
| 1 | notification-provider.js | Phase2-17.1 | ✅ |
| 2 | smtp.js | Phase2-13 | ✅ |
| 3 | telegram.js | Phase2-14 | ✅ |
| 4 | discord.js | Phase2-15 | ✅ |
| 5 | slack.js | Phase2-16 | ✅ |
| 6 | webhook.js | Phase2-17 | ✅ |
| 7 | gotify.js | Phase2-17.2 | ✅ |
| 8 | pushover.js | Phase2-17.3 | ✅ |
| 9 | teams.js | Phase2-17.4 | ✅ |
| 10 | line.js | Phase2-17.5 | ✅ |
| 11 | email.js | Phase2-17.6 | ✅ |
| 12 | alertnow.js | Phase2-17.7 | ✅ |
| 13 | aliyun-sms.js | Phase2-17.8 | ✅ |
| 14 | bark.js | Phase2-17.9 | ✅ |
| 15 | call-me-bot.js | Phase2-17.10 | ✅ |
| 16 | cellsynt.js | Phase2-17.11 | ✅ |
| 17 | clicksendsms.js | Phase2-17.12 | ✅ |
| 18 | dingding.js | Phase2-17.13 | ✅ |
| 19 | feishu.js | Phase2-17.14 | ✅ |
| 20 | fluxer.js | Phase2-17.15 | ✅ |
| 21 | freemobile.js | Phase2-17.16 | ✅ |
| 22 | gorush.js | Phase2-17.17 | ✅ |
| 23 | grafana-oncall.js | Phase2-17.18 | ✅ |
| 24 | heii-oncall.js | Phase2-17.19 | ✅ |
| 25 | home-assistant.js | Phase2-17.20 | ✅ |
| 26 | jira-service-management.js | Phase2-17.21 | ✅ |
| 27 | kook.js | Phase2-17.22 | ✅ |
| 28 | lunasea.js | Phase2-17.23 | ✅ |
| 29 | matrix.js | Phase2-17.24 | ✅ |
| 30 | mattermost.js | Phase2-17.25 | ✅ |
| 31 | nostr.js | Phase2-17.26 | ✅ |
| 32 | ntfy.js | Phase2-17.27 | ✅ |
| 33 | octopush.js | Phase2-17.28 | ✅ |
| 34 | onebot.js | Phase2-17.29 | ✅ |
| 35 | opsgenie.js | Phase2-17.30 | ✅ |
| 36 | pagerduty.js | Phase2-17.31 | ✅ |
| 37 | pagertree.js | Phase2-17.32 | ✅ |
| 38 | promosms.js | Phase2-17.33 | ✅ |
| 39 | pushbullet.js | Phase2-17.34 | ✅ |
| 40 | pushdeer.js | Phase2-17.35 | ✅ |
| 41 | pushplus.js | Phase2-17.36 | ✅ |
| 42 | pushy.js | Phase2-17.37 | ✅ |
| 43 | rocket-chat.js | Phase2-17.38 | ✅ |
| 44 | serverchan.js | Phase2-17.39 | ✅ |
| 45 | signal.js | Phase2-17.40 | ✅ |
| 46 | smseagle.js | Phase2-17.41 | ✅ |
| 47 | smsir.js | Phase2-17.42 | ✅ |
| 48 | twilio.js | Phase2-17.43 | ✅ |
| 49 | wecom.js | Phase2-17.44 | ✅ |
| 50 | yzj.js | Phase2-17.45 | ✅ |
| 51 | 46elks.js | Phase2-17.46 | ✅ |
| 52 | bale.js | Phase2-17.47 | ✅ |
| 53 | bitrix24.js | Phase2-17.48 | ✅ |
| 54 | brevo.js | Phase2-17.49 | ✅ |
| 55 | flashduty.js | Phase2-17.50 | ✅ |
| 56 | goalert.js | Phase2-17.51 | ✅ |
| 57 | google-chat.js | Phase2-17.52 | ✅ |
| 58 | google-sheets.js | Phase2-17.53 | ✅ |
| 59 | gtx-messaging.js | Phase2-17.54 | ✅ |
| 60 | keep.js | Phase2-17.55 | ✅ |
| 61 | notifery.js | Phase2-17.56 | ✅ |
| 62 | onechat.js | Phase2-17.57 | ✅ |
| 63 | onesender.js | Phase2-17.58 | ✅ |
| 64 | pumble.js | Phase2-17.59 | ✅ |
| 65 | resend.js | Phase2-17.60 | ✅ |
| 66 | send-grid.js | Phase2-17.61 | ✅ |
| 67 | serwersms.js | Phase2-17.62 | ✅ |
| 68 | sevenio.js | Phase2-17.63 | ✅ |
| 69 | smspartner.js | Phase2-17.64 | ✅ |
| 70 | splunk.js | Phase2-17.65 | ✅ |
| 71 | spugpush.js | Phase2-17.66 | ✅ |
| 72 | squadcast.js | Phase2-17.67 | ✅ |
| 73 | stackfield.js | Phase2-17.68 | ✅ |
| 74 | techulus-push.js | Phase2-17.69 | ✅ |
| 75 | telnyx.js | Phase2-17.70 | ✅ |
| 76 | teltonika.js | Phase2-17.71 | ✅ |
| 77 | threema.js | Phase2-17.72 | ✅ |
| 78 | vk.js | Phase2-17.73 | ✅ |
| 79 | waha.js | Phase2-17.74 | ✅ |
| 80 | whapi.js | Phase2-17.75 | ✅ |
| 81 | wpush.js | Phase2-17.76 | ✅ |
| 82 | zoho-cliq.js | Phase2-17.77 | ✅ |
| 83 | apprise.js | Phase2-17.78 | ✅ |
| 84 | 360messenger.js | Phase2-17.79 | ✅ |
| 85 | HaloPSA.js | Phase2-17.80 | ✅ |
| 86 | evolution.js | Phase2-17.81 | ✅ |
| 87 | alerta.js | Phase2-17.82 | ✅ |
| 88 | sms-planet.js | Phase2-17.83 | ✅ |
| 89 | smsc.js | Phase2-17.84 | ✅ |
| 90 | smsmanager.js | Phase2-17.85 | ✅ |
| 91 | signl4.js | Phase2-17.86 | ✅ |
| 92 | max.js | Phase2-17.87 | ✅ |
| 93 | nextcloudtalk.js | Phase2-17.88 | ✅ |
| 94 | Webpush.js | Phase2-17.89 | ✅ |

**合計: 94 項全數列入 (93 + notification.js = 94)**

---

## 八、routers/ (2 個)

| 序號 | 檔案 | 已在 Check List |
|------|------|---------------|
| 1 | api-router.js | ✅ Phase3-26 |
| 2 | status-page-router.js | ✅ Phase3-26.1 |

**合計: 2 項全數列入**

---

## 九、socket-handlers/ (10 個)

| 序號 | 檔案 | 已在 Check List |
|------|------|---------------|
| 1 | general-socket-handler.js | ✅ Phase9-41 |
| 2 | api-key-socket-handler.js | ✅ Phase9-41.1 |
| 3 | chart-socket-handler.js | ✅ Phase9-41.2 |
| 4 | cloudflared-socket-handler.js | ✅ Phase9-41.3 |
| 5 | database-socket-handler.js | ✅ Phase9-41.4 |
| 6 | docker-socket-handler.js | ✅ Phase9-41.5 |
| 7 | maintenance-socket-handler.js | ✅ Phase9-41.6 |
| 8 | proxy-socket-handler.js | ✅ Phase9-41.7 |
| 9 | remote-browser-socket-handler.js | ✅ Phase9-41.8 |
| 10 | status-page-socket-handler.js | ✅ Phase9-41.9 |

**合計: 10 項全數列入**

---

## 十、utils/ (外部庫，不需轉換)

| 目錄 | 說明 |
|------|------|
| array-with-key.js | 外部庫 (knex 工具) |
| knex/ | 外部庫 |

---

## 十一、modules/ (外部庫，不需轉換)

| 目錄 | 說明 |
|------|------|
| apicache/ | 外部快取庫 |
| axios-ntlm/ | 外部 NTLM 庫 |
| dayjs/ | 外部日期庫 |

---

## 總統計

| 類別 | 總數 | 已列入 | 漏列 | 不需轉換 |
|------|------|--------|------|--------|
| server/ 根目錄 | 24 | 24 | 0 | 0 |
| analytics/ | 5 | 5 | 0 | 0 |
| jobs/ | 2 | 2 | 0 | 0 |
| model/ | 13 | 13 | 0 | 0 |
| monitor-types/ | 25 | 24 | 0 | 1 |
| monitor-conditions/ | 4 | 4 | 0 | 0 |
| notification-providers/ | 94 | 94 | 0 | 0 |
| routers/ | 2 | 2 | 0 | 0 |
| socket-handlers/ | 10 | 10 | 0 | 0 |
| utils/ | - | 0 | 0 | 是 |
| modules/ | - | 0 | 0 | 是 |
| **合計** | **179** | **174** | **0** | **5** |

---

## 結論

✅ **所有 server 目錄中的程式檔案已完整列入 Check List，無漏列項目。**

需注意的不需轉換項目：
- `ping.js` - 使用外部 @louislam/ping 套件
- `utils/` 目錄 - 外部 knex 工具庫
- `modules/` 目錄 - 外部快取和 NTLM 庫