import { invoke } from '@tauri-apps/api/core';

export interface TemperatureMetric {
  label: string;
  celsius: number;
}

export interface SystemMetrics {
  cpuPercent: number;
  memoryUsedBytes: number;
  memoryTotalBytes: number;
  networkRxBytesPerSec: number;
  networkTxBytesPerSec: number;
  temperatures: TemperatureMetric[];
}

/** 拉取整机监视快照（CPU / 内存 / 合计网速 / 温度） */
export async function getSystemMetrics(): Promise<SystemMetrics> {
  return invoke<SystemMetrics>('get_system_metrics');
}
