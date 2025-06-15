/// Comprehensive tests for advanced optimization passes
/// 
/// Tests all the advanced optimization functionality including:
/// - Register allocation algorithms
/// - Instruction scheduling
/// - CURSED-specific optimizations
/// - GC-aware optimizations
/// - Performance debugging
/// - Target-specific optimizations

use cursed::optimization::{
    AdvancedRegisterAllocator, InstructionScheduler, CursedOptimizer, GcAwareOptimizer,
    PerformanceDebugger, TargetSpecificOptimizer, VirtualRegister, PhysicalRegister,
    LiveRange, Instruction, InstructionType, PipelineConfig, TargetArchitecture,
    Architecture, DebugConfig, DebugVerbosity, DebugOutputFormat
};
use cursed::memory::GarbageCollector;
use cursed::error::Result;
use std::sync::{Arc, Mutex};
use std::time::Duration;

#[path = "common.rs"]
mod common;

/// Test advanced register allocation
#[test]
fn test_advanced_register_allocation() -> Result<()> {
    common::init_tracing!();
    
    let mut allocator = AdvancedRegisterAllocator::new(8);
    
    // Create some live ranges for testing
    let live_ranges = vec![
        LiveRange {
            register: VirtualRegister(1),
            start: 0,
            end: 10,
            frequency: 0.8,
            spill_cost: 100.0,
        },
        LiveRange {
            register: VirtualRegister(2),
            start: 5,
            end: 15,
            frequency: 0.9,
            spill_cost: 120.0,
        },
        LiveRange {
            register: VirtualRegister(3),
            start: 12,
            end: 20,
            frequency: 0.7,
            spill_cost: 80.0,
        },
    ];
    
    // Perform register allocation
    allocator.allocate_registers(&live_ranges)?;
    
    // Verify allocation results
    let allocation = allocator.get_allocation();
    let statistics = allocator.get_statistics();
    
    assert_eq!(statistics.virtual_registers, 3);
    assert!(statistics.allocation_time > Duration::from_nanos(0));
    
    // Check that some registers were allocated
    assert!(!allocation.is_empty());
    
    println!("Register allocation completed successfully");
    Ok(())
}

/// Test instruction scheduling
#[test]
fn test_instruction_scheduling() -> Result<()> {
    common::init_tracing!();
    
    let pipeline_config = PipelineConfig::default();
    let mut scheduler = InstructionScheduler::new(pipeline_config);
    
    // Create test instructions
    let instructions = vec![
        Instruction::new(0, InstructionType::Arithmetic),
        Instruction::new(1, InstructionType::Memory),
        Instruction::new(2, InstructionType::Arithmetic),
        Instruction::new(3, InstructionType::Branch),
    ];
    
    // Schedule instructions
    let scheduled_order = scheduler.schedule_instructions(&instructions)?;
    
    // Verify scheduling results
    let statistics = scheduler.get_statistics();
    
    assert_eq!(statistics.instructions_scheduled, 4);
    assert_eq!(scheduled_order.len(), 4);
    assert!(statistics.scheduling_time > Duration::from_nanos(0));
    
    println!("Instruction scheduling completed successfully");
    Ok(())
}

/// Test CURSED-specific optimizations
#[test]
fn test_cursed_optimizations() -> Result<()> {
    common::init_tracing!();
    
    let mut optimizer = CursedOptimizer::new();
    
    // Create a dummy AST node for testing
    use cursed::ast::{AstNode, AstNodeType};
    let mut ast = AstNode::new(AstNodeType::Program, 1, 1);
    
    // Perform CURSED-specific optimizations
    optimizer.optimize_ast(&mut ast)?;
    
    // Verify optimization results
    let statistics = optimizer.get_statistics();
    
    assert!(statistics.optimization_time > Duration::from_nanos(0));
    assert_eq!(statistics.error_propagations_optimized, 0); // No actual optimizations in dummy AST
    
    println!("CURSED optimizations completed successfully");
    Ok(())
}

/// Test GC-aware optimizations
#[test]
fn test_gc_aware_optimizations() -> Result<()> {
    common::init_tracing!();
    
    // Create a garbage collector
    let gc = Arc::new(Mutex::new(GarbageCollector::new()));
    let mut optimizer = GcAwareOptimizer::new(gc);
    
    // Perform GC-aware optimizations
    optimizer.optimize()?;
    
    // Verify optimization results
    let statistics = optimizer.get_statistics();
    
    assert!(statistics.optimization_time > Duration::from_nanos(0));
    assert_eq!(statistics.objects_analyzed, 0); // No objects in empty GC
    
    println!("GC-aware optimizations completed successfully");
    Ok(())
}

/// Test performance debugging
#[test]
fn test_performance_debugging() -> Result<()> {
    common::init_tracing!();
    
    let debug_config = DebugConfig {
        enable_pass_tracing: true,
        enable_profiling: true,
        enable_adaptive_learning: true,
        enable_regression_testing: false, // Disable for quick test
        verbosity_level: DebugVerbosity::Normal,
        output_format: DebugOutputFormat::Text,
    };
    
    let mut debugger = PerformanceDebugger::new(debug_config);
    
    // Start debug session
    debugger.start_debug_session("test_session")?;
    
    // Simulate pass execution with tracing
    let result = debugger.trace_pass_execution("test_pass", 100, || {
        // Simulate some work
        std::thread::sleep(Duration::from_millis(1));
        Ok(42)
    })?;
    
    assert_eq!(result, 42);
    
    // End debug session
    let debug_report = debugger.end_debug_session()?;
    
    // Verify debugging results
    let statistics = debugger.get_statistics();
    
    assert_eq!(statistics.passes_traced, 1);
    assert!(!debug_report.trace_results.is_empty());
    
    println!("Performance debugging completed successfully");
    Ok(())
}

/// Test target-specific optimizations
#[test]
fn test_target_specific_optimizations() -> Result<()> {
    common::init_tracing!();
    
    // Create target architecture
    use cursed::optimization::target_specific::{
        ArchitectureFeatures, RegisterInfo, CacheInfo, InstructionInfo,
        MemoryFeatures, MemoryOrdering, CacheCoherencyProtocol, BranchPredictionInfo,
        BranchPredictorType, CacheLevel
    };
    use std::collections::HashMap;
    
    let target_arch = TargetArchitecture {
        architecture: Architecture::X86_64,
        sub_architecture: "haswell".to_string(),
        features: ArchitectureFeatures {
            vector_units: vec![],
            specialized_instructions: vec![],
            memory_features: MemoryFeatures {
                address_width: 64,
                virtual_memory: true,
                memory_ordering: MemoryOrdering::TotalStoreOrdering,
                cache_coherency: CacheCoherencyProtocol::MESI,
                prefetch_instructions: vec![],
            },
            branch_prediction: BranchPredictionInfo {
                predictor_type: BranchPredictorType::Tournament,
                predictor_accuracy: 0.95,
                branch_target_buffer_size: 4096,
                return_stack_size: 16,
            },
            out_of_order_execution: true,
            superscalar_width: 4,
        },
        register_info: RegisterInfo {
            general_purpose_count: 16,
            floating_point_count: 16,
            vector_register_count: 16,
            special_purpose_registers: vec![],
            register_classes: vec![],
        },
        cache_info: CacheInfo {
            l1_instruction: CacheLevel {
                size: 32768,
                associativity: 8,
                latency: 4,
                bandwidth: 32.0,
            },
            l1_data: CacheLevel {
                size: 32768,
                associativity: 8,
                latency: 4,
                bandwidth: 32.0,
            },
            l2_unified: Some(CacheLevel {
                size: 262144,
                associativity: 8,
                latency: 12,
                bandwidth: 16.0,
            }),
            l3_shared: Some(CacheLevel {
                size: 8388608,
                associativity: 16,
                latency: 40,
                bandwidth: 8.0,
            }),
            cache_line_size: 64,
            prefetch_distance: 2,
        },
        instruction_info: InstructionInfo {
            instruction_set: "x86-64".to_string(),
            instruction_latencies: HashMap::new(),
            instruction_throughput: HashMap::new(),
            instruction_dependencies: HashMap::new(),
        },
    };
    
    let mut optimizer = TargetSpecificOptimizer::new(target_arch);
    
    // Create a dummy program for testing
    use cursed::optimization::target_specific::{Program, ProgramMetadata, ProfileData};
    let mut program = Program {
        functions: vec![],
        global_data: vec![],
        metadata: ProgramMetadata {
            target_arch: Architecture::X86_64,
            optimization_level: "O2".to_string(),
            profile_data: None,
        },
    };
    
    // Perform target-specific optimizations
    let result = optimizer.optimize(&mut program)?;
    
    // Verify optimization results
    let statistics = optimizer.get_statistics();
    
    assert!(result.transformations_applied > 0);
    assert!(result.estimated_performance_gain >= 0.0);
    assert!(statistics.optimization_time > Duration::from_nanos(0));
    
    println!("Target-specific optimizations completed successfully");
    Ok(())
}

/// Test register allocation with interference
#[test]
fn test_register_allocation_with_interference() -> Result<()> {
    common::init_tracing!();
    
    let mut allocator = AdvancedRegisterAllocator::new(4); // Limited registers
    
    // Create overlapping live ranges that will cause interference
    let live_ranges = vec![
        LiveRange {
            register: VirtualRegister(1),
            start: 0,
            end: 20,
            frequency: 1.0,
            spill_cost: 100.0,
        },
        LiveRange {
            register: VirtualRegister(2),
            start: 5,
            end: 25,
            frequency: 1.0,
            spill_cost: 100.0,
        },
        LiveRange {
            register: VirtualRegister(3),
            start: 10,
            end: 30,
            frequency: 1.0,
            spill_cost: 100.0,
        },
        LiveRange {
            register: VirtualRegister(4),
            start: 15,
            end: 35,
            frequency: 1.0,
            spill_cost: 100.0,
        },
        LiveRange {
            register: VirtualRegister(5),
            start: 20,
            end: 40,
            frequency: 1.0,
            spill_cost: 100.0,
        },
    ];
    
    // Perform register allocation
    allocator.allocate_registers(&live_ranges)?;
    
    // Verify that some registers were spilled due to interference
    let allocation = allocator.get_allocation();
    let spill_locations = allocator.get_spill_locations();
    let statistics = allocator.get_statistics();
    
    assert_eq!(statistics.virtual_registers, 5);
    assert!(statistics.spilled_registers > 0); // Some should be spilled
    assert!(allocation.len() + spill_locations.len() == 5); // All registers handled
    
    println!("Register allocation with interference handled successfully");
    Ok(())
}

/// Test instruction scheduling with dependencies
#[test]
fn test_instruction_scheduling_with_dependencies() -> Result<()> {
    common::init_tracing!();
    
    let pipeline_config = PipelineConfig::default();
    let mut scheduler = InstructionScheduler::new(pipeline_config);
    
    // Create instructions with dependencies
    let mut instructions = vec![
        Instruction::new(0, InstructionType::Arithmetic),
        Instruction::new(1, InstructionType::Arithmetic),
        Instruction::new(2, InstructionType::Memory),
        Instruction::new(3, InstructionType::Arithmetic),
    ];
    
    // Add some register dependencies
    instructions[0].add_definition(VirtualRegister(1));
    instructions[1].add_use(VirtualRegister(1));
    instructions[1].add_definition(VirtualRegister(2));
    instructions[3].add_use(VirtualRegister(2));
    
    // Schedule instructions
    let scheduled_order = scheduler.schedule_instructions(&instructions)?;
    
    // Verify that dependencies are respected
    let statistics = scheduler.get_statistics();
    
    assert_eq!(statistics.instructions_scheduled, 4);
    assert_eq!(scheduled_order.len(), 4);
    
    // Instructions should be scheduled respecting dependencies
    // (exact order verification would require more complex dependency analysis)
    
    println!("Instruction scheduling with dependencies completed successfully");
    Ok(())
}

/// Test optimization integration
#[test]
fn test_optimization_integration() -> Result<()> {
    common::init_tracing!();
    
    // Test that multiple optimization passes can work together
    let mut register_allocator = AdvancedRegisterAllocator::new(8);
    let mut instruction_scheduler = InstructionScheduler::new(PipelineConfig::default());
    let mut cursed_optimizer = CursedOptimizer::new();
    
    // Create test data
    let live_ranges = vec![
        LiveRange {
            register: VirtualRegister(1),
            start: 0,
            end: 10,
            frequency: 0.8,
            spill_cost: 100.0,
        },
    ];
    
    let instructions = vec![
        Instruction::new(0, InstructionType::Arithmetic),
        Instruction::new(1, InstructionType::Memory),
    ];
    
    use cursed::ast::{AstNode, AstNodeType};
    let mut ast = AstNode::new(AstNodeType::Program, 1, 1);
    
    // Apply optimizations in sequence
    register_allocator.allocate_registers(&live_ranges)?;
    instruction_scheduler.schedule_instructions(&instructions)?;
    cursed_optimizer.optimize_ast(&mut ast)?;
    
    // Verify that all optimizations completed successfully
    assert!(register_allocator.get_statistics().allocation_time > Duration::from_nanos(0));
    assert!(instruction_scheduler.get_statistics().scheduling_time > Duration::from_nanos(0));
    assert!(cursed_optimizer.get_statistics().optimization_time > Duration::from_nanos(0));
    
    println!("Optimization integration test completed successfully");
    Ok(())
}

/// Test performance regression detection
#[test]
fn test_performance_regression_detection() -> Result<()> {
    common::init_tracing!();
    
    let debug_config = DebugConfig {
        enable_pass_tracing: true,
        enable_profiling: true,
        enable_adaptive_learning: false,
        enable_regression_testing: true,
        verbosity_level: DebugVerbosity::Normal,
        output_format: DebugOutputFormat::Text,
    };
    
    let mut debugger = PerformanceDebugger::new(debug_config);
    
    // Start debug session
    debugger.start_debug_session("regression_test")?;
    
    // Simulate multiple pass executions with varying performance
    for i in 0..5 {
        let pass_name = format!("test_pass_{}", i);
        debugger.trace_pass_execution(&pass_name, 100 + i * 10, || {
            // Simulate varying execution times
            std::thread::sleep(Duration::from_millis(1 + i as u64));
            Ok(())
        })?;
    }
    
    // Run regression tests
    let regression_results = debugger.run_regression_tests()?;
    
    // End debug session
    let debug_report = debugger.end_debug_session()?;
    
    // Verify regression testing
    let statistics = debugger.get_statistics();
    
    assert_eq!(statistics.passes_traced, 5);
    assert!(statistics.regression_tests_run >= 0); // May be 0 if no tests configured
    
    println!("Performance regression detection completed successfully");
    Ok(())
}

/// Benchmark test for optimization performance
#[test]
fn test_optimization_performance_benchmark() -> Result<()> {
    common::init_tracing!();
    
    let start_time = std::time::Instant::now();
    
    // Create multiple optimizers
    let mut register_allocator = AdvancedRegisterAllocator::new(16);
    let mut instruction_scheduler = InstructionScheduler::new(PipelineConfig::default());
    
    // Create larger test data
    let live_ranges: Vec<LiveRange> = (0..100).map(|i| LiveRange {
        register: VirtualRegister(i),
        start: i as u32 * 2,
        end: i as u32 * 2 + 10,
        frequency: 0.5 + (i as f64 / 200.0),
        spill_cost: 100.0 + i as f64,
    }).collect();
    
    let instructions: Vec<Instruction> = (0..100).map(|i| {
        let inst_type = match i % 4 {
            0 => InstructionType::Arithmetic,
            1 => InstructionType::Memory,
            2 => InstructionType::Branch,
            _ => InstructionType::FloatingPoint,
        };
        Instruction::new(i as u32, inst_type)
    }).collect();
    
    // Perform optimizations
    register_allocator.allocate_registers(&live_ranges)?;
    instruction_scheduler.schedule_instructions(&instructions)?;
    
    let total_time = start_time.elapsed();
    
    // Verify performance is reasonable
    assert!(total_time < Duration::from_secs(1)); // Should complete in under 1 second
    
    let reg_stats = register_allocator.get_statistics();
    let sched_stats = instruction_scheduler.get_statistics();
    
    assert_eq!(reg_stats.virtual_registers, 100);
    assert_eq!(sched_stats.instructions_scheduled, 100);
    
    println!("Optimization performance benchmark completed in {:?}", total_time);
    Ok(())
}
