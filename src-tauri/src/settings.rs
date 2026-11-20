//! 应用设置：配置目录 `settings.json`（字段缺省与默认合并）。

use std::fs;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Manager, Runtime};

const SETTINGS_FILE: &str = "settings.json";

/// 应用偏好设置。
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppSettings {
    /// 监视器是否置顶（开窗时应用；设置页可改）
    pub monitor_pinned: bool,
}

impl Default for AppSettings {
    fn default() -> Self {
        serde_json::from_str(include_str!("settings/default_settings.json"))
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

/// 合法 JSON 对象 → 与默认合并；坏数据 → None。
fn parse_settings_json(text: &str) -> Option<AppSettings> {
    let parsed: serde_json::Value = serde_json::from_str(text).ok()?;
    let obj = parsed.as_object()?;
    let mut merged = serde_json::to_value(AppSettings::default()).ok()?;
    let merged_obj = merged.as_object_mut()?;
    for (key, value) in obj {
        merged_obj.insert(key.clone(), value.clone());
    }
    serde_json::from_value(merged).ok()
}

fn write_settings_file(path: &Path, settings: &AppSettings) -> Result<(), String> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|error| error.to_string())?;
    }
    let text = serde_json::to_string_pretty(settings).map_err(|error| error.to_string())?;
    fs::write(path, text).map_err(|error| error.to_string())
}

/// 读取设置；合并缺省后写回完整文件。
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

/// 读取应用设置（给前端）。
#[tauri::command]
pub fn load_app_settings(app: AppHandle) -> Result<AppSettings, String> {
    read_app_settings(&app)
}

/// 保存应用设置（给前端）。
#[tauri::command]
pub fn save_app_settings(app: AppHandle, settings: AppSettings) -> Result<(), String> {
    write_app_settings(&app, &settings)
}

/// 内置默认设置，不读写磁盘。
#[tauri::command]
pub fn default_app_settings() -> AppSettings {
    AppSettings::default()
}

#[cfg(test)]
mod tests {
    use super::{parse_settings_json, AppSettings};

    #[test]
    fn merges_partial_json() {
        let partial = parse_settings_json(r#"{"monitorPinned": true}"#).expect("parse");
        assert!(partial.monitor_pinned);
    }

    #[test]
    fn bad_json_is_none() {
        assert!(parse_settings_json("{").is_none());
    }

    #[test]
    fn default_matches_embedded() {
        let defaults = AppSettings::default();
        assert!(defaults.monitor_pinned);
    }
}
