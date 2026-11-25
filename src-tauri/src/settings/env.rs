//! 壳层 resolved 环境：主题 + 语言；与 settings 文档事件分开推送。

use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Emitter, Runtime, Theme};

use super::appearance::{self, AppThemeName};
use super::locale;
use super::model::{AppearancePreference, AppLocale, AppSettings};
use super::persist;
use crate::shell::tray;

const EVENT_APP_ENV_CHANGED: &str = "app-env-changed";

/// 前端 / 托盘共用的已解析壳层快照。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResolvedAppEnv {
    pub theme: AppThemeName,
    pub locale: AppLocale,
}

/// 按 settings + 当前 OS 解析快照。
pub fn resolve_from_settings<R: Runtime>(app: &AppHandle<R>, settings: &AppSettings) -> ResolvedAppEnv {
    ResolvedAppEnv {
        theme: appearance::resolve(settings.general.appearance, appearance::read_system_theme(app)),
        locale: locale::resolve(settings.general.locale),
    }
}

/// 保存 / 启动：对齐原生窗框、托盘，并广播 resolved env（非 settings 原文）。
pub fn apply_from_settings<R: Runtime>(
    app: &AppHandle<R>,
    settings: &AppSettings,
) -> Result<(), String> {
    appearance::apply_native_window_theme(app, settings.general.appearance);
    let env = resolve_from_settings(app, settings);
    push(app, env)
}

/// OS 主题变化：仅外观为 `system` 时重推（语言保持当前解析）。
pub fn on_os_theme_changed<R: Runtime>(app: &AppHandle<R>, system: Theme) {
    let Ok(settings) = persist::read_app_settings(app) else {
        return;
    };
    if settings.general.appearance != AppearancePreference::System {
        return;
    }
    let env = ResolvedAppEnv {
        theme: appearance::resolve(AppearancePreference::System, system),
        locale: locale::resolve(settings.general.locale),
    };
    let _ = push(app, env);
}

/// 启动对齐：当前已保存偏好下的 resolved 快照。
#[tauri::command]
pub fn get_resolved_app_env(app: AppHandle) -> Result<ResolvedAppEnv, String> {
    let settings = persist::read_app_settings(&app)?;
    Ok(resolve_from_settings(&app, &settings))
}

fn push<R: Runtime>(app: &AppHandle<R>, env: ResolvedAppEnv) -> Result<(), String> {
    tray::apply_locale(app, env.locale)?;
    let _ = app.emit(EVENT_APP_ENV_CHANGED, env);
    Ok(())
}
