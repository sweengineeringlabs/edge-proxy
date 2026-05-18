# swe-edge-proxy

Dispatch facade for the `swe-edge` stack.

Defines the three core traits that every controller must implement — `Job`,
`Router`, and `LifecycleMonitor` — plus the error and health types they
exchange. No transport knowledge; no ingress or egress imports.

## Contracts

| Trait | Role |
|-------|------|
| `Job<Req, Resp>` | Single entry point — `run(req)` dispatches the full request/response cycle |
| `Router<Intent>` | Classifies input text into a domain-specific `Intent` |
| `LifecycleMonitor` | Health, background tasks, and graceful shutdown |

```rust
use async_trait::async_trait;
use edge_proxy::{Job, JobError};

struct EchoJob;

#[async_trait]
impl Job<String, String> for EchoJob {
    async fn run(&self, req: String) -> Result<String, JobError> {
        Ok(req)
    }
}
```

## Error types

| Type | Produced by |
|------|-------------|
| `JobError` | `Job::run` |
| `RoutingError` | `Router::route` |
| `LifecycleError` | `LifecycleMonitor::shutdown` |
| `HandlerError` | Forwarded from domain handlers |

## Health types

| Type | Purpose |
|------|---------|
| `HealthReport` | Aggregate health across all subsystems |
| `ComponentHealth` | Per-subsystem health with optional message |
| `HealthStatus` | `Healthy` / `Degraded` / `Unhealthy` |

## Default implementations

| Factory | Returns | Use case |
|---------|---------|---------|
| `new_null_lifecycle_monitor()` | `Arc<dyn LifecycleMonitor>` | Dev / test no-op |

## Building

```bash
cd proxy
cargo build
cargo test
cargo clippy -- -D warnings
```
