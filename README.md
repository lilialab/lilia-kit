# LiliaKit

简体中文 | [English](docs/i18n/README-en.md)

跨平台桌面工具箱，目前以系统监视为主。托盘常驻；基于 Tauri 2，使用系统 WebView，不打包 Chromium。

<!-- 有截图后放到这里，例如：![LiliaKit](docs/screenshot.png) -->

## 功能

- 系统监视浮窗：CPU、内存、合计网速、最高温度；约 2 秒刷新；紧凑长条、置顶、半透明、跳过任务栏、可拖动
- 指标按负荷上色；网速较快时用提示色
- 全局快捷键切换监视器显隐（默认 `Ctrl+Alt+M`，Mac 为 `⌃⌥M`）；设置里可选预设或关闭
- 系统托盘：显示 / 隐藏主窗、退出；关闭主窗进托盘；macOS 不常驻 Dock
- 可选开机启动：登录后静默进托盘
- 外观：跟随系统 / Latte / Mocha（Catppuccin）
- 界面语言：跟随系统 / 中文 / English（非中文系统回落英文）

当前不做：截图等其它工具、自定义录制热键、账号与同步。

## 安装

从仓库 **Releases** 页下载对应平台安装包：

| 系统 | 推荐文件 |
|------|----------|
| Windows | `*_x64-setup.exe` 或 `*_x64_en-US.msi` |
| macOS（Apple Silicon） | `*_aarch64.dmg` |
| macOS（Intel） | `*_x64.dmg` |
| Linux（Debian / Ubuntu） | `*_amd64.deb` 或 AppImage |
| Linux（Fedora 等） | `*.rpm` |

构建默认**未代码签名**：

- Windows 可能出现 SmartScreen
- macOS 可能被隔离无法打开。可先右键 → 打开；若仍不行，在终端对 `.app` 执行：

```bash
xattr -d com.apple.quarantine /path/to/LiliaKit.app
```

## 使用

- 托盘菜单或左键（Windows / Linux）显隐设置窗；关闭设置窗进入托盘，不退出
- `Ctrl+Alt+M`（Mac：`⌃⌥M`）切换监视器显隐（可在设置中修改或关闭）
- 设置：通用（开机启动、语言、外观）与监视器（启用、置顶、温度、热键）；改完底部统一保存
- 变更记录见 [`CHANGELOG.md`](CHANGELOG.md)

## 开发

需要：Node.js、[pnpm](https://pnpm.io)、Rust 工具链，以及本机系统 WebView（Windows WebView2、macOS WKWebView、Linux WebKitGTK）。

```bash
pnpm install
pnpm tauri dev    # 开发（与安装版隔离为 LiliaKit Dev）
pnpm tauri build  # 打安装包
```

其它：

- `pnpm format` — dprint
- `pnpm lint` — ESLint
- `pnpm test` — 纯函数单测（tsx）

推送 `v*` tag 后由 [`.github/workflows/release.yml`](.github/workflows/release.yml) 打多平台 Draft Release。

技术栈：Tauri 2 · Angular · TypeScript · Tailwind · Transloco · Catppuccin。仓库约定与分层见 [`AGENTS.md`](AGENTS.md)。

## 许可

[MIT](LICENSE) © Liu Yuchuan
