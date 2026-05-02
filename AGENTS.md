# Agent Guidelines

This project is a Rust backend and React/TypeScript frontend re-implementation of Uptime Kuma named `corn-syrup`.

## Current Architecture

- `src/backend/` — Rust backend crate `corn-syrup-backend`, binary `corn-syrup`, and modules for server core, config, settings, logging, auth, API, socket handlers, database/model adaptation, monitors, notifications, jobs, metrics, proxy, Docker integration, remote browser support, utilities, and error handling.
- `src/frontend/` — React/TypeScript frontend workspace `corn-syrup-frontend`, translated from Uptime Kuma Vue sources. It uses Vite, React Router, i18next/react-i18next, Socket.IO client, Axios, Bootstrap-compatible styling, Font Awesome, Chart.js, and React bindings.
- `src/frontend/src/` — Frontend application code: `App.tsx`, `main.tsx`, router, styles, assets, components, layouts, pages, mixins, shared libraries, and i18n resources.
- `src/frontend/src/i18n/locales/` — Frontend locale files. Required locales are `zh-TW`, `en`, and `ja`; `zh-TW` is the default locale.
- `public/` and `src/frontend/public/` — Static assets, icons, manifest, and service worker resources.
- `20.doc/48.spec/backend/` — Backend requirements, specs, plans, tasks, and translation checklists.
- `20.doc/48.spec/frontend/` — Frontend requirements, specs, plans, tasks, and translation checklists.
- `package/modules/` — Offline package download tooling.
- `test/` — Compatibility tests and upstream-style backend/e2e/manual test assets retained for translation and behavior reference.

## Allowed Tasks

- Translate Node.js/TypeScript/Vue Uptime Kuma behavior into Rust backend modules and React/TypeScript frontend modules.
- Preserve backend API, socket, router, monitor, notification, database, and status page compatibility where practical.
- Adapt database models and persistence behavior for SQLite and PostgreSQL-oriented Rust modules.
- Port monitoring protocols, monitor types, jobs, metrics, notification payload construction, proxy handling, Docker integration, and runtime utilities.
- Translate frontend pages, layouts, components, mixins, i18n, assets, service worker behavior, and runtime integrations into the current React workspace.
- Maintain and update documentation, specs, plans, task lists, and translation checklists to match the current implementation.

## Build and Tooling

- Backend commands are run from `src/backend/` using its local `Makefile` or Cargo directly:
  - `make check`
  - `make build`
  - `make dev`
  - `make test`
  - `make clean`
- Frontend commands are run from `src/frontend/` using its local `Makefile` or npm scripts:
  - `make install`
  - `make build`
  - `make dev`
  - `make loop`
  - `make all-in-one`
  - `make clean`
- The repository root currently does not provide the primary build `Makefile`; use the backend and frontend workspace Makefiles directly.

## Translation Guidelines

- Maintain a close 1:1 mapping between original Uptime Kuma intent and the Rust/React equivalents.
- Preserve observable functionality, public API behavior, socket event semantics, user-facing strings, and UI flow unless a documented architectural deviation is required.
- Prefer idiomatic Rust for backend safety and clarity without changing behavior.
- Prefer idiomatic React/TypeScript patterns while keeping source intent and layout close to the original Vue implementation.
- Keep architecture consistent with the current `src/backend/` and `src/frontend/` directory layout; do not reintroduce obsolete `src/frontend-rust/` assumptions.
- Keep frontend i18n synchronized across `zh-TW`, `en`, and `ja`; use `zh-TW` as the default locale.
- Reuse existing modules, render/component patterns, utility layers, and translation documents before adding unrelated frameworks or new architectural layers.
- Document architectural changes, compatibility compromises, and deviations from the upstream design in the relevant spec or task documentation.
