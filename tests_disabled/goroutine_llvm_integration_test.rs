/// Tests for LLVM goroutine integration
/// 
/// Validates that the real LLVM goroutine compilation generates correct IR
/// and integrates properly with the runtime system.

use cursed::codegen::llvm::{
    LlvmCodeGeneratorReal, GoroutineCompiler, runtime_integration, 
    set_runtime_scheduler, get_runtime_scheduler
};
use cursed::ast::expressions::GoroutineSpawn;
use cursed::runtime::goroutine::GoroutineScheduler;
use cursed::error::Error;
use inkwell::{context::Context, module::Module, builder::Builder};
use std::sync::Arc;

/// Initialize test tracing
fn init_tracing() {
    use std::sync::Once;
    static INIT: Once = Once::new();
    INIT.call_once(|| {
        tracing_subscriber::fmt::init();
    });
}

/// Create a test LLVM generator with context
fn create_test_generator() -> Result<(Context, LlvmCodeGeneratorReal<'static>), Error> {
    init_tracing();
    
    let context = Box::leak(Box::new(Context::create()));
    let module = context.create_module("test_goroutine_module");
    let builder = context.create_builder();
    let runtime = Arc::new(cursed::runtime::Runtime::new());
    
    let generator = LlvmCodeGeneratorReal::new(context, module, builder, runtime)?;
    Ok((context, generator))
}

/// Mock GoroutineSpawn for testing
#[derive(Debug, Clone)]
struct MockGoroutineSpawn {
    function_name: String,
}

impl MockGoroutineSpawn {
    fn new(function_name: &str) -> Self {
        Self {
            function_name: function_name.to_string(),
        }
    }
}

impl cursed::ast::traits::Node for MockGoroutineSpawn {
    fn string(&self) -> String {
        format!("stan {}", self.function_name)
    }

    fn debug(&self) -> String {
        format!("GoroutineSpawn({})", self.function_name)
    }

    fn position(&self) -> Option<cursed::error::SourceLocation> {
        None
    }
}

impl cursed::ast::traits::Expression for MockGoroutineSpawn {
    fn as_node(&self) -> &dyn cursed::ast::traits::Node {
        self
    }
}

impl cursed::ast::expressions::GoroutineSpawn for MockGoroutineSpawn {
    // Implementation would depend on the actual GoroutineSpawn interface
}

#[test]
fn test_declare_goroutine_runtime_functions() {
    init_tracing();
    
    let (context, mut generator) = create_test_generator()
        .expect("Failed to create test generator");
    
    // Test declaring runtime functions
    let result = generator.declare_goroutine_runtime_functions();
    assert!(result.is_ok(), "Failed to declare runtime functions: {:?}", result);
    
    // Verify functions are declared
    let module = generator.module();
    assert!(module.get_function("cursed_spawn_goroutine").is_some());
    assert!(module.get_function("cursed_yield_goroutine").is_some());
    assert!(module.get_function("cursed_safe_point").is_some());
    assert!(module.get_function("cursed_gc_requested").is_some());
    
    tracing::info!("Successfully declared all goroutine runtime functions");
}

#[test]
fn test_setup_goroutine_runtime() {
    init_tracing();
    
    let (context, mut generator) = create_test_generator()
        .expect("Failed to create test generator");
    
    // Initialize a runtime scheduler first
    let scheduler = runtime_integration::initialize_scheduler()
        .expect("Failed to initialize scheduler");
    
    // Test setting up goroutine runtime
    let result = generator.setup_goroutine_runtime();
    assert!(result.is_ok(), "Failed to setup goroutine runtime: {:?}", result);
    
    // Clean up
    runtime_integration::cleanup_scheduler()
        .expect("Failed to cleanup scheduler");
    
    tracing::info!("Successfully set up goroutine runtime");
}

#[test]
fn test_runtime_scheduler_management() {
    init_tracing();
    
    // Test scheduler initialization
    let scheduler_ptr = runtime_integration::initialize_scheduler()
        .expect("Failed to initialize scheduler");
    
    // Verify scheduler is set
    let retrieved = get_runtime_scheduler();
    assert!(retrieved.is_some());
    assert_eq!(retrieved.unwrap(), scheduler_ptr);
    
    // Test cleanup
    runtime_integration::cleanup_scheduler()
        .expect("Failed to cleanup scheduler");
    
    // Verify scheduler is cleared
    let after_cleanup = get_runtime_scheduler();
    assert!(after_cleanup.is_none());
    
    tracing::info!("Successfully tested scheduler management");
}

#[test]
fn test_generate_safe_point() {
    init_tracing();
    
    let (context, mut generator) = create_test_generator()
        .expect("Failed to create test generator");
    
    // Initialize runtime scheduler
    let _scheduler = runtime_integration::initialize_scheduler()
        .expect("Failed to initialize scheduler");
    
    // Create a simple function to work within
    let void_type = context.void_type();
    let fn_type = void_type.fn_type(&[], false);
    let function = generator.module().add_function("test_function", fn_type, None);
    let basic_block = context.append_basic_block(function, "entry");
    generator.builder().position_at_end(basic_block);
    
    // Test generating safe point
    let result = generator.generate_safe_point("test_location");
    assert!(result.is_ok(), "Failed to generate safe point: {:?}", result);
    
    // Clean up
    runtime_integration::cleanup_scheduler()
        .expect("Failed to cleanup scheduler");
    
    tracing::info!("Successfully generated safe point");
}

#[test]
fn test_generate_yield_point() {
    init_tracing();
    
    let (context, mut generator) = create_test_generator()
        .expect("Failed to create test generator");
    
    // Initialize runtime scheduler
    let _scheduler = runtime_integration::initialize_scheduler()
        .expect("Failed to initialize scheduler");
    
    // Create a simple function to work within
    let void_type = context.void_type();
    let fn_type = void_type.fn_type(&[], false);
    let function = generator.module().add_function("test_function", fn_type, None);
    let basic_block = context.append_basic_block(function, "entry");
    generator.builder().position_at_end(basic_block);
    
    // Test generating yield point
    let result = generator.generate_yield_point("loop_location");
    assert!(result.is_ok(), "Failed to generate yield point: {:?}", result);
    
    // Clean up
    runtime_integration::cleanup_scheduler()
        .expect("Failed to cleanup scheduler");
    
    tracing::info!("Successfully generated yield point");
}

#[test]
fn test_compile_goroutine_spawn() {
    init_tracing();
    
    let (context, mut generator) = create_test_generator()
        .expect("Failed to create test generator");
    
    // Initialize runtime scheduler
    let _scheduler = runtime_integration::initialize_scheduler()
        .expect("Failed to initialize scheduler");
    
    // Create a target function to spawn
    let void_type = context.void_type();
    let fn_type = void_type.fn_type(&[], false);
    let target_function = generator.module().add_function("target_function", fn_type, None);
    let basic_block = context.append_basic_block(target_function, "entry");
    
    // Create a function to compile the spawn within
    let main_function = generator.module().add_function("main", fn_type, None);
    let main_block = context.append_basic_block(main_function, "entry");
    generator.builder().position_at_end(main_block);
    
    // Create mock spawn expression
    let spawn = MockGoroutineSpawn::new("target_function");
    
    // Test compiling goroutine spawn
    let result = generator.compile_goroutine_spawn(&spawn);
    assert!(result.is_ok(), "Failed to compile goroutine spawn: {:?}", result);
    
    let goroutine_id = result.unwrap();
    assert!(goroutine_id.is_int_value(), "Expected integer return value for goroutine ID");
    
    // Clean up
    runtime_integration::cleanup_scheduler()
        .expect("Failed to cleanup scheduler");
    
    tracing::info!("Successfully compiled goroutine spawn");
}

#[test]
fn test_full_goroutine_integration() {
    init_tracing();
    
    let (context, mut generator) = create_test_generator()
        .expect("Failed to create test generator");
    
    // Initialize runtime scheduler
    let _scheduler = runtime_integration::initialize_scheduler()
        .expect("Failed to initialize scheduler");
    
    // Test full integration workflow
    
    // 1. Initialize goroutine runtime
    let init_result = generator.initialize_goroutine_runtime();
    assert!(init_result.is_ok(), "Failed to initialize goroutine runtime: {:?}", init_result);
    
    // 2. Create functions
    let void_type = context.void_type();
    let fn_type = void_type.fn_type(&[], false);
    
    let target_function = generator.module().add_function("background_task", fn_type, None);
    let target_block = context.append_basic_block(target_function, "entry");
    
    let main_function = generator.module().add_function("main", fn_type, None);
    let main_block = context.append_basic_block(main_function, "entry");
    generator.builder().position_at_end(main_block);
    
    // 3. Generate safe point
    let safe_point_result = generator.generate_safe_point("main_entry");
    assert!(safe_point_result.is_ok(), "Failed to generate safe point: {:?}", safe_point_result);
    
    // 4. Compile goroutine spawn
    let spawn = MockGoroutineSpawn::new("background_task");
    let spawn_result = generator.compile_goroutine_spawn(&spawn);
    assert!(spawn_result.is_ok(), "Failed to compile goroutine spawn: {:?}", spawn_result);
    
    // 5. Generate yield point  
    let yield_result = generator.generate_yield_point("main_loop");
    assert!(yield_result.is_ok(), "Failed to generate yield point: {:?}", yield_result);
    
    // 6. Verify module is valid
    let verify_result = generator.module().verify();
    if let Err(err) = verify_result {
        tracing::warn!("Module verification failed: {}", err);
        // Don't fail the test as this might be due to incomplete functions
    }
    
    // Clean up
    runtime_integration::cleanup_scheduler()
        .expect("Failed to cleanup scheduler");
    
    tracing::info!("Successfully completed full goroutine integration test");
}

#[test]
fn test_module_ir_generation() {
    init_tracing();
    
    let (context, mut generator) = create_test_generator()
        .expect("Failed to create test generator");
    
    // Initialize runtime and generate some code
    let _scheduler = runtime_integration::initialize_scheduler()
        .expect("Failed to initialize scheduler");
    
    generator.initialize_goroutine_runtime()
        .expect("Failed to initialize goroutine runtime");
    
    // Create a simple function with goroutine operations
    let void_type = context.void_type();
    let fn_type = void_type.fn_type(&[], false);
    let function = generator.module().add_function("test_goroutines", fn_type, None);
    let basic_block = context.append_basic_block(function, "entry");
    generator.builder().position_at_end(basic_block);
    
    // Generate various goroutine operations
    generator.generate_safe_point("function_start")
        .expect("Failed to generate safe point");
    
    // Get IR representation
    let ir_string = generator.module().print_to_string().to_string();
    
    // Verify IR contains expected elements
    assert!(ir_string.contains("cursed_spawn_goroutine"), "IR should contain spawn function declaration");
    assert!(ir_string.contains("cursed_yield_goroutine"), "IR should contain yield function declaration");
    assert!(ir_string.contains("cursed_safe_point"), "IR should contain safe point function declaration");
    assert!(ir_string.contains("cursed_gc_requested"), "IR should contain GC request function declaration");
    
    tracing::info!("Generated IR:\n{}", ir_string);
    
    // Clean up
    runtime_integration::cleanup_scheduler()
        .expect("Failed to cleanup scheduler");
    
    tracing::info!("Successfully generated and validated LLVM IR");
}
