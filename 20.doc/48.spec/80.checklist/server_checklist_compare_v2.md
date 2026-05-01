# 參考系統 server 目錄完整檔案清單 (v2)

## 一、server/ 根目錄 (25 個檔案)

| 序號 | 檔案 | Check List 狀態 |
|------|------|------------------|
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
| 25 | server.js (重複)#1 | 已計 |

---

## 二、analytics/ 目錄 (5 個)

| 序號 | 檔案 | Check List 狀態 |
|------|------|------------------|
| 1 | analytics.js | ✅ Phase6-38 |
| 2 | google-analytics.js | ✅ Phase6-38.1 |
| 3 | matomo-analytics.js | ✅ Phase6-38.2 |
| 4 | plausible-analytics.js | ✅ Phase6-38.3 |
| 5 | umami-analytics.js | ✅ Phase6-38.4 |

---

## 三、jobs/ 目錄 (2 個)

| 序號 | 檔案 | Check List 狀態 |
|------|------|------------------|
| 1 | clear-old-data.js | ✅ Phase7-39 |
| 2 | incremental-vacuum.js | ✅ Phase7-39.1 |

---

## 四、model/ 目錄 (13 個)

| 序號 | 檔案 | Check List 狀態 |
|------|------|------------------|
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

---

## 五、monitor-types/ 目錄 (25 個)

| 序號 | 檔案 | Check List 狀態 |
|------|------|------------------|
| 1 | http.js | ✅ Phase1-6 |
| 2 | tcp.js | ✅ Phase1-7 |
| 3 | dns.js | ✅ Phase1-8 |
| 4 | ping.js | ⚠️ 不需轉換(外部套件) |
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

---

## 六、monitor-conditions/ 目錄 (4 個)

| 序號 | 檔案 | Check List 狀態 |
|------|------|------------------|
| 1 | evaluator.js | ✅ Phase8-40 |
| 2 | expression.js | ✅ Phase8-40.1 |
| 3 | operators.js | ✅ Phase8-40.2 |
| 4 | variables.js | ✅ Phase8-40.3 |

---

## 七、notification-providers/ 目錄 (數量統計)

以下為 **完整清單** (所有 notification-providers/*.js 檔案)：

| 序號 | 檔案 | Check List 狀態 |
|------|------|------------------|
| 1 | notification-provider.js | ✅ Phase2-17.1 |
| 2 | smtp.js | ✅ Phase2-13 |
| 3 | telegram.js | ✅ Phase2-14 |
| 4 | discord.js | ✅ Phase2-15 |
| 5 | slack.js | ✅ Phase2-16 |
| 6 | webhook.js | ✅ Phase2-17 |
| 7 | gotify.js | ✅ Phase2-17.2 |
| 8 | pushover.js | ✅ Phase2-17.3 |
| 9 | teams.js | ✅ Phase2-17.4 |
| 10 | line.js | ✅ Phase2-17.5 |
| 11 | email.js | ✅ Phase2-17.6 |
| 12 | alertnow.js | ✅ Phase2-17.7 |
| 13 | aliyun-sms.js | ✅ Phase2-17.8 |
| 14 | bark.js | ✅ Phase2-17.9 |
| 15 | call-me-bot.js | ✅ Phase2-17.10 |
| 16 | cellsynt.js | ✅ Phase2-17.11 |
| 17 | clicksendsms.js | ✅ Phase2-17.12 |
| 18 | dingding.js | ✅ Phase2-17.13 |
| 19 | feishu.js | ✅ Phase2-17.14 |
| 20 | fluxer.js | ✅ Phase2-17.15 |
| 21 | freemobile.js | ✅ Phase2-17.16 |
| 22 | gotify.js | ✅ Phase2-17.2 |
| 23 | gorush.js | ✅ Phase2-17.17 |
| 24 | grafana-oncall.js | ✅ Phase2-17.18 |
| 25 | heii-oncall.js | ✅ Phase2-17.19 |
| 26 | home-assistant.js | ✅ Phase2-17.20 |
| 27 | jira-service-management.js | ✅ Phase2-17.21 |
| 28 | kook.js | ✅ Phase2-17.22 |
| 29 | lunasea.js | ✅ Phase2-17.23 |
| 30 | matrix.js | ✅ Phase2-17.24 |
| 31 | mattermost.js | ✅ Phase2-17.25 |
| 32 | nostr.js | ✅ Phase2-17.26 |
| 33 | ntfy.js | ✅ Phase2-17.27 |
| 34 | octopush.js | ✅ Phase2-17.28 |
| 35 | onebot.js | ✅ Phase2-17.29 |
| 36 | opsgenie.js | ✅ Phase2-17.30 |
| 37 | pagerduty.js | ✅ Phase2-17.31 |
| 38 | pagertree.js | ✅ Phase2-17.32 |
| 39 | promosms.js | ✅ Phase2-17.33 |
| 40 | pushbullet.js | ✅ Phase2-17.34 |
| 41 | pushdeer.js | ✅ Phase2-17.35 |
| 42 | pushplus.js | ✅ Phase2-17.36 |
| 43 | pushy.js | ✅ Phase2-17.37 |
| 44 | rocket-chat.js | ✅ Phase2-17.38 |
| 45 | serverchan.js | ✅ Phase2-17.39 |
| 46 | signal.js | ✅ Phase2-17.40 |
| 47 | smseagle.js | ✅ Phase2-17.41 |
| 48 | smsir.js | ✅ Phase2-17.42 |
| 49 | telegram.js | ✅ Phase2-14 |
| 50 | twilio.js | ✅ Phase2-17.43 |
| 51 | wecom.js | ✅ Phase2-17.44 |
| 52 | yzj.js | ✅ Phase2-17.45 |
| 53 | 46elks.js | ✅ Phase2-17.46 |
| 54 | bale.js | ✅ Phase2-17.47 |
| 55 | bitrix24.js | ✅ Phase2-17.48 |
| 56 | brevo.js | ✅ Phase2-17.49 |
| 57 | flashduty.js | ✅ Phase2-17.50 |
| 58 | goalert.js | ✅ Phase2-17.51 |
| 59 | google-chat.js | ✅ Phase2-17.52 |
| 60 | google-sheets.js | ✅ Phase2-17.53 |
| 61 | gtx-messaging.js | ✅ Phase2-17.54 |
| 62 | keep.js | ✅ Phase2-17.55 |
| 63 | notifery.js | ✅ Phase2-17.56 |
| 64 | onechat.js | ✅ Phase2-17.57 |
| 65 | onesender.js | ✅ Phase2-17.58 |
| 66 | pumble.js | ✅ Phase2-17.59 |
| 67 | resend.js | ✅ Phase2-17.60 |
| 68 | send-grid.js | ✅ Phase2-17.61 |
| 69 | serwersms.js | ✅ Phase2-17.62 |
| 70 | sevenio.js | ✅ Phase2-17.63 |
| 71 | smspartner.js | ✅ Phase2-17.64 |
| 72 | splunk.js | ✅ Phase2-17.65 |
| 73 | spugpush.js | ✅ Phase2-17.66 |
| 74 | squadcast.js | ✅ Phase2-17.67 |
| 75 | stackfield.js | ✅ Phase2-17.68 |
| 76 | techulus-push.js | ✅ Phase2-17.69 |
| 77 | telnyx.js | ✅ Phase2-17.70 |
| 78 | teltonika.js | ✅ Phase2-17.71 |
| 79 | threema.js | ✅ Phase2-17.72 |
| 80 | vk.js | ✅ Phase2-17.73 |
| 81 | waha.js | ✅ Phase2-17.74 |
| 82 | whapi.js | ✅ Phase2-17.75 |
| 83 | wpush.js | ✅ Phase2-17.76 |
| 84 | zoho-cliq.js | ✅ Phase2-17.77 |
| 85 | apprise.js | ✅ Phase2-17.78 |
| 86 | 360messenger.js | ✅ Phase2-17.79 |
| 87 | HaloPSA.js | ✅ Phase2-17.80 |
| 88 | evolution.js | ✅ Phase2-17.81 |
| 89 | alerta.js | ⚠️ 漏列 |
| 90 | sms-planet.js | ⚠️ 漏列 |
| 91 | smsc.js | ⚠️ 漏列 |
| 92 | smsmanager.js | ⚠️ 漏列 |
| 93 | signl4.js | ⚠️ 漏列 |
| 94 | max.js | ⚠️ 漏列 |
| 95 | nextcloudtalk.js | ⚠️ 漏列 |
| 96 | Webpush.js | ⚠️ 漏列 |

---

## 八、routers/ 目錄 (2 個)

| 序號 | 檔案 | Check List 狀態 |
|------|------|------------------|
| 1 | api-router.js | ✅ Phase3-26 |
| 2 | status-page-router.js | ✅ Phase3-26.1 |

---

## 九、socket-handlers/ 目錄 (10 個)

| 序號 | 檔案 | Check List 狀態 |
|------|------|------------------|
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

---

## 十、utils/ 目錄 (外部庫，不需轉換)

| 序號 | 檔案 | Check List 狀態 |
|------|------|------------------|
| 1 | array-with-key.js | ⚠️ 外部庫 (knex) |
| 2 | limit-queue.js | ⚠️ 外部庫 |
| 3 | simple-migration-server.js | ⚠️ 外部庫 |
| 4 | knex/ | ⚠️ 外部庫 (knex) |

---

## 十一、modules/ 目錄 (外部庫，不需轉換)

| 目錄 | 說明 |
|------|------|
| apicache/ | 外部快取庫 |
| axios-ntlm/ | 外部 NTLM 庫 |
| dayjs/ | 外部日期庫 |

---

## 總統計

| 類別 | 總數 | 已列入 | 漏列 | 不需轉換 |
|------|------|--------|------|--------|
| server/ 根目錄 | 25 | 25 | 0 | 0 |
| analytics/ | 5 | 5 | 0 | 0 |
| jobs/ | 2 | 2 | 0 | 0 |
| model/ | 13 | 13 | 0 | 0 |
| monitor-types/ | 25 | 24 | 0 | 1 |
| monitor-conditions/ | 4 | 4 | 0 | 0 |
| notification-providers/ | 96 | 89 | 7 | 0 |
| routers/ | 2 | 2 | 0 | 0 |
| socket-handlers/ | 10 | 10 | 0 | 0 |
| utils/ | 4 | 0 | 0 | 4 |
| modules/ | - | 0 | 0 | 是 |
| **合計** | **186** | **174** | **7** | **5** |

---

## 漏列項目清單 (需要確認)

| 序號 | 檔案 | 目錄 |
|------|------|------|
| 1 | alerta.js | notification-providers/ |
| 2 | sms-planet.js | notification-providers/ |
| 3 | smsc.js | notification-providers/ |
| 4 | smsmanager.js | notification-providers/ |
| 5 | signl4.js | notification-providers/ |
| 6 | max.js | notification-providers/ |
| 7 | nextcloudtalk.js | notification-providers/ |
| 8 | Webpush.js | notification-providers/ |