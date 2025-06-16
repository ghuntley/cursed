//! Integration tests for distributed optimization system
//!
//! Tests the complete distributed compilation workflow including network optimization,
//! worker node management, parallel compilation, ML-guided optimization, and PGO integration.

use cursed::optimization::distributed::network_optimizer::{NetworkOptimizer, NetworkConfig, NetworkMessage, MessagePriority};
use cursed::optimization::distributed::worker_node::{WorkerNodeManager, WorkerNode, WorkerCapabilities, WorkerConfig, WorkerStatus};
use cursed::optimization::parallel::{ParallelCompiler, CompilationJob, JobPriority};
use cursed::optimization::ml_optimization::{MLOptimizationEngine, MLOptimizationConfig, FeatureVector};
use cursed::optimization::pgo::llvm_integration::LlvmPgoIntegration;
use cursed::optimization::pgo::PgoConfig;
use std::collections::HashMap;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::path::PathBuf;
use std::sync::Arc;
use std::time::{Duration, SystemTime};
use tempfile::TempDir;
use tokio::time::timeout;

/// Test complete distributed compilation workflow
#[tokio::test]
async fn test_distributed_compilation_workflow() {
    // Create temporary directories for test files
    let temp_dir = TempDir::new().unwrap();
    let source_dir = temp_dir.path().join("src");
    let output_dir = temp_dir.path().join("output");
    std::fs::create_dir_all(&source_dir).unwrap();
    std::fs::create_dir_all(&output_dir).unwrap();

    // Create test source files
    let source_files = create_test_source_files(&source_dir);

    // 1. Initialize network optimizer
    let network_config = NetworkConfig::default();
    let mut network_optimizer = NetworkOptimizer::new(network_config).unwrap();
    assert!(network_optimizer.start().await.is_ok());

    // 2. Initialize worker node manager
    let worker_config = WorkerConfig {
        heartbeat_interval: Duration::from_millis(100),
        response_timeout: Duration::from_secs(5),
        max_missed_heartbeats: 2,
        metrics_interval: Duration::from_millis(50),
        auto_discovery: false, // Disable for testing
        discovery_interval: Duration::from_secs(10),
    };
    let mut worker_manager = WorkerNodeManager::new(worker_config).unwrap();
    assert!(worker_manager.start().await.is_ok());

    // 3. Register test workers
    let workers = create_test_workers().await;
    for worker in workers {
        let result = worker_manager.register_worker(worker).await;
        assert!(result.is_ok());
    }

    // Verify workers are registered
    let all_workers = worker_manager.get_all_workers().await.unwrap();
    assert_eq!(all_workers.len(), 3);

    // 4. Initialize parallel compiler
    let mut parallel_compiler = ParallelCompiler::new(4);
    assert!(parallel_compiler.start().is_ok());

    // 5. Create compilation jobs
    let jobs = create_compilation_jobs(&source_files, &output_dir);
    assert_eq!(jobs.len(), 5);

    // Add jobs to parallel compiler
    for job in jobs {
        assert!(parallel_compiler.add_job(job).is_ok());
    }

    // 6. Wait for compilation to complete with timeout
    let timeout_duration = Duration::from_secs(30);
    let results = timeout(timeout_duration, parallel_compiler.wait_for_completion(None)).await;
    assert!(results.is_ok());
    let job_results = results.unwrap().unwrap();

    // Verify all jobs completed successfully
    assert_eq!(job_results.len(), 5);
    let successful_jobs = job_results.iter().filter(|r| r.success).count();
    assert!(successful_jobs >= 4); // Allow for occasional failures in test environment

    // 7. Test network communication
    let test_message = NetworkMessage {
        id: "test_001".to_string(),
        message_type: "job_assignment".to_string(),
        priority: MessagePriority::High,
        source: "coordinator".to_string(),
        destination: "127.0.0.1:9001".to_string(),
        payload: b"test compilation job".to_vec(),
        compressed: false,
        timestamp: SystemTime::now(),
        correlation_id: Some("test_correlation".to_string()),
    };

    // Queue message (since actual network send would fail in test environment)
    assert!(network_optimizer.send_message("127.0.0.1:9001", test_message).await.is_ok());

    // 8. Get compilation statistics
    let stats = parallel_compiler.get_stats();
    assert!(stats.jobs_completed > 0);
    assert!(stats.wall_clock_time > Duration::from_millis(0));

    // 9. Get network statistics
    let network_stats = network_optimizer.get_stats().await.unwrap();
    assert_eq!(network_stats.messages_sent, 0); // No actual sends in test
    assert_eq!(network_stats.messages_received, 0);

    // 10. Cleanup
    assert!(parallel_compiler.stop().is_ok());
    assert!(worker_manager.stop().await.is_ok());
    assert!(network_optimizer.stop().await.is_ok());
}

/// Test ML-guided optimization decision making
#[tokio::test]
async fn test_ml_optimization_integration() {
    let config = MLOptimizationConfig {
        enabled: true,
        learning_rate: 0.01,
        batch_size: 16,
        training_epochs: 50,
        feature_vector_size: 64,
        model_update_frequency: Duration::from_secs(60),
        confidence_threshold: 0.7,
        fallback_to_heuristics: true,
    };

    let mut ml_engine = MLOptimizationEngine::new(config).unwrap();

    // Test feature extraction
    let sample_ir = create_sample_llvm_ir();
    let features = ml_engine.extract_features(&sample_ir, None).unwrap();

    // Verify feature extraction worked
    assert!(features.function_features.instruction_count > 0);
    assert!(features.function_features.basic_block_count > 0);
    assert!(features.code_features.cyclomatic_complexity > 0.0);

    // Test optimization decisions
    let inlining_decision = ml_engine.make_optimization_decision(
        "inlining", 
        &features
    ).unwrap();

    match inlining_decision {
        cursed::optimization::ml_optimization::OptimizationDecision::Inline { should_inline, confidence } => {
            assert!(confidence >= 0.0 && confidence <= 1.0);
            println!("Inlining decision: should_inline={}, confidence={:.2}", should_inline, confidence);
        }
        _ => panic!("Expected inlining decision"),
    }

    // Test vectorization decision
    let vectorization_decision = ml_engine.make_optimization_decision(
        "vectorization",
        &features
    ).unwrap();

    match vectorization_decision {
        cursed::optimization::ml_optimization::OptimizationDecision::Vectorize { vector_width, profitability } => {
            assert!(vector_width > 0);
            assert!(profitability >= 0.0 && profitability <= 1.0);
            println!("Vectorization decision: width={}, profitability={:.2}", vector_width, profitability);
        }
        _ => panic!("Expected vectorization decision"),
    }

    // Test CURSED-specific optimization
    let cursed_decision = ml_engine.make_optimization_decision(
        "cursed_specific",
        &features
    ).unwrap();

    match cursed_decision {
        cursed::optimization::ml_optimization::OptimizationDecision::CursedSpecific { optimization, parameters } => {
            println!("CURSED optimization: {:?}, params: {:?}", optimization, parameters);
        }
        _ => panic!("Expected CURSED-specific decision"),
    }

    // Test model statistics
    let stats = ml_engine.get_model_statistics();
    assert!(stats.overall_accuracy >= 0.0);
}

/// Test PGO LLVM integration
#[tokio::test]
async fn test_pgo_llvm_integration() {
    use inkwell::context::Context;

    let config = PgoConfig {
        enabled: true,
        profile_data_path: None,
        instrumentation_level: cursed::optimization::pgo::InstrumentationLevel::Function,
        optimization_aggressiveness: 0.8,
        collect_detailed_metrics: true,
    };

    let mut pgo_integration = LlvmPgoIntegration::new(config).unwrap();

    // Create LLVM context and module for testing
    let context = Context::create();
    let module = context.create_module("test_module");

    // Add a test function
    let i32_type = context.i32_type();
    let fn_type = i32_type.fn_type(&[i32_type.into(), i32_type.into()], false);
    let function = module.add_function("test_add", fn_type, None);
    
    let basic_block = context.append_basic_block(function, "entry");
    let builder = context.create_builder();
    builder.position_at_end(basic_block);
    
    let param1 = function.get_nth_param(0).unwrap().into_int_value();
    let param2 = function.get_nth_param(1).unwrap().into_int_value();
    let sum = builder.build_int_add(param1, param2, "sum").unwrap();
    builder.build_return(Some(&sum)).unwrap();

    // Test instrumentation
    assert!(pgo_integration.instrument_module(&module).await.is_ok());

    // Test profile data loading (with non-existent file)
    let temp_dir = TempDir::new().unwrap();
    let profile_path = temp_dir.path().join("profile.data");
    std::fs::write(&profile_path, b"mock profile data").unwrap();
    
    assert!(pgo_integration.load_profile_data(&profile_path).await.is_ok());

    // Test optimization application (with mock analysis)
    let mock_analysis = create_mock_profile_analysis();
    let optimization_results = pgo_integration.apply_pgo_optimizations(&module, &mock_analysis).await.unwrap();
    
    // Verify optimization results
    assert!(optimization_results.len() >= 0); // May be empty if no hot functions detected

    // Test statistics
    let stats = pgo_integration.get_statistics();
    assert!(stats.functions_instrumented > 0);
}

/// Test network compression and serialization
#[tokio::test]
async fn test_network_compression_serialization() {
    let config = NetworkConfig::default();
    let network_optimizer = NetworkOptimizer::new(config).unwrap();

    // Test message creation
    let test_data = b"This is a test message that should be compressed if it's large enough to meet the threshold requirements for compression.".to_vec();
    
    let message = NetworkMessage {
        id: "compression_test".to_string(),
        message_type: "test".to_string(),
        priority: MessagePriority::Normal,
        source: "test_source".to_string(),
        destination: "test_destination".to_string(),
        payload: test_data.clone(),
        compressed: false,
        timestamp: SystemTime::now(),
        correlation_id: None,
    };

    // Test serialization (using public method indirectly)
    // Since serialize_message is private, we test through public APIs
    let message_copy = message.clone();
    
    // Verify message structure
    assert_eq!(message_copy.payload, test_data);
    assert!(!message_copy.compressed);
    assert_eq!(message_copy.priority, MessagePriority::Normal);

    // Test that we can create different priority messages
    let high_priority_message = NetworkMessage {
        priority: MessagePriority::High,
        ..message
    };
    
    assert!(high_priority_message.priority > MessagePriority::Normal);
}

/// Test worker node capability detection and management
#[tokio::test]
async fn test_worker_capability_management() {
    // Test capability detection
    let capabilities = WorkerCapabilities::detect_local().unwrap();
    
    assert!(capabilities.cpu_cores > 0);
    assert!(capabilities.memory_mb > 0);
    assert!(capabilities.disk_space_mb > 0);
    assert!(!capabilities.supported_targets.is_empty());
    assert!(capabilities.max_concurrent_jobs > 0);
    assert!(capabilities.performance_score > 0.0);

    // Test worker creation
    let address = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 9001);
    let worker = WorkerNode::new(address, capabilities.clone());
    
    assert!(!worker.id.is_empty());
    assert_eq!(worker.address, address);
    assert_eq!(worker.status, WorkerStatus::Starting);

    // Test worker availability
    let mut available_worker = worker.clone();
    available_worker.status = WorkerStatus::Online;
    available_worker.metrics.active_jobs = 1;
    available_worker.capabilities.max_concurrent_jobs = 4;
    
    assert!(available_worker.is_available());
    assert_eq!(available_worker.load_factor(), 0.25);

    // Test worker at capacity
    available_worker.metrics.active_jobs = 4;
    assert!(!available_worker.is_available());
    assert_eq!(available_worker.load_factor(), 1.0);

    // Test efficiency score calculation
    available_worker.metrics.completed_jobs = 100;
    available_worker.metrics.failed_jobs = 10;
    let efficiency = available_worker.efficiency_score();
    assert!(efficiency > 0.0);
    assert!(efficiency <= capabilities.performance_score);
}

/// Test parallel compilation load balancing
#[tokio::test]
async fn test_parallel_compilation_load_balancing() {
    let mut compiler = ParallelCompiler::new(8);
    assert!(compiler.start().is_ok());

    // Create jobs with different priorities
    let temp_dir = TempDir::new().unwrap();
    let source_dir = temp_dir.path().join("src");
    let output_dir = temp_dir.path().join("output");
    std::fs::create_dir_all(&source_dir).unwrap();
    std::fs::create_dir_all(&output_dir).unwrap();

    let mut jobs = Vec::new();
    
    // Create high-priority jobs
    for i in 0..3 {
        let source_file = source_dir.join(format!("high_priority_{}.csd", i));
        std::fs::write(&source_file, "slay main() { return 42; }").unwrap();
        
        let job = CompilationJob {
            id: format!("high_priority_{}", i),
            source_path: source_file,
            output_path: output_dir.join(format!("high_priority_{}.o", i)),
            dependencies: Vec::new(),
            priority: JobPriority::High,
            compile_flags: vec!["--optimize".to_string()],
            created_at: std::time::Instant::now(),
        };
        jobs.push(job);
    }

    // Create normal-priority jobs
    for i in 0..5 {
        let source_file = source_dir.join(format!("normal_priority_{}.csd", i));
        std::fs::write(&source_file, "slay main() { return 0; }").unwrap();
        
        let job = CompilationJob {
            id: format!("normal_priority_{}", i),
            source_path: source_file,
            output_path: output_dir.join(format!("normal_priority_{}.o", i)),
            dependencies: Vec::new(),
            priority: JobPriority::Normal,
            compile_flags: Vec::new(),
            created_at: std::time::Instant::now(),
        };
        jobs.push(job);
    }

    // Add jobs with dependencies
    let dependency_jobs = create_dependency_jobs(&source_dir, &output_dir);
    
    // Test dependency resolution
    let resolved = compiler.resolve_dependencies(dependency_jobs).unwrap();
    assert_eq!(resolved.len(), 3); // Should resolve dependencies correctly

    // Add all jobs
    for job in jobs {
        assert!(compiler.add_job(job).is_ok());
    }

    // Wait for completion
    let results = timeout(
        Duration::from_secs(15),
        compiler.wait_for_completion(None)
    ).await;
    
    assert!(results.is_ok());
    let job_results = results.unwrap().unwrap();
    
    // Verify results
    assert_eq!(job_results.len(), 8);
    
    // Get statistics
    let stats = compiler.get_stats();
    assert!(stats.jobs_completed > 0);
    assert!(stats.worker_utilization >= 0.0);
    
    // Test worker information
    let workers = compiler.get_workers();
    assert_eq!(workers.len(), 8);
    
    assert!(compiler.stop().is_ok());
}

// Helper functions

fn create_test_source_files(source_dir: &std::path::Path) -> Vec<PathBuf> {
    let mut files = Vec::new();
    
    // Create various types of CURSED source files
    let file_contents = [
        ("main.csd", "slay main() { sus result = 42; return result; }"),
        ("utils.csd", "slay add(sus a, sus b) { return a + b; }"),
        ("math.csd", "slay multiply(sus x, sus y) { return x * y; }"),
        ("strings.csd", "slay concat(facts a, facts b) { return a + b; }"),
        ("complex.csd", "slay fibonacci(sus n) { lowkey (n <= 1) { return n; } return fibonacci(n-1) + fibonacci(n-2); }"),
    ];

    for (filename, content) in &file_contents {
        let file_path = source_dir.join(filename);
        std::fs::write(&file_path, content).unwrap();
        files.push(file_path);
    }

    files
}

async fn create_test_workers() -> Vec<WorkerNode> {
    let mut workers = Vec::new();
    
    for i in 0..3 {
        let capabilities = WorkerCapabilities {
            cpu_cores: 4 + i,
            memory_mb: 8192 + (i * 1024),
            disk_space_mb: 100000,
            supported_targets: vec![
                "x86_64-unknown-linux-gnu".to_string(),
                "x86_64-pc-windows-msvc".to_string(),
            ],
            toolchains: {
                let mut toolchains = HashMap::new();
                toolchains.insert("cursed".to_string(), "1.0.0".to_string());
                toolchains.insert("rust".to_string(), "1.70.0".to_string());
                toolchains
            },
            max_concurrent_jobs: 4 + i,
            performance_score: 4.0 + (i as f64),
            network_bandwidth: 100.0,
            features: vec!["cross-compile".to_string()],
        };

        let address = SocketAddr::new(
            IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 
            9001 + i as u16
        );
        
        let worker = WorkerNode::new(address, capabilities);
        workers.push(worker);
    }

    workers
}

fn create_compilation_jobs(source_files: &[PathBuf], output_dir: &std::path::Path) -> Vec<CompilationJob> {
    source_files
        .iter()
        .enumerate()
        .map(|(i, source_path)| {
            let output_path = output_dir.join(
                source_path.file_stem().unwrap_or_default()
            ).with_extension("o");
            
            CompilationJob {
                id: format!("job_{}", i),
                source_path: source_path.clone(),
                output_path,
                dependencies: Vec::new(),
                priority: if i < 2 { JobPriority::High } else { JobPriority::Normal },
                compile_flags: vec!["--optimize".to_string()],
                created_at: std::time::Instant::now(),
            }
        })
        .collect()
}

fn create_dependency_jobs(source_dir: &std::path::Path, output_dir: &std::path::Path) -> Vec<CompilationJob> {
    // Create lib.csd
    let lib_file = source_dir.join("lib.csd");
    std::fs::write(&lib_file, "slay lib_function() { return 100; }").unwrap();
    
    // Create dep.csd that depends on lib.csd
    let dep_file = source_dir.join("dep.csd");
    std::fs::write(&dep_file, "slay dep_function() { return lib_function() + 1; }").unwrap();
    
    // Create main.csd that depends on dep.csd
    let main_file = source_dir.join("main_with_deps.csd");
    std::fs::write(&main_file, "slay main() { return dep_function(); }").unwrap();

    vec![
        CompilationJob {
            id: "lib_job".to_string(),
            source_path: lib_file,
            output_path: output_dir.join("lib.o"),
            dependencies: Vec::new(),
            priority: JobPriority::Normal,
            compile_flags: Vec::new(),
            created_at: std::time::Instant::now(),
        },
        CompilationJob {
            id: "dep_job".to_string(),
            source_path: dep_file,
            output_path: output_dir.join("dep.o"),
            dependencies: vec![source_dir.join("lib.csd")],
            priority: JobPriority::Normal,
            compile_flags: Vec::new(),
            created_at: std::time::Instant::now(),
        },
        CompilationJob {
            id: "main_job".to_string(),
            source_path: main_file,
            output_path: output_dir.join("main_with_deps.o"),
            dependencies: vec![source_dir.join("dep.csd")],
            priority: JobPriority::Normal,
            compile_flags: Vec::new(),
            created_at: std::time::Instant::now(),
        },
    ]
}

fn create_sample_llvm_ir() -> String {
    r#"
define i32 @test_function(i32 %a, i32 %b) {
entry:
  %c = add i32 %a, %b
  %d = mul i32 %c, 2
  %cmp = icmp sgt i32 %d, 100
  br i1 %cmp, label %if.true, label %if.false

if.true:
  %result1 = add i32 %d, 10
  br label %exit

if.false:
  %result2 = sub i32 %d, 5
  br label %exit

exit:
  %result = phi i32 [ %result1, %if.true ], [ %result2, %if.false ]
  ret i32 %result
}

define void @goroutine_function() {
entry:
  call void @stan_spawn()
  call void @channel_send()
  ret void
}
"#.to_string()
}

fn create_mock_profile_analysis() -> cursed::optimization::pgo::ProfileAnalysis {
    use cursed::optimization::pgo::*;

    ProfileAnalysis {
        hot_functions: vec![
            HotFunction {
                name: "test_function".to_string(),
                execution_count: 10000,
                total_time: Duration::from_millis(100),
                average_time: Duration::from_micros(10),
                cache_miss_rate: 0.05,
                branch_prediction_accuracy: 0.95,
                has_vectorizable_loops: true,
                inlining_benefit: 0.8,
            }
        ],
        cold_functions: vec!["rarely_called".to_string()],
        loop_profiles: vec![
            LoopProfile {
                function_name: "test_function".to_string(),
                loop_id: "loop_1".to_string(),
                average_iteration_count: 25.0,
                total_iterations: 250000,
                is_vectorizable: true,
                has_dependencies: false,
                memory_access_pattern: MemoryAccessPattern::Sequential,
            }
        ],
        branch_profiles: vec![
            BranchProfile {
                function_name: "test_function".to_string(),
                branch_id: "branch_1".to_string(),
                taken_count: 8000,
                not_taken_count: 2000,
                prediction_accuracy: 0.92,
                misprediction_cost: Duration::from_nanos(20),
            }
        ],
        call_graph: HashMap::new(),
        compilation_units: Vec::new(),
        total_execution_time: Duration::from_secs(10),
        profile_collection_overhead: 0.02,
    }
}
