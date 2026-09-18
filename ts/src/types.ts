/**
 * TypeScript definitions for Panto Control API.
 * Generated from OpenAPI 3.0.3 specification.
 */

export interface VersionResponse {
  version: string;
  os: string;
  arch: string;
  compiler: string;
}

export interface StatusResponse {
  running: boolean;
  uptime_seconds: number;
  mixed_port: number;
  assigned_ip?: string;
  mode?: 'rule' | 'direct' | 'global' | string;
  global_target?: string;
  active_endpoints: number;
  active_groups: number;
  active_rules: number;
}

export interface ModeResponse {
  mode: 'rule' | 'direct' | 'global' | string;
  global_target: string;
}

export interface SetModeRequest {
  mode: 'rule' | 'direct' | 'global' | string;
  global_target?: string;
}

export interface TailscaleExitNodeItem {
  id: string;
  name: string;
  ip: string;
  online: boolean;
  active: boolean;
  location?: string;
}

export interface TailscaleExitNodesResponse {
  exit_nodes: TailscaleExitNodeItem[];
}

export interface SetTailscaleExitNodeRequest {
  node_id: string;
}

export interface TailscaleExitNodeResult {
  active_node: string;
}

export interface MagicIPCandidate {
  ip: string;
  device_name: string;
  source: string;
}

export interface MagicIPConflictItem {
  conflict_ip: string;
  candidates: MagicIPCandidate[];
}

export interface MagicIPPendingResponse {
  conflicts: MagicIPConflictItem[];
}

export interface MagicIPChoiceItem {
  ip: string;
  chosen_device: string;
}

export interface MagicIPChoicesResponse {
  choices: MagicIPChoiceItem[];
}

export interface MagicIPDecideRequest {
  conflict_ip: string;
  chosen_device: string;
}

export interface MagicIPDecideResponse {
  status: string;
}

export interface MagicIPDeleteResponse {
  status: string;
}

export interface TopologyNode {
  id: string;
  kind: string;
  underlay?: string;
  effective_mtu: number;
  overhead: number;
  dependents?: string[];
  path: string[];
}

export interface TopologyResponse {
  nodes: TopologyNode[];
}

export interface GroupItem {
  id: string;
  kind: 'select' | 'url-test' | 'fallback' | 'load-balance' | string;
  members: string[];
  current?: string;
  delays?: Record<string, number>;
}

export interface GroupsResponse {
  groups: GroupItem[];
}

export interface SelectMemberRequest {
  selected: string;
}

export interface SelectMemberResponse {
  id: string;
  current: string;
}

export interface DelayTestRequest {
  url?: string;
  timeout_ms?: number;
}

export interface DelayTestResponse {
  delays: Record<string, number>;
}

export interface RuleItem {
  id: number;
  type: string;
  payload: string;
  target: string;
}

export interface RulesResponse {
  rules: RuleItem[];
}

export interface RuleMatchRequest {
  host: string;
  port?: number;
}

export interface RuleMatchResponse {
  matched: boolean;
  rule_target: string;
  selected_endpoint: string;
  chain: string[];
}

export interface TrafficResponse {
  up_bytes: number;
  down_bytes: number;
  up_rate_bps: number;
  down_rate_bps: number;
}

export interface ConfigResponse {
  content: string;
}

export interface ErrorResponse {
  error: string;
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
  timeout_ms?: number;
}

export interface ProbeResultItem {
  id: string;
  domain: string;
  rule_target: string;
  selected_endpoint: string;
  chain: string[];
  latency_ms: number;
  status: string;
  error?: string;
}

export interface ProbeTestResponse {
  results: ProbeResultItem[];
}
