# edge-proxy

L2 Proxy — dispatch facade sitting between ingress and domain.

Implements the 5-Concern Controller pattern: **Job**, **Routing**, **Lifecycle**, **Validation**, and composition via **`ProxyComposer`**.

## Usage

```toml
edge-proxy = { git = "https://github.com/sweengineeringlabs/edge-proxy", tag = "v0.3.4" }
```

## Architecture

| Layer | Directory | Purpose |
|-------|-----------|---------|
| L1 | `api/` | Public port contracts (traits, types, errors) |
| L2 | `core/` | Default implementations (`pub(crate)`) |
| SAF | `saf/` | Public factory facade |
| SPI | `spi/` | Extension points (`CanonicalFactory`) |

See `docs/3-design/architecture.md` for the full dependency graph and `examples/dispatch.rs` for the canonical dispatch path.
