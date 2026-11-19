import { Component, DestroyRef, inject, signal } from '@angular/core';
import type { SystemMetrics, TemperatureMetric } from '../lib/tauri/metrics';
import { getSystemMetrics } from '../lib/tauri/metrics';

const REFRESH_MS = 2000;

@Component({
  selector: 'app-monitor-page',
  templateUrl: './monitor.page.html',
})
export class MonitorPage {
  private readonly destroyRef = inject(DestroyRef);

  readonly metrics = signal<SystemMetrics | null>(null);
  readonly error = signal<string | null>(null);

  constructor() {
    const tick = (): void => {
      void getSystemMetrics()
        .then((next) => {
          this.metrics.set(next);
          this.error.set(null);
        })
        .catch((err: unknown) => {
          this.error.set(err instanceof Error ? err.message : String(err));
        });
    };

    tick();
    const id = window.setInterval(tick, REFRESH_MS);
    this.destroyRef.onDestroy(() => window.clearInterval(id));
  }

  cpuLabel(metrics: SystemMetrics): string {
    return `${metrics.cpuPercent.toFixed(1)}%`;
  }

  memoryLabel(metrics: SystemMetrics): string {
    const used = formatGib(metrics.memoryUsedBytes);
    const total = formatGib(metrics.memoryTotalBytes);
    const percent = percentOf(metrics.memoryUsedBytes, metrics.memoryTotalBytes);
    return `${used} / ${total} GiB (${percent.toFixed(1)}%)`;
  }

  networkLabel(metrics: SystemMetrics): string {
    return `↓ ${formatRate(metrics.networkRxBytesPerSec)}  ↑ ${
      formatRate(metrics.networkTxBytesPerSec)
    }`;
  }

  temperatureLabel(item: TemperatureMetric): string {
    return `${item.celsius.toFixed(1)} °C`;
  }
}

function percentOf(part: number, total: number): number {
  return total === 0 ? 0 : (part / total) * 100;
}

function formatGib(bytes: number): string {
  return (bytes / (1024 ** 3)).toFixed(1);
}

function formatRate(bytesPerSec: number): string {
  if (bytesPerSec >= 1024 ** 2) {
    return `${(bytesPerSec / (1024 ** 2)).toFixed(1)} MB/s`;
  }
  if (bytesPerSec >= 1024) {
    return `${(bytesPerSec / 1024).toFixed(1)} KB/s`;
  }
  return `${bytesPerSec.toFixed(0)} B/s`;
}
