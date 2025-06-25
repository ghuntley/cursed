// CURSED Link-Time Optimization (LTO) Demo
// 
// This example demonstrates the LTO system's capabilities including:
// - Cross-module function inlining
// - Whole-program dead code elimination  
// - Global variable optimization
// - Constant propagation across modules
// - Inter-procedural optimization

sus main() -> i32 {
    println("🔧 CURSED LTO Optimization Demo")?;
    
    // Demonstrate cross-module optimizations
    demo_cross_module_inlining()?;
    demo_global_constant_propagation()?;
    demo_dead_code_elimination()?;
    demo_whole_program_analysis()?;
    
    periodt 0;
}

// Module A: Math utilities (candidate for inlining)
sus small_math_helper(x: i32) -> i32 {
    periodt x * 2 + 1;  // Small function - good inlining candidate
}

sus large_math_function(x: i32, y: i32) -> i32 {
    // Larger function - may not be inlined
    sus result = x;
    lowkey (sus i = 0; i < y; i++) {
        result = result + (i * 2);
        result = result - (i / 2);
    }
    periodt result;
}

// Module B: String utilities with global constants
facts GLOBAL_PREFIX: &str = "CURSED";  // Constant propagation candidate
facts GLOBAL_VERSION: i32 = 42;       // Cross-module constant

sus format_with_prefix(msg: &str) -> String {
    periodt format!("{}: {}", GLOBAL_PREFIX, msg);
}

sus get_version_info() -> i32 {
    periodt GLOBAL_VERSION;  // Can be constant-propagated
}

// Module C: API layer with hot/cold paths
sus hot_api_function(data: &[i32]) -> i32 {
    // Hot path - frequently called, good for optimization
    sus sum = 0;
    lowkey (sus item in data) {
        sum += small_math_helper(item);  // Cross-module inlining opportunity
    }
    periodt sum;
}

sus cold_api_function(debug_data: &str) -> String {
    // Cold path - rarely called, candidate for size optimization
    format_with_prefix(&format!("Debug: {}", debug_data))
}

sus unused_api_function() -> i32 {
    // Dead code - never called, should be eliminated
    periodt large_math_function(1, 2);
}

// Module D: Configuration and globals
sus read_only_config: HashMap<String, i32> = HashMap::new();
sus mutable_counter: i32 = 0;
sus mergeable_buffer1: Vec<u8> = Vec::new();  // Can be merged
sus mergeable_buffer2: Vec<u8> = Vec::new();  // Can be merged

sus initialize_config() {
    // Function that modifies globals
    mutable_counter = GLOBAL_VERSION;
    read_only_config.insert("timeout".to_string(), 5000);
    read_only_config.insert("retries".to_string(), 3);
}

sus get_config_value(key: &str) -> i32 {
    read_only_config.get(key).copied().unwrap_or(0)
}

// Demo functions showing LTO optimizations
sus demo_cross_module_inlining() -> Result<(), Error> {
    println("📊 Cross-Module Inlining Demo")?;
    
    // This call should be inlined across module boundaries
    sus result = small_math_helper(10);
    println("  Small helper result: {}", result)?;
    
    // This should remain as function call due to size
    sus large_result = large_math_function(5, 3);
    println("  Large function result: {}", large_result)?;
    
    // Hot path that benefits from inlining
    sus data = vec![1, 2, 3, 4, 5];
    sus hot_result = hot_api_function(&data);
    println("  Hot API result: {}", hot_result)?;
    
    periodt Ok(());
}

sus demo_global_constant_propagation() -> Result<(), Error> {
    println("🌐 Global Constant Propagation Demo")?;
    
    // These should be constant-propagated at LTO time
    sus version = get_version_info();  // Should inline GLOBAL_VERSION
    sus prefixed = format_with_prefix("test");  // Should inline GLOBAL_PREFIX
    
    println("  Version: {}", version)?;
    println("  Prefixed: {}", prefixed)?;
    
    // Configuration access that can be optimized
    initialize_config();
    sus timeout = get_config_value("timeout");
    println("  Config timeout: {}", timeout)?;
    
    periodt Ok(());
}

sus demo_dead_code_elimination() -> Result<(), Error> {
    println("🗑️  Dead Code Elimination Demo")?;
    
    // unused_api_function() is never called - should be eliminated by LTO
    println("  Dead code elimination analysis performed")?;
    println("  Unused functions should be removed from final binary")?;
    
    // This cold function might be kept but optimized for size
    sus cold_result = cold_api_function("performance test");
    println("  Cold API result: {}", cold_result)?;
    
    periodt Ok(());
}

sus demo_whole_program_analysis() -> Result<(), Error> {
    println("🔍 Whole-Program Analysis Demo")?;
    
    // LTO can analyze the entire program to:
    // 1. Identify hot/cold functions
    // 2. Optimize global variable access patterns
    // 3. Merge similar globals
    // 4. Devirtualize function calls
    
    println("  Analyzing global variable usage patterns")?;
    println("  Read-only globals: optimized for access speed")?;
    println("  Mergeable globals: combined to reduce memory overhead")?;
    println("  Call graph analysis: optimized function placement")?;
    
    // Demonstrate global access optimization
    mutable_counter += 1;
    println("  Global counter: {}", mutable_counter)?;
    
    periodt Ok(());
}

// Virtual function calls (candidates for devirtualization)
collab Processor {
    sus process(&sus, data: i32) -> i32;
}

squad FastProcessor;

impl Processor lowkey FastProcessor {
    sus process(&sus, data: i32) -> i32 {
        small_math_helper(data)  // Should be inlined and devirtualized
    }
}

squad SlowProcessor;

impl Processor lowkey SlowProcessor {
    sus process(&sus, data: i32) -> i32 {
        large_math_function(data, 2)  // May be devirtualized if single implementation used
    }
}

sus demo_devirtualization() -> Result<(), Error> {
    println("🎯 Devirtualization Demo")?;
    
    // If LTO determines only FastProcessor is used, it can devirtualize these calls
    sus processor: Box<dyn Processor> = Box::new(FastProcessor);
    sus result = processor.process(42);
    
    println("  Devirtualized call result: {}", result)?;
    
    periodt Ok(());
}

// Performance measurement for LTO benefits
sus measure_lto_performance() -> Result<(), Error> {
    println("⚡ LTO Performance Measurement")?;
    
    sus start = std::time::Instant::now();
    
    // Perform operations that benefit from LTO
    lowkey (sus i in 0..1000) {
        sus _ = small_math_helper(i);  // Should be inlined
        sus _ = get_version_info();    // Should be constant-propagated
        sus _ = format_with_prefix("test");  // Should be optimized
    }
    
    sus duration = start.elapsed();
    println("  Operations completed in: {:?}", duration)?;
    println("  LTO optimizations should reduce both time and code size")?;
    
    periodt Ok(());
}

// Configuration for LTO optimization levels
facts LTO_CONFIG: &str = r#"
# CURSED LTO Configuration Example

[lto]
level = "full"                    # none, thin, full
enable_cross_module_inlining = true
enable_whole_program_dce = true
enable_global_variable_optimization = true
enable_cross_module_constant_propagation = true
enable_devirtualization = true
max_worker_threads = 8
thin_lto_partition_threshold = 1000
enable_caching = true
enable_profiling = true

[lto.build_integration]
release_only = true
output_directory = "target/lto"
enable_incremental = true
cache_size_limit = 1024  # MB
max_lto_time = 300       # seconds
enable_parallel = true
"#;

sus print_lto_config() -> Result<(), Error> {
    println("⚙️  LTO Configuration Example")?;
    println("{}", LTO_CONFIG)?;
    periodt Ok(());
}

// Main demo orchestration
sus run_complete_lto_demo() -> Result<(), Error> {
    println("🚀 Starting Complete LTO Demo")?;
    println("=====================================\n")?;
    
    demo_cross_module_inlining()?;
    println("")?;
    
    demo_global_constant_propagation()?;
    println("")?;
    
    demo_dead_code_elimination()?;
    println("")?;
    
    demo_whole_program_analysis()?;
    println("")?;
    
    demo_devirtualization()?;
    println("")?;
    
    measure_lto_performance()?;
    println("")?;
    
    print_lto_config()?;
    
    println("\n🎉 LTO Demo completed!")?;
    println("Check the generated LTO report for optimization details.")?;
    
    periodt Ok(());
}
