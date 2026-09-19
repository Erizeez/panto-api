/**
 * TypeScript definitions for Panto Control API.
 * Single source of truth corresponding to OpenAPI 3.0.3 specification.
 */

export interface ErrorResponse {
  error: string;
}

export interface VersionResponse {
  version: string;
  git_commit: string;
  build_time: string;
  compiler: string;
}

export interface TrafficStats {
  upload_total: number;
  download_total: number;
  upload_rate_bps?: number;
  download_rate_bps?: number;
}

export interface StatusResponse {
  running: boolean;
  mode: 'rule' | 'global' | 'direct' | string;
  global_exit: string;
  uptime_seconds: number;
  connections_count: number;
  traffic: TrafficStats;
}

export interface ModeResponse {
  mode: 'rule' | 'global' | 'direct' | string;
  global_exit: string;
}

export interface SetModeRequest {
  mode: 'rule' | 'global' | 'direct' | string;
  global_exit?: string;
}

export interface TopologyNode {
  id: string;
  kind: string;
  underlay?: string | null;
  effective_mtu: number;
  overhead: number;
  dependents: string[];
  path: string[];
}

export interface TopologyGraph {
  nodes: Record<string, TopologyNode>;
  root: string;
}

export interface GroupItem {
  id: string;
  kind: string;
  members: string[];
  selected?: string | null;
}

export interface GroupsResponse {
  groups: GroupItem[];
}

export interface SelectGroupRequest {
  selected: string;
}

export interface SelectGroupResponse {
  success: boolean;
  group_id: string;
  selected: string;
}

export interface FlowRecord {
  id: string;
  proto: string;
  src: string;
  dst: string;
  target: string;
  rule?: string | null;
  created_at_ms: number;
  last_active_at_ms: number;
  upload_bytes: number;
  download_bytes: number;
  packets: number;
}

export interface FlowsResponse {
  total: number;
  flows: FlowRecord[];
}

export interface ProbeSite {
  id: string;
  name: string;
  domain: string;
  port: number;
  category: string;
  icon: string;
  description: string;
}

export interface ProbeSitesResponse {
  sites: ProbeSite[];
}

export interface ProbeTestRequest {
  site_id?: string;
  category?: string;
  timeout_ms?: number;
  proxy_addr?: string;
}

export type ProbeStatus = 'ok' | 'timeout' | 'error' | 'blocked';

export interface ProbeResultItem {
  id: string;
  name: string;
  domain: string;
  category: string;
  latency_ms: number;
  status: ProbeStatus;
  error?: string | null;
}

export interface ProbeTestResponse {
  total: number;
  reachable: number;
  failed: number;
  results: ProbeResultItem[];
}

export type ConsentStatus = 'pending' | 'granted' | 'denied' | 'revoked';

export interface ObservationConsentItem {
  node_id: string;
  provider_name?: string | null;
  url: string;
  status: ConsentStatus;
  requested_at_ms: number;
  decided_at_ms?: number | null;
}

export interface DecideConsentRequest {
  status: 'granted' | 'denied';
}

export interface DecideConsentResponse {
  success: boolean;
  node_id: string;
  status: string;
}

export interface PushResultRecord {
  last_pushed_at_ms: number;
  success: boolean;
  message?: string | null;
  upload_bytes: number;
  download_bytes: number;
}

export interface ObservationStatusResponse {
  running: boolean;
  active_nodes: number;
  pending_nodes: number;
  last_push_results: Record<string, PushResultRecord>;
}
