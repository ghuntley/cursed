use std::sync::Once;

// We need to call init_test_tracing only once
static INIT: Once = Once::new();

#[path = "tracing_setup.rs"]
pub mod tracing_setup;

// Macro for initializing tracing in tests
macro_rules! init_tracing {
    () => {
        INIT.call_once(|| {
            tracing_setup::init_test_tracing();
        });
    };
}

// Import required test utilities
use cursed::lexer::Lexer;
use cursed::parser::Parser;
use cursed::object::{Object, ObjectRef};
use cursed::core::{JitOptions, InterpretOptions};

// Helper function to run JIT tests on Cursed code
fn run_jit_test(input: &str) -> Result<ObjectRef, String> {
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program()?;
    
    // Check for parser errors
    if !parser.errors().is_empty() {
        let error_msg = parser.errors().join("\n");
        return Err(format!("Parser errors:\n{}", error_msg));
    }
    
    // Run the program with default JIT options
    let options = JitOptions::default()
        .with_main_args(vec![]);
        
    let result = cursed::code::jit_compile_and_run(&program, options)?;
    Ok(result)
}

#[test]
fn test_interface_type_assertion_runtime_checking() {
    init_tracing!();
    
    // Define a program that performs complex type assertions
    let input = r#"
        // Define a base interface
        collab Valuable {
            value() lit;
        }
        
        // Define derived interfaces
        collab NumericValuable {
            value() lit;
            numericType() tea;
        }
        
        collab StringValuable {
            value() lit;
            stringType() tea;
        }
        
        // Define concrete types
        squad IntValue {
            val lit
        }
        
        squad FloatValue {
            val snack
        }
        
        squad StringValue {
            val tea,
            length lit
        }
        
        // Implement interfaces for concrete types
        slay (i IntValue) value() lit {
            return i.val
        }
        
        slay (i IntValue) numericType() tea {
            return "integer"
        }
        
        slay (f FloatValue) value() lit {
            return f.val
        }
        
        slay (f FloatValue) numericType() tea {
            return "float"
        }
        
        slay (s StringValue) value() lit {
            return s.length
        }
        
        slay (s StringValue) stringType() tea {
            return "string"
        }
        
        // Function that tests type assertions with runtime checking
        slay testTypeAssertions() tea {
            // Create values of different types
            sus i = IntValue{val: 42}
            sus f = FloatValue{val: 3.14}
            sus s = StringValue{val: "hello", length: 5}
            
            // Assign to base interface
            sus v1 Valuable = i
            sus v2 Valuable = f
            sus v3 Valuable = s
            
            // Successful assertions
            sus i1, iOk = v1.(IntValue)
            sus f1, fOk = v2.(FloatValue)
            sus s1, sOk = v3.(StringValue)
            
            // Failed assertions - different concrete types
            sus i2, i2Ok = v2.(IntValue)  // Should fail, v2 is FloatValue
            sus f2, f2Ok = v3.(FloatValue)  // Should fail, v3 is StringValue
            sus s2, s2Ok = v1.(StringValue)  // Should fail, v1 is IntValue
            
            // Interface assertions
            sus n1, n1Ok = v1.(NumericValuable)  // Should succeed, IntValue implements NumericValuable
            sus n2, n2Ok = v2.(NumericValuable)  // Should succeed, FloatValue implements NumericValuable
            sus n3, n3Ok = v3.(NumericValuable)  // Should fail, StringValue doesn't implement NumericValuable
            
            sus str1, str1Ok = v1.(StringValuable)  // Should fail, IntValue doesn't implement StringValuable
            sus str2, str2Ok = v3.(StringValuable)  // Should succeed, StringValue implements StringValuable
            
            // Format result string showing all test outcomes
            sus result = ""
            result = result + "IntValue assertion: " + vibe.yoloBool(iOk) + "\n"
            result = result + "FloatValue assertion: " + vibe.yoloBool(fOk) + "\n"
            result = result + "StringValue assertion: " + vibe.yoloBool(sOk) + "\n"
            result = result + "Failed IntValue assertion: " + vibe.yoloBool(!i2Ok) + "\n"
            result = result + "Failed FloatValue assertion: " + vibe.yoloBool(!f2Ok) + "\n"
            result = result + "Failed StringValue assertion: " + vibe.yoloBool(!s2Ok) + "\n"
            result = result + "NumericValuable from IntValue: " + vibe.yoloBool(n1Ok) + "\n"
            result = result + "NumericValuable from FloatValue: " + vibe.yoloBool(n2Ok) + "\n"
            result = result + "NumericValuable from StringValue: " + vibe.yoloBool(!n3Ok) + "\n"
            result = result + "StringValuable from IntValue: " + vibe.yoloBool(!str1Ok) + "\n"
            result = result + "StringValuable from StringValue: " + vibe.yoloBool(str2Ok) + "\n"
            
            return result
        }
        
        // Main function
        slay main() tea {
            return testTypeAssertions()
        }
    "#;
    
    // Run the test and verify results
    match run_jit_test(input) {
        Ok(result) => {
            let out = result.as_string().unwrap();
            println!("Test output: {}", out);
            
            // We expect all tests to pass
            assert!(out.contains("IntValue assertion: true"));
            assert!(out.contains("FloatValue assertion: true"));
            assert!(out.contains("StringValue assertion: true"));
            assert!(out.contains("Failed IntValue assertion: true"));
            assert!(out.contains("Failed FloatValue assertion: true"));
            assert!(out.contains("Failed StringValue assertion: true"));
            assert!(out.contains("NumericValuable from IntValue: true"));
            assert!(out.contains("NumericValuable from FloatValue: true"));
            assert!(out.contains("NumericValuable from StringValue: true"));
            assert!(out.contains("StringValuable from IntValue: true"));
            assert!(out.contains("StringValuable from StringValue: true"));
        },
        Err(e) => panic!("Failed to run test: {}", e),
    }
}

#[test]
fn test_interface_type_assertion_null_handling() {
    init_tracing!();
    
    // Test handling of null/cap interface values
    let input = r#"
        // Define an interface
        collab Processor {
            process(data tea) tea;
        }
        
        // Define a concrete type
        squad DataProcessor {
            prefix tea
        }
        
        // Implement interface
        slay (dp DataProcessor) process(data tea) tea {
            return dp.prefix + ": " + data
        }
        
        // Function to test null interface handling
        slay testNullInterface() tea {
            // Initialize a nil interface value
            sus p Processor = cap
            
            // Attempt type assertion on nil interface
            sus dp, ok = p.(DataProcessor)
            
            if ok {
                return "FAIL: Type assertion on nil interface should fail"
            } else {
                return "PASS: Type assertion on nil interface correctly failed"
            }
        }
        
        // Main function
        slay main() tea {
            return testNullInterface()
        }
    "#;
    
    // Run the test and verify results
    match run_jit_test(input) {
        Ok(result) => {
            let out = result.as_string().unwrap();
            assert!(out.contains("PASS"));
            assert!(!out.contains("FAIL"));
        },
        Err(e) => panic!("Failed to run test: {}", e),
    }
}

#[test]
fn test_interface_type_assertion_deep_hierarchy() {
    init_tracing!();
    
    // Test with deep interface hierarchy
    let input = r#"
        // Define interface hierarchy
        collab Entity {
            id() lit;
        }
        
        collab Named {
            name() tea;
        }
        
        collab Person {
            id() lit;
            name() tea;
            age() lit;
        }
        
        collab Employee {
            id() lit;
            name() tea;
            age() lit;
            salary() lit;
        }
        
        // Define concrete implementation
        squad Worker {
            workerId lit,
            workerName tea,
            workerAge lit,
            workerSalary lit
        }
        
        // Implement all interfaces
        slay (w Worker) id() lit {
            return w.workerId
        }
        
        slay (w Worker) name() tea {
            return w.workerName
        }
        
        slay (w Worker) age() lit {
            return w.workerAge
        }
        
        slay (w Worker) salary() lit {
            return w.workerSalary
        }
        
        // Function to test interface hierarchy
        slay testHierarchy() tea {
            // Create a Worker instance
            sus w = Worker{
                workerId: 101,
                workerName: "John",
                workerAge: 30,
                workerSalary: 50000
            }
            
            // Test assertions for different levels of the hierarchy
            sus e Entity = w
            sus n Named = w
            sus p Person = w
            sus emp Employee = w
            
            // Test downcasting from various interface levels
            sus worker1, ok1 = e.(Worker)    // Entity -> Worker
            sus worker2, ok2 = n.(Worker)    // Named -> Worker
            sus worker3, ok3 = p.(Worker)    // Person -> Worker
            sus worker4, ok4 = emp.(Worker)  // Employee -> Worker
            
            // Test interface-to-interface assertions
            sus person1, pok1 = e.(Person)    // Entity -> Person
            sus person2, pok2 = n.(Person)    // Named -> Person
            sus employee1, eok1 = p.(Employee)  // Person -> Employee
            
            // Format result string
            sus result = ""
            result = result + "Entity to Worker: " + vibe.yoloBool(ok1) + "\n"
            result = result + "Named to Worker: " + vibe.yoloBool(ok2) + "\n"
            result = result + "Person to Worker: " + vibe.yoloBool(ok3) + "\n"
            result = result + "Employee to Worker: " + vibe.yoloBool(ok4) + "\n"
            result = result + "Entity to Person: " + vibe.yoloBool(pok1) + "\n"
            result = result + "Named to Person: " + vibe.yoloBool(pok2) + "\n"
            result = result + "Person to Employee: " + vibe.yoloBool(eok1) + "\n"
            
            return result
        }
        
        // Main function
        slay main() tea {
            return testHierarchy()
        }
    "#;
    
    // Run the test and verify results
    match run_jit_test(input) {
        Ok(result) => {
            let out = result.as_string().unwrap();
            
            // All assertions should succeed
            assert!(out.contains("Entity to Worker: true"));
            assert!(out.contains("Named to Worker: true"));
            assert!(out.contains("Person to Worker: true"));
            assert!(out.contains("Employee to Worker: true"));
            assert!(out.contains("Entity to Person: true"));
            assert!(out.contains("Named to Person: true"));
            assert!(out.contains("Person to Employee: true"));
        },
        Err(e) => panic!("Failed to run test: {}", e),
    }
}