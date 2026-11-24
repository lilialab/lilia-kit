//! 全局快捷键：按绑定表注册；监视器显隐；截图等后续追加。

use std::collections::HashMap;
use std::sync::Mutex;

use tauri::{AppHandle, Manager, Runtime};
use tauri_plugin_global_shortcut::{GlobalShortcutExt, Shortcut, ShortcutState};

use crate::settings::model::AppSettings;
use crate::tools;

/// 热键槽位（多工具共用一张表）。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BindingId {
    MonitorVisibility,
}

/// 当前已注册的 accel（按槽位）。
pub struct HotkeysState {
    registered: Mutex<HashMap<BindingId, String>>,
}

impl HotkeysState {
    pub fn new() -> Self {
        Self {
            registered: Mutex::new(HashMap::new()),
        }
    }
}

/// 保存路径失败要整次回滚；启动路径只打日志。
#[derive(Debug, Clone, Copy)]
pub enum ApplyMode {
    Strict,
    Soft,
}

/// 按 settings 对齐全部绑定。
pub fn apply_from_settings<R: Runtime>(
    app: &AppHandle<R>,
    settings: &AppSettings,
    mode: ApplyMode,
) -> Result<(), String> {
    let desired = desired_bindings(settings);
    for (id, accel) in desired {
        if let Err(err) = apply_binding(app, id, &accel) {
            match mode {
                ApplyMode::Strict => return Err(err),
                ApplyMode::Soft => {
                    eprintln!("skip shortcut {id:?} ({accel}): {err}");
                }
            }
        }
    }
    Ok(())
}

fn desired_bindings(settings: &AppSettings) -> Vec<(BindingId, String)> {
    vec![(
        BindingId::MonitorVisibility,
        settings.monitor.visibility_hotkey.clone(),
    )]
}

fn apply_binding<R: Runtime>(
    app: &AppHandle<R>,
    id: BindingId,
    accel: &str,
) -> Result<(), String> {
    let state = app.state::<HotkeysState>();
    let mut registered = state
        .registered
        .lock()
        .map_err(|_| "hotkeys state poisoned".to_string())?;

    let previous = registered.get(&id).cloned();
    if previous.as_deref() == Some(accel) || (previous.is_none() && accel.is_empty()) {
        return Ok(());
    }

    if let Some(old) = previous.as_deref() {
        unregister_accel(app, old)?;
        registered.remove(&id);
    }

    if accel.is_empty() {
        return Ok(());
    }

    match register_binding(app, id, accel) {
        Ok(()) => {
            registered.insert(id, accel.to_string());
            Ok(())
        }
        Err(err) => {
            if let Some(old) = previous.as_deref() {
                if register_binding(app, id, old).is_ok() {
                    registered.insert(id, old.to_string());
                }
            }
            Err(err)
        }
    }
}

fn unregister_accel<R: Runtime>(app: &AppHandle<R>, accel: &str) -> Result<(), String> {
    let shortcut: Shortcut = accel
        .parse()
        .map_err(|err| format!("无效快捷键「{accel}」: {err}"))?;
    app.global_shortcut()
        .unregister(shortcut)
        .map_err(|err| err.to_string())
}

fn register_binding<R: Runtime>(
    app: &AppHandle<R>,
    id: BindingId,
    accel: &str,
) -> Result<(), String> {
    let shortcut: Shortcut = accel
        .parse()
        .map_err(|err| format!("无效快捷键「{accel}」: {err}"))?;

    let result = match id {
        BindingId::MonitorVisibility => app.global_shortcut().on_shortcut(
            shortcut,
            |app, _shortcut, event| {
                if event.state == ShortcutState::Pressed {
                    tools::monitor::emit_toggle(app);
                }
            },
        ),
    };

    result.map_err(|_| {
        format!("快捷键「{accel}」可能被占用，请换一条后重试")
    })
}
