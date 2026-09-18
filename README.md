# Panto API & Multi-Language Client SDKs

[![Multi-SDK CI](https://github.com/Erizeez/panto-api/actions/workflows/ci.yml/badge.svg)](https://github.com/Erizeez/panto-api/actions/workflows/ci.yml)
[![npm version](https://img.shields.io/npm/v/@erizeez/panto-api.svg)](https://www.npmjs.com/package/@erizeez/panto-api)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)

Official OpenAPI 3.0.3 specification and multi-language client SDKs (Go, TypeScript/JavaScript, Rust) for **Panto** (Programmable Adaptive Network Tunnel Orchestrator).

---

## 📦 Packages & Modules

| Language | Package / Module | Registry / Path | Installation |
| :--- | :--- | :--- | :--- |
| **Go** | `github.com/Erizeez/panto-api/go` | [pkg.go.dev](https://pkg.go.dev/github.com/Erizeez/panto-api/go) | `go get github.com/Erizeez/panto-api/go` |
| **TypeScript / JS** | `@erizeez/panto-api` | [npmjs.com](https://www.npmjs.com/package/@erizeez/panto-api) | `npm install @erizeez/panto-api` |
| **Rust** | `panto-api` | [crates.io](https://crates.io/crates/panto-api) / Git | `cargo add panto-api` |

---

## 🚀 Quick Start

### 1. TypeScript / JavaScript (Node.js, Bun, Deno, Browser)

```bash
npm install @erizeez/panto-api
```

```typescript
import { PantoClient } from '@erizeez/panto-api';

const client = new PantoClient({
  baseUrl: 'http://127.0.0.1:9090',
});

// 1. Get system status
const status = await client.getStatus();
console.log(`Running: ${status.running}, Uptime: ${status.uptime_seconds}s`);

// 2. Query Tailscale exit nodes
const { exit_nodes } = await client.getTailscaleExitNodes();
console.log('Available exit nodes:', exit_nodes);

// 3. Switch mode
await client.setMode({ mode: 'rule' });
```

---

### 2. Go

```bash
go get github.com/Erizeez/panto-api/go
```

```go
package main

import (
	"context"
	"fmt"
	"log"

	pantoapi "github.com/Erizeez/panto-api/go"
)

func main() {
	client, err := pantoapi.NewClientWithResponses("http://127.0.0.1:9090")
	if err != nil {
		log.Fatalf("failed to create client: %v", err)
	}

	resp, err := client.GetStatusWithResponse(context.Background())
	if err != nil {
		log.Fatalf("failed to get status: %v", err)
	}

	fmt.Printf("Panto is running: %v\n", resp.JSON200.Running)
}
```

---

### 3. Rust

In `Cargo.toml`:
```toml
[dependencies]
panto-api = { git = "https://github.com/Erizeez/panto-api.git", branch = "main" }
tokio = { version = "1.0", features = ["full"] }
```

```rust
use panto_api::PantoClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = PantoClient::new("http://127.0.0.1:9090")?;

    let status = client.get_status().await?;
    println!("Running: {}, Mixed Port: {}", status.running, status.mixed_port);

    let nodes = client.get_tailscale_exit_nodes().await?;
    println!("Found {} exit nodes", nodes.exit_nodes.len());

    Ok(())
}
```

---

## 🛠 Repository Structure

```
panto-api/
├── openapi.yaml           # Source of Truth: OpenAPI 3.0.3 Specification
├── go/                    # Go SDK (generated via oapi-codegen)
├── ts/                    # TypeScript SDK (typed fetch client, ESM)
├── rust/                  # Rust SDK (async client via reqwest + serde)
├── .github/workflows/
│   ├── ci.yml             # Continuous integration testing across all SDKs
│   └── publish-npm.yml    # Automated tag-driven release to npmjs.com
└── LICENSE                # MIT License
```

---

## 🚢 Publishing to npm

This repository includes automated CI/CD for publishing `@erizeez/panto-api` to the official npm registry:

1. Add your npm access token as a GitHub Actions secret named `NPM_TOKEN` (Settings -> Secrets and variables -> Actions -> New repository secret).
2. Create and push a git tag:
   ```bash
   git tag v0.1.0
   git push origin v0.1.0
   ```
3. GitHub Actions will automatically test, build, and publish `@erizeez/panto-api` to [npmjs.com](https://www.npmjs.com/package/@erizeez/panto-api).

---

## 📄 License

This repository is licensed under the [MIT License](LICENSE).
