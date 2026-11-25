import { Component, DestroyRef, inject, signal } from '@angular/core';
import { TranslocoPipe } from '@jsverse/transloco';
import { LucideCpu, LucideMemoryStick } from '@lucide/angular';
import type { SystemMetrics } from '../lib/tauri/metrics';
import { getSystemMetrics } from '../lib/tauri/metrics';
import { onMonitorToggle, setMonitorWindowVisible } from '../lib/tauri/monitor';
import { loadAppSettings, onAppSettingsChanged } from '../lib/tauri/settings';
import { MonitorMetricBar } from './components/monitor-metric-bar';
import { MonitorNetTemp } from './components/monitor-net-temp';

const REFRESH_MS = 2000;
/** 与 CSS `--monitor-fade-ms` 对齐；无 transition 时作兜底 */
const FADE_MS = 180;

@Component({
  selector: 'app-monitor-page',
  templateUrl: './monitor.page.html',
  imports: [MonitorMetricBar, MonitorNetTemp, LucideCpu, LucideMemoryStick, TranslocoPipe],
})
export class MonitorPage {
  private readonly destroyRef = inject(DestroyRef);
  private hideTimer: number | undefined;

  readonly iconSize = 14;
  readonly metrics = signal<SystemMetrics | null>(null);
  readonly showTemperature = signal(true);
  readonly error = signal<string | null>(null);
  /** 逻辑显现：驱动淡入淡出；首屏先透明再淡入 */
  readonly revealed = signal(false);

  constructor() {
    requestAnimationFrame(() => this.revealed.set(true));

    void loadAppSettings()
      .then((settings) => {
        this.showTemperature.set(settings.monitor.showTemperature);
      })
      .catch(() => {
        /* 设置读失败时沿用默认显示温度 */
      });

    void onAppSettingsChanged((settings) => {
      this.showTemperature.set(settings.monitor.showTemperature);
    }).then((unlisten) => {
      this.destroyRef.onDestroy(unlisten);
    });

    void onMonitorToggle(() => {
      void this.toggleVisibility();
    }).then((unlisten) => {
      this.destroyRef.onDestroy(unlisten);
    });

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
    this.destroyRef.onDestroy(() => {
      window.clearInterval(id);
      this.clearHideTimer();
    });
  }

  memoryPercent(metrics: SystemMetrics): number {
    return percentOf(metrics.memoryUsedBytes, metrics.memoryTotalBytes);
  }

  private async toggleVisibility(): Promise<void> {
    if (this.revealed()) {
      this.beginHide();
      return;
    }
    await this.beginShow();
  }

  private async beginShow(): Promise<void> {
    this.clearHideTimer();
    await setMonitorWindowVisible(true);
    requestAnimationFrame(() => this.revealed.set(true));
  }

  private beginHide(): void {
    this.clearHideTimer();
    this.revealed.set(false);
    this.hideTimer = window.setTimeout(() => {
      this.hideTimer = undefined;
      void setMonitorWindowVisible(false);
    }, FADE_MS);
  }

  private clearHideTimer(): void {
    if (this.hideTimer !== undefined) {
      window.clearTimeout(this.hideTimer);
      this.hideTimer = undefined;
    }
  }
}

function percentOf(part: number, total: number): number {
  return total === 0 ? 0 : (part / total) * 100;
}
