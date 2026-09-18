import type {
  VersionResponse,
  StatusResponse,
  ModeResponse,
  SetModeRequest,
  TailscaleExitNodesResponse,
  SetTailscaleExitNodeRequest,
  TailscaleExitNodeResult,
  MagicIPPendingResponse,
  MagicIPChoicesResponse,
  MagicIPDecideRequest,
  MagicIPDecideResponse,
  MagicIPDeleteResponse,
  TopologyResponse,
  GroupsResponse,
  GroupItem,
  SelectMemberRequest,
  SelectMemberResponse,
  DelayTestRequest,
  DelayTestResponse,
  RulesResponse,
  RuleMatchRequest,
  RuleMatchResponse,
  TrafficResponse,
  ConfigResponse,
  ProbeSitesResponse,
  ProbeTestRequest,
  ProbeTestResponse,
} from './types.js';

export interface PantoClientOptions {
  baseUrl?: string;
  fetch?: typeof fetch;
  headers?: Record<string, string>;
}

/**
 * Panto API Client for Node.js, Browsers, and Deno/Bun runtimes.
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
    const url = `${this.baseUrl}${path.startsWith('/') ? path : `/${path}`}`;
    const headers = {
      'Accept': 'application/json',
      'Content-Type': 'application/json',
      ...this.customHeaders,
      ...options.headers,
    };

    const res = await this.customFetch(url, {
      ...options,
      headers,
    });

    if (!res.ok) {
      let msg = `HTTP error ${res.status} ${res.statusText}`;
      try {
        const body = await res.json();
        if (body && typeof body.error === 'string') {
          msg = body.error;
        }
      } catch {
        // ignore json parse error
      }
      throw new Error(msg);
    }

    return (await res.json()) as T;
  }

  /** 获取核心版本信息 */
  async getVersion(): Promise<VersionResponse> {
    return this.request<VersionResponse>('/api/v1/version');
  }

  /** 获取系统运行状态 */
  async getStatus(): Promise<StatusResponse> {
    return this.request<StatusResponse>('/api/v1/status');
  }

  /** 获取当前运行模式与全局出口 */
  async getMode(): Promise<ModeResponse> {
    return this.request<ModeResponse>('/api/v1/mode');
  }

  /** 切换运行模式与全局出口 */
  async setMode(req: SetModeRequest): Promise<ModeResponse> {
    return this.request<ModeResponse>('/api/v1/mode', {
      method: 'PUT',
      body: JSON.stringify(req),
    });
  }

  /** 获取 Tailnet 中所有可供消费的远端 Exit Node 列表 */
  async getTailscaleExitNodes(): Promise<TailscaleExitNodesResponse> {
    return this.request<TailscaleExitNodesResponse>('/api/v1/tailscale/exit-nodes');
  }

  /** 设置消费的远端 Exit Node（空字符串表示停用） */
  async setTailscaleExitNode(req: SetTailscaleExitNodeRequest): Promise<TailscaleExitNodeResult> {
    return this.request<TailscaleExitNodeResult>('/api/v1/tailscale/exit-nodes', {
      method: 'PUT',
      body: JSON.stringify(req),
    });
  }

  /** 获取当前待处理的 Magic IP 冲突列表 */
  async getMagicIPPending(): Promise<MagicIPPendingResponse> {
    return this.request<MagicIPPendingResponse>('/api/v1/tailscale/magic-ip/pending');
  }

  /** 获取用户已做出的 Magic IP 路由裁决历史记录 */
  async getMagicIPChoices(): Promise<MagicIPChoicesResponse> {
    return this.request<MagicIPChoicesResponse>('/api/v1/tailscale/magic-ip/choices');
  }

  /** 提交特定冲突 IP 的设备归属裁决 */
  async decideMagicIP(req: MagicIPDecideRequest): Promise<MagicIPDecideResponse> {
    return this.request<MagicIPDecideResponse>('/api/v1/tailscale/magic-ip/decide', {
      method: 'POST',
      body: JSON.stringify(req),
    });
  }

  /** 撤销或清除指定 IP 的历史路由裁决 */
  async deleteMagicIPChoice(ip: string): Promise<MagicIPDeleteResponse> {
    return this.request<MagicIPDeleteResponse>('/api/v1/tailscale/magic-ip/decide', {
      method: 'DELETE',
      body: JSON.stringify({ ip }),
    });
  }

  /** 获取链路有向图拓扑 */
  async getTopology(): Promise<TopologyResponse> {
    return this.request<TopologyResponse>('/api/v1/topology');
  }

  /** 获取全部策略分组 */
  async getGroups(): Promise<GroupsResponse> {
    return this.request<GroupsResponse>('/api/v1/groups');
  }

  /** 获取单个策略组详情 */
  async getGroup(id: string): Promise<GroupItem> {
    return this.request<GroupItem>(`/api/v1/groups/${encodeURIComponent(id)}`);
  }

  /** 切换选择组的活跃端点 */
  async selectGroupMember(id: string, req: SelectMemberRequest): Promise<SelectMemberResponse> {
    return this.request<SelectMemberResponse>(`/api/v1/groups/${encodeURIComponent(id)}/select`, {
      method: 'PUT',
      body: JSON.stringify(req),
    });
  }

  /** 对策略组内所有节点并发测速 */
  async testGroupDelay(id: string, req?: DelayTestRequest): Promise<DelayTestResponse> {
    return this.request<DelayTestResponse>(`/api/v1/groups/${encodeURIComponent(id)}/delay`, {
      method: 'POST',
      body: req ? JSON.stringify(req) : undefined,
    });
  }

  /** 获取全部规则列表 */
  async getRules(): Promise<RulesResponse> {
    return this.request<RulesResponse>('/api/v1/rules');
  }

  /** 模拟规则匹配测试 */
  async matchRule(req: RuleMatchRequest): Promise<RuleMatchResponse> {
    return this.request<RuleMatchResponse>('/api/v1/rules/match', {
      method: 'POST',
      body: JSON.stringify(req),
    });
  }

  /** 获取当前流量速率统计 */
  async getTraffic(): Promise<TrafficResponse> {
    return this.request<TrafficResponse>('/api/v1/traffic');
  }

  /** 获取原始配置文件内容 */
  async getConfig(): Promise<ConfigResponse> {
    return this.request<ConfigResponse>('/api/v1/config');
  }

  /** 获取预置全球常见测试网站列表 */
  async getProbeSites(): Promise<ProbeSitesResponse> {
    return this.request<ProbeSitesResponse>('/api/v1/probe/sites');
  }

  /** 对单个或全部预置网站执行分流解析与连通性测速 */
  async testProbeSites(req?: ProbeTestRequest): Promise<ProbeTestResponse> {
    return this.request<ProbeTestResponse>('/api/v1/probe/test', {
      method: 'POST',
      body: req ? JSON.stringify(req) : undefined,
    });
  }

  /** 获取实时流式并发测速 SSE URL */
  getProbeStreamUrl(timeoutMs?: number): string {
    const params = timeoutMs ? `?timeout_ms=${timeoutMs}` : '';
    return `${this.baseUrl}/api/v1/probe/stream${params}`;
  }
}
