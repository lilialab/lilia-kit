//! 系统监视指标：CPU / 内存 / 网速 / 温度。

use std::time::Instant;

use serde::Serialize;
use sysinfo::{Components, CpuRefreshKind, MemoryRefreshKind, Networks, RefreshKind, System};

/// 整机监视快照。
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SystemMetrics {
    pub cpu_percent: f32,
    pub memory_used_bytes: u64,
    pub memory_total_bytes: u64,
    /// 网卡合计下行字节/秒
    pub network_rx_bytes_per_sec: f64,
    /// 网卡合计上行字节/秒
    pub network_tx_bytes_per_sec: f64,
    pub temperatures: Vec<TemperatureMetric>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TemperatureMetric {
    pub label: String,
    pub celsius: f32,
}

/// 延迟初始化：避免在 Tauri 窗口创建前于主线程初始化 COM（会触发 OleInitialize RPC_E_CHANGED_MODE）。
pub struct MetricsState {
    inner: std::sync::Mutex<Option<MetricsInner>>,
}

struct MetricsInner {
    system: System,
    networks: Networks,
    components: Components,
    last_instant: Instant,
}

impl MetricsState {
    pub fn new() -> Self {
        Self {
            inner: std::sync::Mutex::new(None),
        }
    }

    /// 刷新并返回当前快照。
    pub fn snapshot(&self) -> Result<SystemMetrics, String> {
        let mut guard = self
            .inner
            .lock()
            .map_err(|_| "metrics lock poisoned".to_string())?;

        if guard.is_none() {
            *guard = Some(MetricsInner::new());
        }
        let inner = guard.as_mut().expect("metrics inner just initialized");
        Ok(inner.refresh())
    }
}

impl MetricsInner {
    fn new() -> Self {
        let mut system = System::new_with_specifics(
            RefreshKind::nothing()
                .with_cpu(CpuRefreshKind::nothing().with_cpu_usage())
                .with_memory(MemoryRefreshKind::nothing().with_ram()),
        );
        system.refresh_cpu_usage();
        system.refresh_memory();

        let mut networks = Networks::new_with_refreshed_list();
        networks.refresh(true);

        let mut components = Components::new_with_refreshed_list();
        components.refresh(true);

        Self {
            system,
            networks,
            components,
            last_instant: Instant::now(),
        }
    }

    fn refresh(&mut self) -> SystemMetrics {
        let elapsed = self.last_instant.elapsed().as_secs_f64().max(0.001);
        self.last_instant = Instant::now();

        self.system.refresh_cpu_usage();
        self.system.refresh_memory();
        self.networks.refresh(true);
        self.components.refresh(true);

        let mut network_rx = 0u64;
        let mut network_tx = 0u64;
        for (_name, data) in self.networks.iter() {
            network_rx += data.received();
            network_tx += data.transmitted();
        }

        let temperatures = self
            .components
            .iter()
            .filter_map(|component| {
                component.temperature().map(|celsius| TemperatureMetric {
                    label: component.label().to_string(),
                    celsius,
                })
            })
            .collect();

        SystemMetrics {
            cpu_percent: self.system.global_cpu_usage(),
            memory_used_bytes: self.system.used_memory(),
            memory_total_bytes: self.system.total_memory(),
            network_rx_bytes_per_sec: network_rx as f64 / elapsed,
            network_tx_bytes_per_sec: network_tx as f64 / elapsed,
            temperatures,
        }
    }
}
