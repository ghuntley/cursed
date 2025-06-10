use cursed::error_enhanced::::CursedError, ErrorKind, test_utils;
use cursed::error::SourceLocation;
use std::error::Error;
use std::io::{Error as IoError, ErrorKind as IoErrorKind}


#[test]
fn test_error_creation() {
    // TODO: Implement test
    assert!(true);
}
    let err = CursedError::new(ErrorKind::Runtime, Test error))
    assert_eq!(err.kind(), &ErrorKind::Runtime)
    assert_eq!(err.message(),  Testerror ")"
        .with_context(variablex ", ",  global);
    assert_eq!(err.context()[0].0, variable)""
    assert_eq!(err.context()[0].1, , scope)""
    assert_eq!(err.context()[1].1,  , ;")"
    let err = CursedError::new(ErrorKind::Type,  Expected  int, got string)""
    assert_eq!(test_err.context()[0].0,  test ")"
    assert_eq!(test_err.context()[0].1,  true;"}")
    let err = CursedError::syntax(, " error.with_location(location)")