//! 按平台套用窗口外观：mac 系统毛玻璃；Windows 实色半透明（无特效）。

use tauri::{Runtime, WebviewWindow};

/// 给指定窗套系统特效（失败则保持 CSS 实色半透明底）。
pub fn apply_to<R: Runtime>(window: &WebviewWindow<R>) {
    #[cfg(target_os = "macos")]
    apply_macos(window);

    #[cfg(windows)]
    apply_windows(window);

    #[cfg(not(any(target_os = "macos", windows)))]
    {
        let _ = window;
    }
}

#[cfg(target_os = "macos")]
fn apply_macos<R: Runtime>(window: &WebviewWindow<R>) {
    use tauri::window::{Effect, EffectState, EffectsBuilder};

    let _ = window.set_shadow(true);
    let _ = window.set_effects(
        EffectsBuilder::new()
            .effects([Effect::HudWindow, Effect::Popover, Effect::Sidebar])
            .state(EffectState::Active)
            .radius(12.0)
            .build(),
    );
}

#[cfg(windows)]
fn apply_windows<R: Runtime>(window: &WebviewWindow<R>) {
    // 全 Windows：不用 Mica/Acrylic（拖动不跟手）；实底透明度交给 CSS。
    let _ = window.set_shadow(false);
    let _ = window.set_effects(None::<tauri::utils::config::WindowEffectsConfig>);
}
