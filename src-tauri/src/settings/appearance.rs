//! 外观解析：偏好 + OS → 主题名；原生窗框 `set_theme`；emit；OS 变化时仅 `system` 再推送。

use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Emitter, Manager, Runtime, Theme};

use super::model::{AppearancePreference, AppSettings};
use super::persist;
use crate::shell::window_labels;

const EVENT_APP_THEME_CHANGED: &str = "app-theme-changed";

/// 已解析的 Catppuccin 主题名（与前端 `data-theme` 一致）。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum AppThemeName {
    Latte,
    Mocha,
}

/// 偏好 + 当前 OS 明暗 → 主题名。
pub fn resolve(appearance: AppearancePreference, system: Theme) -> AppThemeName {
    match appearance {
        AppearancePreference::Latte => AppThemeName::Latte,
        AppearancePreference::Mocha => AppThemeName::Mocha,
        AppearancePreference::System => theme_from_os(system),
    }
}

fn theme_from_os(system: Theme) -> AppThemeName {
    if matches!(system, Theme::Dark) {
        AppThemeName::Mocha
    } else {
        AppThemeName::Latte
    }
}

/// `system` → 跟随 OS（`None`，才能继续收 `ThemeChanged`）；固定主题 → Light/Dark。
fn preference_to_window_theme(appearance: AppearancePreference) -> Option<Theme> {
    match appearance {
        AppearancePreference::System => None,
        AppearancePreference::Latte => Some(Theme::Light),
        AppearancePreference::Mocha => Some(Theme::Dark),
    }
}

/// 同步原生窗框/标题栏主题（平台能力范围内；监视窗无边框时影响有限）。
fn apply_native_window_theme<R: Runtime>(app: &AppHandle<R>, appearance: AppearancePreference) {
    let theme = preference_to_window_theme(appearance);
    for window in app.webview_windows().into_values() {
        let _ = window.set_theme(theme);
    }
}

/// 从主窗读取 OS/有效主题；失败时偏 Mocha（与 CSS 默认一致）。
pub fn read_system_theme<R: Runtime>(app: &AppHandle<R>) -> Theme {
    app.get_webview_window(window_labels::MAIN)
        .and_then(|window| window.theme().ok())
        .unwrap_or(Theme::Dark)
}

/// 按已保存 settings：先对齐原生窗框，再解析并广播主题名。
pub fn apply_from_settings<R: Runtime>(app: &AppHandle<R>, settings: &AppSettings) {
    let appearance = settings.general.appearance;
    // 固定→跟随 时须先清强制主题，再读 OS
    apply_native_window_theme(app, appearance);
    let theme = resolve(appearance, read_system_theme(app));
    emit_theme_changed(app, theme);
}

/// OS 外观变化：仅当偏好为 `system` 时再解析并广播（原生已跟随，无需再 `set_theme`）。
pub fn on_os_theme_changed<R: Runtime>(app: &AppHandle<R>, system: Theme) {
    let Ok(settings) = persist::read_app_settings(app) else {
        return;
    };
    if settings.general.appearance != AppearancePreference::System {
        return;
    }
    emit_theme_changed(app, theme_from_os(system));
}

fn emit_theme_changed<R: Runtime>(app: &AppHandle<R>, theme: AppThemeName) {
    let _ = app.emit(EVENT_APP_THEME_CHANGED, theme);
}

/// 当前已保存偏好下的解析主题（给前端启动对齐）。
#[tauri::command]
pub fn get_resolved_app_theme(app: AppHandle) -> Result<AppThemeName, String> {
    let settings = persist::read_app_settings(&app)?;
    Ok(resolve(settings.general.appearance, read_system_theme(&app)))
}

#[cfg(test)]
mod tests {
    use super::{preference_to_window_theme, resolve, theme_from_os, AppThemeName};
    use crate::settings::model::AppearancePreference;
    use tauri::Theme;

    #[test]
    fn fixed_themes_ignore_os() {
        assert_eq!(
            resolve(AppearancePreference::Latte, Theme::Dark),
            AppThemeName::Latte
        );
        assert_eq!(
            resolve(AppearancePreference::Mocha, Theme::Light),
            AppThemeName::Mocha
        );
    }

    #[test]
    fn system_follows_os() {
        assert_eq!(
            resolve(AppearancePreference::System, Theme::Dark),
            AppThemeName::Mocha
        );
        assert_eq!(
            resolve(AppearancePreference::System, Theme::Light),
            AppThemeName::Latte
        );
    }

    #[test]
    fn os_theme_maps() {
        assert_eq!(theme_from_os(Theme::Dark), AppThemeName::Mocha);
        assert_eq!(theme_from_os(Theme::Light), AppThemeName::Latte);
    }

    #[test]
    fn preference_maps_to_window_theme() {
        assert_eq!(preference_to_window_theme(AppearancePreference::System), None);
        assert_eq!(
            preference_to_window_theme(AppearancePreference::Latte),
            Some(Theme::Light)
        );
        assert_eq!(
            preference_to_window_theme(AppearancePreference::Mocha),
            Some(Theme::Dark)
        );
    }
}
