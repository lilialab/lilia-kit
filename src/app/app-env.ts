import { DestroyRef, inject } from '@angular/core';
import { patchState, signalStore, withHooks, withMethods, withState } from '@ngrx/signals';
import { getResolvedAppTheme, onAppThemeChanged } from './lib/tauri/theme';
import {
  AppPlatform,
  type AppPlatform as AppPlatformId,
  AppTheme,
  type AppThemeName,
} from './types/appSettings';

const platform: AppPlatformId = /Mac/i.test(navigator.userAgent)
  ? AppPlatform.Mac
  : AppPlatform.Win;

/** 壳层环境：平台与 resolved 主题（同步 `documentElement` dataset 供 CSS）。 */
export const AppEnv = signalStore(
  { providedIn: 'root' },
  withState<{ platform: AppPlatformId; theme: AppThemeName }>({
    platform,
    theme: AppTheme.Mocha,
  }),
  withMethods((store) => ({
    applyTheme(theme: AppThemeName): void {
      patchState(store, { theme });
      document.documentElement.dataset['theme'] = theme;
    },
  })),
  withHooks({
    onInit(store) {
      const destroyRef = inject(DestroyRef);
      document.documentElement.dataset['platform'] = store.platform();

      void (async () => {
        try {
          store.applyTheme(await getResolvedAppTheme());
        } catch {
          /* 启动对齐失败时保持 CSS / state 默认 mocha */
        }
        const unlisten = await onAppThemeChanged((theme) => store.applyTheme(theme));
        destroyRef.onDestroy(unlisten);
      })();
    },
  }),
);
