//! 应用设置域：模型、持久化、apply 分发、壳层 env。

mod apply;
mod appearance;
mod commands;
mod env;
mod locale;
pub(crate) mod model;
mod persist;

pub use apply::{apply_app_settings, apply_runtime_from_settings};
pub use commands::{
    default_app_settings, get_resolved_app_env, load_app_settings, save_app_settings,
};
pub use env::on_os_theme_changed;
pub use persist::read_app_settings;
