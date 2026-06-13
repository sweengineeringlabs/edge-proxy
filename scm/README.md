# edge-proxy

L2 Proxy — dispatch facade sitting between ingress and domain.

Implements the 5-Concern Controller pattern: **Job**, **Routing**, **Lifecycle**, **Validation**, and **Gateway**.

## Usage

```toml
edge-proxy = { git = "https://github.com/sweengineeringlabs/edge-proxy", tag = "v0.3.0" }
```

## Architecture

| Layer | Directory | Purpose |
|-------|-----------|---------|
| L1 | `api/` | Public port contracts (traits, types, errors) |
| L2 | `core/` | Default implementations (`pub(crate)`) |
| SAF | `saf/` | Public factory facade |
| Gateway | `gateway/` | Boundary re-export surface |

See `examples/dispatch.rs` for the canonical dispatch path.
