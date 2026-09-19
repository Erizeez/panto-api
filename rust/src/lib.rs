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
    fn test_client_instantiation() {
        let client = PantoClient::new("http://127.0.0.1:9090").unwrap();
        assert_eq!(client.endpoint("api/v1/status").unwrap().as_str(), "http://127.0.0.1:9090/api/v1/status");
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
    fn test_observation_consent_record() {
        let json = r#"{
            "node_id": "wg-jp",
            "provider_name": "Acme VPN",
            "url": "https://obs.acme.com/push",
            "status": "pending",
            "requested_at_ms": 1726700000000
        }"#;

        let item: ObservationConsentItem = serde_json::from_str(json).expect("should deserialize");
        assert_eq!(item.node_id, "wg-jp");
        assert_eq!(item.provider_name.as_deref(), Some("Acme VPN"));
        assert_eq!(item.status, "pending");
    }
}
