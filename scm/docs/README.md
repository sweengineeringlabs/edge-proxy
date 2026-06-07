# edge-proxy

## WHAT

The L2 dispatch facade for swe-edge — sits between ingress and the domain layer; classifies input
via `Router`, dispatches to `Handler` via `Job`, and manages lifecycle via `LifecycleMonitor`.

Key capabilities:

- **`Job<Req, Resp>`** — core dispatch trait: single entry point called by ingress; no transport knowledge
- **`Router<Intent>`** — classifies input text into domain-specific intents (handler IDs)
- **`LifecycleMonitor`** — manages health, background tasks, and graceful shutdown; aggregates component health into `HealthReport`
- **`ProxySvc`** — SAF factory: `new_null_lifecycle_monitor()`, `new_router()`, `new_job()` — constructs the dispatch chain
- **`JobError`** / **`RoutingError`** / **`LifecycleError`** — typed error enums; `JobError` bridges from domain `HandlerError`
- **`HealthReport`** / **`ComponentHealth`** / **`HealthStatus`** — per-subsystem health aggregation for readiness probes

## WHY

| Problem | Solution |
|---------|----------|
| Ingress transports (HTTP, gRPC) directly invoke handler registries, coupling transport to domain | `Job` trait is the single entry point; ingress only knows `Job::run(req)` |
| Request routing logic embedded in each ingress handler | `Router` trait separates routing from transport; routing is testable without spinning up a server |
| Health checks implemented differently per service | `LifecycleMonitor` aggregates subsystem health into a typed `HealthReport`; readiness probes call one method |
| Null lifecycle implementations for tests repeat boilerplate | `ProxySvc::new_null_lifecycle_monitor()` returns a noop impl; no boilerplate per test |
| Diamond dep conflicts when proxy contract types change | One crate, one tag — all consumers pin the same version; kgraph detects conflicts pre-commit |
