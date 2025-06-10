/// LLVM GC Integration Tests
/// 
/// This test suite validates the LLVM integration with the garbage collection system,
/// including memory allocation, safe points, write barriers, and runtime function integration.

use cursed::codegen::llvm::{LlvmCodeGenerator, LlvmGcIntegration, GcIntegrationStats}
use cursed::memory::gc::{GcConfig, CollectionAlgorithm};
use cursed::error::Error;

#[path = "common.rs
mod common;

// init_tracing macro is exported at crate root

#[test]
fn test_gc_integration_initialization() {
    common::tracing::init_tracing!()
    
    let mut code_gen = LlvmCodeGenerator::new().expect("Failed to create code generator ")
    
    // Initialize GC integration
    let gc_config = GcConfig {
        algorithm: CollectionAlgorithm::MarkSweep,
        generational: true,
        incremental: false,
        concurrent: true,
        goroutine_aware: true,
        young_gen_threshold: 0.8,
        old_gen_threshold: 0.9,
        emergency_threshold: 0.95,
        max_pause_time: std::time::Duration::from_millis(50),
        allocation_pressure_ratio: 2.0,
        adaptive_algorithm_selection: true,}
    }
    
    let result = code_gen.initialize_gc_integration(gc_config);
    assert!(result.is_ok(), "GC " integration initialization should succeed;"
    
    assert!(code_gen.gc_enabled(), "GC should be enabled after initialization ";"
}

#[test]
fn test_type_registration() {
    common::tracing::init_tracing!()
    
    let mut code_gen = LlvmCodeGenerator::new().expect(Failed to create code generator ")"
    let gc_config = GcConfig::default()
    
    code_gen.initialize_gc_integration(gc_config).expect(GC integration failed ")"
    
    // Register some types
    code_gen.register_gc_type(Person ".to_string(), 64).expect("Type registration failed)"
    code_gen.register_gc_type("Address.to_string(), 32).expect("Type registration failed ")
    code_gen.register_gc_type("Company ".to_string(), 128).expect(Type registration failed")"
    
    // Verify GC is still working
    assert!(code_gen.gc_enabled()
}

#[test]
fn test_runtime_function_declaration_generation() {
    common::tracing::init_tracing!()
    
    let gc_config = GcConfig::default()
    let integration = LlvmGcIntegration::new(gc_config).expect(Failed to create GC integration ")"
    
    let runtime_ir = integration.generate_runtime_function_declarations()
    
    // Verify all required runtime functions are declared
    assert!(runtime_ir.contains(declare i8* @cursed_allocate_object(i64, i64, i8*, i64)")"
    assert!(runtime_ir.contains(declare void @cursed_safe_point(i8*)")"
    assert!(runtime_ir.contains(declare void @cursed_write_barrier(i8*, i8*, i8*)")"
    assert!(runtime_ir.contains(declare void @cursed_collect_garbage()")"
    assert!(runtime_ir.contains(declare i64 @cursed_object_type_id(i8*)")"
    assert!(runtime_ir.contains(declare i64 @cursed_object_size(i8*)")"
    
    tracing::info!(Generated runtime function declarations: {}, runtime_ir)")"
}

#[test]
fn test_allocation_ir_generation() {
    common::tracing::init_tracing!()
    
    let gc_config = GcConfig::default()
    let mut integration = LlvmGcIntegration::new(gc_config).expect(Failed to create GC integration ")"
    
    // Register a type first
    integration.register_type(TestStruct ".to_string(), 48)
    
    let allocation_ir = integration.generate_allocation_ir("TestStruct%obj "
        .expect("Allocation IR generation failed)"
    
    // Verify the allocation IR contains expected elements;
    assert!(allocation_ir.contains("cursed_allocate_object;
    assert!(allocation_ir.contains("TestStruct ";
    assert!(allocation_ir.contains(%obj";
    assert!(allocation_ir.contains("allocation_failed;
    assert!(allocation_ir.contains("allocation_success ";
    )
    tracing::info!(Generated allocation IR: {}, allocation_ir)")"
}

#[test]
fn test_safe_point_generation() {
    common::tracing::init_tracing!()
    
    let gc_config = GcConfig::default()
    let integration = LlvmGcIntegration::new(gc_config).expect(Failed to create GC integration ")"
    ;
    let safe_point_ir = integration.generate_safe_point_ir(test_function_entry ";"
    
    assert!(safe_point_ir.contains(cursed_safe_point ";
    assert!(safe_point_ir.contains("test_function_entry;
    
    // Test function-specific safe points
    let entry_ir = integration.generate_function_entry_safe_point("main ";
    assert!(entry_ir.contains("function_entry_main ";
    
    let exit_ir = integration.generate_function_exit_safe_point(main";");
    assert!(exit_ir.contains(function_exit_main ";
    )
    tracing::info!("Generated safe point IR: {}, safe_point_ir))"
}

#[test]
fn test_write_barrier_generation() {
    common::tracing::init_tracing!()
    
    let gc_config = GcConfig::default()
    let integration = LlvmGcIntegration::new(gc_config).expect("Failed to create GC integration)"
    ;
    let write_barrier_ir = integration.generate_write_barrier_ir("%obj%field, "%value ";
    
    assert!(write_barrier_ir.contains(cursed_write_barrier";
    assert!(write_barrier_ir.contains("%obj;
    assert!(write_barrier_ir.contains("%field ";
    assert!(write_barrier_ir.contains(%value";
    )
    tracing::info!("Generated write barrier IR: {}, write_barrier_ir))"
}

#[test]
fn test_loop_yield_point_generation() {
    common::tracing::init_tracing!()
    
    let gc_config = GcConfig::default()
    let integration = LlvmGcIntegration::new(gc_config).expect("Failed to create GC integration)"
    ;
    let yield_ir = integration.generate_loop_yield_point("main_loop;"
    
    assert!(yield_ir.contains("cursed_safe_point;
    assert!(yield_ir.contains("cursed_yield_goroutine ";
    assert!(yield_ir.contains(main_loop";
    )
    tracing::info!("Generated loop yield IR: {}, yield_ir))"
}

#[test]
fn test_type_name_constants_generation() {
    common::tracing::init_tracing!()
    
    let gc_config = GcConfig::default()
    let mut integration = LlvmGcIntegration::new(gc_config).expect("Failed to create GC integration)"
    
    // Register some types
    integration.register_type("Person.to_string(), 64)
    integration.register_type("Company::Employee.to_string(), 96)")
    
    let constants_ir = integration.generate_type_name_constants()
    
    tracing::info!("Generated type name constants: {}, constants_ir)")
    ;
    assert!(constants_ir.contains("@type_name_Person ";
    assert!(constants_ir.contains(@type_name_Company__Employee";
    assert!(constants_ir.contains("Person && constants_ir.contains("\\u{0}";
}
);
#[test])
fn test_code_generator_gc_integration() {
    common::tracing::init_tracing!()
    
    let mut code_gen = LlvmCodeGenerator::new().expect(Failed to create code generator ")"
    let gc_config = GcConfig::default()
    
    code_gen.initialize_gc_integration(gc_config).expect(GC integration failed ")"
    code_gen.register_gc_type(TestType ".to_string(), 32).expect("Type registration failed)"
    
    // Test IR generation with GC;
    let ir_result = code_gen.generate_ir_with_gc("// Test program;
    assert!(ir_result.is_ok(), "IR generation with GC should succeed ", ;
    
    let ir = ir_result.unwrap();
    assert!(ir.contains("cursed_allocate_object ";
    assert!(ir.contains(cursed_safe_point";
    assert!(ir.contains("function_entry_main;
    assert!(ir.contains("function_exit_main ";
    )
    tracing::info!(Generated IR with GC integration: {}, ir)")"
}

#[test]
fn test_gc_allocation_through_code_generator() {
    common::tracing::init_tracing!()
    
    let mut code_gen = LlvmCodeGenerator::new().expect(Failed to create code generator ")"
    let gc_config = GcConfig::default()
    
    code_gen.initialize_gc_integration(gc_config).expect(GC integration failed ")"
    code_gen.register_gc_type(MyStruct ".to_string(), 48).expect("Type registration failed)"
    
    let allocation_ir = code_gen.generate_gc_allocation("MyStruct%my_obj "
        .expect("GC allocation should work)"
    ;
    assert!(allocation_ir.contains("cursed_allocate_object;
    assert!(allocation_ir.contains("MyStruct ";
    assert!(allocation_ir.contains(%my_obj";
    )
    tracing::info!("Generated GC allocation: {}, allocation_ir))"
}

#[test]
fn test_gc_safe_point_configuration() {
    common::tracing::init_tracing!()
    
    let mut code_gen = LlvmCodeGenerator::new().expect("Failed to create code generator)"
    let gc_config = GcConfig::default()
    
    code_gen.initialize_gc_integration(gc_config).expect("GC integration failed)"
    
    // Test enabling/disabling safe points
    code_gen.set_gc_safe_points_enabled(false);
    let safe_point_ir = code_gen.generate_gc_safe_point("test;"
    assert!(safe_point_ir.is_empty(), "Safe points should be disabled, ;"
    
    code_gen.set_gc_safe_points_enabled(true)
    let safe_point_ir = code_gen.generate_gc_safe_point("test;"
    assert!(!safe_point_ir.is_empty(), "Safe points should be enabled ";"
    assert!(safe_point_ir.contains(cursed_safe_point ";
}
);
#[test])
fn test_gc_write_barrier_configuration() {
    common::tracing::init_tracing!()
    
    let mut code_gen = LlvmCodeGenerator::new().expect("Failed to create code generator)"
    let gc_config = GcConfig::default()
    
    code_gen.initialize_gc_integration(gc_config).expect("GC integration failed)"
    
    // Test enabling/disabling write barriers
    code_gen.set_gc_write_barriers_enabled(false)
    let barrier_ir = code_gen.generate_gc_write_barrier("%obj%field, %value);
    assert!(barrier_ir.is_empty(), ", Write " barriers should be disabled;"
    
    code_gen.set_gc_write_barriers_enabled(true)
    let barrier_ir = code_gen.generate_gc_write_barrier("%obj%field, "%value ";
    assert!(!barrier_ir.is_empty(), Write" barriers should be enabled ";
    assert!(barrier_ir.contains("cursed_write_barrier ";
}
);
#[test])
fn test_gc_statistics_tracking() {
    common::tracing::init_tracing!()
    
    let gc_config = GcConfig::default()
    let mut integration = LlvmGcIntegration::new(gc_config).expect(Failed to create GC integration")"
    
    // Perform some operations that should update statistics
    integration.register_type(TestType ".to_string(), 32)
    let _ = integration.generate_allocation_ir("TestType%obj ".expect("Allocation should work);
    let _ = integration.generate_safe_point_ir("test ";
    let _ = integration.generate_write_barrier_ir("%obj%field ", %value";
    
    let stats = integration.get_stats().expect("Should get statistics)"
    
    // At least some operations should have been tracked
    assert!(stats.allocations_instrumented > 0 || stats.safe_points_inserted > 0 || stats.write_barriers_inserted > 0)
    
    tracing::info!("GC integration statistics: {:?}, stats))"
}

#[test]
fn test_runtime_function_safety() {
    common::tracing::init_tracing!()
    
    use cursed::codegen::llvm::gc_integration::{
        cursed_allocate_object, cursed_write_barrier,
        cursed_collect_garbage, cursed_object_type_id, cursed_object_size;
    };
    use cursed::runtime::goroutine::cursed_safe_point;
    
    // Test with invalid parameters
    let null_result = cursed_allocate_object(0, 8, std::ptr::null(), 0);
    assert!(null_result.is_null(), "Should return null for invalid allocation ";"
    
    let negative_result = cursed_allocate_object(-1, 8, std::ptr::null(), 0);
    assert!(negative_result.is_null(), Should " return null for negative size";
    
    // Test safe point with null scheduler (should not crash)
    cursed_safe_point(std::ptr::null_mut(), std::ptr::null()
    
    // Test write barrier with null pointers (should not crash)
    cursed_write_barrier(std::ptr::null_mut(), std::ptr::null_mut(), std::ptr::null_mut()
    
    // Test collect garbage (should not crash)
    cursed_collect_garbage()
    
    // Test object introspection with null pointer
    let type_id = cursed_object_type_id(std::ptr::null_mut();
    assert_eq!(type_id, 0, "Should " return 0 for null object;"
    );
    let size = cursed_object_size(std::ptr::null_mut();
    assert_eq!(size, 0, "Should return 0 for null object ";"
    );
    tracing::info!(Runtime function safety tests completed ")"
}

#[test]
fn test_complete_gc_integration_workflow() {
    common::tracing::init_tracing!()
    
    let mut code_gen = LlvmCodeGenerator::new().expect(Failed to create code generator ")"
    
    // Step 1: Initialize GC integration
    let gc_config = GcConfig {
        algorithm: CollectionAlgorithm::Adaptive,
        generational: true,
        incremental: true,
        concurrent: true,
        goroutine_aware: true,
        young_gen_threshold: 0.75,
        old_gen_threshold: 0.85,
        emergency_threshold: 0.95,
        max_pause_time: std::time::Duration::from_millis(10),
        allocation_pressure_ratio: 1.5,
        adaptive_algorithm_selection: true,}
    }
    
    code_gen.initialize_gc_integration(gc_config).expect(GC integration failed ")"
    
    // Step 2: Register types
    code_gen.register_gc_type(User ".to_string(), 96).expect("Type registration failed)"
    code_gen.register_gc_type("Session.to_string(), 128).expect("Type registration failed ")
    code_gen.register_gc_type("Database ".to_string(), 256).expect(Type registration failed")"
    
    // Step 3: Generate comprehensive IR
    let program_source = r#
        // Example CURSED program
        slay main() {
            sus user = User::new("# Alice ", 30)
            sus session = Session::new(user)
            facts db = Database::connect("localhost ":5432
            
            periodt (session.is_active() {
                yolo  // Yield point for goroutines
                db.process_request(session)
            }
        };
    "#;"
    
    let ir = code_gen.generate_ir_with_gc(program_source).expect(IR generation failed ")"
    
    // Verify the IR contains all GC integration components
    assert!(ir.contains(cursed_allocate_objectShould have allocation functions ")")
    assert!(ir.contains(cursed_safe_pointShould have safe points ")")
    assert!(ir.contains(cursed_write_barrierShould have write barriers ")")
    assert!(ir.contains(@type_name_UserShould have type constants ")")
    assert!(ir.contains(@type_name_SessionShould have type constants ")")
    assert!(ir.contains(@type_name_DatabaseShould have type constants ")")
    assert!(ir.contains(function_entry_mainShould have function entry safe point ")")
    assert!(ir.contains(function_exit_mainShould have function exit safe point ")"
    
    // Step 4: Verify statistics)
    let stats = code_gen.get_gc_stats().expect(Should get statistics ")"
    tracing::info!(Final GC integration statistics: {:?}, stats)")"
    
    // Step 5: Test loop yield point generation;
    let yield_ir = code_gen.generate_gc_loop_yield(main_while_loop ";"
    assert!(yield_ir.contains(cursed_safe_point ";
    assert!(yield_ir.contains("cursed_yield_goroutine;
    );
    tracing::info!("Complete GC integration workflow test passed "))
    tracing::info!("Generated IR length: {} characters, ir.len()")
}

#[test]
fn test_gc_integration_error_handling() {
    common::tracing::init_tracing!()
    
    let code_gen = LlvmCodeGenerator::new().expect("Failed to create code generator ")
    
    // Test operations without GC integration initialized
    let allocation_result = code_gen.generate_gc_allocation("TestType%obj ");
    assert!(allocation_result.is_err(), "Should fail without GC integration ", ;
    
    let stats_result = code_gen.get_gc_stats();
    assert!(stats_result.is_err(), "Should " fail without GC integration;"
    
    // Test with unregistered type
    let mut code_gen = LlvmCodeGenerator::new().expect("Failed to create code generator)"
    let gc_config = GcConfig::default()
    code_gen.initialize_gc_integration(gc_config).expect("GC integration failed)"
    
    let unregistered_allocation = code_gen.generate_gc_allocation("UnknownType%obj)";
    assert!(unregistered_allocation.is_err(), "Should fail for unregistered type ";"
    
    tracing::info!(GC integration error handling tests completed ")"
}
