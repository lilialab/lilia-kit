//! 设置持久化：`settings.json` 读写与缺省深合并。

use std::fs;
use std::path::{Path, PathBuf};

use serde_json::Value;
use tauri::{AppHandle, Emitter, Manager, Runtime};

use super::model::AppSettings;

const SETTINGS_FILE: &str = "settings.json";
const EVENT_APP_SETTINGS_CHANGED: &str = "app-settings-changed";

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
pub fn load_app_settings(app: AppHandle) -> Result<AppSettings, String> {
    read_app_settings(&app)
}

/// 内置默认设置，不读写磁盘。
#[tauri::command]
pub fn default_app_settings() -> AppSettings {
    AppSettings::default()
}

#[cfg(test)]
mod tests {
    use super::parse_settings_json;
    use crate::settings::model::{AppearancePreference, AppSettings};

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
        assert_eq!(defaults.general.appearance, AppearancePreference::System);
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

    #[test]
    fn missing_general_defaults_appearance_to_system() {
        let partial =
            parse_settings_json(r#"{"monitor":{"pinned":false}}"#).expect("parse");
        assert_eq!(partial.general.appearance, AppearancePreference::System);
    }

    #[test]
    fn empty_general_object_keeps_appearance_default() {
        let partial = parse_settings_json(r#"{"general":{}}"#).expect("parse");
        assert_eq!(partial.general.appearance, AppearancePreference::System);
    }

    #[test]
    fn parses_appearance_theme_names() {
        let latte =
            parse_settings_json(r#"{"general":{"appearance":"latte"}}"#).expect("parse");
        assert_eq!(latte.general.appearance, AppearancePreference::Latte);
        let mocha =
            parse_settings_json(r#"{"general":{"appearance":"mocha"}}"#).expect("parse");
        assert_eq!(mocha.general.appearance, AppearancePreference::Mocha);
    }
}
