package pantoapi_test

import (
	"encoding/json"
	"testing"

	pantoapi "github.com/Erizeez/panto-api/go"
)

func TestNewClient(t *testing.T) {
	client, err := pantoapi.NewClientWithResponses("http://127.0.0.1:9090")
	if err != nil {
		t.Fatalf("expected no error, got: %v", err)
	}
	if client == nil {
		t.Fatal("expected non-nil client")
	}
}

func TestModelSerialization(t *testing.T) {
	// 1. StatusResponse with nested TrafficStats
	statusJSON := `{
		"running": true,
		"mode": "rule",
		"global_exit": "DIRECT",
		"uptime_seconds": 3600,
		"connections_count": 12,
		"traffic": {
			"upload_total": 10240,
			"download_total": 20480,
			"upload_rate_bps": 512,
			"download_rate_bps": 1024
		}
	}`
	var status pantoapi.StatusResponse
	if err := json.Unmarshal([]byte(statusJSON), &status); err != nil {
		t.Fatalf("failed to unmarshal StatusResponse: %v", err)
	}
	if !status.Running || status.Mode != "rule" || status.GlobalExit != "DIRECT" {
		t.Errorf("unexpected status fields: %+v", status)
	}
	if status.Traffic.UploadTotal != 10240 || status.Traffic.DownloadTotal != 20480 {
		t.Errorf("unexpected traffic totals: %+v", status.Traffic)
	}

	// 2. ObservationConsentItem
	consentJSON := `{
		"node_id": "wg-jp",
		"provider_name": "TestProvider",
		"url": "https://obs.provider.com/usage",
		"status": "pending",
		"requested_at_ms": 1726700000000
	}`
	var consent pantoapi.ObservationConsentItem
	if err := json.Unmarshal([]byte(consentJSON), &consent); err != nil {
		t.Fatalf("failed to unmarshal ObservationConsentItem: %v", err)
	}
	if consent.NodeId != "wg-jp" || consent.Status != pantoapi.ObservationConsentItemStatusPending {
		t.Errorf("unexpected consent fields: %+v", consent)
	}

	// 3. FlowRecord
	flowJSON := `{
		"id": "flow-123",
		"proto": "TCP",
		"src": "127.0.0.1:12345",
		"dst": "1.1.1.1:443",
		"target": "wg-jp",
		"created_at_ms": 1726700000000,
		"last_active_at_ms": 1726700005000,
		"upload_bytes": 500,
		"download_bytes": 1200,
		"packets": 10
	}`
	var flow pantoapi.FlowRecord
	if err := json.Unmarshal([]byte(flowJSON), &flow); err != nil {
		t.Fatalf("failed to unmarshal FlowRecord: %v", err)
	}
	if flow.Id != "flow-123" || flow.Target != "wg-jp" {
		t.Errorf("unexpected flow fields: %+v", flow)
	}
}
