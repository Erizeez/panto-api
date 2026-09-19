import test from 'node:test';
import assert from 'node:assert/strict';
import { PantoClient } from '../dist/index.js';

test('PantoClient can be instantiated with custom baseUrl', () => {
  const client = new PantoClient({ baseUrl: 'http://127.0.0.1:19090/' });
  assert.equal(typeof client.getVersion, 'function');
  assert.equal(typeof client.getStatus, 'function');
  assert.equal(typeof client.getMode, 'function');
  assert.equal(typeof client.setMode, 'function');
  assert.equal(typeof client.getTopology, 'function');
  assert.equal(typeof client.getGroups, 'function');
  assert.equal(typeof client.selectGroupMember, 'function');
  assert.equal(typeof client.getFlows, 'function');
  assert.equal(typeof client.getProbeSites, 'function');
  assert.equal(typeof client.testProbeSites, 'function');
  assert.equal(typeof client.getObservationConsents, 'function');
  assert.equal(typeof client.decideObservationConsent, 'function');
  assert.equal(typeof client.revokeObservationConsent, 'function');
  assert.equal(typeof client.getObservationStatus, 'function');
});

test('PantoClient performs mock request and parses nested response', async () => {
  const mockStatus = {
    running: true,
    mode: 'rule',
    global_exit: 'DIRECT',
    uptime_seconds: 120,
    connections_count: 5,
    traffic: {
      upload_total: 1024,
      download_total: 4096,
      upload_rate_bps: 128,
      download_rate_bps: 256,
    },
  };

  const mockFetch = async (url, options) => {
    assert.equal(url, 'http://127.0.0.1:9090/api/v1/status');
    return {
      ok: true,
      status: 200,
      json: async () => mockStatus,
    };
  };

  const client = new PantoClient({
    baseUrl: 'http://127.0.0.1:9090',
    fetch: mockFetch,
  });

  const status = await client.getStatus();
  assert.equal(status.running, true);
  assert.equal(status.mode, 'rule');
  assert.equal(status.traffic.upload_total, 1024);
  assert.equal(status.traffic.download_total, 4096);
});

test('PantoClient correctly calls observation consent decide endpoint', async () => {
  let calledUrl = '';
  let calledMethod = '';
  let calledBody = '';

  const mockFetch = async (url, options) => {
    calledUrl = url;
    calledMethod = options.method;
    calledBody = options.body;
    return {
      ok: true,
      status: 200,
      json: async () => ({ success: true, node_id: 'wg-jp', status: 'granted' }),
    };
  };

  const client = new PantoClient({
    baseUrl: 'http://127.0.0.1:9090',
    fetch: mockFetch,
  });

  const resp = await client.decideObservationConsent('wg-jp', { status: 'granted' });
  assert.equal(calledUrl, 'http://127.0.0.1:9090/api/v1/observation/consents/wg-jp/decide');
  assert.equal(calledMethod, 'POST');
  assert.equal(calledBody, JSON.stringify({ status: 'granted' }));
  assert.equal(resp.success, true);
  assert.equal(resp.node_id, 'wg-jp');
});
