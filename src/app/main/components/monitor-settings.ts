import { ChangeDetectionStrategy, Component, inject } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { HotkeyPreset } from '../../shared/hotkey-preset';
import { AppSettingsStore } from '../app-settings.store';

/** 主窗右侧：监视器工具配置 */
@Component({
  selector: 'app-monitor-settings',
  templateUrl: './monitor-settings.html',
  imports: [FormsModule, HotkeyPreset],
  changeDetection: ChangeDetectionStrategy.OnPush,
})
export class MonitorSettingsPanel {
  readonly appSettings = inject(AppSettingsStore);

  onCheck(
    key: 'enabled' | 'pinned' | 'showTemperature',
    value: boolean,
  ): void {
    this.appSettings.patchTool('monitor', key, value);
  }

  onVisibilityHotkeyChange(accel: string): void {
    this.appSettings.patchTool('monitor', 'visibilityHotkey', accel);
  }
}
