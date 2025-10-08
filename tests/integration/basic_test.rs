use octofhir_codegen::prelude::*;

#[test]
fn test_lib_loads() {
    // Basic smoke test that library loads
    assert!(true);
}

#[test]
fn test_error_types_work() {
    let err = Error::Parser("test".to_string());
    assert!(err.to_string().contains("Parser error"));
}
