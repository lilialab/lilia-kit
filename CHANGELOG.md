# Changelog

## [Unreleased]

### Changed

- 产品更名为 LiliaKit（原 LiliaPulse）；包名 / identifier / 文案同步
- 主窗改为左右分栏：左选分组（通用 / 监视器）；右为配置草稿，底部统一保存整份 settings；外观保存后写入主题名 `data-theme`
- 监视器拆为独立浮窗（设置里启用；快捷键仅显隐）
- 设置落地统一由 Rust `apply_app_settings`（写盘 → `apply_runtime` 分发 tools/appearance → 回推）；启动同走 `apply_runtime`；`tools::apply_from_settings` 汇总各工具；前端只编辑会话与 save
- 去掉工具 `available` 占位与 `tools-changed`；侧栏只列已实现工具
- Rust 按域分：`settings/`（persist/apply/appearance）、`tools/`、`metrics/`、`shell/`；`lib` 只装配；前端 `lib/tauri/settings`
- 监视浮窗改为紧凑长条（CPU/内存进度条 + 网速/最高温并排，Lucide 图标）；固定约 220×78，不可拉伸/最大化
- 指标负荷四档色（idle/ok/busy/critical → green/teal/peach/red）；利用率阈值 50/70/90，温度 60/70/90
- 网速 ≥1M/s 用 sapphire 提示色（上下行同规则；B/K 仍 muted，非告警）
- 全局快捷键 Win+Alt+.（Mac：⌘+⌥+.）：已启用时切换监视器显隐；注册失败不阻断启动；后续截图快捷键同此模块
- 监视器显隐：热键只发 `monitor-toggle`；前端淡入淡出后再 `set_monitor_window_visible`（对齐系统 hide/show 不对称）
- 监视器快切开/关：串行销毁/重建，避免 `failed to acquire webview reference`
- 应用设置骨架（`settings.json` 按分组深合并）：`general.appearance`（system/latte/mocha）+ `monitor.*`；窗位置/尺寸由 `tauri-plugin-window-state` 记忆

### Added

- 系统监视器第一版：整机 CPU %、内存已用/总量，约 2 秒刷新
- 网速（合计）、温度传感器
- 修复 Windows 启动 panic（sysinfo 过早初始化 COM 与 Tauri OleInitialize 冲突）；指标改为延迟初始化，并关闭 dragDropEnabled
- 去掉进程表、磁盘面板与分网卡列表（保留合计网速）
- 悬浮窗：置顶、无边框、半透明、跳过任务栏；标题区可拖动（补 `allow-start-dragging`）
- 窗口外观：mac 系统 vibrancy+radius + 淡 tint；Windows 全程无毛玻璃、CSS 约 95% 实底；去掉 CSS 圆角与 backdrop-filter
- 系统托盘：显示/隐藏、退出；主窗 X 进托盘；mac 用 Accessory 去掉 Dock 常驻
- 接入 Tailwind CSS v4（`@tailwindcss/postcss` + `.postcssrc.json`）
- 主题：Catppuccin via `data-theme`（`latte` / `mocha`）+ `--ctp-*` → `--app-*` 语义变量
- Rust 解析 `general.appearance` → 主题名并 `app-theme-changed`；`system` 时跟主窗 `ThemeChanged`，固定主题忽略 OS
- 前端各窗启动 `get_resolved_app_theme` + 订阅 `app-theme-changed` 写 `data-theme`；设置 store 不再本地解析换肤
- 原生窗框跟色：`appearance` 为 `system` 时 `set_theme(None)`，固定 latte/mocha 时 Light/Dark；与 WebView 主题一并 apply

## [0.1.0] — 2026-11-18

### Notes

- 初始脚手架：Tauri 2 + Angular