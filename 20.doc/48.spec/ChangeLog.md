# ChangeLog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [Unreleased]

### Added

- 專案初始化：corn-syrup Rust 監控系統
- 需求文件 (Requirements.md)：分為 5 階段的需求規範
- 規格文件 (Spec.md)：對應需求的規格定義
- 工作計劃 (Plan.md)：分階段的工作計畫
- 工作項目 (Task.md)：具體工作任務清單
- 轉換檢查表 (80.translate_check-list.md)：43 項轉換檢查項目
- Framework 選擇文件 (50.Framework.md)：Rust 框架對應選擇

### Verified & Fixed

- 檢查表完整性驗證：比對 uptime-kuma 原始碼確認項目齊全
- 修復重複 Phase 編號：Phase 11/12/13 整併為單一 Phase 11
- 新增遺漏項目：
  - pages: Dashboard.vue, DashboardHome.vue, Setup.vue 等 16 項
  - settings: APIKeys.vue, MonitorHistory.vue 等 12 項
  - mixins: socket.js, theme.js, lang.js 等 6 項
  - layouts: Layout.vue, EmptyLayout.vue

### Added - 目錄結構

- 90.directory_structure.md：目標目錄結構文件
  - backend (backend/): ~213 檔案
  - frontend-rust (frontend-rust/): ~195 檔案
  - 總計: ~408 檔案

### Updated

- 檢查表目標目錄：src → backend, src/web → frontend-rust
corn-syrup
### Addcorn-syrupripts

- util_compile.sh：標準編譯腳本
- util_corn-dbio-kit-loop-exec.sh：無限迴圈執行腳本
- util_corn-dbio-kit.sh：單次執行腳本
- util_all-in-one-compile.sh：All-in-One 編譯腳本
- download_all_package-cron.sh：離線套件下載腳本
- Makefile：建置目標管理

### Added - Documentation

- 需求文件 (01.Requirements/Requirements.md)
- 規格文件 (02.Spec/Spec.md)
- 工作計劃 (03.Plan/Plan.md)
- 工作項目 (05.Task/Task.md)
- 轉換檢查表 (80.checklist/80.translate_check-list.md)
- Framework 文件 (50.Framework.md)
- Resume 文件 (15.resumes/Resume{yyyy}{MM}{dd}-{hh}{mm}{ss}.md)

---

## [0.0.1] - 2026-05-01

### Added

- 初始專案建立
- 參考系統分析：uptime-kuma (Node.js/TypeScript)
- Framework 分析與選擇：Rust 生態對應套件
- 前置規劃文件全部建立完成