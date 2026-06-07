# Architecture — edge-proxy

## Sequence

> The runtime calls `Job::run` per request; the proxy routes it, checks lifecycle health, dispatches to a handler, and reports completion.

```mermaid
sequenceDiagram
    participant Runtime
    participant Job
    participant Router
    participant LifecycleMonitor
    participant HandlerRegistry
    participant Handler

    Runtime->>Job: run(RequestContext)
    Job->>LifecycleMonitor: health_check()
    LifecycleMonitor-->>Job: HealthReport

    Job->>Router: route(ctx)
    Router-->>Job: Result<route_id, RoutingError>

    Job->>HandlerRegistry: get(route_id)
    HandlerRegistry-->>Job: Arc<dyn Handler>

    Job->>+Handler: execute(ctx)
    Handler-->>-Job: Result<Response, HandlerError>

    Job->>LifecycleMonitor: on_complete(outcome)
    Job-->>Runtime: Result<Response, JobError>
```

## Data Flow

> An inbound `RequestContext` is classified by `Router`, resolved to a handler, and the handler's response flows back through the `Job` boundary.

```mermaid
flowchart LR
    A["RequestContext\n───────────\npath, method\nheaders, body"] --> B["Router::route"]
    B -->|Ok| C["route_id: String"]
    B -->|Err| E["JobError::Routing\n(RoutingError)"]

    C --> D["HandlerRegistry\n::get(route_id)"]
    D -->|Some| F["Handler::execute(ctx)"]
    D -->|None| G["JobError::NotFound"]

    F -->|Ok| H["Response"]
    F -->|Err| I["JobError::Handler\n(HandlerError)"]

    H --> J["Result<Response, JobError>"]
    I --> J
    E --> J
    G --> J
```
