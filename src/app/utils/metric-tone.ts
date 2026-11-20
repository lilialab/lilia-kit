/** 指标负荷五档（由低到高） */
export type MetricTone = 'idle' | 'ok' | 'busy' | 'hot' | 'critical';

/** 四道阈值：ok / busy / hot / critical 的下限 */
type ToneStops = readonly [okAt: number, busyAt: number, hotAt: number, criticalAt: number];

const UTILIZATION_STOPS: ToneStops = [50, 70, 85, 95];
const TEMPERATURE_STOPS: ToneStops = [60, 70, 80, 90];

const TONE_CSS: Record<MetricTone, string> = {
  idle: 'var(--app-tone-idle)',
  ok: 'var(--app-tone-ok)',
  busy: 'var(--app-tone-busy)',
  hot: 'var(--app-tone-hot)',
  critical: 'var(--app-tone-critical)',
};

/** 五档 → CSS 变量（供 style / 进度条填色） */
export function metricToneCss(tone: MetricTone): string {
  return TONE_CSS[tone];
}

/**
 * 利用率（0–100，如 CPU / 内存）→ 五档。
 * 阈值：低于 50 idle；≥50 ok；≥70 busy；≥85 hot；≥95 critical。
 */
export function utilizationTone(percent: number): MetricTone {
  return toneFromStops(clamp(percent, 0, 100), UTILIZATION_STOPS);
}

/**
 * 温度（°C）→ 五档。
 * 阈值：低于 60 idle；≥60 ok；≥70 busy；≥80 hot；≥90 critical。
 */
export function temperatureTone(celsius: number): MetricTone {
  return toneFromStops(celsius, TEMPERATURE_STOPS);
}

function toneFromStops(
  value: number,
  [okAt, busyAt, hotAt, criticalAt]: ToneStops,
): MetricTone {
  if (value >= criticalAt) {
    return 'critical';
  }
  if (value >= hotAt) {
    return 'hot';
  }
  if (value >= busyAt) {
    return 'busy';
  }
  if (value >= okAt) {
    return 'ok';
  }
  return 'idle';
}

function clamp(value: number, min: number, max: number): number {
  if (value < min) {
    return min;
  }
  if (value > max) {
    return max;
  }
  return value;
}
