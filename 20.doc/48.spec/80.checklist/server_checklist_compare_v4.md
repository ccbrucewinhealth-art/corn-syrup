# 參考系統 server 目錄完整檔案清單 (v4) - 1對1檢查

## server/ 根目錄 (26 個檔案)

| 序號 | 檔案路徑 | Check List 編號 | 狀態 |
|------|----------|---------------|------|
| 1 | server/2fa.js | Phase1-24 | ✅ |
| 2 | server/auth.js | Phase1-23 | ✅ |
| 3 | server/check-version.js | Phase1-10 | ✅ |
| 4 | server/client.js | Phase1-8 | ✅ |
| 5 | server/config.js | Phase1-5 | ✅ |
| 6 | server/database.js | Phase1-2 | ✅ |
| 7 | server/docker.js | Phase1-17 | ✅ |
| 8 | server/embedded-mariadb.js | Phase1-18 | ✅ |
| 9 | server/image-data-uri.js | Phase1-19 | ✅ |
| 10 | server/jobs.js | Phase1-21 | ✅ |
| 11 | server/notification.js | Phase1-13 | ✅ |
| 12 | server/password-hash.js | Phase1-11 | ✅ |
| 13 | server/prometheus.js | Phase1-14 | ✅ |
| 14 | server/proxy.js | Phase1-15 | ✅ |
| 15 | server/radius-client.js | Phase1-20 | ✅ |
| 16 | server/rate-limiter.js | Phase1-9 | ✅ |
| 17 | server/remote-browser.js | Phase1-16 | ✅ |
| 18 | server/server.js | Phase1-1 | ✅ |
| 19 | server/settings.js | Phase1-5 | ✅ |
| 20 | server/setup-database.js | Phase1-22 | ✅ |
| 21 | server/translatable-error.js | Phase1-12 | ✅ |
| 22 | server/uptime-calculator.js | Phase1-7 | ✅ |
| 23 | server/uptime-kuma-server.js | Phase1-6 | ✅ |
| 24 | server/util-server.js | Phase1-3 | ✅ |
| 25 | server/jobs/clear-old-data.js | Phase7-120 | ✅ |
| 26 | server/jobs/incremental-vacuum.js | Phase7-121 | ✅ |

**合計: 26 項全數列入** ✅

---

## analytics/ 目錄 (5 個)

| 序號 | 檔案路徑 | Check List 編號 | 狀態 |
|------|----------|---------------|------|
| 1 | server/analytics/analytics.js | Phase8-130 | ✅ |
| 2 | server/analytics/google-analytics.js | Phase8-131 | ✅ |
| 3 | server/analytics/matomo-analytics.js | Phase8-132 | ✅ |
| 4 | server/analytics/plausible-analytics.js | Phase8-133 | ✅ |
| 5 | server/analytics/umami-analytics.js | Phase8-134 | ✅ |

**合計: 5 項全數列入** ✅

---

## model/ 目錄 (13 個)

| 序號 | 檔案路徑 | Check List 編號 | 狀態 |
|------|----------|---------------|------|
| 1 | server/model/api_key.js | Phase3-68 | ✅ |
| 2 | server/model/docker_host.js | Phase3-71 | ✅ |
| 3 | server/model/domain_expiry.js | Phase3-67 | ✅ |
| 4 | server/model/group.js | Phase3-63 | ✅ |
| 5 | server/model/heartbeat.js | Phase3-61 | ✅ |
| 6 | server/model/incident.js | Phase3-72 | ✅ |
| 7 | server/model/maintenance.js | Phase3-66 | ✅ |
| 8 | server/model/monitor.js | Phase3-60 | ✅ |
| 9 | server/model/proxy.js | Phase3-69 | ✅ |
| 10 | server/model/remote_browser.js | Phase3-70 | ✅ |
| 11 | server/model/status_page.js | Phase3-65 | ✅ |
| 12 | server/model/tag.js | Phase3-64 | ✅ |
| 13 | server/model/user.js | Phase3-62 | ✅ |

**合計: 13 項全數列入** ✅

---

## monitor-conditions/ 目錄 (4 個)

| 序號 | 檔案路徑 | Check List 編號 | 狀態 |
|------|----------|---------------|------|
| 1 | server/monitor-conditions/evaluator.js | Phase6-110 | ✅ |
| 2 | server/monitor-conditions/expression.js | Phase6-111 | ✅ |
| 3 | server/monitor-conditions/operators.js | Phase6-112 | ✅ |
| 4 | server/monitor-conditions/variables.js | Phase6-113 | ✅ |

**合計: 4 項全數列入** ✅

---

## monitor-types/ 目錄 (25 個)

| 序號 | 檔案路徑 | Check List 編號 | 狀態 |
|------|----------|---------------|------|
| 1 | server/monitor-types/dns.js | Phase2-32 | ✅ |
| 2 | server/monitor-types/gamedig.js | Phase2-38 | ✅ |
| 3 | server/monitor-types/globalping.js | Phase2-39 | ✅ |
| 4 | server/monitor-types/group.js | Phase2-40 | ✅ |
| 5 | server/monitor-types/grpc.js | Phase2-48 | ✅ |
| 6 | server/monitor-types/manual.js | Phase2-34 | ✅ |
| 7 | server/monitor-types/mongodb.js | Phase2-54 | ✅ |
| 8 | server/monitor-types/monitor-type.js | Phase2-41 | ✅ |
| 9 | server/monitor-types/mqtt.js | Phase2-42 | ✅ |
| 10 | server/monitor-types/mssql.js | Phase2-43 | ✅ |
| 11 | server/monitor-types/mysql.js | Phase2-44 | ✅ |
| 12 | server/monitor-types/oracledb.js | Phase2-37 | ✅ |
| 13 | server/monitor-types/postgres.js | Phase2-45 | ✅ |
| 14 | server/monitor-types/rabbitmq.js | Phase2-46 | ✅ |
| 15 | server/monitor-types/real-browser-monitor-type.js | Phase2-52 | ✅ |
| 16 | server/monitor-types/redis.js | Phase2-47 | ✅ |
| 17 | server/monitor-types/sip-options.js | Phase2-53 | ✅ |
| 18 | server/monitor-types/smtp.js | Phase2-36 | ✅ |
| 19 | server/monitor-types/snmp.js | Phase2-49 | ✅ |
| 20 | server/monitor-types/system-service.js | Phase2-50 | ✅ |
| 21 | server/monitor-types/tailscale-ping.js | Phase2-51 | ✅ |
| 22 | server/monitor-types/tcp.js | Phase2-31 | ✅ |
| 23 | server/monitor-types/websocket-upgrade.js | Phase2-35 | ✅ |
| 24 | server/monitor-types/ping.js | Phase2-33 | ⚠️ 不需轉換 |
| 25 | server/monitor-types/http.js | - | ⚠️ **漏列** |

---

## notification-providers/ 目錄 (94 個)

| 序號 | 檔案路徑 | Check List 編號 | 狀態 |
|------|----------|---------------|------|
| 1 | server/notification-providers/360messenger.js | Phase9-223 | ✅ |
| 2 | server/notification-providers/46elks.js | Phase9-190 | ✅ |
| 3 | server/notification-providers/HaloPSA.js | Phase9-224 | ✅ |
| 4 | server/notification-providers/Webpush.js | Phase9-233 | ✅ |
| 5 | server/notification-providers/alerta.js | Phase9-226 | ✅ |
| 6 | server/notification-providers/alertnow.js | Phase9-151 | ✅ |
| 7 | server/notification-providers/aliyun-sms.js | Phase9-152 | ✅ |
| 8 | server/notification-providers/apprise.js | Phase9-222 | ✅ |
| 9 | server/notification-providers/bale.js | Phase9-191 | ✅ |
| 10 | server/notification-providers/bark.js | Phase9-153 | ✅ |
| 11 | server/notification-providers/bitrix24.js | Phase9-192 | ✅ |
| 12 | server/notification-providers/brevo.js | Phase9-193 | ✅ |
| 13 | server/notification-providers/call-me-bot.js | Phase9-154 | ✅ |
| 14 | server/notification-providers/cellsynt.js | Phase9-155 | ✅ |
| 15 | server/notification-providers/clicksendsms.js | Phase9-156 | ✅ |
| 16 | server/notification-providers/dingding.js | Phase9-157 | ✅ |
| 17 | server/notification-providers/discord.js | Phase9-143 | ✅ |
| 18 | server/notification-providers/evolution.js | Phase9-225 | ✅ |
| 19 | server/notification-providers/feishu.js | Phase9-158 | ✅ |
| 20 | server/notification-providers/flashduty.js | Phase9-194 | ✅ |
| 21 | server/notification-providers/fluxer.js | Phase9-159 | ✅ |
| 22 | server/notification-providers/freemobile.js | Phase9-160 | ✅ |
| 23 | server/notification-providers/goalert.js | Phase9-195 | ✅ |
| 24 | server/notification-providers/google-chat.js | Phase9-196 | ✅ |
| 25 | server/notification-providers/google-sheets.js | Phase9-197 | ✅ |
| 26 | server/notification-providers/gorush.js | Phase9-161 | ✅ |
| 27 | server/notification-providers/gotify.js | Phase9-146 | ✅ |
| 28 | server/notification-providers/grafana-oncall.js | Phase9-162 | ✅ |
| 29 | server/notification-providers/gtx-messaging.js | Phase9-198 | ✅ |
| 30 | server/notification-providers/heii-oncall.js | Phase9-163 | ✅ |
| 31 | server/notification-providers/home-assistant.js | Phase9-164 | ✅ |
| 32 | server/notification-providers/jira-service-management.js | Phase9-165 | ✅ |
| 33 | server/notification-providers/keep.js | Phase9-199 | ✅ |
| 34 | server/notification-providers/kook.js | Phase9-166 | ✅ |
| 35 | server/notification-providers/line.js | Phase9-149 | ✅ |
| 36 | server/notification-providers/lunasea.js | Phase9-167 | ✅ |
| 37 | server/notification-providers/matrix.js | Phase9-168 | ✅ |
| 38 | server/notification-providers/mattermost.js | Phase9-169 | ✅ |
| 39 | server/notification-providers/max.js | Phase9-231 | ✅ |
| 40 | server/notification-providers/nextcloudtalk.js | Phase9-232 | ✅ |
| 41 | server/notification-providers/nostr.js | Phase9-170 | ✅ |
| 42 | server/notification-providers/notifery.js | Phase9-200 | ✅ |
| 43 | server/notification-providers/notification-provider.js | Phase9-140 | ✅ |
| 44 | server/notification-providers/ntfy.js | Phase9-171 | ✅ |
| 45 | server/notification-providers/octopush.js | Phase9-172 | ✅ |
| 46 | server/notification-providers/onebot.js | Phase9-173 | ✅ |
| 47 | server/notification-providers/onechat.js | Phase9-201 | ✅ |
| 48 | server/notification-providers/onesender.js | Phase9-202 | ✅ |
| 49 | server/notification-providers/opsgenie.js | Phase9-174 | ✅ |
| 50 | server/notification-providers/pagerduty.js | Phase9-175 | ✅ |
| 51 | server/notification-providers/pagertree.js | Phase9-176 | ✅ |
| 52 | server/notification-providers/promosms.js | Phase9-177 | ✅ |
| 53 | server/notification-providers/pumble.js | Phase9-203 | ✅ |
| 54 | server/notification-providers/pushbullet.js | Phase9-178 | ✅ |
| 55 | server/notification-providers/pushdeer.js | Phase9-179 | ✅ |
| 56 | server/notification-providers/pushover.js | Phase9-147 | ✅ |
| 57 | server/notification-providers/pushplus.js | Phase9-180 | ✅ |
| 58 | server/notification-providers/pushy.js | Phase9-181 | ✅ |
| 59 | server/notification-providers/resend.js | Phase9-204 | ✅ |
| 60 | server/notification-providers/rocket-chat.js | Phase9-182 | ✅ |
| 61 | server/notification-providers/send-grid.js | Phase9-205 | ✅ |
| 62 | server/notification-providers/serverchan.js | Phase9-183 | ✅ |
| 63 | server/notification-providers/serwersms.js | Phase9-206 | ✅ |
| 64 | server/notification-providers/sevenio.js | Phase9-207 | ✅ |
| 65 | server/notification-providers/signal.js | Phase9-184 | ✅ |
| 66 | server/notification-providers/signl4.js | Phase9-230 | ✅ |
| 67 | server/notification-providers/slack.js | Phase9-144 | ✅ |
| 68 | server/notification-providers/sms-planet.js | Phase9-227 | ✅ |
| 69 | server/notification-providers/smsc.js | Phase9-228 | ✅ |
| 70 | server/notification-providers/smseagle.js | Phase9-185 | ✅ |
| 71 | server/notification-providers/smsir.js | Phase9-186 | ✅ |
| 72 | server/notification-providers/smsmanager.js | Phase9-229 | ✅ |
| 73 | server/notification-providers/smspartner.js | Phase9-208 | ✅ |
| 74 | server/notification-providers/smtp.js | Phase9-141 | ✅ |
| 75 | server/notification-providers/splunk.js | Phase9-209 | ✅ |
| 76 | server/notification-providers/spugpush.js | Phase9-210 | ✅ |
| 77 | server/notification-providers/squadcast.js | Phase9-211 | ✅ |
| 78 | server/notification-providers/stackfield.js | Phase9-212 | ✅ |
| 79 | server/notification-providers/teams.js | Phase9-148 | ✅ |
| 80 | server/notification-providers/techulus-push.js | Phase9-213 | ✅ |
| 81 | server/notification-providers/telegram.js | Phase9-142 | ✅ |
| 82 | server/notification-providers/telnyx.js | Phase9-214 | ✅ |
| 83 | server/notification-providers/teltonika.js | Phase9-215 | ✅ |
| 84 | server/notification-providers/threema.js | Phase9-216 | ✅ |
| 85 | server/notification-providers/twilio.js | Phase9-187 | ✅ |
| 86 | server/notification-providers/vk.js | Phase9-217 | ✅ |
| 87 | server/notification-providers/waha.js | Phase9-218 | ✅ |
| 88 | server/notification-providers/webhook.js | Phase9-145 | ✅ |
| 89 | server/notification-providers/wecom.js | Phase9-188 | ✅ |
| 90 | server/notification-providers/whapi.js | Phase9-219 | ✅ |
| 91 | server/notification-providers/wpush.js | Phase9-220 | ✅ |
| 92 | server/notification-providers/yzj.js | Phase9-189 | ✅ |
| 93 | server/notification-providers/zoho-cliq.js | Phase9-221 | ✅ |

**合計: 93 項全數列入** ✅

---

## routers/ 目錄 (2 個)

| 序號 | 檔案路徑 | Check List 編號 | 狀態 |
|------|----------|---------------|------|
| 1 | server/routers/api-router.js | Phase4-80 | ✅ |
| 2 | server/routers/status-page-router.js | Phase4-81 | ✅ |

**合計: 2 項全數列入** ✅

---

## socket-handlers/ 目錄 (10 個)

| 序號 | 檔案路徑 | Check List 編號 | 狀態 |
|------|----------|---------------|------|
| 1 | server/socket-handlers/api-key-socket-handler.js | Phase5-91 | ✅ |
| 2 | server/socket-handlers/chart-socket-handler.js | Phase5-92 | ✅ |
| 3 | server/socket-handlers/cloudflared-socket-handler.js | Phase5-93 | ✅ |
| 4 | server/socket-handlers/database-socket-handler.js | Phase5-94 | ✅ |
| 5 | server/socket-handlers/docker-socket-handler.js | Phase5-95 | ✅ |
| 6 | server/socket-handlers/general-socket-handler.js | Phase5-90 | ✅ |
| 7 | server/socket-handlers/maintenance-socket-handler.js | Phase5-96 | ✅ |
| 8 | server/socket-handlers/proxy-socket-handler.js | Phase5-97 | ✅ |
| 9 | server/socket-handlers/remote-browser-socket-handler.js | Phase5-98 | ✅ |
| 10 | server/socket-handlers/status-page-socket-handler.js | Phase5-99 | ✅ |

**合計: 10 項全數列入** ✅

---

## utils/ 目錄 (不需轉換)

| 序號 | 檔案路徑 | Check List 編號 | 狀態 |
|------|----------|---------------|------|
| 1 | server/utils/array-with-key.js | Phase1-25 | ⚠️ 不需轉換 |
| 2 | server/utils/limit-queue.js | Phase1-26 | ⚠️ 不需轉換 |
| 3 | server/utils/simple-migration-server.js | Phase1-27 | ⚠️ 不需轉換 |
| 4 | server/utils/knex/ | Phase1-28 | ⚠️ 不需轉換 |

---

## modules/ 目錄 (不需轉換)

| 序號 | 檔案路徑 | Check List 編號 | 狀態 |
|------|----------|---------------|------|
| 1 | server/modules/apicache/ | Phase10-240 | ⚠️ 不需轉換 |
| 2 | server/modules/axios-ntlm/ | Phase10-241 | ⚠️ 不需轉換 |
| 3 | server/modules/dayjs/ | Phase10-242 | ⚠️ 不需轉換 |

---

## 總統計

| 類別 | 總數 | 已列入 | 漏列 | 不需轉換 |
|------|------|--------|------|--------|
| server/ 根目錄 | 26 | 26 | 0 | 0 |
| analytics/ | 5 | 5 | 0 | 0 |
| model/ | 13 | 13 | 0 | 0 |
| monitor-conditions/ | 4 | 4 | 0 | 0 |
| monitor-types/ | 25 | 24 | **1** | 1 |
| notification-providers/ | 93 | 93 | 0 | 0 |
| routers/ | 2 | 2 | 0 | 0 |
| socket-handlers/ | 10 | 10 | 0 | 0 |
| utils/ | 4 | 0 | 0 | 4 |
| modules/ | 3 | 0 | 0 | 3 |
| **合計** | **185** | **177** | **1** | **8** |

---

## 漏列項目

| 檔案 | 目錄 | 說明 |
|------|------|------|
| http.js | server/monitor-types/ | **HTTP 監控** - 主要監控類型，漏列 |

此項目在上一版曾列入，這次重新整理時遺漏了。

---

## 確認項目

請問是否要將 `server/monitor-types/http.js` 加入 check list？
1. **是** - 將 http.js 加入
2. **否** - 維持現狀