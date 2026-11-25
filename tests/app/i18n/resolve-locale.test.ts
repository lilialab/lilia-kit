import assert from 'node:assert/strict';
import { describe, it } from 'node:test';
import { localeFromSystemTag, resolveLocale } from '../../../src/app/i18n/resolve-locale';
import { AppLocale, LocalePreference } from '../../../src/app/types/appSettings';

describe('localeFromSystemTag', () => {
  it('maps chinese to zh and others to en', () => {
    assert.equal(localeFromSystemTag('zh-CN'), AppLocale.Zh);
    assert.equal(localeFromSystemTag('zh_TW'), AppLocale.Zh);
    assert.equal(localeFromSystemTag('en-US'), AppLocale.En);
    assert.equal(localeFromSystemTag('ja-JP'), AppLocale.En);
  });
});

describe('resolveLocale', () => {
  it('honors fixed preference over system tag', () => {
    assert.equal(resolveLocale(LocalePreference.Zh, 'en-US'), AppLocale.Zh);
    assert.equal(resolveLocale(LocalePreference.En, 'zh-CN'), AppLocale.En);
    assert.equal(resolveLocale(LocalePreference.System, 'zh-CN'), AppLocale.Zh);
    assert.equal(resolveLocale(LocalePreference.System, 'fr-FR'), AppLocale.En);
  });
});
