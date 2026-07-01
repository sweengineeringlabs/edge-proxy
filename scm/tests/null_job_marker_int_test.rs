//! Integration tests for the NullJobMarker type.
//! @covers: api/job/null_job.rs

use edge_proxy::NullJobMarker;

#[test]
fn test_null_job_marker_is_zero_sized_happy() {
    assert_eq!(std::mem::size_of::<NullJobMarker>(), 0);
}

#[test]
fn test_null_job_marker_can_be_constructed_happy() {
    let m = NullJobMarker;
    assert_eq!(std::mem::size_of_val(&m), 0);
}

#[test]
fn test_null_job_marker_is_in_public_api_edge() {
    fn accept(v: NullJobMarker) -> NullJobMarker {
        v
    }
    let m = accept(NullJobMarker);
    assert_eq!(std::mem::size_of_val(&m), 0);
}
