use edge_proxy::JOB_SVC_FACTORY;

#[test]
fn test_job_svc_factory_constant_value_happy() {
    assert_eq!(JOB_SVC_FACTORY, "job_factory");
}

#[test]
fn test_job_svc_factory_constant_not_empty_error() {
    assert!(
        !JOB_SVC_FACTORY.is_empty(),
        "JOB_SVC_FACTORY must not be empty"
    );
}

#[test]
fn test_job_svc_factory_constant_no_whitespace_edge() {
    assert!(
        !JOB_SVC_FACTORY.contains(char::is_whitespace),
        "JOB_SVC_FACTORY must not contain whitespace"
    );
}
