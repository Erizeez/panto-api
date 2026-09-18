package pantoapi_test

import (
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
