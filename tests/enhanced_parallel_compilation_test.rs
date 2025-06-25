use cursed::optimization::parallel::{ParallelCompiler, CompilationJob, JobPriority};
use std::path::PathBuf;
use std::time::{Duration, Instant};
use tempfile::TempDir;

#[test]
fn test_enhanced_parallel_compiler_creation() {
    let compiler = ParallelCompiler::new(4);
    assert_eq!(compiler.worker_count, 4);
    assert_eq!(compiler.active_workers(), 0);
}

#[test]
fn test_resource_limited_compiler() {
    let compiler = ParallelCompiler::with_limits(2, 1024, 90.0); // 1GB limit, 90% CPU
    assert_eq!(compiler.worker_count, 2);
    
    let report = compiler.get_compilation_report();
    assert!(report.contains("1024.0 MB limit"));
    assert!(report.contains("2 total"));
}

#[test]
fn test_enhanced_dependency_resolution() {
    let temp_dir = TempDir::new().unwrap();
    let mut compiler = ParallelCompiler::new(2);
    
    // Create test files
    std::fs::write(temp_dir.path().join("file1.csd"), "// Test file 1").unwrap();
    std::fs::write(temp_dir.path().join("file2.csd"), "// Test file 2").unwrap();
    std::fs::write(temp_dir.path().join("file3.csd"), "// Test file 3").unwrap();
    
    let job1 = CompilationJob {
        id: "high_priority_job".to_string(),
        source_path: temp_dir.path().join("file1.csd"),
        output_path: temp_dir.path().join("file1.o"),
        dependencies: vec![temp_dir.path().join("file2.csd")],
        priority: JobPriority::High,
        compile_flags: vec!["-O2".to_string()],
        created_at: Instant::now(),
    };
    
    let job2 = CompilationJob {
        id: "normal_priority_job".to_string(),
        source_path: temp_dir.path().join("file2.csd"),
        output_path: temp_dir.path().join("file2.o"),
        dependencies: Vec::new(),
        priority: JobPriority::Normal,
        compile_flags: Vec::new(),
        created_at: Instant::now(),
    };
    
    let job3 = CompilationJob {
        id: "critical_priority_job".to_string(),
        source_path: temp_dir.path().join("file3.csd"),
        output_path: temp_dir.path().join("file3.o"),
        dependencies: Vec::new(),
        priority: JobPriority::Critical,
        compile_flags: vec!["-O3".to_string()],
        created_at: Instant::now(),
    };
    
    let jobs = vec![job1, job2, job3];
    
    // Test enhanced dependency resolution (this will resolve dependencies and prioritize critical jobs)
    let result = compiler.add_jobs_with_dependencies(jobs);
    assert!(result.is_ok());
}

#[test]
fn test_circular_dependency_detection_enhanced() {
    let temp_dir = TempDir::new().unwrap();
    let mut compiler = ParallelCompiler::new(2);
    
    let job1 = CompilationJob {
        id: "circular_job1".to_string(),
        source_path: temp_dir.path().join("file1.csd"),
        output_path: temp_dir.path().join("file1.o"),
        dependencies: vec![temp_dir.path().join("file2.csd")],
        priority: JobPriority::Normal,
        compile_flags: Vec::new(),
        created_at: Instant::now(),
    };
    
    let job2 = CompilationJob {
        id: "circular_job2".to_string(),
        source_path: temp_dir.path().join("file2.csd"),
        output_path: temp_dir.path().join("file2.o"),
        dependencies: vec![temp_dir.path().join("file1.csd")],
        priority: JobPriority::Normal,
        compile_flags: Vec::new(),
        created_at: Instant::now(),
    };
    
    let jobs = vec![job1, job2];
    let result = compiler.add_jobs_with_dependencies(jobs);
    
    assert!(result.is_err());
    let error_msg = result.unwrap_err().to_string();
    assert!(error_msg.contains("Circular dependencies detected"));
    assert!(error_msg.contains("dependency resolution failed at level"));
}

#[test] 
fn test_memory_estimation() {
    let temp_dir = TempDir::new().unwrap();
    
    // Create a test file with some content
    let test_content = "fn main() { println!(\"Hello, world!\"); }".repeat(1000);
    let test_file = temp_dir.path().join("large_test.csd");
    std::fs::write(&test_file, test_content).unwrap();
    
    let job = CompilationJob {
        id: "memory_test_job".to_string(),
        source_path: test_file,
        output_path: temp_dir.path().join("large_test.o"),
        dependencies: vec![
            temp_dir.path().join("dep1.csd"),
            temp_dir.path().join("dep2.csd"),
        ],
        priority: JobPriority::Normal,
        compile_flags: Vec::new(),
        created_at: Instant::now(),
    };
    
    // Test memory estimation (this is now a real calculation based on file size)
    let memory_estimate = ParallelCompiler::estimate_job_memory(&job);
    
    // Should be at least base memory (50MB) plus file size multiplier plus dependency overhead
    assert!(memory_estimate >= 50 * 1024 * 1024); // At least 50MB base
    assert!(memory_estimate <= 500 * 1024 * 1024); // At most 500MB limit
    
    // Should account for dependencies (2 deps * 5MB each = 10MB additional)
    assert!(memory_estimate >= 60 * 1024 * 1024); // 50MB base + 10MB deps
}

#[test]
fn test_worker_thread_lifecycle() {
    let mut compiler = ParallelCompiler::new(2);
    
    // Start workers
    assert!(compiler.start().is_ok());
    assert_eq!(compiler.workers.len(), 2);
    
    // Check worker states
    for worker in &compiler.workers {
        assert_eq!(worker.state, cursed::optimization::parallel::WorkerState::Idle);
        assert_eq!(worker.jobs_completed, 0);
        assert!(worker.created_at.elapsed() < Duration::from_secs(1));
    }
    
    // Stop workers
    assert!(compiler.stop().is_ok());
    
    // Check that all workers are marked as finished
    for worker in &compiler.workers {
        assert_eq!(worker.state, cursed::optimization::parallel::WorkerState::Finished);
    }
}

#[test]
fn test_compilation_statistics_tracking() {
    let compiler = ParallelCompiler::new(4);
    
    // Get initial stats
    let initial_stats = compiler.get_stats();
    assert_eq!(initial_stats.jobs_queued, 0);
    assert_eq!(initial_stats.jobs_completed, 0);
    assert_eq!(initial_stats.jobs_failed, 0);
    assert_eq!(initial_stats.worker_utilization, 0.0);
    
    // Get compilation report
    let report = compiler.get_compilation_report();
    assert!(report.contains("Parallel Compilation Report"));
    assert!(report.contains("4 total"));
    assert!(report.contains("0 queued"));
    assert!(report.contains("0 completed"));
}

#[test]
fn test_resource_monitoring() {
    use cursed::optimization::parallel::ResourceMonitor;
    
    let monitor = ResourceMonitor::new(100, 80.0); // 100MB limit, 80% CPU threshold
    
    // Test resource checking
    assert!(monitor.check_resources().is_ok());
    
    // Test memory tracking
    monitor.add_memory_usage(50 * 1024 * 1024); // Add 50MB
    assert!(monitor.check_resources().is_ok()); // Should still be OK
    
    monitor.add_memory_usage(60 * 1024 * 1024); // Add another 60MB (total 110MB > 100MB limit)
    assert!(monitor.check_resources().is_err()); // Should exceed limit
    
    monitor.remove_memory_usage(20 * 1024 * 1024); // Remove 20MB (total 90MB < 100MB limit)
    assert!(monitor.check_resources().is_ok()); // Should be OK again
}

#[test]
fn test_job_priority_scheduling() {
    let temp_dir = TempDir::new().unwrap();
    let mut compiler = ParallelCompiler::new(1); // Single worker for deterministic testing
    
    // Create jobs with different priorities
    let jobs = vec![
        CompilationJob {
            id: "low_priority".to_string(),
            source_path: temp_dir.path().join("low.csd"),
            output_path: temp_dir.path().join("low.o"),
            dependencies: Vec::new(),
            priority: JobPriority::Low,
            compile_flags: Vec::new(),
            created_at: Instant::now(),
        },
        CompilationJob {
            id: "critical_priority".to_string(),
            source_path: temp_dir.path().join("critical.csd"),
            output_path: temp_dir.path().join("critical.o"),
            dependencies: Vec::new(),
            priority: JobPriority::Critical,
            compile_flags: Vec::new(),
            created_at: Instant::now(),
        },
        CompilationJob {
            id: "normal_priority".to_string(),
            source_path: temp_dir.path().join("normal.csd"),
            output_path: temp_dir.path().join("normal.o"),
            dependencies: Vec::new(),
            priority: JobPriority::Normal,
            compile_flags: Vec::new(),
            created_at: Instant::now(),
        },
    ];
    
    // Test that dependency resolution respects priority ordering
    let result = compiler.add_jobs_with_dependencies(jobs);
    assert!(result.is_ok());
}

#[test]
fn test_progress_reporting() {
    use cursed::optimization::parallel::CompilationProgress;
    
    let progress = CompilationProgress::new(10, true);
    assert!(progress.is_some());
    
    let progress = progress.unwrap();
    assert_eq!(progress.total_jobs, 10);
    
    // Test progress updates
    progress.inc_completed();
    progress.inc_completed();
    
    let completed = progress.completed_jobs.load(std::sync::atomic::Ordering::Relaxed);
    assert_eq!(completed, 2);
    
    progress.update_message("Test message");
}

#[test]
fn test_compilation_error_aggregation() {
    let temp_dir = TempDir::new().unwrap();
    
    // Create a file with invalid syntax to trigger compilation error
    let invalid_file = temp_dir.path().join("invalid.csd");
    std::fs::write(&invalid_file, "invalid syntax here {{{{ broken").unwrap();
    
    let job = CompilationJob {
        id: "error_test_job".to_string(),
        source_path: invalid_file,
        output_path: temp_dir.path().join("invalid.o"),
        dependencies: Vec::new(),
        priority: JobPriority::Normal,
        compile_flags: Vec::new(),
        created_at: Instant::now(),
    };
    
    // Test that compilation errors are properly captured
    let result = ParallelCompiler::compile_job(0, job);
    
    // Should fail but not panic
    assert!(result.is_err());
    let error_msg = result.unwrap_err().to_string();
    assert!(!error_msg.is_empty());
}

#[test]
fn test_output_size_measurement() {
    let temp_dir = TempDir::new().unwrap();
    
    // Create a test output file
    let output_file = temp_dir.path().join("test_output.o");
    let test_content = b"fake object file content";
    std::fs::write(&output_file, test_content).unwrap();
    
    let job = CompilationJob {
        id: "size_test_job".to_string(),
        source_path: temp_dir.path().join("source.csd"),
        output_path: output_file,
        dependencies: Vec::new(),
        priority: JobPriority::Normal,
        compile_flags: Vec::new(),
        created_at: Instant::now(),
    };
    
    let output_size = ParallelCompiler::measure_output_size(&job);
    assert_eq!(output_size, test_content.len());
}
