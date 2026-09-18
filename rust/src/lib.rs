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
        let stream_url = client.probe_stream_url(Some(3000)).unwrap();
        assert_eq!(
            stream_url.as_str(),
            "http://127.0.0.1:9090/api/v1/probe/stream?timeout_ms=3000"
        );
    }

    #[test]
    fn test_models_serialization() {
        let req = SetModeRequest {
            mode: "rule".to_string(),
            global_target: None,
        };
        let json = serde_json::to_string(&req).unwrap();
        assert_eq!(json, r#"{"mode":"rule"}"#);
    }
}
