# LiliaKit

Tauri 2 + Angular 桌面工具箱（系统监视等）。

## 推荐 IDE

[VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer) + [Angular Language Service](https://marketplace.visualstudio.com/items?itemName=Angular.ng-template) + [dprint](https://marketplace.visualstudio.com/items?itemName=dprint.dprint) + [ESLint](https://marketplace.visualstudio.com/items?itemName=dbaeumer.vscode-eslint)。

## 开发

需要：Node.js、[pnpm](https://pnpm.io)、Rust 工具链，以及本机系统 WebView（Windows WebView2）。

```bash
pnpm install
pnpm tauri dev    # 开发（与安装版隔离为 LiliaKit Dev）
pnpm tauri build  # 打安装包
```

其它：

- `pnpm format` — dprint
- `pnpm lint` — ESLint
- `pnpm start` — 仅前端（给 `tauri dev` 的 beforeDevCommand 用，不是产品运行方式）
- 样式：Tailwind v4 + Catppuccin（`data-theme`=`latte`|`mocha`；语义色 `--app-*`）

推送 `v*` tag 后由 [`.github/workflows/release.yml`](.github/workflows/release.yml) 打多平台 Draft Release。变更记在 [`CHANGELOG.md`](CHANGELOG.md)。

仓库约定见 [`AGENTS.md`](AGENTS.md)。

## 许可

[MIT](LICENSE) © Liu Yuchuan
