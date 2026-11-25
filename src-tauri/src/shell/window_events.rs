//! 多窗生命周期与 OS 主题事件。

use tauri::{Manager, Window, WindowEvent};

use super::window_labels::{MAIN, MONITOR};
use crate::settings::on_os_theme_changed;
use crate::tools::{self, ToolsState};

/// 主窗关进托盘、监视窗销毁回写、主窗主题变化重推 env。
pub fn on_window_event(window: &Window, event: &WindowEvent) {
    match event {
        WindowEvent::CloseRequested { api, .. } => {
            if window.label() == MAIN {
                // 主窗 X → 进托盘；真正退出走托盘「退出」
                api.prevent_close();
                let _ = window.hide();
            }
        }
        WindowEvent::Destroyed => {
            if window.label() == MONITOR {
                let app = window.app_handle().clone();
                if let Some(state) = app.try_state::<ToolsState>() {
                    tools::monitor::on_destroyed(&app, state.inner());
                }
            }
        }
        // 只跟主窗，避免多窗重复 emit；窗 theme 为 None 时才会收到
        WindowEvent::ThemeChanged(theme) => {
            if window.label() == MAIN {
                on_os_theme_changed(window.app_handle(), *theme);
            }
        }
        _ => {}
    }
}
