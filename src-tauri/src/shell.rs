//! 桌面壳：托盘、快捷键、自启、窗名与平台外观。

#[cfg(desktop)]
pub mod autostart;
#[cfg(desktop)]
pub mod hotkeys;
pub mod tray;
pub mod window_chrome;
pub mod window_labels;
