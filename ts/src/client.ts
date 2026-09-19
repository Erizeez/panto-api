import type {
  VersionResponse,
  StatusResponse,
  ModeResponse,
  SetModeRequest,
  ConfigResponse,
  TopologyGraph,
  GroupsResponse,
  SelectGroupRequest,
  SelectGroupResponse,
  GroupDelayRequest,
  GroupDelayResponse,
  RulesResponse,
  RuleMatchRequest,
  RuleMatchResponse,
  FlowsResponse,
  ProbeSitesResponse,
  ProbeTestRequest,
  ProbeTestResponse,
  TailscaleExitNodesResponse,
  SetTailscaleExitNodeRequest,
  TailscaleExitNodeResult,
  MagicIPPendingResponse,
  MagicIPChoicesResponse,
  MagicIPDecideRequest,
  MagicIPDecideResponse,
  MagicIPDeleteResponse,
  ObservationConsentItem,
  DecideConsentRequest,
  DecideConsentResponse,
  ObservationStatusResponse,
} from './types.js';

export interface PantoClientOptions {
  baseUrl?: string;
  fetch?: typeof fetch;
  headers?: Record<string, string>;
}

export class PantoApiError extends Error {
  readonly status: number;
  readonly body: unknown;

  constructor(status: number, message: string, body?: unknown) {
    super(`Panto API error ${status}: ${message}`);
    this.name = 'PantoApiError';
    this.status = status;
    this.body = body;
  }
}

/**
 * High-performance, strongly-typed Panto API Client.
 */
export class PantoClient {
  private readonly baseUrl: string;
  private readonly customFetch: typeof fetch;
  private readonly customHeaders: Record<string, string>;

  constructor(options: PantoClientOptions = {}) {
    this.baseUrl = (options.baseUrl || 'http://127.0.0.1:9090').replace(/\/+$/, '');
    this.customFetch = options.fetch || globalThis.fetch;
    this.customHeaders = options.headers || {};
  }

  private async request<T>(path: string, options: RequestInit = {}): Promise<T> {
    const url = `${this.baseUrl}/api/v1${path.startsWith('/') ? path : `/${path}`}`;
    const headers: Record<string, string> = {
      Accept: 'application/json',
      ...this.customHeaders,
      ...(options.headers as Record<string, string> | undefined),
    };

    if (options.body && typeof options.body === 'string' && !headers['Content-Type']) {
      headers['Content-Type'] = 'application/json';
    }

    const response = await this.customFetch(url, {
      ...options,
      headers,
    });

    if (!response.ok) {
      let errorMessage = response.statusText;
      let errorBody: unknown;
      try {
        errorBody = await response.json();
        if (errorBody && typeof errorBody === 'object' && 'error' in errorBody) {
          errorMessage = String((errorBody as { error: string }).error);
        }
      } catch {
        // Fall back to status text
      }
      throw new PantoApiError(response.status, errorMessage, errorBody);
    }

    return (await response.json()) as T;
  }

  // ── System ──────────────────────────────────────────────────────────
  async getVersion(): Promise<VersionResponse> {
    return this.request<VersionResponse>('/version');
  }

  async getStatus(): Promise<StatusResponse> {
    return this.request<StatusResponse>('/status');
  }

  async getMode(): Promise<ModeResponse> {
    return this.request<ModeResponse>('/mode');
  }

  async setMode(request: SetModeRequest): Promise<ModeResponse> {
    return this.request<ModeResponse>('/mode', {
      method: 'PUT',
      body: JSON.stringify(request),
    });
  }

  // ── Config ──────────────────────────────────────────────────────────
  async getConfig(): Promise<ConfigResponse> {
    return this.request<ConfigResponse>('/config');
  }

  // ── Topology & Routing ──────────────────────────────────────────────
  async getTopology(): Promise<TopologyGraph> {
    return this.request<TopologyGraph>('/topology');
  }

  async getGroups(): Promise<GroupsResponse> {
    return this.request<GroupsResponse>('/groups');
  }

  async selectGroupMember(
    groupId: string,
    request: SelectGroupRequest
  ): Promise<SelectGroupResponse> {
    return this.request<SelectGroupResponse>(`/groups/${encodeURIComponent(groupId)}/select`, {
      method: 'PUT',
      body: JSON.stringify(request),
    });
  }

  async testGroupDelay(
    groupId: string,
    request?: GroupDelayRequest
  ): Promise<GroupDelayResponse> {
    return this.request<GroupDelayResponse>(`/groups/${encodeURIComponent(groupId)}/delay`, {
      method: 'POST',
      body: request ? JSON.stringify(request) : undefined,
    });
  }

  // ── Rules ───────────────────────────────────────────────────────────
  async getRules(): Promise<RulesResponse> {
    return this.request<RulesResponse>('/rules');
  }

  async matchRule(request: RuleMatchRequest): Promise<RuleMatchResponse> {
    return this.request<RuleMatchResponse>('/rules/match', {
      method: 'POST',
      body: JSON.stringify(request),
    });
  }

  // ── Flows ───────────────────────────────────────────────────────────
  async getFlows(): Promise<FlowsResponse> {
    return this.request<FlowsResponse>('/flows');
  }

  // ── Probe Benchmark ─────────────────────────────────────────────────
  async getProbeSites(): Promise<ProbeSitesResponse> {
    return this.request<ProbeSitesResponse>('/probe/sites');
  }

  async testProbeSites(request?: ProbeTestRequest): Promise<ProbeTestResponse> {
    return this.request<ProbeTestResponse>('/probe/test', {
      method: 'POST',
      body: request ? JSON.stringify(request) : undefined,
    });
  }

  getProbeStreamUrl(options: { timeoutMs?: number; category?: string } = {}): string {
    const params = new URLSearchParams();
    if (options.timeoutMs) params.set('timeout_ms', String(options.timeoutMs));
    if (options.category) params.set('category', options.category);
    const qs = params.toString();
    return `${this.baseUrl}/api/v1/probe/stream${qs ? `?${qs}` : ''}`;
  }

  // ── Tailscale ───────────────────────────────────────────────────────
  async getTailscaleExitNodes(): Promise<TailscaleExitNodesResponse> {
    return this.request<TailscaleExitNodesResponse>('/tailscale/exit-nodes');
  }

  async setTailscaleExitNode(
    request: SetTailscaleExitNodeRequest
  ): Promise<TailscaleExitNodeResult> {
    return this.request<TailscaleExitNodeResult>('/tailscale/exit-nodes', {
      method: 'PUT',
      body: JSON.stringify(request),
    });
  }

  async getMagicIPPending(): Promise<MagicIPPendingResponse> {
    return this.request<MagicIPPendingResponse>('/tailscale/magic-ip/pending');
  }

  async getMagicIPChoices(): Promise<MagicIPChoicesResponse> {
    return this.request<MagicIPChoicesResponse>('/tailscale/magic-ip/choices');
  }

  async decideMagicIP(request: MagicIPDecideRequest): Promise<MagicIPDecideResponse> {
    return this.request<MagicIPDecideResponse>('/tailscale/magic-ip/decide', {
      method: 'POST',
      body: JSON.stringify(request),
    });
  }

  async deleteMagicIPChoice(ip: string): Promise<MagicIPDeleteResponse> {
    return this.request<MagicIPDeleteResponse>(
      `/tailscale/magic-ip/choices/${encodeURIComponent(ip)}`,
      {
        method: 'DELETE',
      }
    );
  }

  // ── Observation ─────────────────────────────────────────────────────
  async getObservationConsents(): Promise<ObservationConsentItem[]> {
    return this.request<ObservationConsentItem[]>('/observation/consents');
  }

  async decideObservationConsent(
    nodeId: string,
    request: DecideConsentRequest
  ): Promise<DecideConsentResponse> {
    return this.request<DecideConsentResponse>(
      `/observation/consents/${encodeURIComponent(nodeId)}/decide`,
      {
        method: 'POST',
        body: JSON.stringify(request),
      }
    );
  }

  async revokeObservationConsent(nodeId: string): Promise<DecideConsentResponse> {
    return this.request<DecideConsentResponse>(
      `/observation/consents/${encodeURIComponent(nodeId)}`,
      {
        method: 'DELETE',
      }
    );
  }

  async getObservationStatus(): Promise<ObservationStatusResponse> {
    return this.request<ObservationStatusResponse>('/observation/status');
  }
}
