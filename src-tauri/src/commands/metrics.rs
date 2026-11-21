//! 系统指标命令。

use tauri::State;

use crate::metrics::{MetricsState, SystemMetrics};

/// 刷新并返回 CPU / 内存 / 合计网速 / 温度。
#[tauri::command]
pub(crate) fn get_system_metrics(
    state: State<'_, MetricsState>,
) -> Result<SystemMetrics, String> {
    state.snapshot()
}
