import { ChangeDetectionStrategy, Component, inject } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { TranslocoPipe } from '@jsverse/transloco';
import {
  Appearance,
  type AppearancePreference,
  LocalePreference,
  type LocalePreference as LocalePreferenceId,
} from '../../types/appSettings';
import { AppSettingsStore } from '../app-settings.store';

const LOCALE_OPTIONS: {
  value: LocalePreferenceId;
  labelKey: string;
  hintKey: string;
}[] = [
  {
    value: LocalePreference.System,
    labelKey: 'general.locale.system',
    hintKey: 'general.locale.systemHint',
  },
  {
    value: LocalePreference.Zh,
    labelKey: 'general.locale.zh',
    hintKey: 'general.locale.zhHint',
  },
  {
    value: LocalePreference.En,
    labelKey: 'general.locale.en',
    hintKey: 'general.locale.enHint',
  },
];

const APPEARANCE_OPTIONS: {
  value: AppearancePreference;
  labelKey: string;
  hintKey: string;
}[] = [
  {
    value: Appearance.System,
    labelKey: 'general.appearance.system',
    hintKey: 'general.appearance.systemHint',
  },
  {
    value: Appearance.Latte,
    labelKey: 'general.appearance.latte',
    hintKey: 'general.appearance.latteHint',
  },
  {
    value: Appearance.Mocha,
    labelKey: 'general.appearance.mocha',
    hintKey: 'general.appearance.mochaHint',
  },
];

/** 主窗右侧：通用设置 */
@Component({
  selector: 'app-general-settings',
  templateUrl: './general-settings.html',
  imports: [FormsModule, TranslocoPipe],
  changeDetection: ChangeDetectionStrategy.OnPush,
})
export class GeneralSettingsPanel {
  readonly appSettings = inject(AppSettingsStore);
  readonly localeOptions = LOCALE_OPTIONS;
  readonly appearanceOptions = APPEARANCE_OPTIONS;

  onLaunchAtLoginChange(value: boolean): void {
    this.appSettings.patchTool('general', 'launchAtLogin', value);
  }

  onLocaleChange(value: LocalePreferenceId): void {
    this.appSettings.patchTool('general', 'locale', value);
  }

  onAppearanceChange(value: AppearancePreference): void {
    this.appSettings.patchTool('general', 'appearance', value);
  }
}
