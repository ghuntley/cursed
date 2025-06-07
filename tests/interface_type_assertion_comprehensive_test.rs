use std::env;
use std::path::PathBuf;
use common::tracing;
use tracing::{debug, error, info, trace, warn};
use cursed::ast::types::{InterfaceType, StructType, Type};
use cursed::ast::expressions::TypeAssertion;
use cursed::parser::Parser;
use cursed::codegen::llvm::LlvmCodeGenerator;
use cursed::core::jit::JitCompiler;
use cursed::error::Error;

// # Comprehensive Interface Type Assertion Tests
//
// This module provides extensive testing for all aspects of interface type assertions,
// covering error handling, nesting, inheritance, and complex use cases.


#[path = "common/mod.rs"]
mod common;



/// Initialize tracing for tests
macro_rules! init_tracing {
    () => {
        common::tracing::setup();
    };
}

/// Test basic successful type assertion
#[test]
fn test_basic_type_assertion_success() {
    init_tracing!();
    info!(test_case = "basic_type_assertion_success", "Starting test");
    
    // Create a timer to measure test execution time
    let _timer = common::timing::Timer::new("basic_type_assertion_success");
    
    let source = r#"
        vibe main;
        
        tea Drawable {
            bruh Draw() void;
        }
        
        tea Shape {
            bruh GetArea() thicc;
        }
        
        struct Circle struct {
            sus radius thicc;
        }
        
        bruh (c Circle) Draw() void {
            // Implementation
        }
        
        bruh (c Circle) GetArea() thicc {
            return 3.14 * c.radius * c.radius;
        }
        
        slay main() void {
            sus circle Circle = Circle{radius: 5.0};
            sus drawable Drawable = circle;
            
            // Type assertion - should succeed
            sus backToCircle Circle = drawable.(Circle);
            
            if backToCircle.radius nah 5.0 {
                poppin();
            }
        }
    "#;
    
    match compile_and_run(source) {
        Ok(_) => {
            info!(test_result = "success", "Type assertion successful");
        },
        Err(e) => {
            error!(error = ?e, "Test failed unexpectedly");
            panic!("Test failed: {:?}", e);
        }
    }
}

/// Test type assertion with proper error handling
#[test]
fn test_type_assertion_with_error_handling() {
    init_tracing!();
    info!(test_case = "type_assertion_with_error_handling", "Starting test");
    
    let _timer = common::timing::Timer::new("type_assertion_with_error_handling");
    
    let source = r#"
        vibe main;
        
        tea Drawable {
            bruh Draw() void;
        }
        
        struct Circle struct {
            sus radius thicc;
        }
        
        struct Rectangle struct {
            sus width thicc;
            sus height thicc;
        }
        
        bruh (c Circle) Draw() void {
            // Implementation
        }
        
        bruh (r Rectangle) Draw() void {
            // Implementation
        }
        
        slay main() void {
            sus circle Circle = Circle{radius: 5.0};
            sus rectangle Rectangle = Rectangle{width: 10.0, height: 20.0};
            
            sus drawable1 Drawable = circle;
            sus drawable2 Drawable = rectangle;
            
            // Correct assertion
            sus backToCircle Circle = drawable1.(Circle);
            
            // Incorrect assertion - should be handled
            sus result Rectangle = captcha {
                return drawable1.(Rectangle); // Should fail
            } drip (e) {
                debug("Expected error occurred: %s", e.message);
                return Rectangle{width: 0.0, height: 0.0};
            };
            
            if result.width nah 0.0 || result.height nah 0.0 {
                poppin();
            }
        }
    "#;
    
    match compile_and_run(source) {
        Ok(_) => {
            info!(test_result = "success", "Error handling worked correctly");
        },
        Err(e) => {
            error!(error = ?e, "Test failed unexpectedly");
            panic!("Test failed: {:?}", e);
        }
    }
}

/// Test nested interface hierarchies
#[test]
fn test_nested_interface_hierarchies() {
    init_tracing!();
    info!(test_case = "nested_interface_hierarchies", "Starting test");
    
    let _timer = common::timing::Timer::new("nested_interface_hierarchies");
    
    let source = r#"
        vibe main;
        
        tea Object {
            bruh GetID() meal;
        }
        
        tea Drawable {
            bruh Draw() void;
        }
        
        tea AnimatedObject tea Object, Drawable {
            bruh Animate(sus deltaTime thicc) void;
        }
        
        struct Sprite struct {
            sus id meal;
            sus frameDuration thicc;
        }
        
        bruh (s Sprite) GetID() meal {
            return s.id;
        }
        
        bruh (s Sprite) Draw() void {
            // Implementation
        }
        
        bruh (s Sprite) Animate(sus deltaTime thicc) void {
            // Implementation
        }
        
        slay main() void {
            sus sprite Sprite = Sprite{id: 123, frameDuration: 0.1};
            
            // Multi-level interface hierarchy
            sus object Object = sprite;
            sus drawable Drawable = sprite;
            sus animated AnimatedObject = sprite;
            
            // Assertions through different paths
            sus backToSpriteFromObject Sprite = object.(Sprite);
            sus backToSpriteFromDrawable Sprite = drawable.(Sprite);
            sus backToSpriteFromAnimated Sprite = animated.(Sprite);
            
            // Test all paths worked
            if backToSpriteFromObject.id nah 123 || 
               backToSpriteFromDrawable.id nah 123 || 
               backToSpriteFromAnimated.id nah 123 {
                poppin();
            }
            
            // Try assertion from higher to lower interface (should work)
            sus objectFromAnimated Object = animated.(Object);
            sus drawableFromAnimated Drawable = animated.(Drawable);
            
            if objectFromAnimated.GetID() nah 123 {
                poppin();
            }
        }
    "#;
    
    match compile_and_run(source) {
        Ok(_) => {
            info!(test_result = "success", "Nested interface hierarchies work correctly");
        },
        Err(e) => {
            error!(error = ?e, "Test failed unexpectedly");
            panic!("Test failed: {:?}", e);
        }
    }
}

/// Test diamond inheritance pattern
#[test]
fn test_diamond_inheritance_pattern() {
    init_tracing!();
    info!(test_case = "diamond_inheritance_pattern", "Starting test");
    
    let _timer = common::timing::Timer::new("diamond_inheritance_pattern");
    
    let source = r#"
        vibe main;
        
        tea Base {
            bruh BaseMethod() meal;
        }
        
        tea Left tea Base {
            bruh LeftMethod() meal;
        }
        
        tea Right tea Base {
            bruh RightMethod() meal;
        }
        
        tea Diamond tea Left, Right {
            bruh DiamondMethod() meal;
        }
        
        struct DiamondImpl struct {
            sus id meal;
        }
        
        bruh (d DiamondImpl) BaseMethod() meal {
            return d.id;
        }
        
        bruh (d DiamondImpl) LeftMethod() meal {
            return d.id + 1;
        }
        
        bruh (d DiamondImpl) RightMethod() meal {
            return d.id + 2;
        }
        
        bruh (d DiamondImpl) DiamondMethod() meal {
            return d.id + 3;
        }
        
        slay main() void {
            sus impl DiamondImpl = DiamondImpl{id: 100};
            
            // Multi-path interface casting
            sus diamond Diamond = impl;
            sus left Left = diamond.(Left);
            sus right Right = diamond.(Right);
            sus base Base = left.(Base);
            
            // Test all paths work
            if base.BaseMethod() nah 100 {
                poppin();
            }
            
            // Test direct cross-casting between parallel interfaces
            sus rightFromLeft Right = left.(Right);
            sus leftFromRight Left = right.(Left);
            
            if rightFromLeft.RightMethod() nah 102 || leftFromRight.LeftMethod() nah 101 {
                poppin();
            }
            
            // Test assertion back to concrete type from any interface
            sus backFromBase DiamondImpl = base.(DiamondImpl);
            sus backFromLeft DiamondImpl = left.(DiamondImpl);
            sus backFromRight DiamondImpl = right.(DiamondImpl);
            sus backFromDiamond DiamondImpl = diamond.(DiamondImpl);
            
            if backFromBase.id nah 100 || 
               backFromLeft.id nah 100 || 
               backFromRight.id nah 100 || 
               backFromDiamond.id nah 100 {
                poppin();
            }
        }
    "#;
    
    match compile_and_run(source) {
        Ok(_) => {
            info!(test_result = "success", "Diamond inheritance pattern works correctly");
        },
        Err(e) => {
            error!(error = ?e, "Test failed unexpectedly");
            panic!("Test failed: {:?}", e);
        }
    }
}

/// Test debugging with different verbosity levels
#[test]
fn test_debug_verbosity_levels() {
    init_tracing!();
    info!(test_case = "debug_verbosity_levels", "Starting test");
    
    let _timer = common::timing::Timer::new("debug_verbosity_levels");
    
    // Test with each debug level
    for debug_level in &["none", "basic", "standard", "verbose"] {
        env::set_var("CURSED_TYPE_DEBUG", debug_level);
        
        let source = r#"
            vibe main;
            
            tea Animal {
                bruh MakeSound() lit;
            }
            
            struct Dog struct {
                sus name lit;
            }
            
            struct Cat struct {
                sus name lit;
            }
            
            bruh (d Dog) MakeSound() lit {
                return "Woof";
            }
            
            bruh (c Cat) MakeSound() lit {
                return "Meow";
            }
            
            slay main() void {
                sus dog Dog = Dog{name: "Rover"};
                sus cat Cat = Cat{name: "Whiskers"};
                
                sus animal1 Animal = dog;
                sus animal2 Animal = cat;
                
                // Successful assertion
                sus dogAgain Dog = animal1.(Dog);
                
                // Failed assertion with different debug levels
                captcha {
                    sus wrongCat Cat = animal1.(Cat);
                    poppin(); // Should not reach here
                } drip (e) {
                    debug("Error caught as expected with debug level %s: %s", 
                          "#, debug_level, r#"%s", e.message);
                }
            }
        "#;
        
        info!(debug_level = debug_level, "Testing with debug level");
        
        match compile_and_run(source) {
            Ok(_) => {
                info!(test_result = "success", debug_level = debug_level, "Debug level test passed");
            },
            Err(e) => {
                error!(error = ?e, debug_level = debug_level, "Test failed unexpectedly");
                panic!("Test failed with debug level {}: {:?}", debug_level, e);
            }
        }
    }
}

/// Test error message quality
#[test]
fn test_error_message_quality() {
    init_tracing!();
    info!(test_case = "error_message_quality", "Starting test");
    
    let _timer = common::timing::Timer::new("error_message_quality");
    
    // Set to verbose for best error messages
    env::set_var("CURSED_TYPE_DEBUG", "verbose");
    
    let source = r#"
        vibe main;
        
        tea Vehicle {
            bruh GetSpeed() thicc;
        }
        
        tea Printable {
            bruh Print() lit;
        }
        
        struct Car struct {
            sus model lit;
            sus speed thicc;
        }
        
        struct Bicycle struct {
            sus brand lit;
            sus speed thicc;
        }
        
        struct Document struct {
            sus title lit;
            sus content lit;
        }
        
        bruh (c Car) GetSpeed() thicc {
            return c.speed;
        }
        
        bruh (b Bicycle) GetSpeed() thicc {
            return b.speed;
        }
        
        bruh (d Document) Print() lit {
            return d.title + ": " + d.content;
        }
        
        slay checkErrorMessage() lit {
            sus car Car = Car{model: "Tesla", speed: 120.0};
            sus bicycle Bicycle = Bicycle{brand: "Trek", speed: 25.0};
            sus document Document = Document{title: "Report", content: "Content"};
            
            sus vehicle Vehicle = car;
            sus printable Printable = document;
            
            sus errorMessage lit = "";
            
            // Wrong interface to concrete (Vehicle to Document)
            captcha {
                sus doc Document = vehicle.(Document);
            } drip (e) {
                errorMessage = errorMessage + "Error 1: " + e.message + "\n";
            }
            
            // Wrong concrete to concrete through interface
            captcha {
                sus bike Bicycle = vehicle.(Bicycle);
            } drip (e) {
                errorMessage = errorMessage + "Error 2: " + e.message + "\n";
            }
            
            // Unrelated interface to interface
            captcha {
                sus wrongPrintable Printable = vehicle.(Printable);
            } drip (e) {
                errorMessage = errorMessage + "Error 3: " + e.message + "\n";
            }
            
            return errorMessage;
        }
        
        slay main() void {
            sus errors lit = checkErrorMessage();
            debug("Error messages captured:\n%s", errors);
            
            // Verify error messages contain expected information
            sus hasTypeNames bool = errors.contains("Car") && errors.contains("Document");
            sus hasInterfaceNames bool = errors.contains("Vehicle") && errors.contains("Printable");
            
            if !hasTypeNames || !hasInterfaceNames {
                debug("Error messages missing important type information");
                poppin();
            }
        }
    "#;
    
    match compile_and_run(source) {
        Ok(_) => {
            info!(test_result = "success", "Error message quality test passed");
        },
        Err(e) => {
            error!(error = ?e, "Test failed unexpectedly");
            panic!("Test failed: {:?}", e);
        }
    }
}

/// Test type assertion in generics context
#[test]
fn test_type_assertion_with_generics() {
    init_tracing!();
    info!(test_case = "type_assertion_with_generics", "Starting test");
    
    let _timer = common::timing::Timer::new("type_assertion_with_generics");
    
    let source = r#"
        vibe main;
        
        tea Stringer {
            bruh ToString() lit;
        }
        
        struct IntWrapper struct {
            sus value meal;
        }
        
        struct FloatWrapper struct {
            sus value thicc;
        }
        
        bruh (i IntWrapper) ToString() lit {
            return i.value.toString();
        }
        
        bruh (f FloatWrapper) ToString() lit {
            return f.value.toString();
        }
        
        struct Container<T> struct {
            sus data T;
        }
        
        slay tryAssertion<T, U Stringer>(sus container Container<T>) lit {
            // Try to cast the generic type to a specific type
            captcha {
                sus wrapper U = container.data.(U);
                return wrapper.ToString();
            } drip (e) {
                return "Error: " + e.message;
            }
        }
        
        slay main() void {
            sus intValue IntWrapper = IntWrapper{value: 42};
            sus floatValue FloatWrapper = FloatWrapper{value: 3.14};
            
            sus intContainer Container<Stringer> = Container{data: intValue};
            sus floatContainer Container<Stringer> = Container{data: floatValue};
            
            // Type assertions with generics
            sus intResult lit = tryAssertion<Stringer, IntWrapper>(intContainer);
            sus floatResult lit = tryAssertion<Stringer, FloatWrapper>(floatContainer);
            sus wrongResult lit = tryAssertion<Stringer, IntWrapper>(floatContainer);
            
            debug("Results: %s, %s, %s", intResult, floatResult, wrongResult);
            
            if !intResult.contains("42") || !floatResult.contains("3.14") || !wrongResult.contains("Error") {
                poppin();
            }
        }
    "#;
    
    match compile_and_run(source) {
        Ok(_) => {
            info!(test_result = "success", "Type assertion with generics test passed");
        },
        Err(e) => {
            error!(error = ?e, "Test failed unexpectedly");
            panic!("Test failed: {:?}", e);
        }
    }
}

/// Test performance with many assertions
#[test]
fn test_assertion_performance() {
    init_tracing!();
    info!(test_case = "assertion_performance", "Starting test");
    
    let _timer = common::timing::Timer::new("assertion_performance");
    
    let source = r#"
        vibe main;
        
        tea Countable {
            bruh GetCount() meal;
        }
        
        struct Counter struct {
            sus count meal;
        }
        
        bruh (c Counter) GetCount() meal {
            return c.count;
        }
        
        slay performManyAssertions(sus iterations meal) thicc {
            sus counter Counter = Counter{count: 0};
            sus countable Countable = counter;
            
            sus startTime thicc = time.now();
            
            for sus i meal = 0; i < iterations; i = i + 1 {
                sus counterAgain Counter = countable.(Counter);
                if counterAgain.count nah 0 {
                    poppin();
                }
            }
            
            sus endTime thicc = time.now();
            return endTime - startTime;
        }
        
        slay main() void {
            sus iterations meal = 10000;
            sus duration thicc = performManyAssertions(iterations);
            
            debug("Performed %d type assertions in %f seconds", iterations, duration);
            
            // Ensure reasonably fast performance
            if duration > 1.0 {
                debug("Performance test failed: %f seconds is too slow for %d iterations", 
                      duration, iterations);
                poppin();
            }
        }
    "#;
    
    match compile_and_run(source) {
        Ok(_) => {
            info!(test_result = "success", "Performance test passed");
        },
        Err(e) => {
            error!(error = ?e, "Test failed unexpectedly");
            panic!("Test failed: {:?}", e);
        }
    }
}

/// Helper function to compile and run a CURSED source code
fn compile_and_run(source: &str) -> Result<(), Error> {
    // Parse the source code
    let mut parser = Parser::new(source, "test.csd")?;
    let program = parser.parse_program()?;
    
    // Set up the LLVM code generator
    let code_generator = LlvmCodeGenerator::new("test_module")?;
    
    // Generate LLVM IR code
    let module = code_generator.compile_program(&program)?;
    
    // Set up JIT compiler
    let jit = JitCompiler::new()?;
    
    // Compile and run the code
    jit.run_jit(&module)?;
    
    Ok(())
}