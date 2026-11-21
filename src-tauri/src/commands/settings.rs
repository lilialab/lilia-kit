//! 应用设置：配置目录 `settings.json`（字段缺省与默认深合并）。

use std::fs;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use serde_json::Value;
use tauri::{AppHandle, Emitter, Manager, Runtime};

const SETTINGS_FILE: &str = "settings.json";
const EVENT_APP_SETTINGS_CHANGED: &str = "app-settings-changed";

/// 监视器工具偏好。
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MonitorSettings {
    /// 是否启用（保存后装载/卸载浮窗）
    pub enabled: bool,
    /// 是否置顶（开窗时应用；设置页可改）
    pub pinned: bool,
    /// 是否显示温度
    pub show_temperature: bool,
}

/// 应用偏好设置（按工具分组）。
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppSettings {
    pub monitor: MonitorSettings,
}

impl Default for AppSettings {
    fn default() -> Self {
        serde_json::from_str(include_str!("default_settings.json"))
            .expect("default_settings.json must deserialize to AppSettings")
    }
}

fn settings_path<R: Runtime>(app: &AppHandle<R>) -> Result<PathBuf, String> {
    let dir = app
        .path()
        .app_config_dir()
        .map_err(|error| error.to_string())?;
    Ok(dir.join(SETTINGS_FILE))
}

/// 将 overlay 深合并进 base（对象递进；其它类型直接覆盖）。
fn deep_merge(base: &mut Value, overlay: &Value) {
    match (base, overlay) {
        (Value::Object(base_map), Value::Object(overlay_map)) => {
            for (key, value) in overlay_map {
                match base_map.get_mut(key) {
                    Some(existing) => deep_merge(existing, value),
                    None => {
                        base_map.insert(key.clone(), value.clone());
                    }
                }
            }
        }
        (base_slot, overlay_value) => {
            *base_slot = overlay_value.clone();
        }
    }
}

/// 合法 JSON 对象 → 与默认深合并；坏数据 → None。
fn parse_settings_json(text: &str) -> Option<AppSettings> {
    let parsed: Value = serde_json::from_str(text).ok()?;
    if !parsed.is_object() {
        return None;
    }
    let mut merged = serde_json::to_value(AppSettings::default()).ok()?;
    deep_merge(&mut merged, &parsed);
    serde_json::from_value(merged).ok()
}

fn write_settings_file(path: &Path, settings: &AppSettings) -> Result<(), String> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|error| error.to_string())?;
    }
    let text = serde_json::to_string_pretty(settings).map_err(|error| error.to_string())?;
    fs::write(path, text).map_err(|error| error.to_string())
}

/// 读取设置；深合并缺省后写回完整文件。
pub fn read_app_settings<R: Runtime>(app: &AppHandle<R>) -> Result<AppSettings, String> {
    let path = settings_path(app)?;
    let settings = match fs::read_to_string(&path) {
        Ok(text) => parse_settings_json(&text).unwrap_or_default(),
        Err(_) => AppSettings::default(),
    };
    write_settings_file(&path, &settings)?;
    Ok(settings)
}

/// 写入设置。
pub fn write_app_settings<R: Runtime>(
    app: &AppHandle<R>,
    settings: &AppSettings,
) -> Result<(), String> {
    write_settings_file(&settings_path(app)?, settings)
}

/// 通知各窗设置已变更（载荷为完整 `AppSettings`）。
pub fn emit_app_settings_changed<R: Runtime>(app: &AppHandle<R>, settings: &AppSettings) {
    let _ = app.emit(EVENT_APP_SETTINGS_CHANGED, settings);
}

/// 读取应用设置（给前端）。
#[tauri::command]
pub(crate) fn load_app_settings(app: AppHandle) -> Result<AppSettings, String> {
    read_app_settings(&app)
}

/// 内置默认设置，不读写磁盘。
#[tauri::command]
pub(crate) fn default_app_settings() -> AppSettings {
    AppSettings::default()
}

#[cfg(test)]
mod tests {
    use super::{parse_settings_json, AppSettings};

    #[test]
    fn deep_merges_partial_monitor() {
        let partial =
            parse_settings_json(r#"{"monitor":{"pinned":false}}"#).expect("parse");
        assert!(!partial.monitor.pinned);
        assert!(partial.monitor.show_temperature);
    }

    #[test]
    fn empty_monitor_object_keeps_defaults() {
        let partial = parse_settings_json(r#"{"monitor":{}}"#).expect("parse");
        assert!(partial.monitor.pinned);
        assert!(partial.monitor.show_temperature);
    }

    #[test]
    fn bad_json_is_none() {
        assert!(parse_settings_json("{").is_none());
    }

    #[test]
    fn default_matches_embedded() {
        let defaults = AppSettings::default();
        assert!(defaults.monitor.enabled);
        assert!(defaults.monitor.pinned);
        assert!(defaults.monitor.show_temperature);
    }

    #[test]
    fn deep_merges_keeps_enabled_default() {
        let partial =
            parse_settings_json(r#"{"monitor":{"pinned":false}}"#).expect("parse");
        assert!(partial.monitor.enabled);
        assert!(!partial.monitor.pinned);
    }
}
