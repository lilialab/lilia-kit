# Changelog

## [Unreleased]

### Changed

- 产品更名为 LiliaKit（原 LiliaPulse）；包名 / identifier / 文案同步
- 主窗改为工具开关面板；监视器拆为独立浮窗（可由主窗开关）

### Added

- 系统监视器第一版：整机 CPU %、内存已用/总量，约 2 秒刷新
- 网速（合计）、温度传感器
- 修复 Windows 启动 panic（sysinfo 过早初始化 COM 与 Tauri OleInitialize 冲突）；指标改为延迟初始化，并关闭 dragDropEnabled
- 去掉进程表、磁盘面板与分网卡列表（保留合计网速）
- 悬浮窗：置顶、无边框、半透明、跳过任务栏；标题区可拖动（补 `allow-start-dragging`）
- 窗口外观：mac 系统 vibrancy+radius + 淡 tint；Windows 全程无毛玻璃、CSS 约 95% 实底；去掉 CSS 圆角与 backdrop-filter
- 系统托盘：显示/隐藏、退出；主窗 X 进托盘；mac 用 Accessory 去掉 Dock 常驻
- 截图、剪切板历史入口占位（即将推出）
- 接入 Tailwind CSS v4（`@tailwindcss/postcss` + `.postcssrc.json`）
- 主题：Catppuccin Macchiato（`@catppuccin/palette`）+ `--app-*` 语义变量

## [0.1.0] — 2026-11-18

### Notes

- 初始脚手架：Tauri 2 + Angular；开发约定对齐 LiliaMark 工具链习惯