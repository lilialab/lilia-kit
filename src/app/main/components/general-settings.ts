import { ChangeDetectionStrategy, Component, inject } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { Appearance, type AppearancePreference } from '../../types/appSettings';
import { AppSettingsStore } from '../app-settings.store';

const APPEARANCE_OPTIONS: { value: AppearancePreference; label: string; hint: string }[] = [
  {
    value: Appearance.System,
    label: '跟随系统',
    hint: '浅色 → Latte，深色 → Mocha',
  },
  { value: Appearance.Latte, label: 'Latte', hint: '浅色主题' },
  { value: Appearance.Mocha, label: 'Mocha', hint: '深色主题' },
];

/** 主窗右侧：通用设置 */
@Component({
  selector: 'app-general-settings',
  templateUrl: './general-settings.html',
  imports: [FormsModule],
  changeDetection: ChangeDetectionStrategy.OnPush,
})
export class GeneralSettingsPanel {
  readonly appSettings = inject(AppSettingsStore);
  readonly appearanceOptions = APPEARANCE_OPTIONS;

  onAppearanceChange(value: AppearancePreference): void {
    this.appSettings.patchTool('general', 'appearance', value);
  }

  onLaunchAtLoginChange(value: boolean): void {
    this.appSettings.patchTool('general', 'launchAtLogin', value);
  }
}
