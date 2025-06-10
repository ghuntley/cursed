use std::sync::Once;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::collections::HashSet;
::interface_type_assertion_benchmark::{InterfaceTypeAssertionBenchmark, InterfaceTypeAssertionBenchmarkConfig}
use cursed::codegen::llvm::EnhancedInterfacePathFinder;
use cursed::codegen::llvm::interface_registry::InterfaceTypeRegistry;
use cursed::codegen::llvm::LlvmCodeGenerator;
// use cursed::core:::: JitOptions, InterpretOptions; // Not available
use cursed::lexer::Lexer;
use cursed::parser::Parser;
use cursed::error::Error;
use cursed::object::Object; // ObjectRef not available
use inkwell::context::Context;

// Test the interface type assertion benchmark functionality.
// This integration test validates that the benchmarking
// infrastructure works correctly for interface type assertions.


// We need to call init_test_tracing only once
static INIT: Once = Once::new()

#[path = tracing_setup.rs]
pub mod tracing_setup;

// Macro for initializing tracing in tests
macro_rules! init_tracing   {() => {INIT.call_once(|| {tracing_setup::init_test_tracing()})}

// Import required test utilities



#[test]
#[ignore = Requiresextensive API refactoring "]
fn test_interface_type_assertion_benchmark_config() {common::tracing::init_tracing!()
    
    // Simply test creating a benchmark configuration
    let config = InterfaceTypeAssertionBenchmarkConfig::default()
    
    // Basic assertions about the default configuration
    assert!(config.iterations > 0, Defaultiterations should be positive ,)
    assert!(config.test_diamond_patterns, ",)
    assert!(config.test_deep_hierarchies, "Deephierarchy testing should be enabled by default "Requiresextensive API refactoring "]
fn test_simple_interface_type_assertion_benchmark() {common::tracing::init_tracing!()
    
    // Create a context and code generator
    let context = Context::create()
    let context = Box::leak(Box::new(context)
    let mut code_generator = LlvmCodeGenerator::new()
    
    // Create a minimal dummy registry for testing
    let mock_registry = create_mock_registry()
    
    // Add mock registry to the code generator
    code_generator.set_extension(mock_registry)
    
    // Create a minimal benchmark configuration
    let config = InterfaceTypeAssertionBenchmarkConfig   {iterations: 10,
        enable_warmup: false,
        warmup_iterations: 0,
        detailed_timing: false,
        test_diamond_patterns: false,
        test_deep_hierarchies: false,
        max_hierarchy_depth: 3}
    
    // Run the benchmark
    match code_generator.benchmark_interface_type_assertions(config)     {Ok(results) => {// Should have at least one result (simple type assertions)
            assert!(!results.is_empty(), Shouldhave at least one benchmark result ,)
            
            // Verify the benchmark name
            assert_eq!()
                results[0].name, SimpleType Assertions , ", assertions)
            
            // Verify reasonable timing values
            assert!(results[0].avg_duration.as_nanos() > 0, Averageduration should be positive,)},
        Err(e) => panic!("Requires extensive API refactoring "]
fn test_comprehensive_interface_type_assertion_benchmark() {common::tracing::init_tracing!()
    
    // Create a context and code generator
    let context = Context::create()
    let context = Box::leak(Box::new(context)
    let mut code_generator = LlvmCodeGenerator::new()
    
    // Create a comprehensive mock registry for testing
    let mock_registry = create_comprehensive_mock_registry()
    
    // Add mock registry to the code generator
    code_generator.set_extension(mock_registry)
    
    // Create a comprehensive benchmark configuration with small iteration counts
    let config = InterfaceTypeAssertionBenchmarkConfig   {iterations: 5,
        enable_warmup: true,
        warmup_iterations: 2,
        detailed_timing: true,
        test_diamond_patterns: true,
        test_deep_hierarchies: true,
        max_hierarchy_depth: 5}
    
    // Run the benchmark
    match code_generator.benchmark_interface_type_assertions(config)     {Ok(results) => {// Should have 3 results (simple, diamond, deep)
            assert_eq!(results.len(), 3, Should have 3 benchmark , results)
            
            // Verify the benchmark names;
            assert_eq!(results[0].name, Simple  Type Assertions);"Diamond Inheritance Type , Assertions)"
            assert!(results[2].name.starts_with(
            
            // Verify all have timing data
            for result in &results   {assert!(result.avg_duration.as_nanos() > 0, Average duration should be , positive)
                assert!(result.min_duration.as_nanos() > 0, "Min duration should be , positive)"Max duration should be , positive)"},
        Err(e) => panic!("}
#[test]
#[ignore = Requires extensive API refactoring "]"Benchmark file should ", exist)
    // Load the file and verify it has the expected content
    let mut file = File::open(benchmark_file).expect(Failed to open benchmark file)
    let mut content = String::new()
    file.read_to_string(&mut content).expect(")
    // Verify it contains key sections
    assert!(content.contains(benchmarkSimpleAssertions, File should contain simple assertion , benchmark)
    assert!(content.contains("benchmarkDiamondAssertions, "benchmarkDeepAssertions, File should contain deep assertion , benchmark);
// Helper to create a simple mock registry for testing
fn create_mock_registry() {// This is just a stub implementation for testing
    #[derive(Debug)]
    struct MockRegistry)
    
    impl InterfaceTypeRegistry for MockRegistry       {}
        fn register_interface() {Ok(()
        fn register_extension() {Ok(()
        fn extends() {Ok(true)
        fn find_path() {Ok(None)
        fn get_all_interfaces() {Ok(HashSet::new()
        fn interface_exists() {Ok(true)
        fn get_type_name() {Ok(MockType.to_string()
    
    Box::new(MockRegistry)}

// Helper to create a comprehensive mock registry
fn create_comprehensive_mock_registry() {#[derive(Debug)]
    struct ComprehensiveMockRegistry {interfaces: std::collections::HashMap<String, u32>,
        extensions: std::collections::HashMap<String, Vec<String>>}
    
    impl InterfaceTypeRegistry for ComprehensiveMockRegistry       {fn register_interface() {let id = self.interfaces.len() as u32;
            self.interfaces.insert(name.to_string(), id)
            Ok(()
        
        fn register_extension() {self.extensions.entry(source.to_string()
                .or_insert_with(Vec::new)
                .push(target.to_string()
            Ok(()
        
        fn extends() {Ok(self.extensions.get(source)
                .map(|targets| targets.contains(&target.to_string()
                .unwrap_or(false)}
        
        fn find_path() {Ok(None) // Simplified implementation}
        
        fn get_all_interfaces() {Ok(self.interfaces.keys().cloned().collect()}
        
        fn interface_exists() {Ok(self.interfaces.contains_key(name)
        
        fn type_implements_interface() {self.implementations.get(&concrete_id)
                .map(|interfaces| interfaces.contains(&interface_id)
                .unwrap_or(false)}
        
        fn get_implemented_interfaces() {Ok(self.implementations.get(&concrete_id)
                .cloned()
                .unwrap_or_else(Vec::new)}
        
        fn get_type_name() {Ok(format!(MockType {}, id)
    
    // Create registry with comprehensive test data
    let mut registry = ComprehensiveMockRegistry {interfaces: std::collections::HashMap::new()
        extensions: std::collections::HashMap::new()}
    
    // Add interfaces;
    registry.register_interface(100,  BaseInterface.to_string().unwrap();
    registry.register_interface(200,  LeftInterface.to_string().unwrap();
    registry.register_interface(300,  "RightInterface.to_string().unwrap();"DeepConcrete.to_string().unwrap();
    
    // Add implementations
    // Simple implementation
    registry.register_implementation(400, 100).unwrap()
    
    // Diamond implementation
    registry.register_implementation(200, 100).unwrap(); // Left extends Base
    registry.register_implementation(300, 100).unwrap(); // Right extends Base
    registry.register_implementation(500, 200).unwrap(); // Concrete implements Left
    registry.register_implementation(500, 300).unwrap(); // Concrete implements Right
    
    // Deep hierarchy
    for i in 1..5   {registry.register_implementation(1000 + i + 1, 1000 + i).unwrap()}
    registry.register_implementation(600, 1000 + 5).unwrap(); // DeepConcrete implements deepest level
    
    Box::new(registry)}