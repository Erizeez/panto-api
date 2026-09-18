.PHONY: all test test-go test-ts test-rust build clean

all: test

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
