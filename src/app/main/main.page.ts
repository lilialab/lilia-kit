import { Component, inject } from '@angular/core';
import { AppSettingsStore } from './app-settings.store';
import { MonitorSettingsPanel } from './components/monitor-settings';

@Component({
  selector: 'app-main-page',
  templateUrl: './main.page.html',
  providers: [AppSettingsStore],
  imports: [MonitorSettingsPanel],
})
export class MainPage {
  readonly appSettings = inject(AppSettingsStore);
}
