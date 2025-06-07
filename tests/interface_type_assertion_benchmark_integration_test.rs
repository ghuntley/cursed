use std::sync::Once;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use cursed::benchmark::interface_type_assertion_benchmark::{InterfaceTypeAssertionBenchmark, InterfaceTypeAssertionBenchmarkConfig};
use cursed::codegen::llvm::EnhancedInterfacePathFinder;
use cursed::codegen::llvm::InterfaceTypeRegistry;
use cursed::codegen::llvm::LlvmCodeGenerator;
use cursed::core::{JitOptions, InterpretOptions};
use cursed::lexer::Lexer;
use cursed::parser::Parser;
use cursed::error::Error;
use cursed::object::{Object, ObjectRef};
use inkwell::context::Context;

// Test the interface type assertion benchmark functionality.
// This integration test validates that the benchmarking
// infrastructure works correctly for interface type assertions.


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



#[test]
fn test_interface_type_assertion_benchmark_config() {
    init_tracing!();
    
    // Simply test creating a benchmark configuration
    let config = InterfaceTypeAssertionBenchmarkConfig::default();
    
    // Basic assertions about the default configuration
    assert!(config.iterations > 0, "Default iterations should be positive");
    assert!(config.test_diamond_patterns, "Diamond pattern testing should be enabled by default");
    assert!(config.test_deep_hierarchies, "Deep hierarchy testing should be enabled by default");
}

#[test]
fn test_simple_interface_type_assertion_benchmark() {
    init_tracing!();
    
    // Create a context and code generator
    let context = Context::create();
    let mut code_generator = LlvmCodeGenerator::new(&context);
    
    // Create a minimal dummy registry for testing
    let mock_registry = create_mock_registry();
    
    // Add mock registry to the code generator
    code_generator.internal_fields.insert(
        "interface_registry".to_string(),
        Box::new(mock_registry) as Box<dyn std::any::Any>
    );
    
    // Create a minimal benchmark configuration
    let config = InterfaceTypeAssertionBenchmarkConfig {
        iterations: 10,
        enable_warmup: false,
        warmup_iterations: 0,
        detailed_timing: false,
        test_diamond_patterns: false,
        test_deep_hierarchies: false,
        max_hierarchy_depth: 3,
    };
    
    // Run the benchmark
    match code_generator.benchmark_interface_type_assertions(config) {
        Ok(results) => {
            // Should have at least one result (simple type assertions)
            assert!(!results.is_empty(), "Should have at least one benchmark result");
            
            // Verify the benchmark name
            assert_eq!(
                results[0].name, 
                "Simple Type Assertions", 
                "First benchmark should be simple type assertions"
            );
            
            // Verify reasonable timing values
            assert!(results[0].avg_duration.as_nanos() > 0, "Average duration should be positive");
        },
        Err(e) => panic!("Benchmark failed: {}", e),
    }
}

#[test]
fn test_comprehensive_interface_type_assertion_benchmark() {
    init_tracing!();
    
    // Create a context and code generator
    let context = Context::create();
    let mut code_generator = LlvmCodeGenerator::new(&context);
    
    // Create a comprehensive mock registry for testing
    let mock_registry = create_comprehensive_mock_registry();
    
    // Add mock registry to the code generator
    code_generator.internal_fields.insert(
        "interface_registry".to_string(),
        Box::new(mock_registry) as Box<dyn std::any::Any>
    );
    
    // Create a comprehensive benchmark configuration with small iteration counts
    let config = InterfaceTypeAssertionBenchmarkConfig {
        iterations: 5,
        enable_warmup: true,
        warmup_iterations: 2,
        detailed_timing: true,
        test_diamond_patterns: true,
        test_deep_hierarchies: true,
        max_hierarchy_depth: 5,
    };
    
    // Run the benchmark
    match code_generator.benchmark_interface_type_assertions(config) {
        Ok(results) => {
            // Should have 3 results (simple, diamond, deep)
            assert_eq!(results.len(), 3, "Should have 3 benchmark results");
            
            // Verify the benchmark names
            assert_eq!(results[0].name, "Simple Type Assertions");
            assert_eq!(results[1].name, "Diamond Inheritance Type Assertions");
            assert!(results[2].name.starts_with("Deep Hierarchy Type Assertions");
            
            // Verify all have timing data
            for result in &results {
                assert!(result.avg_duration.as_nanos() > 0, "Average duration should be positive");
                assert!(result.min_duration.as_nanos() > 0, "Min duration should be positive");
                assert!(result.max_duration.as_nanos() > 0, "Max duration should be positive");
            }
        },
        Err(e) => panic!("Benchmark failed: {}", e),
    }
}

#[test]
fn test_interface_type_assertion_benchmark_csd_file() {
    init_tracing!();
    
    // Test that the benchmark file exists
    let benchmark_file = "benchmarks/cursed/interface_type_assertion_benchmark.csd";
    assert!(Path::new(benchmark_file).exists(), "Benchmark file should exist");
    
    // Load the file and verify it has the expected content
    let mut file = File::open(benchmark_file).expect("Failed to open benchmark file");
    let mut content = String::new();
    file.read_to_string(&mut content).expect("Failed to read benchmark file");
    
    // Verify it contains key sections
    assert!(content.contains("benchmarkSimpleAssertions"), "File should contain simple assertion benchmark");
    assert!(content.contains("benchmarkDiamondAssertions"), "File should contain diamond assertion benchmark");
    assert!(content.contains("benchmarkDeepAssertions"), "File should contain deep assertion benchmark");
}

// Helper to create a simple mock registry for testing
fn create_mock_registry() -> Box<dyn InterfaceTypeRegistry> {
    // This is just a stub implementation for testing
    struct MockRegistry;
    
    impl InterfaceTypeRegistry for MockRegistry {
        fn register_interface(&mut self, _id: u32, _name: String) -> Result<(), Error> { Ok(()) }
        fn register_concrete_type(&mut self, _id: u32, _name: String) -> Result<(), Error> { Ok(()) }
        fn register_implementation(&mut self, _concrete_id: u32, _interface_id: u32) -> Result<(), Error> { Ok(()) }
        fn type_implements_interface(&self, _concrete_id: u32, _interface_id: u32) -> bool { true }
        fn get_implemented_interfaces(&self, _concrete_id: u32) -> Result<Vec<u32>, Error> { Ok(vec![100, 200]) }
        fn get_type_name(&self, _id: u32) -> Result<String, Error> { Ok("MockType".to_string()) }
    }
    
    Box::new(MockRegistry)
}

// Helper to create a comprehensive mock registry
fn create_comprehensive_mock_registry() -> Box<dyn InterfaceTypeRegistry> {
    struct ComprehensiveMockRegistry {
        interfaces: std::collections::HashMap<u32, String>,
        concrete_types: std::collections::HashMap<u32, String>,
        implementations: std::collections::HashMap<u32, Vec<u32>>, // concrete_id -> [interface_ids]
    }
    
    impl InterfaceTypeRegistry for ComprehensiveMockRegistry {
        fn register_interface(&mut self, id: u32, name: String) -> Result<(), Error> {
            self.interfaces.insert(id, name);
            Ok(())
        }
        
        fn register_concrete_type(&mut self, id: u32, name: String) -> Result<(), Error> {
            self.concrete_types.insert(id, name);
            Ok(())
        }
        
        fn register_implementation(&mut self, concrete_id: u32, interface_id: u32) -> Result<(), Error> {
            self.implementations.entry(concrete_id)
                .or_insert_with(Vec::new)
                .push(interface_id);
            Ok(())
        }
        
        fn type_implements_interface(&self, concrete_id: u32, interface_id: u32) -> bool {
            self.implementations.get(&concrete_id)
                .map(|interfaces| interfaces.contains(&interface_id))
                .unwrap_or(false)
        }
        
        fn get_implemented_interfaces(&self, concrete_id: u32) -> Result<Vec<u32>, Error> {
            Ok(self.implementations.get(&concrete_id)
                .cloned()
                .unwrap_or_else(Vec::new))
        }
        
        fn get_type_name(&self, id: u32) -> Result<String, Error> {
            if let Some(name) = self.interfaces.get(&id) {
                return Ok(name.clone();
            }
            
            if let Some(name) = self.concrete_types.get(&id) {
                return Ok(name.clone();
            }
            
            Ok(format!("Type{}", id))
        }
    }
    
    // Create registry with comprehensive test data
    let mut registry = ComprehensiveMockRegistry {
        interfaces: std::collections::HashMap::new(),
        concrete_types: std::collections::HashMap::new(),
        implementations: std::collections::HashMap::new(),
    };
    
    // Add interfaces
    registry.register_interface(100, "BaseInterface".to_string()).unwrap();
    registry.register_interface(200, "LeftInterface".to_string()).unwrap();
    registry.register_interface(300, "RightInterface".to_string()).unwrap();
    
    // Add deep hierarchy interfaces
    for i in 1..=5 {
        registry.register_interface(1000 + i, format!("Level{}", i)).unwrap();
    }
    
    // Add concrete types
    registry.register_concrete_type(400, "SimpleConcrete".to_string()).unwrap();
    registry.register_concrete_type(500, "DiamondConcrete".to_string()).unwrap();
    registry.register_concrete_type(600, "DeepConcrete".to_string()).unwrap();
    
    // Add implementations
    // Simple implementation
    registry.register_implementation(400, 100).unwrap();
    
    // Diamond implementation
    registry.register_implementation(200, 100).unwrap(); // Left extends Base
    registry.register_implementation(300, 100).unwrap(); // Right extends Base
    registry.register_implementation(500, 200).unwrap(); // Concrete implements Left
    registry.register_implementation(500, 300).unwrap(); // Concrete implements Right
    
    // Deep hierarchy
    for i in 1..5 {
        registry.register_implementation(1000 + i + 1, 1000 + i).unwrap();
    }
    registry.register_implementation(600, 1000 + 5).unwrap(); // DeepConcrete implements deepest level
    
    Box::new(registry)
}