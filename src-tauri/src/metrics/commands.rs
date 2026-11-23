//! 系统指标 IPC。

use tauri::State;

use super::state::{MetricsState, SystemMetrics};

/// 刷新并返回 CPU / 内存 / 合计网速 / 温度。
#[tauri::command]
pub fn get_system_metrics(state: State<'_, MetricsState>) -> Result<SystemMetrics, String> {
    state.snapshot()
}
