import assert from 'node:assert/strict';
import { describe, it } from 'node:test';
import {
  metricToneCss,
  temperatureTone,
  utilizationTone,
} from '../../../src/app/utils/metric-tone.ts';

describe('utilizationTone', () => {
  it('maps percent bands', () => {
    assert.equal(utilizationTone(0), 'idle');
    assert.equal(utilizationTone(49.9), 'idle');
    assert.equal(utilizationTone(50), 'ok');
    assert.equal(utilizationTone(69.9), 'ok');
    assert.equal(utilizationTone(70), 'busy');
    assert.equal(utilizationTone(84.9), 'busy');
    assert.equal(utilizationTone(85), 'hot');
    assert.equal(utilizationTone(94.9), 'hot');
    assert.equal(utilizationTone(95), 'critical');
    assert.equal(utilizationTone(120), 'critical');
    assert.equal(utilizationTone(-10), 'idle');
  });
});

describe('temperatureTone', () => {
  it('maps celsius bands', () => {
    assert.equal(temperatureTone(40), 'idle');
    assert.equal(temperatureTone(59.9), 'idle');
    assert.equal(temperatureTone(60), 'ok');
    assert.equal(temperatureTone(69.9), 'ok');
    assert.equal(temperatureTone(70), 'busy');
    assert.equal(temperatureTone(79.9), 'busy');
    assert.equal(temperatureTone(80), 'hot');
    assert.equal(temperatureTone(89.9), 'hot');
    assert.equal(temperatureTone(90), 'critical');
  });
});

describe('metricToneCss', () => {
  it('returns css vars for each tone', () => {
    assert.equal(metricToneCss('idle'), 'var(--app-tone-idle)');
    assert.equal(metricToneCss('critical'), 'var(--app-tone-critical)');
  });
});
