import { computed, DestroyRef, inject, Injectable, signal } from '@angular/core';
import { loadAppSettings, onAppSettingsChanged, saveAppSettings } from '../lib/tauri/settings';
import type { AppSettings } from '../types/appSettings';

/**
 * 主窗设置会话：草稿 ↔ 已保存，还原 / 保存。
 * 写盘与启用/置顶/主题等副作用在 Rust；换肤由 `app-theme-changed` 驱动。
 * 由 `MainPage` `providers` 提供，勿挂 root。
 */
@Injectable()
export class AppSettingsStore {
  private readonly destroyRef = inject(DestroyRef);

  /** 正在编辑的草稿 */
  readonly draft = signal<AppSettings | null>(null);
  /** 上次保存（或外部回推）的基线 */
  readonly saved = signal<AppSettings | null>(null);
  readonly error = signal<string | null>(null);
  readonly ready = signal(false);
  readonly saving = signal(false);

  readonly dirty = computed(() => {
    const draft = this.draft();
    const saved = this.saved();
    return draft !== null && saved !== null && !sameSettings(draft, saved);
  });

  constructor() {
    void loadAppSettings()
      .then((settings) => {
        this.resetSession(settings);
        this.ready.set(true);
      })
      .catch((err: unknown) => {
        this.error.set(toErrorMessage(err));
        this.ready.set(true);
      });

    void onAppSettingsChanged((settings) => {
      if (!this.saving()) {
        this.resetSession(settings);
      }
    }).then((unlisten) => this.destroyRef.onDestroy(unlisten));
  }

  patchTool<T extends keyof AppSettings, K extends keyof AppSettings[T]>(
    tool: T,
    key: K,
    value: AppSettings[T][K],
  ): void {
    const draft = this.draft();
    if (!draft) {
      return;
    }
    this.draft.set({
      ...draft,
      [tool]: { ...draft[tool], [key]: value },
    });
  }

  restore(): void {
    const saved = this.saved();
    if (!saved || !this.dirty() || this.saving()) {
      return;
    }
    this.draft.set(structuredClone(saved));
  }

  async save(): Promise<void> {
    const draft = this.draft();
    if (!draft || !this.dirty() || this.saving()) {
      return;
    }
    this.saving.set(true);
    this.error.set(null);
    try {
      this.resetSession(await saveAppSettings(draft));
    } catch (err: unknown) {
      this.error.set(toErrorMessage(err));
    } finally {
      this.saving.set(false);
    }
  }

  /** 用完整 settings 重置草稿与基线（互不共享引用） */
  private resetSession(settings: AppSettings): void {
    this.draft.set(structuredClone(settings));
    this.saved.set(structuredClone(settings));
  }
}

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
