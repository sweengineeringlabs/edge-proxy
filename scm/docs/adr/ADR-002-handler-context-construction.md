# ADR-002: HandlerContext Construction at the Job Layer

**Status:** Accepted  
**Date:** 2026-06-15  
**Governing ADR:** [ADR-024](https://github.com/sweengineeringlabs/edge/blob/main/docs/3-architecture/adr/ADR-024-handler-execute-contract.md) — Handler::execute() Unified Contract  
**Relates to:** [ADR-001](./ADR-001-security-context-propagation.md) — Security context propagation

---

## Mandate

ADR-024 introduces `HandlerContext<'a>` — a per-request struct carrying `SecurityContext` and `&dyn CommandBus`. `Job` is the composition root where `HandlerContext` is constructed. No downstream layer constructs it.

---

## Contract change

```rust
// Before: Job forwarded SecurityContext separately
fn run(&self, req: Request, ctx: SecurityContext) -> BoxFuture<'_, Result<Response, JobError>>;

// After: Job constructs HandlerContext and passes it into dispatch
fn run(&self, req: Request, ctx: SecurityContext) -> BoxFuture<'_, Result<Response, JobError>>;
// internally:
//   let hctx = HandlerContext { security: ctx, commands: &*self.bus };
//   handler.execute(req, hctx).await
```

`Job::run` signature is **unchanged** — it still receives `SecurityContext` from ingress. The change is internal: `Job` constructs `HandlerContext` before delegating to the pipeline.

---

## Job owns the CommandBus

`Job` implementations hold `Arc<dyn CommandBus>` as a field, injected at construction by the bootstrap assembler:

```rust
pub struct DomainJob {
    router: Arc<dyn Router>,
    registry: Arc<dyn HandlerRegistry<Request = ..., Response = ...>>,
    bus: Arc<dyn CommandBus>,   // injected at construction
}

impl Job<Req, Resp> for DomainJob {
    fn run(&self, req: Req, security: SecurityContext) -> BoxFuture<'_, Result<Resp, JobError>> {
        Box::pin(async move {
            let intent = self.router.route(&req).await?;
            let handler = self.registry.get(&intent).ok_or(JobError::NotFound)?;
            let ctx = HandlerContext {
                security,
                commands: &*self.bus,
            };
            handler.execute(req, ctx).await.map_err(JobError::from)
        })
    }
}
```

---

## Bus selection is the assembler's responsibility

`Job` never names a concrete bus type. The bootstrap assembler selects and stacks bus implementations:

```rust
// bootstrap assembler
let bus: Arc<dyn CommandBus> = Arc::new(
    LoggingCommandBus::new(Arc::new(DirectCommandBus))
);
let job = DomainJob::new(router, registry, bus);
```

Swapping the bus stack requires no changes to `Job`, pipeline, or any handler.

---

## Boundary rules

**P1 — HandlerContext is constructed once per request.** At the `Job` layer, after `SecurityContext` is received from ingress. No downstream layer reconstructs it (ADR-024 S11).

**P2 — Job holds the bus; ingress does not.** Ingress transports build `SecurityContext` and call `Job::run`. They have no knowledge of `CommandBus`.

**P3 — NullJob passes a noop bus.** `NullJob` (bring-up and testing) constructs `HandlerContext` with `SecurityContext::unauthenticated()` and `NoopCommandBus`.
