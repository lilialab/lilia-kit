import { Component, inject, signal } from '@angular/core';
import { TranslocoPipe } from '@jsverse/transloco';
import type { SettingsSectionId } from '../types/appSettings';
import { AppSettingsStore } from './app-settings.store';
import { GeneralSettingsPanel } from './components/general-settings';
import { MonitorSettingsPanel } from './components/monitor-settings';

@Component({
  selector: 'app-main-page',
  templateUrl: './main.page.html',
  providers: [AppSettingsStore],
  imports: [GeneralSettingsPanel, MonitorSettingsPanel, TranslocoPipe],
})
export class MainPage {
  readonly appSettings = inject(AppSettingsStore);
  readonly section = signal<SettingsSectionId>('general');

  selectSection(id: SettingsSectionId): void {
    this.section.set(id);
  }
}
