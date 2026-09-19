//! Panto Control API Client SDK for Rust
//!
//! Provides strongly-typed models and an asynchronous HTTP client to interact
//! with the Panto Network Tunnel Orchestrator.

pub mod client;
pub mod errors;
pub mod models;

pub use client::PantoClient;
pub use errors::{Error, Result};
pub use models::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_instantiation_and_probe_stream_url() {
        let client = PantoClient::new("http://127.0.0.1:9090").unwrap();
        assert_eq!(client.endpoint("api/v1/status").unwrap().as_str(), "http://127.0.0.1:9090/api/v1/status");

        let stream_url = client.probe_stream_url(Some(5000), Some("ai")).unwrap();
        assert_eq!(
            stream_url.as_str(),
            "http://127.0.0.1:9090/api/v1/probe/stream?timeout_ms=5000&category=ai"
        );
    }

    #[test]
    fn test_models_serialization() {
        let req = SetModeRequest {
            mode: "rule".to_string(),
            global_exit: None,
        };
        let json = serde_json::to_string(&req).unwrap();
        assert_eq!(json, r#"{"mode":"rule"}"#);
    }

    #[test]
    fn test_status_response_nested_traffic() {
        let json = r#"{
            "running": true,
            "mode": "rule",
            "global_exit": "DIRECT",
            "uptime_seconds": 3600,
            "connections_count": 8,
            "traffic": {
                "upload_total": 5120,
                "download_total": 10240,
                "upload_rate_bps": 256,
                "download_rate_bps": 512
            }
        }"#;

        let status: StatusResponse = serde_json::from_str(json).expect("should deserialize");
        assert!(status.running);
        assert_eq!(status.mode, "rule");
        assert_eq!(status.traffic.upload_total, 5120);
        assert_eq!(status.traffic.download_total, 10240);
        assert_eq!(status.traffic.upload_rate_bps, Some(256));
    }

    #[test]
    fn test_rules_and_config_models() {
        let config_json = r#"{"content":"version: 1.0\nmode: rule\n","version":"1.0"}"#;
        let config_resp: ConfigResponse = serde_json::from_str(config_json).unwrap();
        assert_eq!(config_resp.version.as_deref(), Some("1.0"));

        let rule_json = r#"{"matched":true,"rule":"DOMAIN-SUFFIX,google.com,Global-Proxy","target":"Global-Proxy"}"#;
        let match_resp: RuleMatchResponse = serde_json::from_str(rule_json).unwrap();
        assert!(match_resp.matched);
        assert_eq!(match_resp.target.as_deref(), Some("Global-Proxy"));
    }

    #[test]
    fn test_tailscale_models() {
        let json = r#"{
            "exit_nodes": [
                {
                    "id": "node-1",
                    "name": "jp-exit",
                    "ip": "100.64.0.1",
                    "online": true,
                    "active": true,
                    "location": "Tokyo"
                }
            ]
        }"#;
        let resp: TailscaleExitNodesResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.exit_nodes.len(), 1);
        assert_eq!(resp.exit_nodes[0].name, "jp-exit");
        assert!(resp.exit_nodes[0].active);
    }
}
