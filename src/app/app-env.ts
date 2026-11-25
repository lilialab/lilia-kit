import { DestroyRef, inject } from '@angular/core';
import { TranslocoService } from '@jsverse/transloco';
import { patchState, signalStore, withHooks, withMethods, withState } from '@ngrx/signals';
import { resolveLocale } from './i18n/resolve-locale';
import { getResolvedAppEnv, onAppEnvChanged, type ResolvedAppEnv } from './lib/tauri/env';
import {
  AppLocale,
  type AppLocale as AppLocaleId,
  AppPlatform,
  type AppPlatform as AppPlatformId,
  AppTheme,
  type AppThemeName,
} from './types/appSettings';

const platform: AppPlatformId = /Mac/i.test(navigator.userAgent)
  ? AppPlatform.Mac
  : AppPlatform.Win;

/** 壳层环境：平台、resolved 主题 / 语言（权威在 Rust `app-env-changed`）。 */
export const AppEnv = signalStore(
  { providedIn: 'root' },
  withState<{ platform: AppPlatformId; theme: AppThemeName; locale: AppLocaleId }>({
    platform,
    theme: AppTheme.Mocha,
    // 启动瞬间兜底；随后以 getResolvedAppEnv / 事件为准
    locale: resolveLocale('system', navigator.language),
  }),
  withMethods((store) => {
    const transloco = inject(TranslocoService);

    const applyPlatform = (platform: AppPlatformId): void => {
      patchState(store, { platform });
      document.documentElement.dataset['platform'] = platform;
    };

    const applyLocale = (locale: AppLocaleId): void => {
      patchState(store, { locale });
      transloco.setActiveLang(locale);
      document.documentElement.lang = locale === AppLocale.Zh ? 'zh-CN' : 'en';
      document.documentElement.dataset['locale'] = locale;
    };

    const applyTheme = (theme: AppThemeName): void => {
      patchState(store, { theme });
      document.documentElement.dataset['theme'] = theme;
    };

    return {
      applyPlatform,
      applyTheme,
      applyLocale,

      /** 应用 Rust 推送的 resolved 快照 */
      applyResolved(env: ResolvedAppEnv): void {
        applyTheme(env.theme);
        applyLocale(env.locale);
      },
    };
  }),
  withHooks({
    onInit(store) {
      const destroyRef = inject(DestroyRef);
      store.applyPlatform(store.platform());
      store.applyTheme(store.theme());
      store.applyLocale(store.locale());

      async function syncFromRust(): Promise<void> {
        try {
          store.applyResolved(await getResolvedAppEnv());
        } catch {
          /* 保持启动兜底 */
        }
        const unlisten = await onAppEnvChanged((env) => store.applyResolved(env));
        destroyRef.onDestroy(unlisten);
      }

      void syncFromRust();
    },
  }),
);
