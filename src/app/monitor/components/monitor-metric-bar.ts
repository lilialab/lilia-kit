import { ChangeDetectionStrategy, Component, computed, input } from '@angular/core';
import { metricToneCss, utilizationTone } from '../../utils/metric-tone';

@Component({
  selector: 'app-monitor-metric-bar',
  templateUrl: './monitor-metric-bar.html',
  changeDetection: ChangeDetectionStrategy.OnPush,
})
export class MonitorMetricBar {
  readonly percent = input.required<number>();

  readonly barWidth = computed(() => clampPercent(this.percent()));
  readonly barColor = computed(() => metricToneCss(utilizationTone(this.barWidth())));
  readonly percentLabel = computed(() => `${this.barWidth().toFixed(0)}%`);
}

function clampPercent(percent: number): number {
  if (percent < 0) {
    return 0;
  }
  if (percent > 100) {
    return 100;
  }
  return percent;
}
