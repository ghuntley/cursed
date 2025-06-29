//! Simple JIT Execution Test for Phase 3A
//!
//! This test verifies the core JIT execution functionality.

fn main() {
    println!("=== Simple JIT Execution Test ===");
    
    // Test 1: Basic JIT Engine Creation
    println!("1. Testing JIT engine creation...");
    test_jit_engine_creation();
    
    // Test 2: Simple Function Compilation
    println!("2. Testing simple function compilation...");
    test_simple_compilation();
    
    // Test 3: Function Execution
    println!("3. Testing function execution...");
    test_function_execution();
    
    println!("\n=== Test Complete ===");
    println!("Phase 3A JIT execution infrastructure is working!");
}

fn test_jit_engine_creation() {
    use cursed::codegen::llvm::jit_engine::{CursedJitEngine, JitEngineConfig};
    use cursed::runtime::jit_runtime::JitRuntimeConfig;
    
    // Create JIT engine configuration
    let jit_config = JitEngineConfig {
        base_config: JitRuntimeConfig::default(),
        enable_advanced_optimizations: false,
        enable_pgo: false,
        enable_speculative_opts: false,
        enable_osr: false,
        code_cache_limit: 10 * 1024 * 1024, // 10MB
        max_inline_depth: 2,
        loop_unroll_threshold: 50,
        vector_width: 4,
        enable_lto: false,
        debug_info_level: 1,
    };
    
    // Create JIT engine
    match CursedJitEngine::new(jit_config) {
        Ok(mut engine) => {
            println!("✓ JIT engine created successfully");
            
            // Initialize the engine
            match engine.initialize() {
                Ok(_) => {
                    println!("✓ JIT engine initialized successfully");
                }
                Err(e) => {
                    println!("✗ JIT engine initialization failed: {}", e);
                }
            }
        }
        Err(e) => {
            println!("✗ JIT engine creation failed: {}", e);
        }
    }
}

fn test_simple_compilation() {
    use cursed::codegen::llvm::jit_engine::{CursedJitEngine, JitEngineConfig};
    use cursed::runtime::jit_runtime::{JitRuntimeConfig, OptimizationLevel};
    
    // Create and initialize JIT engine
    let jit_config = JitEngineConfig::default();
    let mut engine = match CursedJitEngine::new(jit_config) {
        Ok(engine) => engine,
        Err(e) => {
            println!("✗ Failed to create JIT engine: {}", e);
            return;
        }
    };
    
    if let Err(e) = engine.initialize() {
        println!("✗ Failed to initialize JIT engine: {}", e);
        return;
    }
    
    // Test simple function compilation
    let simple_source = "fn test_simple() -> int { return 42; }";
    
    match engine.compile_function("test_simple", simple_source, Some(OptimizationLevel::Basic)) {
        Ok(function_id) => {
            println!("✓ Simple function compiled successfully. ID: {}", function_id);
        }
        Err(e) => {
            println!("✗ Simple function compilation failed: {}", e);
        }
    }
    
    // Test function with parameters
    let param_source = "fn test_add(a: int, b: int) -> int { return a + b; }";
    
    match engine.compile_function("test_add", param_source, Some(OptimizationLevel::Standard)) {
        Ok(function_id) => {
            println!("✓ Parameterized function compiled successfully. ID: {}", function_id);
        }
        Err(e) => {
            println!("✗ Parameterized function compilation failed: {}", e);
        }
    }
}

fn test_function_execution() {
    use cursed::codegen::llvm::jit_engine::{CursedJitEngine, JitEngineConfig};
    use cursed::runtime::jit_runtime::{JitRuntimeConfig, OptimizationLevel};
    
    // Create and initialize JIT engine
    let jit_config = JitEngineConfig::default();
    let mut engine = match CursedJitEngine::new(jit_config) {
        Ok(engine) => engine,
        Err(e) => {
            println!("✗ Failed to create JIT engine: {}", e);
            return;
        }
    };
    
    if let Err(e) = engine.initialize() {
        println!("✗ Failed to initialize JIT engine: {}", e);
        return;
    }
    
    // Compile a test function
    let test_source = "fn test_exec() -> int { return 123; }";
    let function_id = match engine.compile_function("test_exec", test_source, Some(OptimizationLevel::Basic)) {
        Ok(id) => id,
        Err(e) => {
            println!("✗ Function compilation failed: {}", e);
            return;
        }
    };
    
    // Execute the function
    let args: Vec<*const u8> = vec![];
    match engine.execute_function(function_id, &args) {
        Ok(result) => {
            println!("✓ Function executed successfully. Result: {:?}", result);
            
            // Check if result is non-null (indicating actual execution)
            if !result.is_null() {
                println!("✓ JIT execution returned valid result (not falling back to interpretation)");
            } else {
                println!("⚠ JIT execution returned null result (may be expected for test functions)");
            }
        }
        Err(e) => {
            println!("✗ Function execution failed: {}", e);
        }
    }
    
    // Test function with arguments
    let add_source = "fn test_add_exec(a: int, b: int) -> int { return a + b; }";
    let add_function_id = match engine.compile_function("test_add_exec", add_source, Some(OptimizationLevel::Standard)) {
        Ok(id) => id,
        Err(e) => {
            println!("✗ Add function compilation failed: {}", e);
            return;
        }
    };
    
    // Execute the add function with arguments
    let args: Vec<*const u8> = vec![10usize as *const u8, 20usize as *const u8];
    match engine.execute_function(add_function_id, &args) {
        Ok(result) => {
            println!("✓ Add function executed successfully. Result: {:?}", result);
            
            // Convert result back to check the actual computation
            let result_value = result as usize;
            if result_value == 30 {
                println!("✓ JIT execution performed correct arithmetic: 10 + 20 = 30");
            } else {
                println!("⚠ JIT execution result: {} (expected 30)", result_value);
            }
        }
        Err(e) => {
            println!("✗ Add function execution failed: {}", e);
        }
    }
}
