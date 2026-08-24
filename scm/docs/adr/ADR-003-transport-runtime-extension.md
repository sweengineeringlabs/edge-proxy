# ADR-003: Transport Runtime Extension — `ProxyRuntime` SPI over Pingora / Hyper / Tower / Axum

**Status:** Proposed
**Date:** 2026-08-24
**Relates to:** [architecture.md](../3-design/architecture.md) — "What this repo is" (no transport knowledge), [#8](https://github.com/sweengineeringlabs/edge-proxy/issues/8) — Job→Router wiring enforcement, [#9](https://github.com/sweengineeringlabs/edge-proxy/issues/9) — dependency-direction layering discipline

---

## Mandate

`edge-proxy` today is a dispatch **facade** only: `Job`, `Router`, `LifecycleMonitor`, `Validator`,
`ProxyComposer` are contracts with no transport knowledge — `architecture.md` states this as a
design tenet ("no transport knowledge of its own (no ingress/egress imports)") and asserts this
repo cannot see what runtime holds `Arc<dyn Job<Request, Response>>` or what sits upstream of it.

Consequence: nothing in the swe-edge ecosystem actually binds a `Job` to a real listener. Every
consumer must hand-roll its own transport wiring against whichever HTTP/L4/L7 stack it picks, with
no shared, tested binding and no guarantee that binding respects the SEA layering this crate
otherwise enforces (`no_foreign_type`, `api/` → `core/` → `saf/` → `spi/`).

**Decision:** extend `edge-proxy` with a fifth concern, `ProxyRuntime`, that owns the transport
boundary — binding a listener, translating wire requests into `ExecutionRequest`, and returning
`JobResponse`/`JobError` back over the wire. Multiple runtime backends are supported as SPI
implementations behind Cargo features, unified under one `api/`-defined trait, with a dependency-free
**`DefaultProxy`** as the always-on native implementation.

This reverses the "no transport knowledge" tenet as previously stated. `architecture.md` must be
updated alongside this ADR's implementation (see "Rollout" below).

---

## Decision

### New concern: `ProxyRuntime`

```rust
// api/proxy/traits/proxy_runtime.rs
pub trait ProxyRuntime {
    /// Bind the runtime to a listen target and start serving.
    /// `job` is the dispatch entry point every inbound request is routed through.
    async fn serve(
        &self,
        req: ServeRequest<'_>,
        job: Arc<dyn Job<String, String>>,
    ) -> Result<ServeResponse, ProxyRuntimeError>;

    /// Stable identity name for this runtime backend (`"default"`, `"pingora"`, `"hyper"`, ...).
    fn runtime_name(&self) -> &'static str;
}
```

`ServeRequest`/`ServeResponse`/`ProxyRuntimeError` are this crate's own DTOs/errors (`api/proxy/dto`,
`api/proxy/errors`) — per the existing `no_foreign_type` rule, the trait signature never names a
Pingora, Hyper, Tower, or Axum type directly. Backend-specific configuration (TLS, connection
pooling, worker counts, ...) is threaded through `ServeRequest`'s fields or a backend-specific config
struct passed by value, never by exposing the backend's own config type in the trait.

### `DefaultProxy` — native implementation

`core/proxy/default_proxy.rs`. Built on what's already in this crate's dependency graph
(`std`/`tokio` primitives only — no new required dependency) — a minimal HTTP/1.1 listener sufficient
to drive `Job::run` end to end. Always compiled, no feature flag, exactly like `NullJob` /
`NoopValidator` today. It is the reference implementation and the one this crate can support without
taking on an external transport ecosystem's release cadence, feature surface, or CVE exposure by
default.

`DefaultProxy` is explicitly **not** meant to be production-grade for high-throughput ingress — it is
the bring-up/reference/testing default, same role `NullJob`/`CanonicalJobImpl` play for `Job`.
Production deployments are expected to select a backed-in SPI runtime.

### SPI runtime backends

Each lives in `spi/proxy/<backend>.rs`, each an optional dependency gated behind a same-named Cargo
feature, each implementing `ProxyRuntime`:

| Backend | Type | Feature flag | Optional dependency |
|---|---|---|---|
| `PingoraProxy` | `spi/proxy/pingora.rs` | `pingora` | `pingora` |
| `HyperProxy` | `spi/proxy/hyper.rs` | `hyper` | `hyper` |
| `TowerProxy` | `spi/proxy/tower.rs` | `tower` | `tower` |
| `AxumProxy` | `spi/proxy/axum.rs` | `axum` | `axum` |

```toml
[dependencies]
pingora = { version = "...", optional = true }
hyper   = { version = "...", optional = true }
tower   = { version = "...", optional = true }
axum    = { version = "...", optional = true }

[features]
default  = []
pingora  = ["dep:pingora"]
hyper    = ["dep:hyper"]
tower    = ["dep:tower"]
axum     = ["dep:axum"]
```

None of the four are pulled in unless a consumer opts in. `DefaultProxy` needs no feature — it is
part of `core/` and ships in every build, matching this crate's existing null-object convention of
having a working default with zero configuration.

### Selection point

`ProxyComposer::compose()` (or a new `ProxySvc::new_default_runtime()` /
`ProxySvc::new_pingora_runtime()`-style factory family, mirroring `new_null_job` /
`new_canonical_job`) returns `Arc<dyn ProxyRuntime>`. Backend selection is the composer/bootstrap
assembler's responsibility — `ProxyRuntime`'s own trait surface carries no knowledge of which
concrete backend it is.

---

## SEA layering placement

```
api/proxy/traits/proxy_runtime.rs   — ProxyRuntime trait (new)
api/proxy/dto/                      — ServeRequest, ServeResponse (new)
api/proxy/errors/                   — ProxyRuntimeError (new)
core/proxy/default_proxy.rs         — DefaultProxy, native impl (new)
spi/proxy/pingora.rs                — PingoraProxy, behind `pingora` feature (new)
spi/proxy/hyper.rs                  — HyperProxy, behind `hyper` feature (new)
spi/proxy/tower.rs                  — TowerProxy, behind `tower` feature (new)
spi/proxy/axum.rs                   — AxumProxy, behind `axum` feature (new)
saf/proxy/                          — ProxyRuntime re-export + PROXY_RUNTIME_CONCERN /
                                       PROXY_RUNTIME_SVC_FACTORY identity constants (new)
```

This is additive to the existing block diagram in `architecture.md` — no existing trait
(`Job`/`Router`/`LifecycleMonitor`/`Validator`/`ProxyComposer`) changes shape. `saf/` continues to
never import `spi/` directly, consistent with today's rule that a consumer of this crate need not
know `spi/` exists unless it opts into a specific backend feature.

---

## Boundary rules

**P1 — `api/` never names a foreign transport type.** No `pingora::*`, `hyper::*`, `tower::*`, or
`axum::*` type appears in `ProxyRuntime`'s signature or in any `api/` DTO. This is the same
`no_foreign_type` discipline `saf/` already applies to `HandlerContext`/`CommandBus`/`SecurityContext`
— those are re-exported through `saf/`, never named in `api/`; foreign transport types must not
appear in `api/` at all, not even via `saf/` re-export, since they are backend-specific and
feature-gated rather than universal.

**P2 — `DefaultProxy` has zero optional dependencies.** It must build and pass tests with no Cargo
features enabled. If a future requirement cannot be met without pulling in an external crate,
`DefaultProxy` does not gain that requirement — a new SPI backend does.

**P3 — one backend, one feature, one file.** No feature enables more than one backend module; no
backend module compiles without its feature. `#[cfg(feature = "...")]` gates the `mod` declaration
in `spi/proxy/mod.rs`, not just the impl internals, so disabled backends contribute zero code and
zero transitive dependency resolution.

**P4 — `ProxyRuntime` does not replace `Job`.** The runtime's only job is binding a listener and
calling `job.run(...)`; request routing, validation, and lifecycle remain the existing four concerns'
responsibility. `ProxyRuntime` must not duplicate `Router`/`Validator` logic internally.

---

## Explicitly out of scope

- **No default backend preference beyond `DefaultProxy`.** This ADR does not declare Pingora,
  Hyper, Tower, or Axum as "the recommended" backend — that is a per-consumer choice based on their
  own throughput/latency/ecosystem needs. This crate stays neutral among the four.
- **No feature-flag combination testing matrix beyond CI running each feature in isolation plus
  the zero-feature default.** Testing all C(4,2)+ combinations of simultaneously-enabled backend
  features is not required by this ADR; each backend is independent and additive.
- **No change to `Job`'s trait surface.** This ADR is transport-layer only; it does not touch
  [#8](https://github.com/sweengineeringlabs/edge-proxy/issues/8)'s proposed `Job`-owns-`Router`
  associated type — that remains a separate decision.

---

## Rollout

1. Add `ProxyRuntime` trait + DTOs/errors to `api/proxy/` (P1 above must hold from the first commit).
2. Implement `DefaultProxy` in `core/`, with tests exercising a real bind-and-serve round trip against
   an in-process `Job` (happy path, connection-refused/bind-failure sad path, malformed-request edge
   case).
3. Add one SPI backend at a time (`pingora` first — matches this crate's `service_type = "proxy"`
   metadata and the ecosystem's most proxy-shaped tool), each as its own PR: Cargo feature, `spi/`
   module, tests gated on that feature via CI matrix (`cargo test --no-default-features`,
   `cargo test --features pingora`, `cargo test --features hyper`, etc.).
4. Update `architecture.md`: replace the "no transport knowledge of its own" framing with the new
   `ProxyRuntime` concern, add it to the block diagram, dataflow diagram, and concern list ("5-Concern
   Controller pattern" becomes 6, or `ProxyRuntime` is documented as a 2nd SAF factory concern
   alongside `ProxyComposer` — reconcile at implementation time based on which reads more accurately).
5. Version bump: additive trait + new optional deps is at minimum a minor bump (`0.3.x` → `0.4.0`);
   treat as major if `arch audit --rs` reclassifies the crate's `service_type` metadata as a result.

## Acceptance criteria

- [ ] `cargo build --no-default-features` succeeds with zero optional transport dependencies pulled in
- [ ] `cargo build --features pingora`, `--features hyper`, `--features tower`, `--features axum`
      each succeed in isolation
- [ ] `DefaultProxy` has a passing happy-path, sad-path, and edge-case test with no features enabled
- [ ] No `pingora::`/`hyper::`/`tower::`/`axum::` symbol appears anywhere under `api/`
- [ ] `arch audit --rs` does not regress from its current baseline for any feature combination in CI
- [ ] `architecture.md` updated to reflect the new concern and dependency graph
