# Proxy Architecture

## Workspace overview

The proxy workspace is a single Rust crate — `swe-edge-proxy` — that defines the dispatch
contracts connecting ingress adapters to domain handlers. It has no knowledge of transport
protocols (HTTP, gRPC) or external dependencies (databases, queues).

| Crate | Package | Purpose |
|-------|---------|---------|
| `proxy` | `swe-edge-proxy` | Dispatch facade — `Job`, `Router`, `LifecycleMonitor` contracts |

---

## SEA module layout

```
src/
├── api/
│   ├── job.rs               # Job<Req, Resp> — single dispatch entry point
│   ├── router.rs            # Router<Intent> — classifies input into domain intents
│   ├── lifecycle_monitor.rs # LifecycleMonitor — health, shutdown
│   ├── health.rs            # HealthReport, ComponentHealth, HealthStatus
│   ├── job_error.rs         # JobError
│   ├── routing_error.rs     # RoutingError
│   ├── lifecycle_error.rs   # LifecycleError
│   └── traits.rs            # SEA interface contract
├── core/
│   └── null_lifecycle_monitor.rs  # No-op LifecycleMonitor for dev/test
├── saf/
│   └── mod.rs               # new_null_lifecycle_monitor() factory
└── lib.rs                   # pub use saf::*
```

---

## Dispatch flow

```
Inbound request
      │
      ▼
  Job::run(req)
      │
      ├── Router::route(req)  ──→  Intent (handler ID + params)
      │
      ├── HandlerRegistry::get(id)  ──→  Arc<dyn Handler>
      │
      └── Handler::execute(req)  ──→  Result<Resp, HandlerError>
```

`Job` is the single public entry point. `Router` classifies the request; `HandlerRegistry`
(from `domain/`) resolves the handler. The proxy crate owns the orchestration contract;
concrete implementations live in `runtime/`.

---

## Key contracts

| Type | Role |
|------|------|
| `Job<Req, Resp>` | Entry point — `run(req)` drives the full dispatch cycle |
| `Router<Intent>` | Classifies input text into a domain-specific `Intent` |
| `LifecycleMonitor` | Health reporting, background tasks, graceful shutdown |
| `HealthReport` | Aggregate health across all registered subsystems |
| `HealthStatus` | `Healthy` / `Degraded` / `Unhealthy` |

---

## See Also

- [Architecture Overview](../../docs/3-architecture/architecture.md)
- [Domain Architecture](../../domain/docs/architecture.md)
- [Runtime Architecture](../../runtime/docs/architecture.md)
- [Developer Guide](../../docs/4-development/developer_guide.md)
