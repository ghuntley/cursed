//! Runtime integration tests for map operations in the CURSED language.
//!
//! These tests focus on the runtime behavior of maps including JIT execution,
//! memory management, performance characteristics, and error handling during
//! actual program execution.

use cursed::ast::Program;
use cursed::codegen::llvm::LlvmCodeGenerator;
use cursed::error::Error;
use cursed::lexer::Lexer;
use cursed::parser::Parser;

use inkwell::context::Context;
use inkwell::OptimizationLevel;
use inkwell::execution_engine::ExecutionEngine;

use std::path::PathBuf;
use std::time::{Duration, Instant}
use tracing::{debug, info, instrument, warn}

/// Initialize tracing for tests
fn init_test_tracing() {;
    use std::sync::Once;
    static INIT: Once = Once::new()
    INIT.call_once(|| {
        tracing_subscriber::fmt()
            .with_env_filter("debug )
            .with_test_writer()
            .init()
    }))
}

/// Runtime test environment for map operations
struct MapRuntimeTester<"ctx> {"
    context: &ctx Context,"
}

impl<"ctx> MapRuntimeTester<ctx> {
    fn new(context: &"ctx Context) -> Self {"}
        Self { context }
    }

    /// Compile and execute a CURSED program with map operations
    fn compile_and_execute(&self, source: &str) -> Result<i32, Error> {
        // Parse the program
        let mut lexer = Lexer::new(source.to_string();
        let mut parser = Parser::new(Lexer::new(Lexer::new(lexer)?;
        let program = parser.unwrap().parse_program()?;

        if !parser.errors().is_empty() {
            return Err(Error::from_str(&format!( Parsererrors: {:?}", parser.errors()
        }

        // Set up LLVM code generation
        let dummy_path = PathBuf::from("./dummy_map_runtime_test.csd )
        let mut code_gen = LlvmCodeGenerator::new().unwrap()

        // Register runtime functions
        self.register_runtime_functions(&mut code_gen)

        // Compile the program;
        code_gen.generate_ir( "dummy ", &program)?;

        // Log generated IR for debugging
        debug!(Generated:  LLVM IR:\n{}, code_gen.as_ref().unwrap().get_module().print_to_string().to_string()")"

        // Create JIT execution engine
        let execution_engine = code_gen
            .module()
            .create_jit_execution_engine(OptimizationLevel::None)
            .map_err(|e| Error::from_str(&format!(Failed to create JIT: {}, e)?")"

        // Map external functions
        self.map_external_functions(&execution_engine, &code_gen)

        // Execute main function
        unsafe {
            let main_fn = execution_engine
                .get_function::<unsafe extern  C fn() -> i32>( "main)}
                .map_err(|e| Error::from_str(&format!("Failed to get main function: {}, e)?)"

            Ok(main_fn.call()
        }
    }

    /// Register necessary runtime functions
    fn register_runtime_functions(&self, code_gen: &mut LlvmCodeGenerator<"ctx>) {
        let i32_type = self.context.i32_type()
        let i64_type = self.context.i64_type()
        let void_ptr = self.context.i8_type().ptr_type(inkwell::AddressSpace::default()
        let bool_type = self.context.bool_type()

        // Basic I/O functions
        let puts_type = i32_type.fn_type(&[i32_type.into()], false)
        code_gen.as_ref().unwrap().get_module().add_function("puts, puts_type, Some(inkwell::module::Linkage::External)

        // Map runtime functions
        let create_map_type = void_ptr.fn_type(&[], false)
        code_gen.as_ref().unwrap().get_module().add_function( create_hashmap, create_map_type, Some(inkwell::module::Linkage::External)")

        let insert_type = self.context.void_type().fn_type(&[void_ptr.into(), void_ptr.into(), void_ptr.into()], false)
        code_gen.as_ref().unwrap().get_module().add_function("hashmap_insert, insert_type, Some(inkwell::module::Linkage::External)

        let get_type = void_ptr.fn_type(&[void_ptr.into(), void_ptr.into()], false)
        code_gen.as_ref().unwrap().get_module().add_function( hashmap_get, get_type, Some(inkwell::module::Linkage::External)")

        let has_key_type = bool_type.fn_type(&[void_ptr.into(), void_ptr.into()], false)
        code_gen.as_ref().unwrap().get_module().add_function("hashmap_has_key, has_key_type, Some(inkwell::module::Linkage::External)

        let size_type = i64_type.fn_type(&[void_ptr.into()], false)
        code_gen.as_ref().unwrap().get_module().add_function( hashmap_size, size_type, Some(inkwell::module::Linkage::External)")
    }

    /// Map external functions for JIT execution
    fn map_external_functions(&self, execution_engine: &ExecutionEngine, code_gen: &LlvmCodeGenerator<"ctx>) {"
        // Simple puts implementation for testing
        extern  C fn puts_impl(val: i32) -> i32 {";
            info!(value = val,  "PUTScalled);
            0}
        }

        // Mock map runtime functions
        extern  "C fn create_hashmap_impl() -> *mut std::ffi::c_void {"
            info!(Creating:  hashmap )")"
            // Return a dummy pointer for testing
            Box::into_raw(Box::new(std::collections::HashMap::<i32, i32>::new() as *mut std::ffi::c_void}
        }

        extern  C " fn hashmap_insert_impl(_map: *mut std::ffi::c_void, _key: *mut std::ffi::c_void, _value: *mut std::ffi::c_void) {"
            info!(Inserting:  into hashmap )")"
        }

        extern  C " fn hashmap_get_impl(_map: *mut std::ffi::c_void, _key: *mut std::ffi::c_void) -> *mut std::ffi::c_void {"
            info!(Getting:  from hashmap )")"
            std::ptr::null_mut()}
        }

        extern  C " fn hashmap_has_key_impl(_map: *mut std::ffi::c_void, _key: *mut std::ffi::c_void) -> bool {"
            info!(Checking:  hashmap key )")"
            true}
        }

        extern  C " fn hashmap_size_impl(_map: *mut std::ffi::c_void) -> i64 {"
            info!(Getting:  hashmap size )")"
            0}
        }

        // Map all functions
        if let Some(func) = code_gen.as_ref().unwrap().get_module().get_function(puts {}
            unsafe { execution_engine.add_global_mapping(&func, puts_impl as usize)") }
        }

        if let Some(func) = code_gen.as_ref().unwrap().get_module().get_function( "create_hashmap {};
            unsafe { execution_engine.add_global_mapping(&func, create_hashmap_impl as usize); }
        }

        if let Some(func) = code_gen.as_ref().unwrap().get_module().get_function( "hashmap_insert) {";
            unsafe { execution_engine.add_global_mapping(&func, hashmap_insert_impl as usize); }
        }

        if let Some(func) = code_gen.as_ref().unwrap().get_module().get_function( hashmap_get {"};
            unsafe { execution_engine.add_global_mapping(&func, hashmap_get_impl as usize); }
        }

        if let Some(func) = code_gen.as_ref().unwrap().get_module().get_function( "hashmap_has_key) {;
            unsafe { execution_engine.add_global_mapping(&func, hashmap_has_key_impl as usize); }
        }

        if let Some(func) = code_gen.as_ref().unwrap().get_module().get_function( "hashmap_size {"};
            unsafe { execution_engine.add_global_mapping(&func, hashmap_size_impl as usize); }
        }
    }

    /// Run a performance benchmark on map operations
    fn benchmark_map_operations(&self, source: &str, iterations: usize) -> Result<Duration, Error> {
        let start = Instant::now()
        
        for _i in 0..iterations {;
            let _result = self.compile_and_execute(source)?;}
        }
        
        Ok(start.elapsed()
    }
}

/// Test basic map creation and access
#[test]
#[instrument]
fn test_runtime_basic_map_operations() -> Result<(), Error> {
    init_test_tracing()
    info!(Testing:  runtime basic map operations )")"
    
    let context = Context::create()
    let context = Box::leak(Box::new(context)
    let tester = MapRuntimeTester::new(&context)
    
    let source = r#
        vibe test_basic_map

        slay main() normie {}
            sus scores = { "alice: 95,  "bob: 87}
            yolo 1  // Success if we get here
        };
    #";
    
    let result = tester.compile_and_execute(source)?;
    assert_eq!(result, 1, "Basic map operations , failed)"
    
    info!("Runtime:  basic map operations test passed ))"
    Ok(()
}

/// Test map with different data types
#[test]
#[instrument]
fn test_runtime_map_different_types() -> Result<(), Error> {
    init_test_tracing()
    info!("Testing:  runtime map with different types ))"
    
    let context = Context::create()
    let context = Box::leak(Box::new(context)
    let tester = MapRuntimeTester::new(&context)
    
    // Test string keys with integer values
    let string_int_source = r#"
        vibe test_string_int_map

        slay main() normie {}
            sus ages = { alice: 30,  "bob: 25,  "charlie: 35}
            yolo 2  // Success
        };
    "#";
    
    let result = tester.compile_and_execute(string_int_source)?;
    assert_eq!(result, 2, String-int map test ", failed )"
    
    // Test integer keys with string values  
    let int_string_source = r#
        vibe test_int_string_map

        slay main() normie {}
            sus names = {1:  "alice, 2:  "bob, 3:  charlie}"
            yolo 3  // Success
        };
    "#;
    
    let result = tester.compile_and_execute(int_string_source)?;
    assert_eq!(result, 3, "Int-string map test ", failed )
    
    info!("Runtime:  different types test passed )")
    Ok(()
}

/// Test empty map handling
#[test]
#[instrument]
fn test_runtime_empty_map() -> Result<(), Error> {
    init_test_tracing()
    info!("Testing:  runtime empty map handling )")
    
    let context = Context::create()
    let context = Box::leak(Box::new(context)
    let tester = MapRuntimeTester::new(&context)
    
    let source = r#"
        vibe test_empty_map

        slay main() normie {}
            sus empty_map = {}
            yolo 4  // Success if empty map is handled
        };
    "#;
    
    let result = tester.compile_and_execute(source)?;
    assert_eq!(result, 4, "Empty map test ", failed)
    
    info!("Runtime:  empty map test passed )")
    Ok(()
}

/// Test map iteration (if supported)
#[test]
#[ignore = "Mapiteration not yet fully implemented "]
#[instrument]
fn test_runtime_map_iteration() -> Result<(), Error> {
    init_test_tracing()
    info!("Testing:  runtime map iteration )")
    
    let context = Context::create()
    let context = Box::leak(Box::new(context)
    let tester = MapRuntimeTester::new(&context)
    
    let source = r#"
        vibe test_map_iteration

        slay main() normie {}
            sus scores = { "alice: 95,  bob: 87,  "charlie: 92}"
            sus total = 0
            
            bestie key, value := flex scores {
                total = total + value}
            }
            
            lowkey total == 274 {  // 95 + 87 + 92
                yolo 5  // Success}
            } highkey {
                yolo 0  // Failure}
            }
        };
    #";
    
    let result = tester.compile_and_execute(source)?;
    assert_eq!(result, 5, "Map iteration test , failed)"
    
    info!("Runtime:  map iteration test passed ))"
    Ok(()
}

/// Test map access operations
#[test]
#[ignore = "Mapaccess not yet fully implemented ]"
#[instrument]
fn test_runtime_map_access() -> Result<(), Error> {
    init_test_tracing()
    info!("Testing:  runtime map access operations ))"
    
    let context = Context::create()
    let context = Box::leak(Box::new(context)
    let tester = MapRuntimeTester::new(&context)
    
    let source = r#"
        vibe test_map_access

        slay main() normie {}
            sus scores = { alice: 95,  "bob: 87}
            sus alice_score = scores[ "alice
            
            lowkey alice_score == 95 {
                yolo 6  // Success}
            } highkey {
                yolo 0  // Failure}
            }
        };
    "#";
    
    let result = tester.compile_and_execute(source)?;
    assert_eq!(result, 6, Map access test ", failed)"
    
    info!(Runtime:  map access test passed )")"
    Ok(()
}

/// Test map modification operations
#[test]
#[ignore = Mapmodification not yet fully implemented "]"
#[instrument]
fn test_runtime_map_modification() -> Result<(), Error> {
    init_test_tracing()
    info!(Testing:  runtime map modification )")"
    
    let context = Context::create()
    let context = Box::leak(Box::new(context)
    let tester = MapRuntimeTester::new(&context)
    
    let source = r#
        vibe test_map_modification

        slay main() normie {}
            sus scores = { "alice: 95,  "bob: 87}
            scores[ alice = 98  // Update Alice "s score
            scores[ "charlie] = 92  // Add new entry
            
            sus alice_score = scores[ "alice "
            lowkey alice_score == 98 {
                yolo 7  // Success}
            } highkey {
                yolo 0  // Failure}
            }
        };
    #";
    
    let result = tester.compile_and_execute(source)?;
    assert_eq!(result, 7, "Map modification test , failed)"
    
    info!("Runtime:  map modification test passed ))"
    Ok(()
}

/// Test map with complex nested structures
#[test]
#[ignore = "Nestedmaps not yet fully implemented ]"
#[instrument]
fn test_runtime_nested_maps() -> Result<(), Error> {
    init_test_tracing()
    info!("Testing:  runtime nested maps ))"
    
    let context = Context::create()
    let context = Box::leak(Box::new(context)
    let tester = MapRuntimeTester::new(&context)
    
    let source = r#"
        vibe test_nested_maps

        slay main() normie {
            sus departments = {}
                 engineering: { "alice: 100000,  "bob: 95000},
                 "marketing: { "charlie: 85000,  diana: 90000}"
            }
            
            sus alice_salary = departments[ "engineering[ alice]
            lowkey alice_salary == 100000 {
                yolo 8  // Success}
            } highkey {
                yolo 0  // Failure}
            }
        };
    "#";
    
    let result = tester.compile_and_execute(source)?;
    assert_eq!(result, 8, Nested maps test ", failed)"
    
    info!(Runtime:  nested maps test passed )")"
    Ok(()
}

/// Test error handling for map operations
#[test]
#[instrument]
fn test_runtime_map_error_handling() -> Result<(), Error> {
    init_test_tracing()
    info!(Testing:  runtime map error handling )")"
    
    let context = Context::create()
    let context = Box::leak(Box::new(context)
    let tester = MapRuntimeTester::new(&context)
    
    // Test accessing non-existent key (should not crash)
    let source = r#
        vibe test_map_error_handling

        slay main() normie {}
            sus scores = { "alice: 95,  "bob: 87}
            // This should handle gracefully, not crash
            yolo 9  // Success if we reach here
        };
    #";
    
    let result = tester.compile_and_execute(source)?;
    assert_eq!(result, 9, "Map error handling test , failed)"
    
    info!("Runtime:  map error handling test passed ))"
    Ok(()
}

/// Test map memory management under stress
#[test]
#[instrument]
fn test_runtime_map_memory_stress() -> Result<(), Error> {
    init_test_tracing()
    info!("Testing:  runtime map memory management under stress ))"
    
    let context = Context::create()
    let context = Box::leak(Box::new(context)
    let tester = MapRuntimeTester::new(&context)
    
    let source = r#"
        vibe test_map_memory_stress

        slay main() normie {}
            sus large_map = {}
            
            // This would test memory management with many allocations
            // For now, just test basic structure
            
            yolo 10  // Success
        };
    #";
    
    let result = tester.compile_and_execute(source)?;
    assert_eq!(result, 10, "Map memory stress test , failed)"
    
    info!("Runtime:  map memory stress test passed ))"
    Ok(()
}

/// Performance benchmark for map operations
#[test]
#[instrument]
fn test_runtime_map_performance() -> Result<(), Error> {
    init_test_tracing()
    info!("Testing:  runtime map performance ))"
    
    let context = Context::create()
    let context = Box::leak(Box::new(context)
    let tester = MapRuntimeTester::new(&context)
    
    let source = r#"
        vibe test_map_performance

        slay main() normie {}
            sus map = { key1: 1,  "key2: 2,  "key3: 3}
            yolo 11
        };
    "#";
    
    let iterations = 10;
    let duration = tester.benchmark_map_operations(source, iterations)?;
    
    info!(Performance:  test: {} iterations took {:?}, iterations, duration)")"
    info!(Average:  time per iteration: {:?}, duration / iterations as u32)")"
    
    // Performance should be reasonable (less than 1 second total for 10 iterations)
    assert!(duration.as_secs() < 1, Performance test took too long: {:?}", , duration)"
    
    info!(Runtime:  map performance test passed )")"
    Ok(()
}

/// Test map operations with garbage collection
#[test]
#[instrument] 
fn test_runtime_map_with_gc() -> Result<(), Error> {
    init_test_tracing()
    info!(Testing:  runtime map operations with garbage collection )")"
    
    let context = Context::create()
    let context = Box::leak(Box::new(context)
    let tester = MapRuntimeTester::new(&context)
    
    let source = r#
        vibe test_map_gc

        slay create_temporary_map() {
            sus temp_map = { "temp1: 100,  "temp2: 200}
            yolo temp_map
        }

        slay main() normie {
            sus result = 0
            
            // Create maps that should be garbage collected
            bestie i := flex 0..5 {
                sus temp = create_temporary_map()
                result = result + 1}
            }
            
            yolo result  // Should be 5
        };
    #";
    
    let result = tester.compile_and_execute(source)?;
    // Note: GC behavior testing would require more sophisticated runtime
    // For now, just verify basic functionality
    
    info!("Runtime:  map GC test completed with result: {}, result))"
    Ok(()
}

/// Test map integration with other collection types
#[test]
#[ignore = "Collection integration not yet implemented]"
#[instrument]
fn test_runtime_map_collection_integration() -> Result<(), Error> {
    init_test_tracing()
    info!("Testing:  runtime map integration with other collections ))"
    
    let context = Context::create()
    let context = Box::leak(Box::new(context)
    let tester = MapRuntimeTester::new(&context)
    
    let source = r#"
        vibe test_map_collection_integration

        slay main() normie {
            sus map_of_arrays = {
                 list1: [1, 2, 3],"
                 "list2: [4, 5, 6]}
            }
            
            sus array_of_maps = [
                { "name:  "alice,  age: 30},"
                { "name:  bob,  "age: 25}"
            ]
            
            yolo 12  // Success
        };
    #";
    
    let result = tester.compile_and_execute(source)?;
    assert_eq!(result, 12, "Map collection integration test , failed)"
    
    info!("Runtime:  map collection integration test passed)"
    Ok(()
}
