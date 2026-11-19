import type { CanMatchFn, Routes } from '@angular/router';
import { getCurrentWindowLabel } from './lib/tauri/window';
import { MainPage } from './main/main.page';
import { MonitorPage } from './monitor/monitor.page';

function matchLabel(label: string): CanMatchFn {
  return () => getCurrentWindowLabel() === label;
}

export const routes: Routes = [
  { path: '', pathMatch: 'full', canMatch: [matchLabel('main')], component: MainPage },
  { path: '', pathMatch: 'full', canMatch: [matchLabel('monitor')], component: MonitorPage },
];
