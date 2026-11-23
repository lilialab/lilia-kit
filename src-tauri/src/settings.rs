//! 应用设置域：模型、持久化、apply 分发、外观。

mod apply;
mod appearance;
pub(crate) mod model;
mod persist;

pub use apply::{apply_app_settings, apply_runtime_from_settings, save_app_settings};
pub use appearance::{get_resolved_app_theme, on_os_theme_changed};
pub use persist::{default_app_settings, load_app_settings, read_app_settings};
