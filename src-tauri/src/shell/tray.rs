//! 系统托盘：显隐主窗与退出。

use tauri::{
    AppHandle, Manager, Runtime,
    menu::{Menu, MenuItem},
    tray::TrayIconBuilder,
};
#[cfg(not(target_os = "macos"))]
use tauri::tray::{MouseButton, MouseButtonState, TrayIconEvent};

use super::window_labels;
use crate::settings::model::AppLocale;

const TRAY_ID: &str = "main";
const MENU_TOGGLE: &str = "toggle";
const MENU_QUIT: &str = "quit";

/// 创建托盘图标与菜单。
pub fn setup<R: Runtime>(app: &AppHandle<R>) -> tauri::Result<()> {
    let menu = build_menu(app, AppLocale::Zh)?;

    let mut builder = TrayIconBuilder::with_id(TRAY_ID)
        .menu(&menu)
        .tooltip("LiliaKit")
        .on_menu_event(|app, event| match event.id().as_ref() {
            MENU_TOGGLE => toggle_main_window(app),
            MENU_QUIT => app.exit(0),
            _ => {}
        });

    // mac 菜单栏：左键出菜单。Win/Linux：左键显隐，右键菜单。
    #[cfg(target_os = "macos")]
    {
        builder = builder.show_menu_on_left_click(true);
    }
    #[cfg(not(target_os = "macos"))]
    {
        builder = builder
            .show_menu_on_left_click(false)
            .on_tray_icon_event(|tray, event| {
                if let TrayIconEvent::Click {
                    button: MouseButton::Left,
                    button_state: MouseButtonState::Up,
                    ..
                } = event
                {
                    toggle_main_window(tray.app_handle());
                }
            });
    }

    if let Some(icon) = app.default_window_icon() {
        builder = builder.icon(icon.clone());
    }

    builder.build(app)?;
    Ok(())
}

/// 按已解析语言更新托盘菜单文案。
pub fn apply_locale<R: Runtime>(app: &AppHandle<R>, locale: AppLocale) -> Result<(), String> {
    let Some(tray) = app.tray_by_id(TRAY_ID) else {
        return Ok(());
    };
    let menu = build_menu(app, locale).map_err(|err| err.to_string())?;
    tray.set_menu(Some(menu)).map_err(|err| err.to_string())?;
    Ok(())
}

fn build_menu<R: Runtime>(
    app: &AppHandle<R>,
    locale: AppLocale,
) -> tauri::Result<Menu<R>> {
    let (toggle_label, quit_label) = match locale {
        AppLocale::Zh => ("显示 / 隐藏", "退出"),
        AppLocale::En => ("Show / Hide", "Quit"),
    };
    let toggle = MenuItem::with_id(app, MENU_TOGGLE, toggle_label, true, None::<&str>)?;
    let quit = MenuItem::with_id(app, MENU_QUIT, quit_label, true, None::<&str>)?;
    Menu::with_items(app, &[&toggle, &quit])
}

/// 显示并聚焦主窗（二次启动 / 托盘找回）。
pub fn show_main_window<R: Runtime>(app: &AppHandle<R>) {
    if let Some(window) = app.get_webview_window(window_labels::MAIN) {
        let _ = window.unminimize();
        let _ = window.show();
        let _ = window.set_focus();
    }
}

fn toggle_main_window<R: Runtime>(app: &AppHandle<R>) {
    let Some(window) = app.get_webview_window(window_labels::MAIN) else {
        return;
    };
    match window.is_visible() {
        Ok(true) => {
            let _ = window.hide();
        }
        _ => {
            let _ = window.unminimize();
            let _ = window.show();
            let _ = window.set_focus();
        }
    }
}
