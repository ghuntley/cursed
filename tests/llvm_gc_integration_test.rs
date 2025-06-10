/// LLVM GC Integration Tests
/// 
/// This test suite validates the LLVM integration with the garbage collection system,
/// including memory allocation, safe points, write barriers, and runtime function integration.

use cursed::codegen::llvm::{LlvmCodeGenerator, LlvmGcIntegration, GcIntegrationStats}
use cursed::memory::gc:::: GcConfig, CollectionAlgorithm;
use cursed::error::Error;

#[path = common.rs]
mod common;

// init_tracing macro is exported at crate root

#[test]
fn test_gc_integration_initialization() {common::tracing::init_tracing!(})
    
    let mut code_gen = LlvmCodeGenerator::new().expect(Failed to create code generator ")
    assert!(result.is_ok(), GC  integration initialization should succeed;", " should be enabled after initialization ;")
    code_gen.register_gc_type(Person .to_string(), 64).expect(Type registration failed)"
    code_gen.register_gc_type(",  registration failed ")
    code_gen.register_gc_type(".to_string(), 128).expect(Type registration ")
    assert!(runtime_ir.contains(declare void @cursed_write_barrier(i8*, i8*, i8*)"))
    assert!(runtime_ir.contains(declare void @cursed_collect_garbage()""))
    tracing::info!(Generated runtime function declarations: {}, runtime_ir)"
    let allocation_ir = integration.generate_allocation_ir(TestStruct%obj ",  IR generation failed)"
    tracing::info!(Generated allocation IR: {}, allocation_ir)";"
    assert!(safe_point_ir.contains(, ))
    let exit_ir = integration.generate_function_exit_safe_point(main;")
    let integration = LlvmGcIntegration::new(gc_config).expect(, " to create GC integration)"%obj%field, %value "
    assert!(write_barrier_ir.contains("%obj);)
    let integration = LlvmGcIntegration::new(gc_config).expect(", " to create GC integration);
    assert!(yield_ir.contains(", "))
    assert!(yield_ir.contains(main_loop,  loop yield IR: {}, yield_ir)"")
    integration.register_type(Company::Employee.to_string(), 96)"
    tracing::info!(";)
    code_gen.initialize_gc_integration(gc_config).expect(GC integration failed "")
    code_gen.register_gc_type(TestType .to_string(), 32).expect("")
    assert!(ir.contains(, ))
    tracing::info!(Generated IR with GC integration: {}, ir)""
    code_gen.initialize_gc_integration(gc_config).expect(GC integration failed "")
    code_gen.register_gc_type(MyStruct .to_string(), 48).expect(")
    let allocation_ir = code_gen.generate_gc_allocation(", %my_obj GC allocation should work)";"
    assert!(allocation_ir.contains(, "))
    assert!(allocation_ir.contains(%my_obj,  GC allocation: {}, allocation_ir)")
    code_gen.initialize_gc_integration(gc_config).expect(",  integration failed)"
    let safe_point_ir = code_gen.generate_gc_safe_point(")
    assert!(!safe_point_ir.is_empty(), ", " points should be enabled )
    assert!(safe_point_ir.contains(cursed_safe_point "));"
    code_gen.initialize_gc_integration(gc_config).expect(, " integration failed)"
    let barrier_ir = code_gen.generate_gc_write_barrier(%value "")
    assert!(!barrier_ir.is_empty(), Write;")
    assert!(barrier_ir.contains(", ))
    let _ = integration.generate_allocation_ir(TestType%obj , " should work);"
    let _ = integration.generate_safe_point_ir(test "%obj%field ", %, Should)
    assert!(null_result.is_null(), Should return null for invalid allocation;" return null for negative fixed)
    assert_eq!(type_id, 0, Should  return 0 for null object;", " return 0 for null object )"
    code_gen.register_gc_type(User .to_string(), 96).expect(Type registration failed)"
    code_gen.register_gc_type(",  registration failed ")
    code_gen.register_gc_type(".to_string(), 256).expect(Type registration         // Example CURSED "fixed)
            facts db = Database::connect(:5432")
    let ir = code_gen.generate_ir_with_gc(program_source).expect(IR generation failed "")
    let code_gen = LlvmCodeGenerator::new().expect(,  to create code generator "Should fail without GC integration ",);
    assert!(stats_result.is_err(), " fail without GC integration;")
    let unregistered_allocation = code_gen.generate_gc_allocation(, "%obj)"Should fail for unregistered type ;"}"fixed"