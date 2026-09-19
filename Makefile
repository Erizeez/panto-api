.PHONY: all generate generate-go test test-go test-ts test-rust build clean

all: generate build test

generate: generate-go

generate-go:
	@echo "==> Generating Go SDK from openapi.yaml"
	go run github.com/oapi-codegen/oapi-codegen/v2/cmd/oapi-codegen@v2.4.1 -package pantoapi -generate types -o go/types.gen.go openapi.yaml
	go run github.com/oapi-codegen/oapi-codegen/v2/cmd/oapi-codegen@v2.4.1 -package pantoapi -generate client -o go/client.gen.go openapi.yaml

test: test-go test-ts test-rust

test-go:
	@echo "==> Testing Go SDK"
	cd go && go test -v ./...

test-ts:
	@echo "==> Building and Testing TypeScript SDK"
	cd ts && npm test

test-rust:
	@echo "==> Testing Rust SDK"
	cd rust && cargo test

build:
	@echo "==> Building all SDKs"
	cd go && go build ./...
	cd ts && npm run build
	cd rust && cargo build

clean:
	rm -rf ts/dist ts/node_modules rust/target
