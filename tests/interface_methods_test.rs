use std::path::PathBuf;
use cursed::ast::*;
use cursed::lexer::Lexer;
use cursed::parser::Parser;
use cursed::core::type_checker::TypeChecker;
use cursed::codegen::llvm::LlvmCodeGenerator;
use cursed::error::Error;
use inkwell::context::Context;

// Test for interface methods implementation in the LLVM code generator


#[test]
fn test_basic_interface_methods() -> Result<(), Error> {
    // Define a simple program with an interface and implementation
    let source = r#"
    fr fr Interface method test
    
    collab Greeter {
        slay greet(tea name) tea;
    }
    
    squad Person {
        tea name;
        normie age;
    }
    
    slay Person_greet(Person self, tea name) tea {
        tea buffer[100];
        vibez sprintf(buffer, "Hello, %s! I'm %s.", name, self.name);
        yolo buffer;
    }
    
    slay main() normie {
        Person p = Person{name: "Alice", age: 30};
        Greeter g = p;
        
        tea msg = g.greet("Bob");
        vibez printf("%s\n", msg);
        
        yolo 0;
    }
    "#;
    
    // Parse program
    let mut lexer = Lexer::new(source);
    let mut parser = Parser::new(&mut lexer)?;
    let program = parser.parse_program()?;
    
    // Type check the program
    let mut type_checker = TypeChecker::new();
    type_checker.check_program(&program)?;
    
    // Generate code
    let context = Context::create();
    let file_path = PathBuf::from("interface_methods_test.csd");
    let mut code_gen = LlvmCodeGenerator::new(&context, "interface_methods_test", file_path);
    
    // Compile the program
    let compile_result = code_gen.compile_program(&program);
    assert!(compile_result.is_ok(), "Program with interface methods failed to compile: {:?}", compile_result.err());
    
    // Verify the functions exist in the compiled module
    let module = code_gen.module();
    assert!(module.get_function("main").is_some(), "main function should exist in module");
    assert!(module.get_function("Person_greet").is_some(), "Person_greet function should exist in module");
    
    Ok(())
}

#[test]
fn test_interface_type_assertion() -> Result<(), Error> {
    // Test program with interface type assertions
    let source = r#"
    fr fr Interface type assertion test
    
    collab Shape {
        slay area() meal;
    }
    
    squad Circle {
        meal radius;
    }
    
    slay Circle_area(Circle self) meal {
        yolo 3.14159 * self.radius * self.radius;
    }
    
    squad Rectangle {
        meal width;
        meal height;
    }
    
    slay Rectangle_area(Rectangle self) meal {
        yolo self.width * self.height;
    }
    
    slay process_shape(Shape shape) meal {
        yolo shape.area();
    }
    
    slay main() normie {
        Circle c = Circle{radius: 5.0};
        Rectangle r = Rectangle{width: 4.0, height: 3.0};
        
        Shape s1 = c;
        Shape s2 = r;
        
        fr fr Type assertions
        lit is_circle = s1 is Circle;
        if (is_circle) {
            Circle c2 = (Circle)s1;
            vibez printf("Circle area: %f\n", c2.area();
        }
        
        lit is_rectangle = s2 is Rectangle;
        if (is_rectangle) {
            Rectangle r2 = (Rectangle)s2;
            vibez printf("Rectangle area: %f\n", r2.area();
        }
        
        yolo 0;
    }
    "#;
    
    // Parse program
    let mut lexer = Lexer::new(source);
    let mut parser = Parser::new(&mut lexer)?;
    let program = parser.parse_program()?;
    
    // Type check the program
    let mut type_checker = TypeChecker::new();
    type_checker.check_program(&program)?;
    
    // Generate code
    let context = Context::create();
    let file_path = PathBuf::from("interface_assertion_test.csd");
    let mut code_gen = LlvmCodeGenerator::new(&context, "interface_assertion_test", file_path);
    
    // Compile the program
    let compile_result = code_gen.compile_program(&program);
    assert!(compile_result.is_ok(), "Program with interface type assertions failed to compile: {:?}", compile_result.err());
    
    Ok(())
}