//! Tests for interface dynamic dispatch in LLVM code generation

use cursed::ast::base::Program;
use cursed::ast::expressions::{CallExpression, Identifier, IntegerLiteral, StringLiteral};
use cursed::ast::statements::block::BlockStatement;
use cursed::ast::statements::{ExpressionStatement, ReturnStatement};
use cursed::ast::declarations::{FunctionStatement, SquadStatement};
use cursed::ast::declarations::ParameterStatement;
use cursed::ast::declarations::GenericConstraint;
use cursed::ast::declarations::CollabStatement;
use cursed::codegen::llvm::LlvmCodeGenerator;
use cursed::core::type_checker::Type;
use inkwell::context::Context;
use inkwell::OptimizationLevel;
use inkwell::values::FunctionValue;
use std::path::PathBuf;
use std::collections::HashMap;

/// This test verifies that we can create interfaces, implement them in structs,
/// and then use dynamic dispatch to call interface methods
#[test]
#[ignore = "Interface dynamic dispatch implementation is not yet complete"]
fn test_basic_interface_dynamic_dispatch() {
    // Create a context and code generator
    let context = Context::create();
    let file_path = PathBuf::from("test_interface_dispatch.csd");
    let mut code_gen = LlvmCodeGenerator::new(&context, "test_interface", file_path);

    // Create a Greeter interface with a single method: greet() -> tea
    let greeter_interface = create_greeter_interface();
    
    // Create a Person struct that will implement the Greeter interface
    let person_struct = create_person_struct();
    
    // Create implementations of the Greeter interface for Person
    let person_greet_impl = create_person_greet_implementation();
    
    // Register the interface and struct with the code generator
    code_gen.compile_collab(&greeter_interface).expect("Failed to compile Greeter interface");
    code_gen.compile_squad(&person_struct).expect("Failed to compile Person struct");
    
    // Register the implementation method
    code_gen.compile_function(&person_greet_impl).expect("Failed to compile Person.greet implementation");
    
    // Create a function that uses the interface dynamically
    let use_greeter_function = create_use_greeter_function();
    
    // Compile the function that uses the interface
    let result = code_gen.compile_function(&use_greeter_function);
    assert!(result.is_ok(), "Compiling function that uses dynamic dispatch should succeed: {:?}", result.err());
    
    // Verify the output module contains the dynamic dispatch logic
    let module = code_gen.module();
    assert!(module.get_function("use_greeter").is_some(), "use_greeter function should exist in module");
}

/// This test verifies that we can handle generic interfaces with dynamic dispatch
#[test]
#[ignore = "Generic interface dynamic dispatch implementation is not yet complete"]
fn test_generic_interface_dynamic_dispatch() {
    // Create a context and code generator
    let context = Context::create();
    let file_path = PathBuf::from("test_generic_interface_dispatch.csd");
    let mut code_gen = LlvmCodeGenerator::new(&context, "test_generic_interface", file_path);

    // Create a generic Container<T> interface with methods: add(T), get(normie) -> T, size() -> normie
    let container_interface = create_container_interface();
    
    // Create a StringList struct that will implement Container<tea>
    let string_list_struct = create_string_list_struct();
    
    // Create implementations of Container<tea> for StringList
    let string_list_add_impl = create_string_list_add_implementation();
    let string_list_get_impl = create_string_list_get_implementation();
    let string_list_size_impl = create_string_list_size_implementation();
    
    // Register the interface and struct with the code generator
    code_gen.compile_collab(&container_interface).expect("Failed to compile Container interface");
    code_gen.compile_squad(&string_list_struct).expect("Failed to compile StringList struct");
    
    // Register the implementation methods
    code_gen.compile_function(&string_list_add_impl).expect("Failed to compile StringList.add implementation");
    code_gen.compile_function(&string_list_get_impl).expect("Failed to compile StringList.get implementation");
    code_gen.compile_function(&string_list_size_impl).expect("Failed to compile StringList.size implementation");
    
    // Create a function that uses the generic interface dynamically
    let use_container_function = create_use_container_function();
    
    // Compile the function that uses the interface
    let result = code_gen.compile_function(&use_container_function);
    assert!(result.is_ok(), "Compiling function that uses generic interface should succeed: {:?}", result.err());
    
    // Verify the output module contains the function
    let module = code_gen.module();
    assert!(module.get_function("use_container").is_some(), "use_container function should exist in module");
}

/// Helper function to create a Greeter interface
fn create_greeter_interface() -> CollabStatement {
    // Interface with a single method: greet() -> tea
    CollabStatement {
        token: "collab".to_string(),
        name: Identifier {
            token: "Greeter".to_string(),
            value: "Greeter".to_string(),
        },
        methods: vec![
            // greet() -> tea
            FunctionStatement {
                token: "slay".to_string(),
                name: Identifier {
                    token: "greet".to_string(),
                    value: "greet".to_string(),
                },
                parameters: vec![],
                body: BlockStatement {
                    token: "{".to_string(),
                    statements: vec![],
                },
                return_type: Some(Box::new(Identifier {
                    token: "tea".to_string(),
                    value: "tea".to_string(),
                })),
                type_parameters: vec![],
                generic_constraints: vec![],
            },
        ],
        type_parameters: vec![],
    }
}

/// Helper function to create a Person struct
fn create_person_struct() -> SquadStatement {
    // Person struct with name: tea and age: normie fields
    SquadStatement {
        token: "squad".to_string(),
        name: Identifier {
            token: "Person".to_string(),
            value: "Person".to_string(),
        },
        fields: vec![
            // name: tea
            cursed::ast::declarations::FieldStatement {
                token: "field".to_string(),
                name: Identifier {
                    token: "name".to_string(),
                    value: "name".to_string(),
                },
                type_name: Box::new(Identifier {
                    token: "tea".to_string(),
                    value: "tea".to_string(),
                }),
                tag: None,
            },
            // age: normie
            cursed::ast::declarations::FieldStatement {
                token: "field".to_string(),
                name: Identifier {
                    token: "age".to_string(),
                    value: "age".to_string(),
                },
                type_name: Box::new(Identifier {
                    token: "normie".to_string(),
                    value: "normie".to_string(),
                }),
                tag: None,
            },
        ],
        type_parameters: vec![],
        implemented_interfaces: vec![
            // Implement Greeter interface
            Box::new(Identifier {
                token: "Greeter".to_string(),
                value: "Greeter".to_string(),
            }),
        ],
    }
}

/// Helper function to create the greet() implementation for Person
fn create_person_greet_implementation() -> FunctionStatement {
    // Person.greet() implementation
    FunctionStatement {
        token: "slay".to_string(),
        name: Identifier {
            token: "Person_greet".to_string(), // Using a naming convention for clarity
            value: "Person_greet".to_string(),
        },
        parameters: vec![
            // self parameter
            ParameterStatement {
                token: "param".to_string(),
                name: Identifier {
                    token: "self".to_string(),
                    value: "self".to_string(),
                },
                type_name: Box::new(Identifier {
                    token: "Person".to_string(),
                    value: "Person".to_string(),
                }),
            },
        ],
        body: BlockStatement {
            token: "{".to_string(),
            statements: vec![
                // Return greeting with name
                Box::new(ReturnStatement {
                    token: "yolo".to_string(),
                    return_value: Some(Box::new(StringLiteral {
                        token: "greeting".to_string(),
                        value: "Hello, I am a Person".to_string(),
                    })),
                }),
            ],
        },
        return_type: Some(Box::new(Identifier {
            token: "tea".to_string(),
            value: "tea".to_string(),
        })),
        type_parameters: vec![],
        generic_constraints: vec![],
    }
}

/// Helper function to create a function that uses the Greeter interface
fn create_use_greeter_function() -> FunctionStatement {
    // Function that uses a Greeter through dynamic dispatch
    FunctionStatement {
        token: "slay".to_string(),
        name: Identifier {
            token: "use_greeter".to_string(),
            value: "use_greeter".to_string(),
        },
        parameters: vec![
            // greeter: Greeter
            ParameterStatement {
                token: "param".to_string(),
                name: Identifier {
                    token: "greeter".to_string(),
                    value: "greeter".to_string(),
                },
                type_name: Box::new(Identifier {
                    token: "Greeter".to_string(),
                    value: "Greeter".to_string(),
                }),
            },
        ],
        body: BlockStatement {
            token: "{".to_string(),
            statements: vec![
                // Return greeter.greet()
                Box::new(ReturnStatement {
                    token: "yolo".to_string(),
                    return_value: Some(Box::new(CallExpression {
                        token: "(".to_string(),
                        function: Box::new(cursed::ast::expressions::PropertyAccessExpression {
                            token: ".".to_string(),
                            object: Box::new(Identifier {
                                token: "greeter".to_string(),
                                value: "greeter".to_string(),
                            }),
                            property: Identifier {
                                token: "greet".to_string(),
                                value: "greet".to_string(),
                            },
                        }),
                        arguments: vec![],
                        type_arguments: vec![],
                    })),
                }),
            ],
        },
        return_type: Some(Box::new(Identifier {
            token: "tea".to_string(),
            value: "tea".to_string(),
        })),
        type_parameters: vec![],
        generic_constraints: vec![],
    }
}

/// Helper function to create a generic Container interface
fn create_container_interface() -> CollabStatement {
    // Generic Container<T> interface with methods: add(T), get(normie) -> T, size() -> normie
    CollabStatement {
        token: "collab".to_string(),
        name: Identifier {
            token: "Container".to_string(),
            value: "Container".to_string(),
        },
        methods: vec![
            // add(item: T)
            FunctionStatement {
                token: "slay".to_string(),
                name: Identifier {
                    token: "add".to_string(),
                    value: "add".to_string(),
                },
                parameters: vec![
                    ParameterStatement {
                        token: "param".to_string(),
                        name: Identifier {
                            token: "item".to_string(),
                            value: "item".to_string(),
                        },
                        type_name: Box::new(Identifier {
                            token: "T".to_string(),
                            value: "T".to_string(),
                        }),
                    },
                ],
                body: BlockStatement {
                    token: "{".to_string(),
                    statements: vec![],
                },
                return_type: None,
                type_parameters: vec![],
                generic_constraints: vec![],
            },
            // get(index: normie) -> T
            FunctionStatement {
                token: "slay".to_string(),
                name: Identifier {
                    token: "get".to_string(),
                    value: "get".to_string(),
                },
                parameters: vec![
                    ParameterStatement {
                        token: "param".to_string(),
                        name: Identifier {
                            token: "index".to_string(),
                            value: "index".to_string(),
                        },
                        type_name: Box::new(Identifier {
                            token: "normie".to_string(),
                            value: "normie".to_string(),
                        }),
                    },
                ],
                body: BlockStatement {
                    token: "{".to_string(),
                    statements: vec![],
                },
                return_type: Some(Box::new(Identifier {
                    token: "T".to_string(),
                    value: "T".to_string(),
                })),
                type_parameters: vec![],
                generic_constraints: vec![],
            },
            // size() -> normie
            FunctionStatement {
                token: "slay".to_string(),
                name: Identifier {
                    token: "size".to_string(),
                    value: "size".to_string(),
                },
                parameters: vec![],
                body: BlockStatement {
                    token: "{".to_string(),
                    statements: vec![],
                },
                return_type: Some(Box::new(Identifier {
                    token: "normie".to_string(),
                    value: "normie".to_string(),
                })),
                type_parameters: vec![],
                generic_constraints: vec![],
            },
        ],
        type_parameters: vec![
            Identifier {
                token: "T".to_string(),
                value: "T".to_string(),
            },
        ],
    }
}

/// Helper function to create a StringList struct implementing Container<tea>
fn create_string_list_struct() -> SquadStatement {
    // StringList struct with items: []tea field
    SquadStatement {
        token: "squad".to_string(),
        name: Identifier {
            token: "StringList".to_string(),
            value: "StringList".to_string(),
        },
        fields: vec![
            // items: []tea
            cursed::ast::declarations::FieldStatement {
                token: "field".to_string(),
                name: Identifier {
                    token: "items".to_string(),
                    value: "items".to_string(),
                },
                type_name: Box::new(cursed::ast::expressions::ArrayType {
                    token: "[".to_string(),
                    element_type: Box::new(Identifier {
                        token: "tea".to_string(),
                        value: "tea".to_string(),
                    }),
                }),
                tag: None,
            },
            // count: normie
            cursed::ast::declarations::FieldStatement {
                token: "field".to_string(),
                name: Identifier {
                    token: "count".to_string(),
                    value: "count".to_string(),
                },
                type_name: Box::new(Identifier {
                    token: "normie".to_string(),
                    value: "normie".to_string(),
                }),
                tag: None,
            },
        ],
        type_parameters: vec![],
        implemented_interfaces: vec![
            // Implement Container<tea> interface
            Box::new(cursed::ast::expressions::GenericType {
                token: "<".to_string(),
                base_type: Box::new(Identifier {
                    token: "Container".to_string(),
                    value: "Container".to_string(),
                }),
                type_arguments: vec![
                    Box::new(Identifier {
                        token: "tea".to_string(),
                        value: "tea".to_string(),
                    }),
                ],
            }),
        ],
    }
}

/// Helper function to create add() implementation for StringList
fn create_string_list_add_implementation() -> FunctionStatement {
    // StringList.add(item: tea) implementation
    FunctionStatement {
        token: "slay".to_string(),
        name: Identifier {
            token: "StringList_add".to_string(),
            value: "StringList_add".to_string(),
        },
        parameters: vec![
            // self parameter
            ParameterStatement {
                token: "param".to_string(),
                name: Identifier {
                    token: "self".to_string(),
                    value: "self".to_string(),
                },
                type_name: Box::new(Identifier {
                    token: "StringList".to_string(),
                    value: "StringList".to_string(),
                }),
            },
            // item parameter
            ParameterStatement {
                token: "param".to_string(),
                name: Identifier {
                    token: "item".to_string(),
                    value: "item".to_string(),
                },
                type_name: Box::new(Identifier {
                    token: "tea".to_string(),
                    value: "tea".to_string(),
                }),
            },
        ],
        body: BlockStatement {
            token: "{".to_string(),
            statements: vec![
                // Simple implementation body - we don't need full implementation for this test
                Box::new(ExpressionStatement {
                    token: "expr".to_string(),
                    expression: Box::new(Identifier {
                        token: "item".to_string(),
                        value: "item".to_string(),
                    }),
                }),
            ],
        },
        return_type: None,
        type_parameters: vec![],
        generic_constraints: vec![],
    }
}

/// Helper function to create get() implementation for StringList
fn create_string_list_get_implementation() -> FunctionStatement {
    // StringList.get(index: normie) -> tea implementation
    FunctionStatement {
        token: "slay".to_string(),
        name: Identifier {
            token: "StringList_get".to_string(),
            value: "StringList_get".to_string(),
        },
        parameters: vec![
            // self parameter
            ParameterStatement {
                token: "param".to_string(),
                name: Identifier {
                    token: "self".to_string(),
                    value: "self".to_string(),
                },
                type_name: Box::new(Identifier {
                    token: "StringList".to_string(),
                    value: "StringList".to_string(),
                }),
            },
            // index parameter
            ParameterStatement {
                token: "param".to_string(),
                name: Identifier {
                    token: "index".to_string(),
                    value: "index".to_string(),
                },
                type_name: Box::new(Identifier {
                    token: "normie".to_string(),
                    value: "normie".to_string(),
                }),
            },
        ],
        body: BlockStatement {
            token: "{".to_string(),
            statements: vec![
                // Return a placeholder value
                Box::new(ReturnStatement {
                    token: "yolo".to_string(),
                    return_value: Some(Box::new(StringLiteral {
                        token: "item".to_string(),
                        value: "placeholder".to_string(),
                    })),
                }),
            ],
        },
        return_type: Some(Box::new(Identifier {
            token: "tea".to_string(),
            value: "tea".to_string(),
        })),
        type_parameters: vec![],
        generic_constraints: vec![],
    }
}

/// Helper function to create size() implementation for StringList
fn create_string_list_size_implementation() -> FunctionStatement {
    // StringList.size() -> normie implementation
    FunctionStatement {
        token: "slay".to_string(),
        name: Identifier {
            token: "StringList_size".to_string(),
            value: "StringList_size".to_string(),
        },
        parameters: vec![
            // self parameter
            ParameterStatement {
                token: "param".to_string(),
                name: Identifier {
                    token: "self".to_string(),
                    value: "self".to_string(),
                },
                type_name: Box::new(Identifier {
                    token: "StringList".to_string(),
                    value: "StringList".to_string(),
                }),
            },
        ],
        body: BlockStatement {
            token: "{".to_string(),
            statements: vec![
                // Return the count field
                Box::new(ReturnStatement {
                    token: "yolo".to_string(),
                    return_value: Some(Box::new(cursed::ast::expressions::PropertyAccessExpression {
                        token: ".".to_string(),
                        object: Box::new(Identifier {
                            token: "self".to_string(),
                            value: "self".to_string(),
                        }),
                        property: Identifier {
                            token: "count".to_string(),
                            value: "count".to_string(),
                        },
                    })),
                }),
            ],
        },
        return_type: Some(Box::new(Identifier {
            token: "normie".to_string(),
            value: "normie".to_string(),
        })),
        type_parameters: vec![],
        generic_constraints: vec![],
    }
}

/// Helper function to create a function that uses a Container<tea> interface
fn create_use_container_function() -> FunctionStatement {
    // Function that uses a Container<tea> through dynamic dispatch
    FunctionStatement {
        token: "slay".to_string(),
        name: Identifier {
            token: "use_container".to_string(),
            value: "use_container".to_string(),
        },
        parameters: vec![
            // container: Container<tea>
            ParameterStatement {
                token: "param".to_string(),
                name: Identifier {
                    token: "container".to_string(),
                    value: "container".to_string(),
                },
                type_name: Box::new(cursed::ast::expressions::GenericType {
                    token: "<".to_string(),
                    base_type: Box::new(Identifier {
                        token: "Container".to_string(),
                        value: "Container".to_string(),
                    }),
                    type_arguments: vec![
                        Box::new(Identifier {
                            token: "tea".to_string(),
                            value: "tea".to_string(),
                        }),
                    ],
                }),
            },
        ],
        body: BlockStatement {
            token: "{".to_string(),
            statements: vec![
                // Add an item to the container
                Box::new(ExpressionStatement {
                    token: "expr".to_string(),
                    expression: Box::new(CallExpression {
                        token: "(".to_string(),
                        function: Box::new(cursed::ast::expressions::PropertyAccessExpression {
                            token: ".".to_string(),
                            object: Box::new(Identifier {
                                token: "container".to_string(),
                                value: "container".to_string(),
                            }),
                            property: Identifier {
                                token: "add".to_string(),
                                value: "add".to_string(),
                            },
                        }),
                        arguments: vec![
                            Box::new(StringLiteral {
                                token: "item".to_string(),
                                value: "test item".to_string(),
                            }),
                        ],
                        type_arguments: vec![],
                    }),
                }),
                // Return container.get(0)
                Box::new(ReturnStatement {
                    token: "yolo".to_string(),
                    return_value: Some(Box::new(CallExpression {
                        token: "(".to_string(),
                        function: Box::new(cursed::ast::expressions::PropertyAccessExpression {
                            token: ".".to_string(),
                            object: Box::new(Identifier {
                                token: "container".to_string(),
                                value: "container".to_string(),
                            }),
                            property: Identifier {
                                token: "get".to_string(),
                                value: "get".to_string(),
                            },
                        }),
                        arguments: vec![
                            Box::new(IntegerLiteral {
                                token: "0".to_string(),
                                value: 0,
                            }),
                        ],
                        type_arguments: vec![],
                    })),
                }),
            ],
        },
        return_type: Some(Box::new(Identifier {
            token: "tea".to_string(),
            value: "tea".to_string(),
        })),
        type_parameters: vec![],
        generic_constraints: vec![],
    }
}