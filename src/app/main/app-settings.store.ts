import { computed, DestroyRef, inject } from '@angular/core';
import {
  patchState,
  signalStore,
  withComputed,
  withHooks,
  withMethods,
  withState,
} from '@ngrx/signals';
import { loadAppSettings, onAppSettingsChanged, saveAppSettings } from '../lib/tauri/settings';
import type { AppSettings } from '../types/appSettings';

interface SettingsSessionState {
  draft: AppSettings | null;
  saved: AppSettings | null;
  error: string | null;
  ready: boolean;
  saving: boolean;
}

/**
 * 主窗设置会话：草稿 ↔ 已保存，还原 / 保存。
 * 写盘与启用/置顶等副作用在 Rust；壳层主题/语言由 `app-env-changed` 驱动。
 * 由 `MainPage` `providers` 提供，勿挂 root。
 */
export const AppSettingsStore = signalStore(
  withState<SettingsSessionState>({
    draft: null,
    saved: null,
    error: null,
    ready: false,
    saving: false,
  }),
  withComputed(({ draft, saved }) => ({
    dirty: computed(() => {
      const d = draft();
      const s = saved();
      return d !== null && s !== null && !sameSettings(d, s);
    }),
  })),
  withMethods((store) => {
    const resetSession = (settings: AppSettings): void => {
      patchState(store, {
        draft: structuredClone(settings),
        saved: structuredClone(settings),
      });
    };

    return {
      patchTool<T extends keyof AppSettings, K extends keyof AppSettings[T]>(
        tool: T,
        key: K,
        value: AppSettings[T][K],
      ): void {
        const draft = store.draft();
        if (!draft) {
          return;
        }
        patchState(store, {
          draft: {
            ...draft,
            [tool]: { ...draft[tool], [key]: value },
          },
        });
      },

      restore(): void {
        const saved = store.saved();
        if (!saved || !store.dirty() || store.saving()) {
          return;
        }
        patchState(store, { draft: structuredClone(saved) });
      },

      async save(): Promise<void> {
        const draft = store.draft();
        if (!draft || !store.dirty() || store.saving()) {
          return;
        }
        patchState(store, { saving: true, error: null });
        try {
          resetSession(await saveAppSettings(draft));
        } catch (err: unknown) {
          patchState(store, { error: toErrorMessage(err) });
        } finally {
          patchState(store, { saving: false });
        }
      },
    };
  }),
  withHooks({
    onInit(store) {
      const destroyRef = inject(DestroyRef);

      void loadAppSettings()
        .then((settings) => {
          patchState(store, {
            draft: structuredClone(settings),
            saved: structuredClone(settings),
            ready: true,
          });
        })
        .catch((err: unknown) => {
          patchState(store, {
            error: toErrorMessage(err),
            ready: true,
          });
        });

      void onAppSettingsChanged((settings) => {
        if (store.saving()) {
          return;
        }
        patchState(store, {
          draft: structuredClone(settings),
          saved: structuredClone(settings),
        });
      }).then((unlisten) => destroyRef.onDestroy(unlisten));
    },
  }),
);

function sameSettings(a: AppSettings, b: AppSettings): boolean {
  return JSON.stringify(a) === JSON.stringify(b);
}

function toErrorMessage(err: unknown): string {
  if (typeof err === 'string') {
    return err;
  }
  if (err instanceof Error) {
    return err.message;
  }
  return String(err);
}
