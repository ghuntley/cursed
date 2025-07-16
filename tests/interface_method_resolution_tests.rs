use cursed::type_system::interface_compliance::InterfaceComplianceChecker;
use cursed::ast::{InterfaceStatement, MethodSignature, Parameter, Type, Visibility};

#[test]
fn test_interface_inheritance_method_resolution() {
    let mut checker = InterfaceComplianceChecker::new();
    
    // Define base Writer interface
    let writer_interface = InterfaceStatement {
        name: "Writer".to_string(),
        type_parameters: vec![],
        extends: vec![],
        compositions: vec![],
        methods: vec![
            MethodSignature {
                name: "write".to_string(),
                receiver: None,
                parameters: vec![
                    Parameter {
                        name: "data".to_string(),
                        param_type: Some(Type::String),
                    }
                ],
                return_type: Some(Type::Integer),
            }
        ],
        visibility: Visibility::Public,
    };
    
    // Define ReadWriter interface that extends Writer
    let read_writer_interface = InterfaceStatement {
        name: "ReadWriter".to_string(),
        type_parameters: vec![],
        extends: vec!["Writer".to_string()],
        compositions: vec![],
        methods: vec![
            MethodSignature {
                name: "read".to_string(),
                receiver: None,
                parameters: vec![],
                return_type: Some(Type::String),
            }
        ],
        visibility: Visibility::Public,
    };
    
    // Register interfaces
    checker.register_interface(&writer_interface).expect("Should register Writer interface");
    checker.register_interface(&read_writer_interface).expect("Should register ReadWriter interface");
    
    // Test inheritance relationship
    assert!(checker.interface_extends("ReadWriter", "Writer"));
    assert!(!checker.interface_extends("Writer", "ReadWriter"));
}

#[test]
fn test_multiple_inheritance_method_resolution() {
    let mut checker = InterfaceComplianceChecker::new();
    
    // Define Reader interface
    let reader_interface = InterfaceStatement {
        name: "Reader".to_string(),
        type_parameters: vec![],
        extends: vec![],
        compositions: vec![],
        methods: vec![
            MethodSignature {
                name: "read".to_string(),
                receiver: None,
                parameters: vec![],
                return_type: Some(Type::String),
            }
        ],
        visibility: Visibility::Public,
    };
    
    // Define Writer interface
    let writer_interface = InterfaceStatement {
        name: "Writer".to_string(),
        type_parameters: vec![],
        extends: vec![],
        compositions: vec![],
        methods: vec![
            MethodSignature {
                name: "write".to_string(),
                receiver: None,
                parameters: vec![
                    Parameter {
                        name: "data".to_string(),
                        param_type: Some(Type::String),
                    }
                ],
                return_type: Some(Type::Integer),
            }
        ],
        visibility: Visibility::Public,
    };
    
    // Define ReadWriter interface that extends both
    let read_writer_interface = InterfaceStatement {
        name: "ReadWriter".to_string(),
        type_parameters: vec![],
        extends: vec!["Reader".to_string(), "Writer".to_string()],
        compositions: vec![],
        methods: vec![
            MethodSignature {
                name: "size".to_string(),
                receiver: None,
                parameters: vec![],
                return_type: Some(Type::Integer),
            }
        ],
        visibility: Visibility::Public,
    };
    
    // Register interfaces
    checker.register_interface(&reader_interface).expect("Should register Reader interface");
    checker.register_interface(&writer_interface).expect("Should register Writer interface");
    checker.register_interface(&read_writer_interface).expect("Should register ReadWriter interface");
    
    // Test multiple inheritance relationships
    assert!(checker.interface_extends("ReadWriter", "Reader"));
    assert!(checker.interface_extends("ReadWriter", "Writer"));
    assert!(!checker.interface_extends("Reader", "Writer"));
    assert!(!checker.interface_extends("Writer", "Reader"));
}

#[test]
fn test_transitive_inheritance_method_resolution() {
    let mut checker = InterfaceComplianceChecker::new();
    
    // Define base Writer interface
    let writer_interface = InterfaceStatement {
        name: "Writer".to_string(),
        type_parameters: vec![],
        extends: vec![],
        compositions: vec![],
        methods: vec![
            MethodSignature {
                name: "write".to_string(),
                receiver: None,
                parameters: vec![
                    Parameter {
                        name: "data".to_string(),
                        param_type: Some(Type::String),
                    }
                ],
                return_type: Some(Type::Integer),
            }
        ],
        visibility: Visibility::Public,
    };
    
    // Define ReadWriter interface that extends Writer
    let read_writer_interface = InterfaceStatement {
        name: "ReadWriter".to_string(),
        type_parameters: vec![],
        extends: vec!["Writer".to_string()],
        compositions: vec![],
        methods: vec![
            MethodSignature {
                name: "read".to_string(),
                receiver: None,
                parameters: vec![],
                return_type: Some(Type::String),
            }
        ],
        visibility: Visibility::Public,
    };
    
    // Define BufferedReadWriter interface that extends ReadWriter
    let buffered_read_writer_interface = InterfaceStatement {
        name: "BufferedReadWriter".to_string(),
        type_parameters: vec![],
        extends: vec!["ReadWriter".to_string()],
        compositions: vec![],
        methods: vec![
            MethodSignature {
                name: "flush".to_string(),
                receiver: None,
                parameters: vec![],
                return_type: Some(Type::Integer),
            }
        ],
        visibility: Visibility::Public,
    };
    
    // Register interfaces
    checker.register_interface(&writer_interface).expect("Should register Writer interface");
    checker.register_interface(&read_writer_interface).expect("Should register ReadWriter interface");
    checker.register_interface(&buffered_read_writer_interface).expect("Should register BufferedReadWriter interface");
    
    // Test transitive inheritance relationships
    assert!(checker.interface_extends("BufferedReadWriter", "ReadWriter"));
    assert!(checker.interface_extends("BufferedReadWriter", "Writer")); // Transitive
    assert!(checker.interface_extends("ReadWriter", "Writer"));
    
    // Test non-inheritance relationships
    assert!(!checker.interface_extends("Writer", "ReadWriter"));
    assert!(!checker.interface_extends("Writer", "BufferedReadWriter"));
    assert!(!checker.interface_extends("ReadWriter", "BufferedReadWriter"));
}
