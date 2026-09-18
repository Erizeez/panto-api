use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct VersionResponse {
    pub version: String,
    pub os: String,
    pub arch: String,
    pub compiler: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StatusResponse {
    pub running: bool,
    pub uptime_seconds: f64,
    pub mixed_port: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assigned_ip: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_target: Option<String>,
    pub active_endpoints: usize,
    pub active_groups: usize,
    pub active_rules: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ModeResponse {
    pub mode: String,
    pub global_target: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SetModeRequest {
    pub mode: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_target: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TailscaleExitNodeItem {
    pub id: String,
    pub name: String,
    pub ip: String,
    pub online: bool,
    pub active: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TailscaleExitNodesResponse {
    pub exit_nodes: Vec<TailscaleExitNodeItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SetTailscaleExitNodeRequest {
    pub node_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TailscaleExitNodeResult {
    pub active_node: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MagicIPCandidate {
    pub ip: String,
    pub device_name: String,
    pub source: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MagicIPConflictItem {
    pub conflict_ip: String,
    pub candidates: Vec<MagicIPCandidate>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MagicIPPendingResponse {
    pub conflicts: Vec<MagicIPConflictItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MagicIPChoiceItem {
    pub ip: String,
    pub chosen_device: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MagicIPChoicesResponse {
    pub choices: Vec<MagicIPChoiceItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MagicIPDecideRequest {
    pub conflict_ip: String,
    pub chosen_device: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MagicIPDecideResponse {
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MagicIPDeleteResponse {
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TopologyNode {
    pub id: String,
    pub kind: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub underlay: Option<String>,
    pub effective_mtu: i32,
    pub overhead: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dependents: Option<Vec<String>>,
    pub path: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TopologyResponse {
    pub nodes: Vec<TopologyNode>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GroupItem {
    pub id: String,
    pub kind: String,
    pub members: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delays: Option<HashMap<String, u32>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GroupsResponse {
    pub groups: Vec<GroupItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SelectMemberRequest {
    pub selected: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SelectMemberResponse {
    pub id: String,
    pub current: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq)]
pub struct DelayTestRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_ms: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DelayTestResponse {
    pub delays: HashMap<String, u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuleItem {
    pub id: i64,
    pub r#type: String,
    pub payload: String,
    pub target: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RulesResponse {
    pub rules: Vec<RuleItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuleMatchRequest {
    pub host: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<u16>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuleMatchResponse {
    pub matched: bool,
    pub rule_target: String,
    pub selected_endpoint: String,
    pub chain: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TrafficResponse {
    pub up_bytes: i64,
    pub down_bytes: i64,
    pub up_rate_bps: i64,
    pub down_rate_bps: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ConfigResponse {
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ErrorResponse {
    pub error: String,
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

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq)]
pub struct ProbeTestRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_ms: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProbeResultItem {
    pub id: String,
    pub domain: String,
    pub rule_target: String,
    pub selected_endpoint: String,
    pub chain: Vec<String>,
    pub latency_ms: i32,
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProbeTestResponse {
    pub results: Vec<ProbeResultItem>,
}
