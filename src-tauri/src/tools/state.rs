//! 工具启用位与生命周期锁。

use std::sync::atomic::{AtomicBool, Ordering};

/// 各工具启用位（与 settings 各工具块 `enabled` 对齐）。
pub struct ToolsState {
    monitor_enabled: AtomicBool,
    /// 串行化监视器开/关，避免快切时销毁未完成又建窗。
    pub(crate) monitor_lifecycle: std::sync::Mutex<()>,
}

impl ToolsState {
    pub fn new(monitor_enabled: bool) -> Self {
        Self {
            monitor_enabled: AtomicBool::new(monitor_enabled),
            monitor_lifecycle: std::sync::Mutex::new(()),
        }
    }

    pub(crate) fn monitor_enabled(&self) -> bool {
        self.monitor_enabled.load(Ordering::SeqCst)
    }

    pub(crate) fn set_monitor_enabled(&self, enabled: bool) {
        self.monitor_enabled.store(enabled, Ordering::SeqCst);
    }
}
