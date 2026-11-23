import { Component, DestroyRef, inject } from '@angular/core';
import { RouterOutlet } from '@angular/router';
import { getResolvedAppTheme, onAppThemeChanged } from './lib/tauri/theme';
import type { AppThemeName } from './types/appSettings';

@Component({
  selector: 'app-root',
  imports: [RouterOutlet],
  template: '<router-outlet />',
})
export class AppComponent {
  private readonly destroyRef = inject(DestroyRef);

  constructor() {
    applyPlatformAttr();
    void syncAppTheme(this.destroyRef);
  }
}

function applyPlatformAttr(): void {
  document.documentElement.dataset['platform'] = /Mac/i.test(navigator.userAgent)
    ? 'mac'
    : 'win';
}

async function syncAppTheme(destroyRef: DestroyRef): Promise<void> {
  const applyTheme = (theme: AppThemeName): void => {
    document.documentElement.dataset['theme'] = theme;
  };

  try {
    applyTheme(await getResolvedAppTheme());
  } catch {
    /* 启动对齐失败时保持 CSS 默认 mocha */
  }
  const unlisten = await onAppThemeChanged(applyTheme);
  destroyRef.onDestroy(unlisten);
}
