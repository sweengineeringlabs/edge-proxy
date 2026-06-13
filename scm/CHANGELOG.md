# Changelog

## [0.3.0] — 2026-06-13

### Breaking
- `Job::run` now accepts `SecurityContext` as second parameter (ADR-001).

### Added
- `SecurityContext` re-exported from `edge-domain` via `edge_proxy::SecurityContext`.
- `LifecycleMonitor::status` — returns `HealthStatus` directly.
- `LifecycleMonitor::component` — returns a named component's health snapshot.

### Fixed
- All 14 pre-existing arch rule failures resolved (173/173).
