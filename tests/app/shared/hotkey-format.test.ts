import assert from 'node:assert/strict';
import { describe, it } from 'node:test';
import {
  DEFAULT_MONITOR_VISIBILITY_HOTKEY,
  formatAccelDisplay,
  HOTKEY_PRESETS,
} from '../../../src/app/shared/hotkey-format';
import { AppPlatform } from '../../../src/app/types/appSettings';

describe('HOTKEY_PRESETS', () => {
  it('includes default monitor visibility hotkey', () => {
    assert.equal(DEFAULT_MONITOR_VISIBILITY_HOTKEY, 'Control+Alt+M');
    assert.ok(HOTKEY_PRESETS.includes(DEFAULT_MONITOR_VISIBILITY_HOTKEY));
  });
});

describe('formatAccelDisplay', () => {
  it('formats win and mac', () => {
    assert.equal(
      formatAccelDisplay('Control+Alt+M', AppPlatform.Win),
      'Ctrl+Alt+M',
    );
    assert.equal(
      formatAccelDisplay('Control+Alt+M', AppPlatform.Mac),
      '⌃⌥M',
    );
    assert.equal(
      formatAccelDisplay('Control+Shift+Period', AppPlatform.Win),
      'Ctrl+Shift+.',
    );
    assert.equal(formatAccelDisplay('', AppPlatform.Win), '');
  });
});
