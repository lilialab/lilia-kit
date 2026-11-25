//! 开机自启：与 settings 对齐；带 `--startup` 时静默进托盘。

use tauri::{AppHandle, Runtime};
use tauri_plugin_autostart::ManagerExt;

/// 自启进程传入的参数（注册 autostart 时写入）。
pub const STARTUP_ARG: &str = "--startup";

/// 保存失败要整次回滚意图；启动路径只打日志。
#[derive(Debug, Clone, Copy)]
pub enum ApplyMode {
    Strict,
    Soft,
}

/// 当前进程是否由开机自启拉起。
pub fn launched_from_startup() -> bool {
    std::env::args().any(|arg| arg == STARTUP_ARG)
}

/// 按 settings 对齐系统「登录时启动」。
pub fn apply_launch_at_login<R: Runtime>(
    app: &AppHandle<R>,
    enabled: bool,
    mode: ApplyMode,
) -> Result<(), String> {
    let manager = app.autolaunch();
    let currently = match manager.is_enabled() {
        Ok(v) => v,
        Err(err) => {
            return match mode {
                ApplyMode::Strict => Err(err.to_string()),
                ApplyMode::Soft => {
                    eprintln!("skip autostart is_enabled: {err}");
                    Ok(())
                }
            };
        }
    };

    if currently == enabled {
        return Ok(());
    }

    let result = if enabled {
        manager.enable()
    } else {
        manager.disable()
    };

    match result {
        Ok(()) => Ok(()),
        Err(err) => match mode {
            ApplyMode::Strict => Err(format!(
                "无法{}开机启动：{err}",
                if enabled { "开启" } else { "关闭" }
            )),
            ApplyMode::Soft => {
                eprintln!("skip autostart apply (enabled={enabled}): {err}");
                Ok(())
            }
        },
    }
}
