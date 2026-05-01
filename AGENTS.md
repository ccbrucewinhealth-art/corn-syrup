# For Human

This project is a Rust re-implementation of Uptime Kuma.

## Allowed Tasks

- Code translation from Node.js/TypeScript sources into Rust implementations
- Backend API compatibility and router translation
- Database model adaptation for SQLite / PostgreSQL
- Monitoring protocol implementation and monitor type porting
- Frontend translation into Rust/Yew render models and components
- Service worker, i18n, and runtime asset integration
- Notification provider wiring and payload construction

## Project Structure

- `src/backend/` — Rust backend modules for config, database, auth, API, socket, jobs, monitor, notification, proxy, metrics, and utilities
- `src/frontend-rust/` — Rust frontend sources for Yew-like pages, components, layouts, router, service worker, and i18n
- `Makefile` — build targets that invoke cargo in the `src` directory

## Translation Guidelines

- Maintain a 1:1 mapping between original source intent and Rust equivalents
- Preserve observable functionality and behavior across backend and frontend
- Use idiomatic Rust where it improves safety and clarity without changing behavior
- Keep architecture consistent with the current `src/backend` and `src/frontend-rust` directory layout
- Document architectural changes and any deviations from the original design
- Prefer existing Rust modules and render-model patterns over introducing unrelated frameworks