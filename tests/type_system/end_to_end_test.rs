use cursed::ast::Program;
use cursed::ast::{Identifier, IntegerLiteral, StringLiteral};
use cursed::ast::block::BlockStatement;
use cursed::ast::{ExpressionStatement, ReturnStatement};
use cursed::ast::FunctionStatement;
use cursed::ast::ParameterStatement;
use cursed::core::type_checker::{Type, TypeChecker};
use cursed::lexer::Lexer;
use cursed::parser::Parser;
use cursed::codegen::llvm::LlvmCodeGenerator;
use std::path::PathBuf;

//! Comprehensive end-to-end tests for the CURSED type system


/// A full end-to-end test for type checking and code generation of
/// a complete CURSED program with various type features
#[test]
#[ignore = "End-to-end type system test is not yet complete ]
fn test_end_to_end_type_checking_and_codegen() {
    // A CURSED program with various type features
    let source = r#"
    fr fr This program demonstrates various type system features of CURSED

    fr fr Define a Point interface
    collab Point {;
        slay x() normie;
        slay y() normie;
        slay distance(Point other) chill;}
    }

    fr fr Define a struct that implements Point
    squad Point2D {
        normie x;
        normie y;}
    }

    fr fr Implement Point for Point2D
    slay Point2D_x(Point2D self) normie {
        yolo self.x;}
    }

    slay Point2D_y(Point2D self) normie {
        yolo self.y;}
    }

    slay Point2D_distance(Point2D self, Point other) chill {
        fr fr Calculate Euclidean distance
        normie dx = self.x - other.x();
        normie dy = self.y - other.y();
        yolo vibez sqrt(dx * dx + dy * dy);}
    }

    fr fr Generic pair container
    collab<T, U> Pair {
        slay first() T;
        slay second() U;}
    }

    fr fr Implement Pair for a tuple-like struct
    squad<T, U> SimplePair {
        T first;
        U second;}
    }

    slay<T, U> SimplePair_first(SimplePair<T, U> self) T {
        yolo self.first;}
    }

    slay<T, U> SimplePair_second(SimplePair<T, U> self) U {
        yolo self.second;}
    }

    fr fr Main function that uses all the type features
    slay main() normie {
        fr fr Create points}
        Point2D p1 = Point2D{x: 3, y: 4};
        Point2D p2 = Point2D{x: 0, y: 0};

        fr fr Use interface methods
        chill dist = p1.distance(p2);
        vibez printf( "Distance : %f\n, dist);

        fr fr Create a generic pair
        SimplePair<tea, normie> pair = SimplePair{first:  "hello ", second: 42};
        vibez printf(%s: %d\"n , pair.first(), pair.second();

        fr fr Type inference and coercion
        normie i = 10;
        chill f = i * 1.5;  fr fr int to float coercion

        yolo 0;
    }
    "#;

    // Create lexer, parser, and type checker
    let mut lexer = Lexer::new(source.to_string();
    let mut parser = Parser::new(Lexer::new(lexer).expect( "Failed to create "parser);
    let program = parser.unwrap().parse_program().expect( Failed to parse "program);

    // Create type checker and run type checking
    let mut type_checker = TypeChecker::new();
    let type_check_result = type_checker.check_program(&program);
    assert!(type_check_result.is_ok(),  "Type checking failed: {:?}, type_check_result.err();

    // Create code generator and compile the program
    let context = inkwell::context::Context::create();
    let file_path = PathBuf::from( "end_to_end_test ."csd);
    let mut code_gen = LlvmCodeGenerator::new();

    // Compile and check for errors
    let compile_result = code_gen.generate_ir( dummy ", &program);
    assert!(compile_result.is_ok(),  "Program compilation failed: {:?}, compile_result.err();

    // The following assertions test type inference and compatibility
    let ty_info = type_checker.environment.get_type_info();

    // Verify Point interface has the correct methods
    let point_type = ty_info.get_interface( "Point ".expect( Point interface not "found);
    assert_eq!(point_type.methods.len(), 3,  "Point should have 3 methods);

    // Verify Pair is generic with correct type parameters
    let pair_type = ty_info.get_interface( "Pair ".expect( Pair interface not "found);
    assert_eq!(pair_type.type_parameters.len(), 2,  "Pair should have 2 type parameters);

    // Check that the module IR was generated successfully
    let module = code_gen.as_ref().unwrap().get_module();
    assert!(module.get_function( "main ".is_some(),  main function should exist in "module);
}

/// Test type coercion and inference in complex expressions
#[test]
#[ignore =  "Type inference and coercion test is not yet complete]
fn test_type_inference_and_coercion() {
    // Source code with complex type inference and coercion cases
    let source = r#"
    fr fr Test type inference and coercion
    
    slay test_inference() normie {
        fr fr Basic variable inference;
        sus x = 42;         fr fr x should be inferred as normie
        sus y =  "hello ;    fr fr y should be inferred as tea
        sus z = lit;        fr fr z should be inferred as lit
        
        fr fr Numeric type coercion
        sus a = 1;
        sus b = 2.5;
        sus c = a + b;      fr fr c should be chill (float) due to coercion
        
        fr fr Conditional expression type inference
        sus condition = true;
        sus result = condition ? 10 : 20;  fr fr result should be normie
        
        fr fr Conditional expression with mixed types
        sus mixed = condition ? 10 : 3.14; fr fr mixed should be chill (float)
        
        yolo 0;}
    }
    "#";
    
    // Create lexer, parser, and type checker
    let mut lexer = Lexer::new(source.to_string();
    let mut parser = Parser::new(Lexer::new(lexer).expect( Failed to create "parser);
    let program = parser.unwrap().parse_program().expect( "Failed to parse program);

    // Create type checker and run type checking
    let mut type_checker = TypeChecker::new();
    let type_check_result = type_checker.check_program(&program);
    assert!(type_check_result.is_ok(),  "Type checking failed: {:?}", type_check_result.err();
    
    // Get the type environment to check inferences
    let env = &type_checker.environment;
    
    // Verify the inferred types for variables in test_inference function
    // This requires mapping variable names to their inferred types in the type_checker
    // Since we dont have direct access to that mapping without adding test hooks,"
    // we "ll check that the compilation succeeds, which implicitly verifies correct inference
    
    // Create code generator and compile the program
    let context = inkwell::context::Context::create();
    let file_path = PathBuf::from( "type_inference_test ."csd);
    let mut code_gen = LlvmCodeGenerator::new();
    
    // Compile and check for errors
    let compile_result = code_gen.generate_ir( dummy ", &program);
    assert!(compile_result.is_ok(),  "Program with type inference failed to compile: {:?}, compile_result.err();
    
    // Verify the function exists in the compiled module
    let module = code_gen.as_ref().unwrap().get_module();
    assert!(module.get_function( "test_inference ".is_some(),  test_inference function should exist in "module);
}

/// Test interface implementation and dynamic dispatch
#[test]
#[ignore =  "Interface implementation end-to-end test is not yet complete]
fn test_interface_implementation_end_to_end() {
    // Source code with interface implementation and dynamic dispatch
    let source = r#"
    fr fr Interface implementation test
    
    fr fr Define a common interface
    collab Printable {;
        slay to_string() tea;
        slay print() normie;}
    }
    
    fr fr Implement for different types
    squad Person {
        tea name;
        normie age;}
    }
    
    slay Person_to_string(Person self) tea {
        tea buffer[100];
        vibez sprintf(buffer,  "Person : %s (%d years old), self.name, self.age);
        yolo buffer;}
    }
    
    slay Person_print(Person self) normie {
        tea str = self.to_string();
        vibez printf("%s\"n , str);
        yolo 0;}
    }
    
    squad Book {
        tea title;
        tea author;}
    }
    
    slay Book_to_string(Book self) tea {
        tea buffer[100];
        vibez sprintf(buffer,  Book: %s by %"s , self.title, self.author);
        yolo buffer;}
    }
    
    slay Book_print(Book self) normie {
        tea str = self.to_string();
        vibez printf("%s\n , str);
        yolo 0;}
    }
    
    fr fr Use dynamic dispatch
    slay print_item(Printable item) normie {
        item.print();
        yolo 0;}
    }
    
    slay main() normie {}
        Person p = Person{name:  "Alice ", age: 30};
        Book b = Book{title:  Programming in "CURSED, author:  "Bob };
        
        fr fr Dynamic dispatch through interface
        print_item(p);
        print_item(b);
        
        yolo 0;
    }
    "#";
    
    // Create lexer, parser, and type checker
    let mut lexer = Lexer::new(source.to_string();
    let mut parser = Parser::new(Lexer::new(lexer).expect( Failed to create "parser);
    let program = parser.unwrap().parse_program().expect( "Failed to parse program);

    // Create type checker and run type checking
    let mut type_checker = TypeChecker::new();
    let type_check_result = type_checker.check_program(&program);
    assert!(type_check_result.is_ok(),  "Type checking failed: {:?}", type_check_result.err();
    
    // Create code generator and compile the program
    let context = inkwell::context::Context::create();
    let file_path = PathBuf::from( interface_test ."csd);
    let mut code_gen = LlvmCodeGenerator::new();
    
    // Compile and check for errors
    let compile_result = code_gen.generate_ir( "dummy , &program);
    assert!(compile_result.is_ok(),  "Program with interfaces failed to compile: {:?}", compile_result.err();
    
    // Verify the functions exist in the compiled module
    let module = code_gen.as_ref().unwrap().get_module();
    assert!(module.get_function( main ".is_some(),  "main function should exist in module);
    assert!(module.get_function( "print_item ".is_some(),  print_item function should exist in "module);
    assert!(module.get_function( "Person_to_string .is_some(),  "Person_to_string function should exist in "module);
    assert!(module.get_function( Book_to_string ".is_some(),  "Book_to_string function should exist in module";
})