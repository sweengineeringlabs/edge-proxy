//! Integration tests for the NullJobMarker type.
//! @covers: api/job/null_job.rs

use edge_proxy::NullJobMarker;

#[test]
fn test_null_job_marker_is_zero_sized_happy() {
    assert_eq!(std::mem::size_of::<NullJobMarker>(), 0);
}

#[test]
fn test_null_job_marker_can_be_constructed_happy() {
    let _m = NullJobMarker;
}

#[test]
fn test_null_job_marker_is_in_public_api_edge() {
    fn _accept(_: NullJobMarker) {}
    _accept(NullJobMarker);
}
