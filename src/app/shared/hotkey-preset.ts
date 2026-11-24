import { ChangeDetectionStrategy, Component, computed, inject, input, output } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { AppEnv } from '../app-env';
import { formatAccelDisplay, HOTKEY_PRESETS } from './hotkey-format';

/** 通用全局热键预设：下拉选择或「无」；产出 accel，不落盘、不换绑。 */
@Component({
  selector: 'app-hotkey-preset',
  templateUrl: './hotkey-preset.html',
  imports: [FormsModule],
  changeDetection: ChangeDetectionStrategy.OnPush,
  host: { class: 'block' },
})
export class HotkeyPreset {
  private readonly appEnv = inject(AppEnv);

  /** 当前 accel；空 = 未绑定 */
  readonly value = input('');
  readonly disabled = input(false);
  /** 候选列表；默认监视器显隐那套 */
  readonly presets = input<readonly string[]>(HOTKEY_PRESETS);
  readonly valueChange = output<string>();

  /** 已保存但不在列表中的旧值，单独列为一项以免无法还原展示 */
  readonly orphanAccel = computed(() => {
    const value = this.value();
    if (!value || this.presets().includes(value)) {
      return null;
    }
    return value;
  });

  labelFor(accel: string): string {
    return formatAccelDisplay(accel, this.appEnv.platform()) || accel;
  }

  onValueChange(accel: string): void {
    if (this.disabled()) {
      return;
    }
    this.valueChange.emit(accel);
  }
}
