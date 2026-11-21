import { ChangeDetectionStrategy, Component, inject } from '@angular/core';
import type { MonitorSettings } from '../../types/appSettings';
import { AppSettingsStore } from '../app-settings.store';

/** 主窗右侧：监视器工具配置 */
@Component({
  selector: 'app-monitor-settings',
  templateUrl: './monitor-settings.html',
  changeDetection: ChangeDetectionStrategy.OnPush,
})
export class MonitorSettingsPanel {
  readonly appSettings = inject(AppSettingsStore);

  onCheck(key: keyof MonitorSettings, event: Event): void {
    this.appSettings.patchTool(
      'monitor',
      key,
      (event.target as HTMLInputElement).checked,
    );
  }
}
