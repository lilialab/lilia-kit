import { Component, DestroyRef, inject, signal } from '@angular/core';
import { LucideCpu, LucideMemoryStick } from '@lucide/angular';
import type { SystemMetrics } from '../lib/tauri/metrics';
import { getSystemMetrics } from '../lib/tauri/metrics';
import { MonitorMetricBar } from './components/monitor-metric-bar';
import { MonitorNetTemp } from './components/monitor-net-temp';

const REFRESH_MS = 2000;

@Component({
  selector: 'app-monitor-page',
  templateUrl: './monitor.page.html',
  imports: [MonitorMetricBar, MonitorNetTemp, LucideCpu, LucideMemoryStick],
})
export class MonitorPage {
  private readonly destroyRef = inject(DestroyRef);

  readonly iconSize = 14;
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

  memoryPercent(metrics: SystemMetrics): number {
    return percentOf(metrics.memoryUsedBytes, metrics.memoryTotalBytes);
  }
}

function percentOf(part: number, total: number): number {
  return total === 0 ? 0 : (part / total) * 100;
}
