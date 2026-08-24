# Changelog

## [0.4.0] — 2026-08-24

### Breaking
- `Job` gains two required associated items: `type Intent`, `type Router: Router<Self::Intent>`,
  and `fn router(&self, req: RouterRequest) -> Result<RouterResponse<'_, Self::Router>, JobError>`
  (#8). A compile-time witness that every `Job` implementation actually wires a `Router` for its
  intent, instead of `ProxyComposer::compose()`'s documentation being the only claim of it.
- `api::NullJob`'s `dyn Job<String, String>` alias is now pinned to
  `Intent = String, Router = Arc<dyn Router<String>>` — any code naming the alias directly (not
  just via `ProxySvc::new_null_job`) must match that erased-router shape.

### Added
- `RouterRequest` / `RouterResponse<'a, R>` — the new `Job::router` request/response pair,
  following this crate's existing `*Request`/`*Response` convention.
- Blanket `impl<Intent> Router<Intent> for Arc<dyn Router<Intent>>` — lets a `Job` pin
  `Router = Arc<dyn Router<Self::Intent>>` to keep its router runtime-swappable and erased,
  without naming a concrete router type in its own signature.

## [0.3.0] — 2026-06-13

### Breaking
- `Job::run` now accepts `SecurityContext` as second parameter (ADR-001).

### Added
- `SecurityContext` re-exported from `edge-security-runtime` via `edge_proxy::SecurityContext`.
- `LifecycleMonitor::status` — returns `HealthStatus` directly.
- `LifecycleMonitor::component` — returns a named component's health snapshot.

### Fixed
- All 14 pre-existing arch rule failures resolved (173/173).
