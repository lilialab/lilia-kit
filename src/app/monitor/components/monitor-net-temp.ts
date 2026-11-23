import { ChangeDetectionStrategy, Component, computed, input } from '@angular/core';
import { LucideArrowDown, LucideArrowUp, LucideNetwork, LucideThermometer } from '@lucide/angular';
import type { TemperatureMetric } from '../../lib/tauri/metrics';
import { metricToneCss, temperatureTone } from '../../utils/metric-tone';

const CATEGORY_ICON_SIZE = 14;
const RATE_ICON_SIZE = 12;

@Component({
  selector: 'app-monitor-net-temp',
  templateUrl: './monitor-net-temp.html',
  changeDetection: ChangeDetectionStrategy.OnPush,
  imports: [LucideNetwork, LucideThermometer, LucideArrowDown, LucideArrowUp],
})
export class MonitorNetTemp {
  readonly categoryIconSize = CATEGORY_ICON_SIZE;
  readonly rateIconSize = RATE_ICON_SIZE;

  readonly rxBytesPerSec = input.required<number>();
  readonly txBytesPerSec = input.required<number>();
  readonly temperatures = input<TemperatureMetric[]>([]);
  /** 是否显示温度；关闭时仅保留网速 */
  readonly showTemperature = input(true);

  readonly rxLabel = computed(() => formatRate(this.rxBytesPerSec()));
  readonly txLabel = computed(() => formatRate(this.txBytesPerSec()));
  readonly rxMega = computed(() => isMegaRate(this.rxBytesPerSec()));
  readonly txMega = computed(() => isMegaRate(this.txBytesPerSec()));

  readonly hottest = computed(() => {
    let best: TemperatureMetric | null = null;
    for (const item of this.temperatures()) {
      if (!best || item.celsius > best.celsius) {
        best = item;
      }
    }
    return best;
  });

  readonly tempLabel = computed(() => {
    const item = this.hottest();
    return item ? `${item.celsius.toFixed(0)}°` : '—';
  });

  readonly tempColor = computed(() => {
    const item = this.hottest();
    if (!item) {
      return 'var(--app-muted)';
    }
    return metricToneCss(temperatureTone(item.celsius));
  });
}

const MEGA_BYTES_PER_SEC = 1024 ** 2;

function formatRate(bytesPerSec: number): string {
  if (bytesPerSec >= MEGA_BYTES_PER_SEC) {
    return `${(bytesPerSec / MEGA_BYTES_PER_SEC).toFixed(1)}M`;
  }
  if (bytesPerSec >= 1024) {
    return `${(bytesPerSec / 1024).toFixed(0)}K`;
  }
  return `${bytesPerSec.toFixed(0)}B`;
}

/** ≥1M/s（上下行同一规则） */
function isMegaRate(bytesPerSec: number): boolean {
  return bytesPerSec >= MEGA_BYTES_PER_SEC;
}
