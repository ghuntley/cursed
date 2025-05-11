//! # Interface Type Assertion with Diamond Inheritance Test
//!
//! This module tests the enhanced interface type assertion implementation
//! with proper error propagation and support for diamond inheritance patterns.

use std::env;
use std::path::PathBuf;

#[path = "common.rs"]
mod common;

use common::tracing;
use tracing::{debug, error, info, trace, warn};

use cursed::ast::types::{InterfaceType, StructType, Type};
use cursed::ast::expressions::TypeAssertion;
use cursed::parser::Parser;
use cursed::codegen::llvm::LlvmCodeGenerator;
use cursed::core::jit::JitCompiler;
use cursed::error::Error;

/// Initialize tracing for tests
macro_rules! init_tracing {
    () => {
        common::tracing::setup();
    };
}

/// Test diamond inheritance pattern with type assertions
#[test]
fn test_diamond_inheritance_type_assertion() {
    init_tracing!();
    info!(test_case = "diamond_inheritance_type_assertion", "Starting test");
    
    let _timer = common::timing::Timer::new("diamond_inheritance_type_assertion");
    
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

/// Test diamond inheritance with proper error handling
#[test]
fn test_diamond_inheritance_with_error_handling() {
    init_tracing!();
    info!(test_case = "diamond_inheritance_with_error_handling", "Starting test");
    
    let _timer = common::timing::Timer::new("diamond_inheritance_with_error_handling");
    
    let source = r#"
        vibe main;
        
        tea Animal {
            bruh MakeSound() tea;
        }
        
        tea Mammal tea Animal {
            bruh WarmBlooded() lit;
        }
        
        tea Carnivore tea Animal {
            bruh EatsMeat() lit;
        }
        
        tea Lion tea Mammal, Carnivore {
            bruh Roar() tea;
        }
        
        tea Bird tea Animal {
            bruh HasFeathers() lit;
        }
        
        struct LionImpl struct {
            sus name tea;
        }
        
        bruh (l LionImpl) MakeSound() tea {
            return "Roar";
        }
        
        bruh (l LionImpl) WarmBlooded() lit {
            return based;
        }
        
        bruh (l LionImpl) EatsMeat() lit {
            return based;
        }
        
        bruh (l LionImpl) Roar() tea {
            return "ROAR!!!";
        }
        
        struct Eagle struct {
            sus wingspan meal;
        }
        
        bruh (e Eagle) MakeSound() tea {
            return "Screech";
        }
        
        bruh (e Eagle) HasFeathers() lit {
            return based;
        }
        
        slay testAssertions() tea {
            sus lion LionImpl = LionImpl{name: "Simba"};
            sus eagle Eagle = Eagle{wingspan: 2.0};
            
            sus animal1 Animal = lion;
            sus animal2 Animal = eagle;
            
            // Valid assertions through diamond hierarchy
            sus mammal Mammal = animal1.(Mammal);
            sus carnivore Carnivore = animal1.(Carnivore);
            sus lionAgain Lion = carnivore.(Lion);
            
            // Cross-interface assertions
            sus carnivoreFromMammal Carnivore = mammal.(Carnivore);
            
            // Error handling for invalid assertions
            sus result tea = "Success";
            
            // Attempt invalid assertions with proper error handling
            captcha {
                sus invalidBird Bird = animal1.(Bird);
                result = "Error: Bird assertion should have failed";
            } drip (e) {
                result = "Caught invalid assertion: " + e.message;
            }
            
            // Try invalid assertion through diamond hierarchy
            captcha {
                sus invalidLion Lion = animal2.(Lion); 
                result = result + ", Error: Lion assertion should have failed";
            } drip (e) {
                result = result + ", Caught invalid Lion assertion";
            }
            
            return result;
        }
        
        slay main() void {
            sus result tea = testAssertions();
            debug("Test result: %s", result);
            
            if !result.contains("Caught invalid assertion") || !result.contains("Caught invalid Lion assertion") {
                debug("Error: Failed to catch invalid assertions properly");
                poppin();
            }
        }
    "#;
    
    match compile_and_run(source) {
        Ok(_) => {
            info!(test_result = "success", "Diamond inheritance with error handling works correctly");
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
    
    Ok()
}