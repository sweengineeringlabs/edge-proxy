# edge-proxy Architecture

**Audience:** Developers and architects working in this repo, and any agent (human or AI)
picking up dispatch-facade work here.

This is the entry point for understanding how `edge-proxy` is structured internally and what it
depends on. It synthesizes the ADRs in `docs/adr/` and the crate's own public surface
(`main/src/lib.rs`); it does not duplicate either, it points at them.

---

## What this repo is

`edge-proxy` (package `edge-proxy`, crate root `edge_proxy`) is a single-crate, SEA-compliant
(`api/` → `core/` → `saf/` → `spi/`) Rust library defining the L2 dispatch facade for swe-edge — the
contract layer between an inbound runtime/transport and the domain layer, with **no transport
knowledge of its own** (no ingress/egress imports).

It declares four contract traits, each one concern of what its own source calls the "5-Concern
Controller pattern":

- **`Job<Request, Response>`** — the single entry point; `run(ExecutionRequest<'_, Request>) ->
  Result<JobResponse<Response>, JobError>`. Note the generic trait parameters (`Request`,
  `Response` on the trait itself, defaulting to `String`) — a different polymorphism shape than
  `edge-application-handler`'s `Handler`, which fixes `Request`/`Response` as associated types.
- **`Router<Intent>`** — classifies input into a domain-specific intent/route id.
- **`LifecycleMonitor`** — health, background tasks, graceful shutdown; aggregates into
  `HealthReport`.
- **`Validator`** — request validation, with a `NoopValidator` default.

`ProxyComposer` is the SAF factory concern that composes these into a running dispatch chain.

It does **not** define `Handler`, `HandlerContext`, `HandlerError`, `ExecutionRequest`, or
`CommandBus`. Those are owned upstream by `edge-application-handler` / `edge-application-command`
and re-exported here unmodified — `saf/mod.rs` re-exports them directly (bypassing `api/`
entirely, since `api/` must never reference a foreign concrete type in a type position — the same
`no_foreign_type` rule `edge-application` itself follows). `SecurityContext` is the same kind of
pass-through, sourced from `edge-security-runtime`.

**Important scoping note:** this repo has zero Cargo-level knowledge of anything that depends on
*it*. The dependency graph below is the only thing this document asserts — what runtime holds
`Arc<dyn Job<Request, Response>>` and calls `.run()`, what transport sits in front of that runtime,
and whether a `Job` implementation internally uses a `HandlerRegistry` are all facts that live in
*some other repo's* Cargo.toml and source, not this one's. `main/src/lib.rs`'s own doc comment
sketches one *suggested* consumption shape (combining `Job`/`Router`/`ProxySvc` from this crate
with `Handler`/`HandlerRegistry` from `edge-application`) — that example is marked
```ignore``` (illustrative, not compiled or verified) precisely because it describes a pattern this
crate recommends, not one it can confirm any consumer actually follows. Confirming an actual live
wiring requires independently inspecting that consumer's own repo.

---

## Dependency graph (from `Cargo.toml`)

```
edge-application-handler   (git tag v0.16.0)  — HandlerContext, HandlerError, ExecutionRequest
edge-application-command   (git tag v0.16.0)  — CommandBus
edge-application-observer  (git tag v0.16.0)  — (dev-dependency only, plus runtime dep)
edge-security-runtime      (git tag v0.3.7)   — SecurityContext
swe-edge-configbuilder     (git tag v0.3.0)   — (config composition, `ApplicationConfigBuilder`)
```

All are pinned to a specific tag, not a branch — this crate does not track `edge-application`'s
`dev` branch. Note the tag skew against `edge-dispatcher` (which pins `edge-application-handler` /
`edge-application-event` at `v0.17.0`): this repo and `edge-dispatcher` are not guaranteed to be
built against the same upstream commit, and nothing in either repo currently detects that.

---

## SEA layering

```
api/       — public contract surface: Job, Router, LifecycleMonitor, Validator traits; their
             DTOs/errors (JobError, RoutingError, LifecycleError, JobResponse, ExecutionRequest
             re-export, health types). No implementation.
core/      — concrete implementations: NullJob, NullRouter, NoopValidator,
             NoopLifecycleMonitor, ApplicationConfigBuilder.
saf/       — Service Abstraction Framework: the public re-export/discovery surface. `_svc.rs`
             files (one `pub const X_CONCERN: &str` identity marker + trait re-export + optional
             factory) for job, router, lifecycle_monitor, monitor, validator, proxy composer.
             Also the only place foreign context types (HandlerContext, CommandBus,
             SecurityContext) are re-exported — `api/` never names them directly.
spi/       — extension points (`canonical.rs`).
```

---

## Block diagram — SEA layer composition

Module-level composition, derived from the actual `use crate::...` edges in each layer (not just
the linear `api → core → saf → spi` gloss in the table above — `core/` and `spi/` cross-reference
each other within the crate):

```mermaid
flowchart TB
    subgraph API["api/ — L1 public contracts (no impl)"]
        ApiTraits["Job · Router · LifecycleMonitor<br/>Validator · ProxyComposer (marker type)"]
        ApiDto["DTOs / errors: ExecutionRequest re-export,<br/>JobResponse, JobError, RouteRequest/Response,<br/>RoutingError, HealthReport, LifecycleError"]
    end

    subgraph CORE["core/ — L2 default implementations (pub(crate))"]
        CoreImpls["NullJob · NullRouter<br/>NoopValidator · NoopLifecycleMonitor<br/>ApplicationConfigBuilder"]
        CoreFactory["impl ProxySvc { new_null_*, new_noop_*, new_canonical_* }"]
    end

    subgraph SPI["spi/ — extension points"]
        SpiFactory["CanonicalFactory<br/>(job, router, null_job, null_router,<br/>null_lifecycle_monitor, noop_validator)"]
        SpiImpls["CanonicalJobImpl / CanonicalRouterImpl<br/>(always Cancelled / NoMatch)"]
    end

    subgraph SAF["saf/ — public facade (Service Abstraction Framework)"]
        SafTraits["Trait re-exports:<br/>Job, Router, LifecycleMonitor, Validator, ProxyComposer"]
        SafConst["CONCERN + SVC_FACTORY identity constants"]
        SafForeign["Foreign re-exports (bypass api/ entirely):<br/>HandlerContext, CommandBus, SecurityContext"]
    end

    API --> CORE
    API --> SPI
    CoreFactory -- "delegates null/noop/canonical<br/>construction to" --> SpiFactory
    SpiFactory -- "returns core's Null* types<br/>for null_job/null_router" --> CoreImpls
    CORE --> SAF
    API --> SAF

    Upstream1(["edge-application-handler"]) -.-> SafForeign
    Upstream2(["edge-application-command"]) -.-> SafForeign
    Upstream3(["edge-security-runtime"]) -.-> SafForeign
```

`api/` is the only layer with zero inbound edges — everything implements or re-exports it, it
depends on nothing internal. `saf/` never imports `spi/` directly; it only re-exports the trait
contracts `api/` defines plus the three foreign context types, so a consumer depending on this
crate never needs to know `spi/` exists.

---

## Governing ADRs

| ADR | Title | Governs |
|---|---|---|
| [001](../adr/ADR-001-security-context-propagation.md) | Security Context Propagation | How `SecurityContext` flows through `Job`/`ExecutionRequest` |
| [002](../adr/ADR-002-handler-context-construction.md) | HandlerContext Construction | Where/how `HandlerContext` is built before reaching `ExecutionRequest::ctx` |

Each mirrors a governing decision made in the `edge` platform repo — see each ADR's own header for
the upstream link. Status reflects this repo's own doc, not necessarily the upstream one's.

---

## Dataflow diagram (confirmed, within this crate's own dependency graph)

```mermaid
graph TB
    subgraph upstream["upstream (external crates, pinned by tag)"]
        Handler["edge_application_handler<br/><i>HandlerContext, HandlerError,<br/>ExecutionRequest</i>"]
        Command["edge_application_command<br/><i>CommandBus</i>"]
        Security["edge_security_runtime<br/><i>SecurityContext</i>"]
        Config["swe_edge_configbuilder<br/><i>config composition</i>"]
    end

    subgraph crate["edge-proxy"]
        Api["api/<br/><i>Job, Router, LifecycleMonitor,<br/>Validator + DTOs/errors</i>"]
        Core["core/<br/><i>NullJob, NullRouter, NoopValidator,<br/>NoopLifecycleMonitor</i>"]
        Saf["saf/<br/><i>ProxyComposer + CONCERN identity<br/>constants; re-exports foreign<br/>context types directly</i>"]
    end

    Handler -->|"ExecutionRequest re-exported<br/>via api/; HandlerContext/<br/>HandlerError re-exported<br/>via saf/ only"| Api
    Handler -->|"HandlerContext, HandlerError"| Saf
    Command -->|"CommandBus"| Saf
    Security -->|"SecurityContext"| Saf
    Config --> Core
    Api --> Core
    Core --> Saf
```

A consumer implements `Job<Request, Response>`, constructing an `ExecutionRequest<'_, Request>`
(whose `ctx: &HandlerContext` is built from `SecurityContext` + `CommandBus`) at the inbound
boundary and returning `JobResponse<Response>` or a `JobError`. `Router` classifies input into a
route id; `LifecycleMonitor` and `Validator` are separate, composable concerns a `Job`
implementation may call into. `ProxyComposer` (SAF) is the suggested composition point for wiring
these together, per its own doc comments.

**Not covered by this document, by design:** what runtime holds and calls `Arc<dyn Job<...>>`,
whether/how a `Job` implementation looks up a domain `Handler` via a `HandlerRegistry`, and what
transport sits upstream of that runtime. Those are downstream facts this repo cannot see from its
own Cargo.toml or source tree.

---

## Sequence diagram — canonical dispatch path

Traced from `examples/dispatch.rs` (compiled, `cargo run --example dispatch` — not a doctest-ignored
sketch). `DispatchJob` is one concrete `Job` impl a consumer could write; the `Router` and
`HandlerRegistry` steps are not part of this crate's own contract (`HandlerRegistry` is
`edge-application-handler`'s), but this is the shape `Job::run` is designed to sit in front of.

```mermaid
sequenceDiagram
    participant RT as Runtime
    participant J as Job (DispatchJob)
    participant R as Router
    participant HR as HandlerRegistry
    participant H as Handler

    RT->>J: run(ExecutionRequest { req, ctx })
    J->>R: route(RouteRequest { input: req.command })
    alt route matched
        R-->>J: Ok(RouteResponse { intent })
        J->>HR: get(HandlerLookupRequest { id: intent })
        alt handler registered
            HR-->>J: Some(handler)
            J->>H: execute(ExecutionRequest { req, ctx })
            alt handler succeeds
                H-->>J: Ok(Response)
                J-->>RT: Ok(JobResponse { payload })
            else handler fails
                H-->>J: Err(HandlerError)
                J-->>RT: Err(JobError::Handler(message))
            end
        else no handler for intent
            HR-->>J: None
            J-->>RT: Err(JobError::HandlerUnavailable)
        end
    else no route match
        R-->>J: Err(RoutingError::NoMatch)
        J-->>RT: Err(JobError::Routing(RoutingError))
    end
```

## Sequence diagram — lifecycle health check

`LifecycleMonitor` is a separate concern from `Job` — a runtime polls it independently of dispatch.
Shown here via the null implementation `ProxySvc::new_null_lifecycle_monitor()` returns (always
`Healthy`, no background tasks):

```mermaid
sequenceDiagram
    participant RT as Runtime
    participant PS as ProxySvc (core/spi factory)
    participant LM as LifecycleMonitor (NoopLifecycleMonitor)

    RT->>PS: new_null_lifecycle_monitor()
    PS-->>RT: Arc<dyn LifecycleMonitor>
    RT->>LM: health(HealthRequest)
    LM-->>RT: Ok(HealthReport { overall: Healthy, .. })
```

---

## See also

- `docs/adr/` — this repo's own ADRs, each mirroring an upstream `edge` decision
- `docs/README.md` — WHAT/WHY overview of this crate's capabilities
- `main/src/lib.rs` — the crate's own doc comment, including the suggested (illustrative,
  doctest-`ignore`d) consumption shape
