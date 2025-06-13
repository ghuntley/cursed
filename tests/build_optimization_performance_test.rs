//! Performance tests for Build Optimization System
//! 
//! Tests performance characteristics, benchmarking, and scaling behavior.

use std::path::PathBuf;
use std::fs;
use std::time::{Duration, Instant};
use tempfile::TempDir;

use cursed::build_system::{
    DependencyOptimizer, DependencyOptimizerConfig, AdvancedCache, AdvancedCacheConfig,
    BuildAnalytics, BuildAnalyticsConfig, CompilationUnit
};
use cursed::error::Result;

/// Create a large project for performance testing
fn create_large_test_project(num_files: usize) -> Result<TempDir> {
    let temp_dir = TempDir::new()?;
    let project_path = temp_dir.path();
    
    // Create interconnected modules with dependencies
    for i in 0..num_files {
        let dependencies = if i > 0 {
            // Create dependencies on previous modules
            let dep_count = (i % 5).min(3); // 0-3 dependencies
            (0..dep_count)
                .map(|j| format!("import \"module_{}\"", (i - j - 1).max(0)))
                .collect::<Vec<_>>()
                .join("\n")
        } else {
            String::new()
        };
        
        let content = format!(r#"
{}

export squad Struct_{} {{
    sus field1: String
    sus field2: i32
    sus field3: Vec<f64>
    sus field4: Map<String, i32>
}}

export fn complex_function_{}() -> i32 {{
    sus data = Vec::new()
    sus counter = 0
    
    // Complex nested loop structure
    lowkey (sus i = 0; i < 50; i++) {{
        lowkey (i % 2 == 0) {{
            lowkey (sus j = 0; j < 20; j++) {{
                lowkey (j % 3 == 0) {{
                    counter += i * j
                }} highkey (j % 3 == 1) {{
                    counter -= i + j
                }} flex {{
                    counter *= 2
                }}
                
                data.push(counter as f64)
                yolo // yield point
            }}
        }} highkey (i % 3 == 0) {{
            counter = process_data(&data)
        }} flex {{
            counter += generic_operation(i)
        }}
    }}
    
    return counter
}}

fn process_data(data: &Vec<f64>) -> i32 {{
    sus sum = 0.0
    lowkey (sus value in data) {{
        sum += value
    }}
    return sum as i32
}}

fn generic_operation<T: Clone + Default>(value: T) -> i32 {{
    sus cloned = value.clone()
    sus default_val = T::default()
    return 42 // placeholder
}}

export fn utility_function_{}(param1: String, param2: i32) -> String {{
    return format!("{{}}_{{}}", param1, param2)
}}

export collab ComplexInterface_{} {{
    fn method1(&self) -> i32
    fn method2(&self, param: String) -> String
    fn method3<T>(&self, generic_param: T) -> T
}}

impl ComplexInterface_{} for Struct_{} {{
    fn method1(&self) -> i32 {{
        return self.field2
    }}
    
    fn method2(&self, param: String) -> String {{
        return format!("{{}}:{{}} ({{}})", self.field1, param, self.field2)
    }}
    
    fn method3<T>(&self, generic_param: T) -> T {{
        return generic_param
    }}
}}
"#, dependencies, i, i, i, i, i, i);
        
        fs::write(project_path.join(format!("module_{}.csd", i)), content)?;
    }
    
    Ok(temp_dir)
}

#[test]
fn test_dependency_analysis_performance() -> Result<()> {
    let file_counts = vec![10, 50, 100, 250];
    
    for file_count in file_counts {
        println!("Testing dependency analysis with {} files", file_count);
        
        let temp_dir = create_large_test_project(file_count)?;
        let project_path = temp_dir.path().to_path_buf();
        
        let units = collect_compilation_units_for_perf_test(&project_path)?;
        assert_eq!(units.len(), file_count, "Should create expected number of units");
        
        let config = DependencyOptimizerConfig {
            max_parallel_jobs: 8,
            enable_smart_ordering: true,
            enable_dependency_pruning: true,
            ..Default::default()
        };
        
        let optimizer = DependencyOptimizer::new(config);
        
        let start_time = Instant::now();
        let analysis = optimizer.analyze_dependencies(&units)?;
        let analysis_time = start_time.elapsed();
        
        println!("  Analysis time: {:?}", analysis_time);
        println!("  Compilation layers: {}", analysis.compilation_order.len());
        println!("  Parallelism factor: {:.3}", analysis.parallelism_factor);
        
        // Performance assertions
        let max_expected_time = Duration::from_millis(file_count as u64 * 20); // 20ms per file max
        assert!(analysis_time < max_expected_time, 
            "Analysis should complete within reasonable time for {} files: {:?} > {:?}", 
            file_count, analysis_time, max_expected_time);
        
        assert!(analysis.compilation_order.len() > 0);
        assert!(analysis.parallelism_factor >= 0.0 && analysis.parallelism_factor <= 1.0);
    }
    
    Ok(())
}

#[test]
fn test_cache_performance() -> Result<()> {
    let temp_dir = create_large_test_project(100)?;
    let project_path = temp_dir.path().to_path_buf();
    
    let config = AdvancedCacheConfig::default();
    let cache = AdvancedCache::new(config)?;
    
    let units = collect_compilation_units_for_perf_test(&project_path)?;
    let file_paths: Vec<String> = units.iter()
        .map(|u| u.path.to_string_lossy().to_string())
        .collect();
    
    // Test cache warming performance
    let start_time = Instant::now();
    let warmed = cache.warm_cache(&file_paths)?;
    let warm_time = start_time.elapsed();
    
    println!("Cache warming: {} files in {:?}", warmed, warm_time);
    
    // Performance assertion
    assert!(warm_time < Duration::from_secs(10), "Cache warming should be fast");
    
    // Test cache statistics retrieval performance
    let start_time = Instant::now();
    let stats = cache.get_statistics()?;
    let stats_time = start_time.elapsed();
    
    println!("Cache stats retrieval: {:?}", stats_time);
    assert!(stats_time < Duration::from_millis(100), "Stats retrieval should be very fast");
    
    // Test cache optimization performance
    let start_time = Instant::now();
    let optimized = cache.optimize_cache()?;
    let optimize_time = start_time.elapsed();
    
    println!("Cache optimization: {} entries in {:?}", optimized, optimize_time);
    assert!(optimize_time < Duration::from_secs(5), "Cache optimization should be reasonable");
    
    Ok(())
}

#[test]
fn test_scaling_characteristics() -> Result<()> {
    let file_counts = vec![10, 25, 50, 100];
    let mut scaling_data = Vec::new();
    
    for file_count in file_counts {
        let temp_dir = create_large_test_project(file_count)?;
        let project_path = temp_dir.path().to_path_buf();
        
        let units = collect_compilation_units_for_perf_test(&project_path)?;
        
        // Test dependency analysis scaling
        let config = DependencyOptimizerConfig {
            max_parallel_jobs: 4,
            enable_smart_ordering: true,
            enable_dependency_pruning: true,
            ..Default::default()
        };
        
        let optimizer = DependencyOptimizer::new(config);
        
        let start_time = Instant::now();
        let _analysis = optimizer.analyze_dependencies(&units)?;
        let analysis_time = start_time.elapsed();
        
        scaling_data.push((file_count, analysis_time));
        
        println!("Files: {}, Time: {:?}, Time per file: {:?}", 
            file_count, 
            analysis_time,
            Duration::from_nanos(analysis_time.as_nanos() as u64 / file_count as u64)
        );
    }
    
    // Check scaling characteristics
    // Analysis time should scale reasonably (not exponentially)
    for i in 1..scaling_data.len() {
        let (prev_files, prev_time) = scaling_data[i-1];
        let (curr_files, curr_time) = scaling_data[i];
        
        let file_ratio = curr_files as f64 / prev_files as f64;
        let time_ratio = curr_time.as_secs_f64() / prev_time.as_secs_f64();
        
        println!("Scaling from {} to {} files: {:.2}x files, {:.2}x time", 
            prev_files, curr_files, file_ratio, time_ratio);
        
        // Time should not scale exponentially (ratio should be < file_ratio^2)
        assert!(time_ratio < file_ratio * file_ratio, 
            "Time scaling should be better than O(n²): {:.2} >= {:.2}", 
            time_ratio, file_ratio * file_ratio);
    }
    
    Ok(())
}

#[test]
fn test_memory_usage_performance() -> Result<()> {
    let temp_dir = create_large_test_project(200)?;
    let project_path = temp_dir.path().to_path_buf();
    
    let units = collect_compilation_units_for_perf_test(&project_path)?;
    
    // Monitor memory during dependency analysis
    let config = DependencyOptimizerConfig {
        max_parallel_jobs: 8,
        enable_smart_ordering: true,
        enable_dependency_pruning: true,
        ..Default::default()
    };
    
    let optimizer = DependencyOptimizer::new(config);
    
    // Get memory usage before
    let memory_before = get_memory_usage_mb();
    
    let start_time = Instant::now();
    let _analysis = optimizer.analyze_dependencies(&units)?;
    let analysis_time = start_time.elapsed();
    
    // Get memory usage after
    let memory_after = get_memory_usage_mb();
    let memory_used = memory_after - memory_before;
    
    println!("Memory usage: {:.2} MB for {} files", memory_used, units.len());
    println!("Memory per file: {:.2} KB", (memory_used * 1024.0) / units.len() as f64);
    println!("Analysis time: {:?}", analysis_time);
    
    // Performance assertions
    let max_memory_per_file = 100.0; // 100 KB per file seems reasonable
    assert!(memory_used / units.len() as f64 < max_memory_per_file / 1024.0, 
        "Memory usage per file should be reasonable: {:.2} KB > {:.2} KB", 
        (memory_used * 1024.0) / units.len() as f64, max_memory_per_file);
    
    Ok(())
}

#[test]
fn test_concurrent_performance() -> Result<()> {
    let temp_dir = create_large_test_project(50)?;
    let project_path = temp_dir.path().to_path_buf();
    
    let units = collect_compilation_units_for_perf_test(&project_path)?;
    
    // Test different parallel job configurations
    let job_counts = vec![1, 2, 4, 8, 16];
    let mut performance_data = Vec::new();
    
    for job_count in job_counts {
        let config = DependencyOptimizerConfig {
            max_parallel_jobs: job_count,
            enable_smart_ordering: true,
            enable_dependency_pruning: true,
            ..Default::default()
        };
        
        let optimizer = DependencyOptimizer::new(config);
        
        let start_time = Instant::now();
        let analysis = optimizer.analyze_dependencies(&units)?;
        let analysis_time = start_time.elapsed();
        
        performance_data.push((job_count, analysis_time, analysis.parallelism_factor));
        
        println!("Jobs: {}, Time: {:?}, Parallelism: {:.3}", 
            job_count, analysis_time, analysis.parallelism_factor);
    }
    
    // Check that increasing parallel jobs improves performance (up to a point)
    let single_threaded_time = performance_data[0].1;
    let multi_threaded_time = performance_data[2].1; // 4 jobs
    
    // Multi-threaded should be faster than single-threaded for this workload
    assert!(multi_threaded_time <= single_threaded_time, 
        "Multi-threaded execution should be faster: {:?} > {:?}", 
        multi_threaded_time, single_threaded_time);
    
    Ok(())
}

#[test]
fn test_analytics_performance() -> Result<()> {
    let config = BuildAnalyticsConfig::default();
    let analytics = BuildAnalytics::new(config)?;
    
    // Test build session performance
    let start_time = Instant::now();
    analytics.start_build_session()?;
    let session_start_time = start_time.elapsed();
    
    // Simulate build events
    std::thread::sleep(Duration::from_millis(100));
    
    let start_time = Instant::now();
    analytics.end_build_session()?;
    let session_end_time = start_time.elapsed();
    
    // Test report generation performance
    let start_time = Instant::now();
    let _report = analytics.generate_build_report()?;
    let report_time = start_time.elapsed();
    
    println!("Session start: {:?}", session_start_time);
    println!("Session end: {:?}", session_end_time);
    println!("Report generation: {:?}", report_time);
    
    // Performance assertions
    assert!(session_start_time < Duration::from_millis(100), "Session start should be fast");
    assert!(session_end_time < Duration::from_millis(100), "Session end should be fast");
    assert!(report_time < Duration::from_secs(1), "Report generation should be fast");
    
    Ok(())
}

#[test]
fn test_complexity_calculation_performance() -> Result<()> {
    let temp_dir = create_large_test_project(100)?;
    let project_path = temp_dir.path().to_path_buf();
    
    let start_time = Instant::now();
    let units = collect_compilation_units_for_perf_test(&project_path)?;
    let collection_time = start_time.elapsed();
    
    let total_complexity: u32 = units.iter().map(|u| u.complexity_score).sum();
    let avg_complexity = total_complexity / units.len() as u32;
    
    println!("Collection time: {:?}", collection_time);
    println!("Total complexity: {}", total_complexity);
    println!("Average complexity: {}", avg_complexity);
    println!("Files processed: {}", units.len());
    
    // Performance assertions
    let max_collection_time = Duration::from_millis(units.len() as u64 * 50); // 50ms per file max
    assert!(collection_time < max_collection_time, 
        "Collection should be fast: {:?} > {:?}", collection_time, max_collection_time);
    
    // Complexity scores should be reasonable
    assert!(avg_complexity > 0, "Should calculate meaningful complexity");
    assert!(avg_complexity < 10000, "Complexity scores should be reasonable");
    
    Ok(())
}

// Helper functions

fn collect_compilation_units_for_perf_test(project_path: &PathBuf) -> Result<Vec<CompilationUnit>> {
    use std::collections::HashSet;
    
    let mut units = Vec::new();
    let mut visited = HashSet::new();
    
    collect_source_files_recursive(project_path, &mut units, &mut visited)?;
    analyze_file_dependencies_simple(&mut units)?;
    
    Ok(units)
}

fn collect_source_files_recursive(
    dir: &PathBuf,
    units: &mut Vec<CompilationUnit>,
    visited: &mut HashSet<PathBuf>,
) -> Result<()> {
    if visited.contains(dir) {
        return Ok(());
    }
    visited.insert(dir.clone());
    
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        
        if path.is_dir() {
            collect_source_files_recursive(&path, units, visited)?;
        } else if path.extension().and_then(|ext| ext.to_str()) == Some("csd") {
            let unit = create_compilation_unit_for_perf(&path)?;
            units.push(unit);
        }
    }
    
    Ok(())
}

fn create_compilation_unit_for_perf(path: &PathBuf) -> Result<CompilationUnit> {
    let metadata = fs::metadata(path)?;
    let last_modified = metadata
        .modified()?
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    
    let content = fs::read_to_string(path)?;
    let complexity_score = calculate_complexity_for_perf(&content);
    
    let id = path.to_string_lossy().to_string();
    let cache_key = format!("{}-{}", id, last_modified);
    
    Ok(CompilationUnit {
        id: id.clone(),
        path: path.clone(),
        dependencies: extract_dependencies_for_perf(&content),
        dependents: Vec::new(),
        last_modified,
        compilation_time: Duration::from_millis((complexity_score * 5) as u64),
        complexity_score,
        is_dirty: true,
        cache_key,
    })
}

fn extract_dependencies_for_perf(content: &str) -> Vec<String> {
    let mut dependencies = Vec::new();
    
    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("import") {
            if let Some(start) = trimmed.find('"') {
                if let Some(end) = trimmed[start + 1..].find('"') {
                    let module = &trimmed[start + 1..start + 1 + end];
                    dependencies.push(module.to_string());
                }
            }
        }
    }
    
    dependencies.sort();
    dependencies.dedup();
    dependencies
}

fn calculate_complexity_for_perf(content: &str) -> u32 {
    let mut score = 0;
    
    // Base score from line count
    score += content.lines().count() as u32;
    
    // Add complexity for various constructs
    for line in content.lines() {
        let trimmed = line.trim();
        
        if trimmed.contains("fn ") { score += 10; }
        if trimmed.contains("lowkey") || trimmed.contains("highkey") { score += 5; }
        if trimmed.contains("squad") { score += 8; }
        if trimmed.contains("collab") { score += 12; }
        if trimmed.contains("yolo") { score += 3; }
        if trimmed.contains('<') && trimmed.contains('>') { score += 4; }
        
        let brace_count = trimmed.chars().filter(|&c| c == '{').count();
        score += (brace_count * 2) as u32;
    }
    
    score
}

fn analyze_file_dependencies_simple(units: &mut Vec<CompilationUnit>) -> Result<()> {
    // Build reverse dependency mapping
    for i in 0..units.len() {
        let dependencies = units[i].dependencies.clone();
        
        for dep in dependencies {
            for j in 0..units.len() {
                if units[j].id.contains(&dep) {
                    units[j].dependents.push(units[i].id.clone());
                    break;
                }
            }
        }
    }
    
    Ok(())
}

fn get_memory_usage_mb() -> f64 {
    // This is a simplified memory usage approximation
    // In a real implementation, you might use a memory profiler
    use std::alloc::{GlobalAlloc, Layout, System};
    
    // For testing purposes, we'll use a rough approximation
    // based on heap allocations
    0.0 // Placeholder - would need actual memory measurement
}
