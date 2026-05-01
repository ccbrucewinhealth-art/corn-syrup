# corn-syrup

Rust-based self-hosted monitoring tool, translated from Uptime Kuma.

## Features

- Monitoring uptime for HTTP(s) / TCP / HTTP(s) keyword / HTTP(s) JSON query / Websocket / Ping / DNS record / Push / Steam Game Server / Docker containers
- Notifications via Telegram, Discord, Gotify, Slack, Pushover, SMTP, and many more providers
- 20-second monitor intervals
- Multiple public status pages
- Certificate / TLS information checks
- Proxy support and reverse proxy integration
- Built-in i18n support with zh-TW, en-US, ja-JP
- Service worker caching and offline-capable frontend assets

## Architecture

- **Backend**: Rust server code under `src/backend/`
- **Frontend**: Rust/Yew-like frontend code under `src/frontend-rust/`
- **Database**: SQLite by default, with PostgreSQL support in backend database modules
- **Build**: `Makefile` invokes `cargo` from `src/`

## Directory Layout

- `src/backend/` — server entrypoint, config, database, auth, API, socket, monitor, notification, proxy, metrics, jobs, utilities
- `src/frontend-rust/` — application entry, router, pages, components, layouts, service worker, i18n, frontend utilities
- `src/frontend-rust/components/notifications/` — notification provider form and payload modules

## Build and Run

From the repository root:

```bash
make build
make dev
make test
make clean
```

The `Makefile` delegates to the Rust build system in `src/`.

## Notes

This repository is a translated Rust implementation inspired by Uptime Kuma. Documentation and code are aligned to the current Rust source tree rather than the original Vue frontend.

- Upstream project: Uptime Kuma
- Upstream repository: https://github.com/louislam/uptime-kuma
- Upstream license: MIT License

Please refer to the upstream repository for the full original license text and notices.
