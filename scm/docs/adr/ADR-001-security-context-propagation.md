# ADR-001: Security Context Propagation — Job trait contract change

**Status:** Implemented (pending arch cleanup — 14 pre-existing rule failures block commit)  
**Date:** 2026-06-12  
**Governing ADR:** [ADR-017](https://github.com/sweengineeringlabs/edge/blob/main/docs/3-architecture/adr/ADR-017-security-context-propagation.md) — Security Context Propagation Pipeline

---

## Mandate

`Job::run` gains a `SecurityContext` second parameter. The proxy boundary now carries verified caller identity through to domain handlers.

---

## Contract change (breaking)

```rust
// Before
fn run(&self, req: Request) -> BoxFuture<'_, Result<Response, JobError>>;

// After
fn run(&self, req: Request, ctx: SecurityContext) -> BoxFuture<'_, Result<Response, JobError>>;
```

`SecurityContext` is from `edge-domain-security` via the `edge-domain/security` feature.

---

## Dependency change

```toml
edge-domain = { git = "https://github.com/sweengineeringlabs/edge-domain", features = ["security"], tag = "..." }
```

---

## What does not change

`Router::route()` and `HandlerRegistry::get()` — both remain context-free. Routing is pure classification; identity is a handler concern.

---

## Invariant (P2 from ADR-017)

`SecurityContext` is never `Option`. Anonymous requests pass `SecurityContext::unauthenticated()`. Absence is a typed state, not a missing argument.

---

## Cascade position

Step 8 of 11 in the ADR-017 migration cascade. Blocked on: dispatch (all Handler impls updated). Unblocks: ingress/http, ingress/grpc (context assembly).
