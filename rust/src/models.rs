use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ErrorResponse {
    pub error: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct VersionResponse {
    pub version: String,
    pub git_commit: String,
    pub build_time: String,
    pub compiler: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct TrafficStats {
    pub upload_total: u64,
    pub download_total: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upload_rate_bps: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub download_rate_bps: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct StatusResponse {
    pub running: bool,
    pub mode: String,
    pub global_exit: String,
    pub uptime_seconds: u64,
    pub connections_count: usize,
    pub traffic: TrafficStats,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ModeResponse {
    pub mode: String,
    pub global_exit: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SetModeRequest {
    pub mode: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_exit: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TopologyNode {
    pub id: String,
    pub kind: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub underlay: Option<String>,
    pub effective_mtu: u32,
    pub overhead: u32,
    pub dependents: Vec<String>,
    pub path: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TopologyGraph {
    pub nodes: HashMap<String, TopologyNode>,
    pub root: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GroupItem {
    pub id: String,
    pub kind: String,
    pub members: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selected: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GroupsResponse {
    pub groups: Vec<GroupItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SelectGroupRequest {
    pub selected: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SelectGroupResponse {
    pub success: bool,
    pub group_id: String,
    pub selected: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct FlowRecord {
    pub id: String,
    pub proto: String,
    pub src: String,
    pub dst: String,
    pub target: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule: Option<String>,
    pub created_at_ms: u64,
    pub last_active_at_ms: u64,
    pub upload_bytes: u64,
    pub download_bytes: u64,
    pub packets: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct FlowsResponse {
    pub total: usize,
    pub flows: Vec<FlowRecord>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProbeSite {
    pub id: String,
    pub name: String,
    pub domain: String,
    pub port: u16,
    pub category: String,
    pub icon: String,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProbeSitesResponse {
    pub sites: Vec<ProbeSite>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct ProbeTestRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_ms: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_addr: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProbeResultItem {
    pub id: String,
    pub name: String,
    pub domain: String,
    pub category: String,
    pub latency_ms: u64,
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProbeTestResponse {
    pub total: usize,
    pub reachable: usize,
    pub failed: usize,
    pub results: Vec<ProbeResultItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ObservationConsentItem {
    pub node_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_name: Option<String>,
    pub url: String,
    pub status: String,
    pub requested_at_ms: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decided_at_ms: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DecideConsentRequest {
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DecideConsentResponse {
    pub success: bool,
    pub node_id: String,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PushResultRecord {
    pub last_pushed_at_ms: i64,
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    pub upload_bytes: u64,
    pub download_bytes: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ObservationStatusResponse {
    pub running: bool,
    pub active_nodes: usize,
    pub pending_nodes: usize,
    pub last_push_results: HashMap<String, PushResultRecord>,
}
