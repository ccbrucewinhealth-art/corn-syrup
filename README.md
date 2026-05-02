# corn-syrup

`corn-syrup` is a Rust backend and React/TypeScript frontend re-implementation of Uptime Kuma. The goal is to preserve the upstream monitoring behavior, API/socket compatibility, status page flow, and user-facing experience while translating the stack into the current Rust and React architecture.

## Features

- Uptime monitoring behavior translated from Uptime Kuma, including HTTP(s), TCP, keyword, JSON query, WebSocket, Ping, DNS record, Push, game server, Docker, and related monitor types as implementation progresses.
- Uptime Kuma-compatible backend concepts for API routes, Socket.IO-style events, authentication, settings, status pages, monitor history, jobs, metrics, proxy handling, and notification payloads.
- Rust backend modules for configuration, logging, auth, database/model adaptation, monitors, notifications, Docker integration, remote browser support, utilities, and error handling.
- React/TypeScript frontend translated from the original Vue UI, using React Router, i18next/react-i18next, Socket.IO client, Axios, Bootstrap-compatible styling, Font Awesome, Chart.js, and Vite.
- Frontend i18n resources for `zh-TW`, `en`, and `ja`, with `zh-TW` as the default locale.
- Service worker and static asset resources retained under both root-level public assets and the frontend workspace.

## Architecture

- **Backend**: Rust crate `corn-syrup-backend` under `src/backend/`, with binary `corn-syrup` and modules for server core, config, settings, logging, auth, API, sockets, database/model adaptation, monitors, notifications, jobs, metrics, proxy, Docker, remote browser support, utilities, and errors.
- **Frontend**: React/TypeScript workspace `corn-syrup-frontend` under `src/frontend/`, built with Vite and organized around `App.tsx`, `main.tsx`, router, layouts, pages, components, mixins, shared libraries, assets, styles, and i18n resources.
- **Documentation**: Translation requirements, specs, plans, tasks, and checklists live under `20.doc/48.spec/backend/` and `20.doc/48.spec/frontend/`.
- **Tests and references**: Upstream-style compatibility tests and manual/e2e assets live under `test/` for behavior reference.
- **Offline tooling**: Package download helpers live under `package/modules/`.

## Directory Layout

- `src/backend/` — Rust backend package, binary, server core, config, settings, logging, auth, API, socket handlers, database/model adaptation, monitor, notification, jobs, metrics, proxy, Docker, remote, utilities, and error modules.
- `src/frontend/` — React/TypeScript frontend package, Vite config, Makefile, build scripts, public assets, and application source.
- `src/frontend/src/` — Frontend app entry points, router, styles, assets, components, layouts, pages, mixins, shared libraries, and i18n setup.
- `src/frontend/src/i18n/locales/` — Locale JSON files for `zh-TW`, `en`, and `ja`.
- `public/` — Root static icons, manifest, and service worker assets retained for compatibility.
- `20.doc/48.spec/backend/` — Backend requirements, specifications, plans, task lists, and translation checklists.
- `20.doc/48.spec/frontend/` — Frontend requirements, specifications, plans, task lists, and translation checklists.
- `test/` — Backend, e2e, and manual test assets retained from or modeled after upstream behavior.

## Build and Run

The repository root currently does not provide the primary build `Makefile`. Run backend and frontend commands from their respective workspaces.

### Backend

```bash
cd src/backend
make check
make build
make dev
make test
make clean
```

Cargo can also be used directly from `src/backend/` when needed.

### Frontend

```bash
cd src/frontend
make install
make build
make dev
make loop
make all-in-one
make clean
```

The frontend npm scripts include `dev`, `build`, `preview`, and `lint`.

## Development Notes

- Keep backend behavior compatible with Uptime Kuma APIs, socket event semantics, monitor behavior, notification payloads, settings, and status page expectations where practical.
- Keep frontend UI flow, route structure, translations, reusable components, and runtime integrations close to the original Vue implementation while using idiomatic React/TypeScript.
- Keep `zh-TW`, `en`, and `ja` i18n resources synchronized; `zh-TW` is the default locale.
- Do not reintroduce obsolete `src/frontend-rust/` assumptions; the active frontend workspace is `src/frontend/`.
- Document compatibility compromises and architectural deviations in the relevant files under `20.doc/48.spec/`.

## Upstream

This repository is a translated implementation inspired by Uptime Kuma.

- Upstream project: Uptime Kuma
- Upstream repository: https://github.com/louislam/uptime-kuma
- Upstream license: MIT License

## Reference Templates

- Kero: https://github.com/iuap-design/kero

Please refer to the upstream repository for the full original license text and notices.
