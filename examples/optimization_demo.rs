/// CURSED Compiler Optimization System Demo
/// 
/// Demonstrates the comprehensive optimization system including different
/// optimization levels, parallel compilation, caching, and performance monitoring.

use std::path::PathBuf;
use std::time::{SystemTime, Duration, Instant};
use cursed::optimization::{
    OptimizationManager, OptimizationConfig, OptimizationLevel, LevelConfig,
    ParallelCompiler, CacheManager,
};
use cursed::optimization::compilation_speed::{CompilationUnit, CompilationStatus};
use cursed::error::Result;

fn main() -> Result<()> {
    // Initialize tracing for better output
    tracing_subscriber::fmt::init();

    println!("🚀 CURSED Compiler Optimization System Demo");
    println!("=" .repeat(60));

    // Demo 1: Optimization Levels
    demo_optimization_levels()?;
    
    // Demo 2: Parallel Compilation
    demo_parallel_compilation()?;
    
    // Demo 3: Caching System
    demo_caching_system()?;
    
    // Demo 4: Performance Comparison
    demo_performance_comparison()?;
    
    // Demo 5: Comprehensive Optimization Manager
    demo_optimization_manager()?;

    println!("\n✅ All optimization demos completed successfully!");
    
    Ok(())
}

fn demo_optimization_levels() -> Result<()> {
    println!("\n📊 Demo 1: Optimization Levels");
    println!("-".repeat(40));

    for level in [
        OptimizationLevel::None,
        OptimizationLevel::Basic, 
        OptimizationLevel::Standard,
        OptimizationLevel::Aggressive,
    ] {
        let config = LevelConfig::for_level(level);
        
        println!("\n🎯 Optimization Level: {}", level);
        println!("   Function inlining: {} (max size: {})", 
                config.enable_inlining, config.max_inline_size);
        println!("   Loop optimization: {} (max unroll: {})", 
                config.enable_loop_optimization, config.max_unroll_count);
        println!("   Dead code elimination: {}", config.enable_dead_code_elimination);
        println!("   Constant propagation: {}", config.enable_constant_propagation);
        println!("   Vectorization: {}", config.enable_vectorization);
        println!("   Link-time optimization: {}", config.enable_lto);
        println!("   Fast math: {}", config.enable_fast_math);
        println!("   Profile-guided optimization: {}", config.enable_pgo);
        println!("   Timeout: {:?}", config.timeout);
    }

    // Demo custom configuration
    let custom_config = LevelConfig::custom(OptimizationLevel::Basic)
        .enable_lto(true)
        .enable_fast_math(true)
        .max_inline_size(1000)
        .timeout(Duration::from_secs(180))
        .build();

    println!("\n🔧 Custom Configuration Example:");
    println!("   Base level: {}", custom_config.level);
    println!("   Custom LTO enabled: {}", custom_config.enable_lto);
    println!("   Custom fast math: {}", custom_config.enable_fast_math);
    println!("   Custom inline size: {}", custom_config.max_inline_size);

    Ok(())
}

fn demo_parallel_compilation() -> Result<()> {
    println!("\n⚡ Demo 2: Parallel Compilation");
    println!("-".repeat(40));

    let config = OptimizationConfig {
        enable_parallel_compilation: true,
        max_parallel_threads: 4,
        ..Default::default()
    };

    let parallel_compiler = ParallelCompiler::new(&config)?;
    
    // Create sample compilation units with dependencies
    let units = vec![
        CompilationUnit {
            id: "stdlib".to_string(),
            source_path: PathBuf::from("stdlib.csd"),
            module_name: "stdlib".to_string(),
            source_code: create_sample_code("stdlib", &[]),
            dependencies: vec![],
            last_modified: SystemTime::now(),
            status: CompilationStatus::Pending,
            priority: 3,
        },
        CompilationUnit {
            id: "utils".to_string(),
            source_path: PathBuf::from("utils.csd"),
            module_name: "utils".to_string(),
            source_code: create_sample_code("utils", &["stdlib"]),
            dependencies: vec!["stdlib".to_string()],
            last_modified: SystemTime::now(),
            status: CompilationStatus::Pending,
            priority: 2,
        },
        CompilationUnit {
            id: "main".to_string(),
            source_path: PathBuf::from("main.csd"),
            module_name: "main".to_string(),
            source_code: create_sample_code("main", &["stdlib", "utils"]),
            dependencies: vec!["stdlib".to_string(), "utils".to_string()],
            last_modified: SystemTime::now(),
            status: CompilationStatus::Pending,
            priority: 1,
        },
        CompilationUnit {
            id: "tests".to_string(),
            source_path: PathBuf::from("tests.csd"),
            module_name: "tests".to_string(),
            source_code: create_sample_code("tests", &["main"]),
            dependencies: vec!["main".to_string()],
            last_modified: SystemTime::now(),
            status: CompilationStatus::Pending,
            priority: 1,
        },
    ];

    println!("📦 Compiling {} modules with dependency relationships:", units.len());
    for unit in &units {
        println!("   {} (depends on: {:?})", unit.id, unit.dependencies);
    }

    let start_time = Instant::now();
    let results = parallel_compiler.compile_parallel(units)?;
    let compilation_time = start_time.elapsed();

    println!("\n✅ Parallel compilation completed!");
    println!("   Total modules: {}", results.len());
    println!("   Wall clock time: {:?}", compilation_time);

    for (unit_id, result) in &results {
        println!("   {} -> {:?} (worker: {}, time: {:?})", 
                unit_id, result.status, result.worker_id, result.compilation_time);
    }

    // Print statistics
    parallel_compiler.print_summary();

    Ok(())
}

fn demo_caching_system() -> Result<()> {
    println!("\n💾 Demo 3: Caching System");
    println!("-".repeat(40));

    let config = OptimizationConfig {
        enable_caching: true,
        ..Default::default()
    };

    let cache_manager = CacheManager::new(&config)?;
    
    // Create test compilation units
    let units = vec![
        CompilationUnit {
            id: "cached_module1".to_string(),
            source_path: PathBuf::from("cached1.csd"),
            module_name: "cached1".to_string(),
            source_code: "let x = 42;".to_string(),
            dependencies: vec![],
            last_modified: SystemTime::now(),
            status: CompilationStatus::Pending,
            priority: 1,
        },
        CompilationUnit {
            id: "cached_module2".to_string(),
            source_path: PathBuf::from("cached2.csd"),
            module_name: "cached2".to_string(),
            source_code: "let y = x * 2;".to_string(),
            dependencies: vec!["cached_module1".to_string()],
            last_modified: SystemTime::now(),
            status: CompilationStatus::Pending,
            priority: 1,
        },
    ];

    println!("🔑 Testing cache operations:");

    for (i, unit) in units.iter().enumerate() {
        let cache_key = cache_manager.generate_cache_key(unit, OptimizationLevel::Standard)?;
        println!("   Module {}: cache key = {}", unit.id, &cache_key[..16]);

        // Test cache miss
        let is_valid_before = cache_manager.is_cache_valid(unit, &cache_key)?;
        println!("   Cache valid before store: {}", is_valid_before);
        assert!(!is_valid_before);

        // Store compiled result
        let compiled_data = format!("compiled_bytecode_for_module_{}", i).into_bytes();
        cache_manager.store_cache_entry(
            unit,
            OptimizationLevel::Standard,
            cursed::optimization::cache::CacheEntryType::Bytecode,
            &compiled_data,
        )?;

        // Test cache hit
        let is_valid_after = cache_manager.is_cache_valid(unit, &cache_key)?;
        println!("   Cache valid after store: {}", is_valid_after);
        assert!(is_valid_after);

        // Retrieve cached data
        if let Some(retrieved_data) = cache_manager.retrieve_cache_entry(&cache_key)? {
            println!("   Retrieved {} bytes from cache", retrieved_data.len());
            assert_eq!(retrieved_data, compiled_data);
        }
    }

    // Print cache statistics
    cache_manager.print_summary();

    Ok(())
}

fn demo_performance_comparison() -> Result<()> {
    println!("\n🏎️  Demo 4: Performance Comparison");
    println!("-".repeat(40));

    let test_units = create_test_compilation_units(20);
    
    println!("📈 Comparing compilation performance across optimization levels:");

    for level in [OptimizationLevel::None, OptimizationLevel::Standard, OptimizationLevel::Aggressive] {
        let config = OptimizationConfig {
            enable_parallel_compilation: true,
            enable_caching: false, // Disable caching for fair comparison
            optimization_level: match level {
                OptimizationLevel::None => 0,
                OptimizationLevel::Basic => 1,
                OptimizationLevel::Standard => 2,
                OptimizationLevel::Aggressive => 3,
            },
            max_parallel_threads: 4,
            ..Default::default()
        };

        let parallel_compiler = ParallelCompiler::new(&config)?;
        
        let start_time = Instant::now();
        let results = parallel_compiler.compile_parallel(test_units.clone())?;
        let compilation_time = start_time.elapsed();

        let stats = parallel_compiler.get_statistics();
        
        println!("\n   {} Level:", level);
        println!("     Compilation time: {:?}", compilation_time);
        println!("     Units processed: {}", results.len());
        println!("     Parallelization efficiency: {:.2}x", stats.efficiency());
        println!("     Average worker utilization: {:.1}%", stats.average_utilization * 100.0);
    }

    Ok(())
}

fn demo_optimization_manager() -> Result<()> {
    println!("\n🎛️  Demo 5: Comprehensive Optimization Manager");
    println!("-".repeat(40));

    let config = OptimizationConfig {
        enable_advanced_llvm: true,
        enable_parallel_compilation: true,
        enable_caching: true,
        enable_jit_optimization: true,
        enable_memory_optimization: true,
        enable_profiling: true,
        optimization_level: 2,
        max_parallel_threads: 4,
        ..Default::default()
    };

    let mut manager = OptimizationManager::new(config)?;
    
    println!("🔧 Created optimization manager with all features enabled");
    
    // Test optimization level switching
    println!("\n🔄 Testing optimization level switching:");
    for level in [OptimizationLevel::Basic, OptimizationLevel::Aggressive, OptimizationLevel::Standard] {
        manager.set_optimization_level(level)?;
        println!("   Switched to: {}", level);
        
        let level_config = manager.optimization_level_manager().current_config();
        println!("     LTO enabled: {}", level_config.enable_lto);
        println!("     Fast math: {}", level_config.enable_fast_math);
        println!("     Max inline size: {}", level_config.max_inline_size);
    }

    // Test PGO manager
    if let Some(pgo_manager) = manager.pgo_manager_mut() {
        println!("\n📊 Testing Profile-Guided Optimization:");
        println!("   PGO ready: {}", pgo_manager.is_ready());
        
        // Simulate loading profile data
        if let Err(_) = pgo_manager.load_profile_data("non_existent_profile.prof") {
            println!("   Profile data loading (simulated): would load real profile data");
        }
    }

    // Print comprehensive summary
    println!("\n📋 Comprehensive optimization system summary:");
    manager.print_comprehensive_summary();

    Ok(())
}

fn create_sample_code(module_name: &str, imports: &[&str]) -> String {
    let mut code = String::new();
    
    // Add imports
    for import in imports {
        code.push_str(&format!("import \"{}\";\n", import));
    }
    
    if !imports.is_empty() {
        code.push('\n');
    }

    // Add module-specific code
    match module_name {
        "stdlib" => {
            code.push_str("sus max(a: i32, b: i32) -> i32 {\n");
            code.push_str("    lowkey (a > b) {\n");
            code.push_str("        return a;\n");
            code.push_str("    } highkey {\n");
            code.push_str("        return b;\n");
            code.push_str("    }\n");
            code.push_str("}\n\n");
            code.push_str("sus min(a: i32, b: i32) -> i32 {\n");
            code.push_str("    return lowkey (a < b) { a } highkey { b };\n");
            code.push_str("}\n");
        }
        "utils" => {
            code.push_str("sus calculate_average(numbers: [i32]) -> f64 {\n");
            code.push_str("    sus sum = 0;\n");
            code.push_str("    lowkey (sus i = 0; i < numbers.length; i++) {\n");
            code.push_str("        sum += numbers[i];\n");
            code.push_str("        yolo;\n");
            code.push_str("    }\n");
            code.push_str("    return sum as f64 / numbers.length as f64;\n");
            code.push_str("}\n");
        }
        "main" => {
            code.push_str("sus main() {\n");
            code.push_str("    facts numbers = [1, 2, 3, 4, 5];\n");
            code.push_str("    facts avg = calculate_average(numbers);\n");
            code.push_str("    facts max_val = max(10, 20);\n");
            code.push_str("    println(\"Average: {}, Max: {}\", avg, max_val);\n");
            code.push_str("}\n");
        }
        "tests" => {
            code.push_str("sus test_max_function() {\n");
            code.push_str("    assert(max(5, 3) == 5);\n");
            code.push_str("    assert(max(-1, -5) == -1);\n");
            code.push_str("}\n\n");
            code.push_str("sus test_min_function() {\n");
            code.push_str("    assert(min(5, 3) == 3);\n");
            code.push_str("    assert(min(-1, -5) == -5);\n");
            code.push_str("}\n");
        }
        _ => {
            code.push_str(&format!("// Module: {}\n", module_name));
            code.push_str("sus example_function() {\n");
            code.push_str("    facts value = 42;\n");
            code.push_str("    return value;\n");
            code.push_str("}\n");
        }
    }

    code
}

fn create_test_compilation_units(count: usize) -> Vec<CompilationUnit> {
    (0..count).map(|i| {
        let dependencies = if i > 0 { 
            vec![format!("module{}", i - 1)]
        } else { 
            vec![] 
        };

        CompilationUnit {
            id: format!("module{}", i),
            source_path: PathBuf::from(format!("module{}.csd", i)),
            module_name: format!("module{}", i),
            source_code: create_sample_code(&format!("module{}", i), &[]),
            dependencies,
            last_modified: SystemTime::now(),
            status: CompilationStatus::Pending,
            priority: 1,
        }
    }).collect()
}
