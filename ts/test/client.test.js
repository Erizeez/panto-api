import test from 'node:test';
import assert from 'node:assert/strict';
import { PantoClient } from '../dist/index.js';

test('PantoClient can be instantiated with custom baseUrl', () => {
  const client = new PantoClient({ baseUrl: 'http://127.0.0.1:19090/' });
  assert.equal(typeof client.getVersion, 'function');
  assert.equal(typeof client.getMode, 'function');
  assert.equal(typeof client.getTailscaleExitNodes, 'function');
  assert.equal(typeof client.getProbeSites, 'function');
});

test('PantoClient generates correct probe stream URL', () => {
  const client = new PantoClient({ baseUrl: 'http://127.0.0.1:9090' });
  const url = client.getProbeStreamUrl(5000);
  assert.equal(url, 'http://127.0.0.1:9090/api/v1/probe/stream?timeout_ms=5000');
});
