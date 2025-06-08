//! Basic runtime tests for type assertion functionality
//! Simple tests that avoid complex dependencies

#[path = "tracing_setup.rs"]
pub mod tracing_setup;

use cursed::runtime::type_assertion_runtime::{TypeAssertionRuntime, RuntimeTypeInfo, PanicConfiguration};
use cursed::error::{Error, SourceLocation};
use cursed::error_enhanced::{CursedError, ErrorKind};
use tracing::{info, debug};

#[test]
fn test_basic_runtime_creation() {
    tracing_setup::init_test_tracing();
    info!("Testing basic runtime creation");

    let runtime = TypeAssertionRuntime::new();
    let stats = runtime.get_statistics().unwrap();
    assert_eq!(stats.total_assertions, 0);
    assert_eq!(stats.successful_assertions, 0);
    assert_eq!(stats.failed_assertions, 0);
    
    info!("Basic runtime creation test passed");
}

#[test]
fn test_runtime_with_panic_config() {
    tracing_setup::init_test_tracing();
    info!("Testing runtime with custom panic configuration");

    let panic_config = PanicConfiguration {
        panic_on_failure: true,
        panic_on_nil: false,
        detailed_panic_messages: true,
        max_stack_trace_depth: 5,
    };
    
    let runtime = TypeAssertionRuntime::with_panic_config(panic_config);
    let stats = runtime.get_statistics().unwrap();
    assert_eq!(stats.total_assertions, 0);
    
    info!("Runtime with panic configuration test passed");
}

#[test]
fn test_type_registration() {
    tracing_setup::init_test_tracing();
    info!("Testing type registration");

    let mut runtime = TypeAssertionRuntime::new();
    
    let type_info = RuntimeTypeInfo {
        type_id: 0x1234567890ABCDEF,
        type_name: "Person".to_string(),
        size: 64,
        implements: vec!["Stringer".to_string()],
        debug_info: Some("Test person type".to_string()),
    };
    
    assert!(runtime.register_type(type_info).is_ok());
    
    let retrieved = runtime.get_type_info(0x1234567890ABCDEF);
    assert!(retrieved.is_some());
    assert_eq!(retrieved.unwrap().type_name, "Person");
    
    info!("Type registration test passed");
}

#[test]
fn test_successful_type_assertion() {
    tracing_setup::init_test_tracing();
    info!("Testing successful type assertion");

    let mut runtime = TypeAssertionRuntime::new();
    
    let type_info = RuntimeTypeInfo {
        type_id: 0x1111222233334444,
        type_name: "Dog".to_string(),
        size: 32,
        implements: vec![],
        debug_info: None,
    };
    
    runtime.register_type(type_info).unwrap();
    
    let result = runtime.assert_type(
        0x1111222233334444,
        "Dog",
        None
    );
    
    assert!(result.is_ok());
    assert!(result.unwrap());
    
    let stats = runtime.get_statistics().unwrap();
    assert_eq!(stats.successful_assertions, 1);
    assert_eq!(stats.total_assertions, 1);
    
    info!("Successful type assertion test passed");
}

#[test]
fn test_failed_type_assertion() {
    tracing_setup::init_test_tracing();
    info!("Testing failed type assertion");

    let mut runtime = TypeAssertionRuntime::new();
    
    let person_info = RuntimeTypeInfo {
        type_id: 0x1111222233334444,
        type_name: "Person".to_string(),
        size: 64,
        implements: vec![],
        debug_info: None,
    };
    
    let dog_info = RuntimeTypeInfo {
        type_id: 0x5555666677778888,
        type_name: "Dog".to_string(),
        size: 32,
        implements: vec![],
        debug_info: None,
    };
    
    runtime.register_type(person_info).unwrap();
    runtime.register_type(dog_info).unwrap();
    
    let result = runtime.assert_type(
        0x1111222233334444, // Person type ID
        "Dog",               // Trying to assert to Dog
        None
    );
    
    assert!(result.is_err());
    
    let error = result.unwrap_err();
    assert_eq!(error.kind(), &ErrorKind::TypeAssertion);
    
    let stats = runtime.get_statistics().unwrap();
    assert_eq!(stats.failed_assertions, 1);
    assert_eq!(stats.total_assertions, 1);
    
    info!("Failed type assertion test passed");
}

#[test]
fn test_nil_interface_handling() {
    tracing_setup::init_test_tracing();
    info!("Testing nil interface assertion handling");

    let runtime = TypeAssertionRuntime::new();
    
    let result = runtime.assert_type(0, "Person", None);
    assert!(result.is_err());
    
    let error = result.unwrap_err();
    assert_eq!(error.kind(), &ErrorKind::TypeAssertion);
    
    let stats = runtime.get_statistics().unwrap();
    assert_eq!(stats.failed_assertions, 1);
    assert_eq!(stats.total_assertions, 1);
    
    info!("Nil interface handling test passed");
}

#[test]
fn test_statistics_tracking() {
    tracing_setup::init_test_tracing();
    info!("Testing statistics tracking");

    let mut runtime = TypeAssertionRuntime::new();
    
    // Register a test type
    let type_info = RuntimeTypeInfo {
        type_id: 0x1111222233334444,
        type_name: "TestType".to_string(),
        size: 32,
        implements: vec![],
        debug_info: None,
    };
    runtime.register_type(type_info).unwrap();
    
    // Perform several assertions
    let _ = runtime.assert_type(0x1111222233334444, "TestType", None); // Success
    let _ = runtime.assert_type(0x1111222233334444, "WrongType", None); // Failure
    let _ = runtime.assert_type(0, "TestType", None); // Nil failure
    
    let stats = runtime.get_statistics().unwrap();
    assert_eq!(stats.total_assertions, 3);
    assert_eq!(stats.successful_assertions, 1);
    assert_eq!(stats.failed_assertions, 2);
    
    info!("Statistics tracking test passed");
}

#[test]
fn test_type_id_lookup() {
    tracing_setup::init_test_tracing();
    info!("Testing type ID lookup functionality");

    let mut runtime = TypeAssertionRuntime::new();
    
    let type_info = RuntimeTypeInfo {
        type_id: 0xAABBCCDDEEFF0011,
        type_name: "LookupTest".to_string(),
        size: 128,
        implements: vec!["TestInterface".to_string()],
        debug_info: Some("Type for lookup testing".to_string()),
    };
    runtime.register_type(type_info).unwrap();
    
    // Test lookup by type ID
    let retrieved_by_id = runtime.get_type_info(0xAABBCCDDEEFF0011);
    assert!(retrieved_by_id.is_some());
    assert_eq!(retrieved_by_id.unwrap().type_name, "LookupTest");
    
    // Test lookup by name
    let retrieved_id = runtime.get_type_id_by_name("LookupTest");
    assert!(retrieved_id.is_some());
    assert_eq!(retrieved_id.unwrap(), 0xAABBCCDDEEFF0011);
    
    // Test missing type
    let missing = runtime.get_type_info(0x9999999999999999);
    assert!(missing.is_none());
    
    let missing_name = runtime.get_type_id_by_name("NonExistentType");
    assert!(missing_name.is_none());
    
    info!("Type ID lookup functionality test passed");
}
