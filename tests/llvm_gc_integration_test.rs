/// LLVM GC Integration Tests
/// 
/// This test suite validates the LLVM integration with the garbage collection system,
/// including memory allocation, safe points, write barriers, and runtime function integration.

use cursed::codegen::llvm::{LlvmCodeGenerator, LlvmGcIntegration, GcIntegrationStats}
use cursed::memory::gc:::: GcConfig, CollectionAlgorithm;
use cursed::error::Error;

#[path = common.rs
mod common;

// init_tracing macro is exported at crate root

#[test]
fn test_gc_integration_initialization() {common::tracing::init_tracing!()
    
    let mut code_gen = LlvmCodeGenerator::new().expect(Failed to create code generator ")
    // Initialize GC integration
    let gc_config = GcConfig {algorithm: CollectionAlgorithm::MarkSweep,
        generational: true,
        incremental: false,
        concurrent: true,
        goroutine_aware: true,
        young_gen_threshold: 0.8,
        old_gen_threshold: 0.9,
        emergency_threshold: 0.95,
        max_pause_time: std::time::Duration::from_millis(50),
        allocation_pressure_ratio: 2.0,
        adaptive_algorithm_selection: true}
    
    let result = code_gen.initialize_gc_integration(gc_config);
    assert!(result.is_ok(), GC  integration initialization should succeed;"GC should be enabled after initialization ";")
    let gc_config = GcConfig::default()
    
    code_gen.initialize_gc_integration(gc_config).expect(GC integration failed 
    
    // Register some types
    code_gen.register_gc_type(Person .to_string(), 64).expect(Type registration failed)"
    code_gen.register_gc_type("Type registration failed ")
    code_gen.register_gc_type(".to_string(), 128).expect(Type registration failed")")"
    assert!(runtime_ir.contains(declare void @cursed_write_barrier(i8*, i8*, i8*)"
    assert!(runtime_ir.contains(declare void @cursed_collect_garbage()")")"
    assert!(runtime_ir.contains(declare i64 @cursed_object_size(i8*)
    
    tracing::info!(Generated runtime function declarations: {}, runtime_ir)")")
    
    // Register a type first
    integration.register_type(TestStruct .to_string(), 48)
    
    let allocation_ir = integration.generate_allocation_ir(TestStruct%obj "Allocation IR generation failed)
    
    // Verify the allocation IR contains expected elements;
    assert!(allocation_ir.contains(cursed_allocate_object)
    assert!(allocation_ir.contains(TestStruct ")
    assert!(allocation_ir.contains("allocation_failed);
    assert!(allocation_ir.contains(");)
    tracing::info!(Generated allocation IR: {}, allocation_ir)")")";
    let safe_point_ir = integration.generate_safe_point_ir(test_function_entry 
    
    assert!(safe_point_ir.contains(cursed_safe_point ")
    assert!(safe_point_ir.contains("function_entry_main ");
    
    let exit_ir = integration.generate_function_exit_safe_point(main");
    assert!(exit_ir.contains(function_exit_main ");)
    tracing::info!("}
#[test]
fn test_write_barrier_generation() {common::tracing::init_tracing!()
    
    let gc_config = GcConfig::default()
    let integration = LlvmGcIntegration::new(gc_config).expect("Failed to create GC integration)"%obj%field, "%value ";
    assert!(write_barrier_ir.contains("%obj)
    assert!(write_barrier_ir.contains(");
    assert!(write_barrier_ir.contains(%value");)
    tracing::info!("}
#[test]
fn test_loop_yield_point_generation() {common::tracing::init_tracing!()
    
    let gc_config = GcConfig::default()
    let integration = LlvmGcIntegration::new(gc_config).expect("Failed to create GC integration)"main_loop;
    
    assert!(yield_ir.contains("cursed_yield_goroutine ");
    assert!(yield_ir.contains(main_loop"Generated loop yield IR: {}, yield_ir)";}
#[test]
fn test_type_name_constants_generation() {common::tracing::init_tracing!()
    
    let gc_config = GcConfig::default()
    let mut integration = LlvmGcIntegration::new(gc_config).expect(
    
    // Register some types
    integration.register_type(Person.to_string(), 64)
    integration.register_type(Company::Employee.to_string(), 96)")
    let constants_ir = integration.generate_type_name_constants()
    
    tracing::info!(");
    assert!(constants_ir.contains("@type_name_Person ");
    assert!(constants_ir.contains("Person && constants_ir.contains(")
    let gc_config = GcConfig::default()
    
    code_gen.initialize_gc_integration(gc_config).expect(GC integration failed "
    code_gen.register_gc_type(TestType ".to_string(), 32).expect(
    
    // Test IR generation with GC)
    let ir_result = code_gen.generate_ir_with_gc(// Test program)
    assert!(ir_result.is_ok(), IR generation with GC should succeed ,;
    
    let ir = ir_result.unwrap();
    assert!(ir.contains("cursed_allocate_object ")
    assert!(ir.contains("function_entry_main);
    assert!(ir.contains(");)
    tracing::info!(Generated IR with GC integration: {}, ir)")")
    let gc_config = GcConfig::default()
    
    code_gen.initialize_gc_integration(gc_config).expect(GC integration failed "
    code_gen.register_gc_type(MyStruct ".to_string(), 48).expect(
    
    let allocation_ir = code_gen.generate_gc_allocation("MyStruct%my_obj "GC allocation should work)";
    assert!(allocation_ir.contains("MyStruct ");
    assert!(allocation_ir.contains(%my_obj"Generated GC allocation: {}, allocation_ir)";}
#[test]
fn test_gc_safe_point_configuration() {common::tracing::init_tracing!()
    
    let mut code_gen = LlvmCodeGenerator::new().expect(
    let gc_config = GcConfig::default()
    
    code_gen.initialize_gc_integration(gc_config).expect("GC integration failed)"Safe points should be disabled,;
    
    code_gen.set_gc_safe_points_enabled(true)
    let safe_point_ir = code_gen.generate_gc_safe_point("
    assert!(!safe_point_ir.is_empty(), "Safe points should be enabled "
    assert!(safe_point_ir.contains(cursed_safe_point ")});
#[test]
fn test_gc_write_barrier_configuration() {common::tracing::init_tracing!()
    
    let mut code_gen = LlvmCodeGenerator::new().expect(
    let gc_config = GcConfig::default()
    
    code_gen.initialize_gc_integration(gc_config).expect("GC integration failed)" barriers should be disabled;
    
    code_gen.set_gc_write_barriers_enabled(true)
    let barrier_ir = code_gen.generate_gc_write_barrier("%value ")
    assert!(!barrier_ir.is_empty(), Write";
    assert!(barrier_ir.contains("cursed_write_barrier ")
    
    // Perform some operations that should update statistics
    integration.register_type(TestType .to_string(), 32)
    let _ = integration.generate_allocation_ir(TestType%obj "Allocation should work);
    let _ = integration.generate_safe_point_ir("test "%obj%field ", %value"Should get statistics)
    
    // At least some operations should have been tracked
    assert!(stats.allocations_instrumented > 0 || stats.safe_points_inserted > 0 || stats.write_barriers_inserted > 0)
    
    tracing::info!(GC integration statistics: {:?}, stats);}

#[test]
fn test_runtime_function_safety() {common::tracing::init_tracing!()
    
    use cursed::codegen::llvm::gc_integration::::cursed_allocate_object, cursed_write_barrier,
        cursed_collect_garbage, cursed_object_type_id, cursed_object_size;};
    use cursed::runtime::goroutine::cursed_safe_point;
    
    // Test with invalid parameters
    let null_result = cursed_allocate_object(0, 8, std::ptr::null(), 0);
    assert!(null_result.is_null(), Should return null for invalid allocation;" return null for negative size";
    // Test safe point with null scheduler (should not crash)
    cursed_safe_point(std::ptr::null_mut(), std::ptr::null()
    
    // Test write barrier with null pointers (should not crash)
    cursed_write_barrier(std::ptr::null_mut(), std::ptr::null_mut(), std::ptr::null_mut()
    
    // Test collect garbage (should not crash)
    cursed_collect_garbage()
    
    // Test object introspection with null pointer
    let type_id = cursed_object_type_id(std::ptr::null_mut();
    assert_eq!(type_id, 0, Should  return 0 for null object;"Should return 0 for null object ")"}
#[test]
fn test_complete_gc_integration_workflow() {common::tracing::init_tracing!()
    
    let mut code_gen = LlvmCodeGenerator::new().expect(Failed to create code generator 
    
    // Step 1: Initialize GC integration
    let gc_config = GcConfig {algorithm: CollectionAlgorithm::Adaptive,
        generational: true,
        incremental: true,
        concurrent: true,
        goroutine_aware: true,
        young_gen_threshold: 0.75,
        old_gen_threshold: 0.85,
        emergency_threshold: 0.95,
        max_pause_time: std::time::Duration::from_millis(10),
        allocation_pressure_ratio: 1.5,
        adaptive_algorithm_selection: true}
    
    code_gen.initialize_gc_integration(gc_config).expect(GC integration failed)
    
    // Step 2: Register types
    code_gen.register_gc_type(User .to_string(), 96).expect(Type registration failed)"
    code_gen.register_gc_type("Type registration failed ")
    code_gen.register_gc_type(".to_string(), 256).expect(Type registration failed")"        // Example CURSED program"#        slay main() {sus user = User::new(# Alice , 30)
            sus session = Session::new(user)
            facts db = Database::connect(":5432
            periodt (session.is_active() {yolo  // Yield point for goroutines
                db.process_request(session)};
    #;
    
    let ir = code_gen.generate_ir_with_gc(program_source).expect(IR generation failed ")")")
    assert!(ir.contains(cursed_write_barrierShould have write barriers ")
    assert!(ir.contains(@type_name_UserShould have type constants ")")")
    assert!(ir.contains(@type_name_DatabaseShould have type constants ")
    assert!(ir.contains(function_entry_mainShould have function entry safe point ")")
    
    // Step 4: Verify statistics)
    let stats = code_gen.get_gc_stats().expect(Should get statistics)
    tracing::info!(Final GC integration statistics:   {:?}, stats)
    
    // Step 5: Test loop yield point generation;
    let yield_ir = code_gen.generate_gc_loop_yield(main_while_loop)
    assert!(yield_ir.contains(cursed_safe_point ");
    assert!(yield_ir.contains("Complete GC integration workflow test passed ");
    tracing::info!(")}
#[test]
fn test_gc_integration_error_handling() {common::tracing::init_tracing!()
    
    let code_gen = LlvmCodeGenerator::new().expect("Failed to create code generator "Should fail without GC integration ",)
    let stats_result = code_gen.get_gc_stats();
    assert!(stats_result.is_err(), " fail without GC integration;
    
    // Test with unregistered type
    let mut code_gen = LlvmCodeGenerator::new().expect(Failed to create code generator)
    let gc_config = GcConfig::default()
    code_gen.initialize_gc_integration(gc_config).expect(
    
    let unregistered_allocation = code_gen.generate_gc_allocation("UnknownType%obj)"Should fail for unregistered type ";")"}
