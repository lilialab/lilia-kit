//! 应用设置域：模型、持久化、apply 分发、壳层 env。

mod apply;
mod appearance;
mod env;
mod locale;
pub(crate) mod model;
mod persist;

pub use apply::{apply_app_settings, apply_runtime_from_settings, save_app_settings};
pub use env::{get_resolved_app_env, on_os_theme_changed};
pub use persist::{default_app_settings, load_app_settings, read_app_settings};
