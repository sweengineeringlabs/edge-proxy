//! Integration tests for proxy public surface (api/traits.rs concern).

use edge_proxy::{JobError, RoutingError};

#[test]
fn test_job_error_is_accessible_from_public_surface() {
    let err: JobError = RoutingError::NoMatch.into();
    assert!(matches!(err, JobError::Routing(RoutingError::NoMatch)));
}
