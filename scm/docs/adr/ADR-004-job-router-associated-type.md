# ADR-004: Job→Router Wiring Enforced at the Trait Level

**Status:** Implemented
**Date:** 2026-08-24
**Relates to:** [#8](https://github.com/sweengineeringlabs/edge-proxy/issues/8) — Enforce Job→Router wiring at the trait level

---

## Mandate

`ProxyComposer::compose()` returns `ProxySvc`, a zero-field unit struct. Nothing — not the
compiler, not this crate's own tests, not the SEA arch-rule checker — could catch a
`ProxyComposer` implementation whose documented `pattern()` claimed one routing shape while
nothing actually wired up a matching `Job`/`Router` pair. `Job` requires a `Router` (#8): every
implementation must name a `Router` for its `Intent`, and expose it, as a compile-time witness
rather than a documentation convention.

---

## Contract change (breaking)

```rust
// Before
pub trait Job<Request = String, Response = String>: Send + Sync
where
    Request: Send + 'static,
    Response: Send + 'static,
{
    async fn run(&self, req: ExecutionRequest<'_, Request>) -> Result<JobResponse<Response>, JobError>;
    // as_null_job / as_null_job_marker unchanged
}

// After
pub trait Job<Request = String, Response = String>: Send + Sync
where
    Request: Send + 'static,
    Response: Send + 'static,
{
    type Intent: Send + 'static;
    type Router: Router<Self::Intent>;

    fn router(&self, req: RouterRequest) -> Result<RouterResponse<'_, Self::Router>, JobError>;

    async fn run(&self, req: ExecutionRequest<'_, Request>) -> Result<JobResponse<Response>, JobError>;
    // as_null_job / as_null_job_marker unchanged
}
```

`router()` follows this crate's existing `*Request` → `Result<*Response, JobError>` convention
(`api_method_takes_request` / `api_method_returns_result`) rather than a bare accessor — the same
shape `as_null_job`/`as_null_job_marker` already use.

---

## The erasure problem, and its resolution

`edge-proxy` had an existing pattern of erasing `Job`/`Router` behind `Arc<dyn Job<Request,
Response>>`/`Arc<dyn Router<Intent>>` — most visibly `api::NullJob`, previously a bare type alias
(`pub type NullJob = dyn Job<String, String>;`). Once `Job` requires associated types, forming a
`dyn Job<..>` value requires pinning every associated type — Rust does not allow eliding them.

Resolved by adding a blanket impl, `impl<Intent> Router<Intent> for Arc<dyn Router<Intent>>`
(`core/router/arc_router.rs`), and pinning every erasure site in this crate to
`Router = Arc<dyn Router<Intent>>`:

- `api::NullJob = dyn Job<String, String, Intent = String, Router = Arc<dyn Router<String>>>`
- `ProxySvc::new_null_job` / `ProxySvc::new_canonical_job` — same pin on their `Arc<dyn Job<..>>`
  return types
- `NullJob` (`core/job/null_job.rs`) and `CanonicalJobImpl` (`spi/canonical.rs`) — each now holds
  an `Arc<dyn Router<String>>` field (constructed from `NullRouter`/`CanonicalRouterImpl`
  respectively) instead of being a bare unit struct, since `router()` must return a real reference

A `Job` implementation that doesn't need runtime-swappable routing can instead name its own
router type directly as `type Router` (see the test doubles in `tests/job_int_test.rs`,
`tests/job_e2e_test.rs`, `tests/api_int_test.rs`, all using a local zero-sized `NoRouting`).

---

## Impact / scope of change

Touched, beyond what #8 originally scoped:

- `api/job/traits/job.rs` — the trait itself, plus its doctest (rewritten to avoid naming
  `router`-theme types directly per `type_theme_cohesion` — see below)
- `api/job/null_job.rs` — the `NullJob` alias, now pinning `Intent`/`Router`
- `api/job/dto/router_request.rs`, `api/job/dto/router_response.rs` — new DTOs (not anticipated
  by #8, required by this crate's own `api_method_takes_request`/`api_method_returns_result`
  arch rules once `router()` became a real trait method)
- `core/router/arc_router.rs` — new blanket `Arc<dyn Router<Intent>>` impl (not anticipated by
  #8; required to keep every existing erasure site compiling)
- `core/job/null_job.rs`, `spi/canonical.rs` — `NullJob`/`CanonicalJobImpl` gain a constructor
  and a router field
- `core/proxy/proxy_svc.rs` — `new_null_job`/`new_canonical_job` return-type pins (flagged in
  #8's review comment before implementation, confirmed necessary)
- `examples/dispatch.rs`, `tests/job_int_test.rs`, `tests/job_e2e_test.rs`,
  `tests/api_int_test.rs`, `tests/trait_object_safety_int_test.rs` — every `Job`/`dyn Job`
  site in this repo

Not touched: `LifecycleMonitor`, `Validator` — #8 explicitly scoped this to `Job`↔`Router` only.

## Verification

- `cargo build --all-targets`, `cargo test --all-targets` (52/52 binaries green), `cargo test
  --doc`, and `cargo clippy --all-targets` all clean.
- `arch audit --rs` diffed against a clean pre-change baseline: identical failing-rule set, zero
  regression (two rule violations surfaced mid-implementation — `type_theme_cohesion` on the
  doctest, `api_method_returns_result`/`api_method_takes_request` on the first `router()` draft
  — both fixed before landing, not carried as new debt).

## Not solved by this change

Structural enforcement only: the compiler now requires a `Router` to exist, not that it's the
*correct* one for what `pattern()`/`compose()` claim. That remains business logic no type system
verifies (#8's own stated limit).
