# LiliaKit

[简体中文](../../README.md) | English

Cross-platform desktop toolkit, starting with a system monitor. Lives in the tray. Built with Tauri 2 and the system WebView — no bundled Chromium.

<!-- Add a screenshot later, e.g. ![LiliaKit](../screenshot.png) -->

## Features

- System monitor overlay: CPU, memory, aggregate network speed, peak temperature; ~2s refresh; compact bar, always-on-top, translucent, skip taskbar, draggable
- Load-based coloring; faster network speeds get an accent hint
- Global hotkey to show / hide the monitor (default `Ctrl+Alt+M`, Mac `⌃⌥M`); presets or off in Settings
- System tray: show / hide main window, quit; closing the main window hides to tray; no Dock icon on macOS
- Optional launch at login: start silently in the tray
- Appearance: system / Latte / Mocha (Catppuccin)
- UI language: system / Chinese / English (non-Chinese OS falls back to English)

Out of scope for now: screenshot and other tools, free-form hotkey recording, accounts and sync.

## Install

Download the build for your platform from the repository **Releases** page:

| Platform | Recommended asset |
|----------|-------------------|
| Windows | `*_x64-setup.exe` or `*_x64_en-US.msi` |
| macOS (Apple Silicon) | `*_aarch64.dmg` |
| macOS (Intel) | `*_x64.dmg` |
| Linux (Debian / Ubuntu) | `*_amd64.deb` or AppImage |
| Linux (Fedora, etc.) | `*.rpm` |

Builds are **unsigned** by default:

- Windows may show SmartScreen
- macOS may quarantine the app. Try right-click → Open first; if it still won’t open, clear quarantine in Terminal:

```bash
xattr -d com.apple.quarantine /path/to/LiliaKit.app
```

## Usage

- Tray menu or left-click (Windows / Linux) to show / hide Settings; closing Settings hides to tray (does not quit)
- `Ctrl+Alt+M` (Mac: `⌃⌥M`) toggles the monitor (change or disable in Settings)
- Settings: General (launch at login, language, appearance) and Monitor (enable, pin, temperature, hotkey); save from the footer
- See [`CHANGELOG.md`](../../CHANGELOG.md) for release notes

## Development

Requires Node.js, [pnpm](https://pnpm.io), a Rust toolchain, and the platform WebView (WebView2 / WKWebView / WebKitGTK).

```bash
pnpm install
pnpm tauri dev    # development (isolated as LiliaKit Dev from the installed app)
pnpm tauri build  # produce installers
```

Also:

- `pnpm format` — dprint
- `pnpm lint` — ESLint
- `pnpm test` — pure-function tests (tsx)

Pushing a `v*` tag runs [`.github/workflows/release.yml`](../../.github/workflows/release.yml) to draft multi-platform releases.

Stack: Tauri 2 · Angular · TypeScript · Tailwind · Transloco · Catppuccin. Project conventions live in [`AGENTS.md`](../../AGENTS.md) (Chinese).

## License

[MIT](../../LICENSE) © Liu Yuchuan
