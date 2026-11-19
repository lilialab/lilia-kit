import { Component, DestroyRef, inject, signal } from '@angular/core';
import {
  getToolsState,
  onToolsChanged,
  setMonitorEnabled,
  type ToolsState,
} from '../lib/tauri/tools';

@Component({
  selector: 'app-main-page',
  templateUrl: './main.page.html',
})
export class MainPage {
  private readonly destroyRef = inject(DestroyRef);

  readonly monitorEnabled = signal(false);
  readonly screenshotAvailable = signal(false);
  readonly clipboardAvailable = signal(false);
  readonly error = signal<string | null>(null);
  readonly ready = signal(false);

  constructor() {
    void getToolsState()
      .then((state) => {
        this.applyState(state);
        this.ready.set(true);
      })
      .catch((err: unknown) => {
        this.error.set(err instanceof Error ? err.message : String(err));
        this.ready.set(true);
      });

    void onToolsChanged((state) => this.applyState(state)).then((unlisten) => {
      this.destroyRef.onDestroy(unlisten);
    });
  }

  async toggleMonitor(): Promise<void> {
    const next = !this.monitorEnabled();
    this.error.set(null);
    try {
      await setMonitorEnabled(next);
      this.monitorEnabled.set(next);
    } catch (err: unknown) {
      this.error.set(formatError(err));
    }
  }

  private applyState(state: ToolsState): void {
    this.monitorEnabled.set(state.monitorEnabled);
    this.screenshotAvailable.set(state.screenshotAvailable);
    this.clipboardAvailable.set(state.clipboardAvailable);
  }
}

function formatError(err: unknown): string {
  if (err instanceof Error) {
    return err.message;
  }
  if (typeof err === 'string') {
    return err;
  }
  if (err && typeof err === 'object' && 'message' in err) {
    return String((err as { message: unknown }).message);
  }
  return String(err);
}
