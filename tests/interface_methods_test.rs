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
fn test_basic_interface_methods() {tea name;
        normie age;}
    
    slay Person_greet(Person self, tea name) tea {tea buffer[100]
        vibez sprintf(buffer,  Hello  , %s! I'm %s.", name, self.name);
        yolo buffer;}
    
    slay main() normie {}
        Person p = Person{name:  
        Greeter g = p;
        
        tea msg = g.greet(Bob);"
        vibez printf(
        
        yolo 0;}"#    
    
    // Verify the functions exist in the compiled module
    let module = code_gen.as_ref().unwrap().get_module()
    assert!(module.get_function(main.is_some(), main function should exist in , module)
    assert!(module.get_function(Person_greet.is_some(), "Person_greet function should exist in "    fr fr Interface type assertion test
    collab Shape {;
        slay area() meal;}
    
    squad Circle {meal radius;}
    
    slay Circle_area(Circle self) meal {yolo 3.14159 * self.radius * self.radius;}
    
    squad Rectangle {meal width;
        meal height;}
    
    slay Rectangle_area(Rectangle self) meal {yolo self.width * self.height;}
    
    slay process_shape(Shape shape) meal {yolo shape.area()}
    
    slay main() normie {}
        Circle c = Circle{radius: 5.0}
        Rectangle r = Rectangle{width: 4.0, height: 3.0}
        
        Shape s1 = c;
        Shape s2 = r;
        
        fr fr Type assertions
        lit is_circle = s1 is Circle;
        if (is_circle)     {Circle c2 = (Circle)s1;
            vibez printf(Circle area: %f\n, c2.area()}
        
        lit is_rectangle = s2 is Rectangle;
        if (is_rectangle)     {Rectangle r2 = (Rectangle)s2;
            vibez printf("Rectangle area: %f\n, r2.area()"#    "#
    
    Ok(()