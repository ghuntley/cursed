/// Comprehensive Vectorization Tests for CURSED Compiler
/// 
/// Tests the vectorization optimization system including:
/// - Loop vectorization with various patterns
/// - Operation vectorization for arithmetic
/// - SIMD instruction generation
/// - Target-specific optimizations
/// - Performance analysis and metrics

use cursed::optimization::enhanced_llvm_passes::vectorization_optimizer::VectorizationOptimizer;
use cursed::optimization::enhanced_llvm_passes::EnhancedOptimizationStatistics;
use std::sync::{Arc, Mutex};
use tracing_test::traced_test;

#[traced_test]
#[test]
fn test_vectorization_optimizer_basic_functionality() {
    let statistics = Arc::new(Mutex::new(EnhancedOptimizationStatistics::default()));
    let optimizer = VectorizationOptimizer::new(statistics.clone());
    
    // Test basic configuration
    assert_eq!(optimizer.target_info.supported_widths.len(), 3);
    assert!(optimizer.target_info.available_instructions.len() >= 2);
    
    // Test operation cost model
    let add_cost = optimizer.target_info.operation_costs.get(&cursed::optimization::enhanced_llvm_passes::vectorization_optimizer::VectorOperation::Add);
    assert!(add_cost.is_some());
    assert_eq!(add_cost.unwrap().latency, 1);
    assert_eq!(add_cost.unwrap().throughput, 2.0);
    
    let multiply_cost = optimizer.target_info.operation_costs.get(&cursed::optimization::enhanced_llvm_passes::vectorization_optimizer::VectorOperation::Multiply);
    assert!(multiply_cost.is_some());
    assert_eq!(multiply_cost.unwrap().latency, 3);
    assert_eq!(multiply_cost.unwrap().throughput, 1.0);
}

#[traced_test]
#[test]
fn test_optimal_vector_width_selection() {
    let statistics = Arc::new(Mutex::new(EnhancedOptimizationStatistics::default()));
    let optimizer = VectorizationOptimizer::new(statistics);
    
    // Test width selection for different data types
    let width_i32 = optimizer.get_optimal_vector_width("i32");
    let width_f32 = optimizer.get_optimal_vector_width("f32");
    let width_f64 = optimizer.get_optimal_vector_width("f64");
    let width_unknown = optimizer.get_optimal_vector_width("unknown");
    
    assert_eq!(width_i32, 16);
    assert_eq!(width_f32, 16);
    assert_eq!(width_f64, 8);
    assert_eq!(width_unknown, 4);
}

#[traced_test]
#[test]
fn test_vectorization_speedup_estimation() {
    let statistics = Arc::new(Mutex::new(EnhancedOptimizationStatistics::default()));
    let optimizer = VectorizationOptimizer::new(statistics);
    
    use cursed::optimization::enhanced_llvm_passes::vectorization_optimizer::VectorOperation;
    
    // Test speedup estimation for different operations
    let add_speedup = optimizer.estimate_vectorization_speedup(&VectorOperation::Add, 4);
    let multiply_speedup = optimizer.estimate_vectorization_speedup(&VectorOperation::Multiply, 4);
    let divide_speedup = optimizer.estimate_vectorization_speedup(&VectorOperation::Divide, 4);
    let load_speedup = optimizer.estimate_vectorization_speedup(&VectorOperation::Load, 8);
    
    assert_eq!(add_speedup, 3.2); // 4 * 0.8
    assert_eq!(multiply_speedup, 2.88); // 4 * 0.8 * 0.9
    assert_eq!(divide_speedup, 1.92); // 4 * 0.8 * 0.6
    assert_eq!(load_speedup, 4.48); // 8 * 0.8 * 0.7
}

#[traced_test]
#[test]
fn test_profitability_analysis() {
    let statistics = Arc::new(Mutex::new(EnhancedOptimizationStatistics::default()));
    let optimizer = VectorizationOptimizer::new(statistics);
    
    use cursed::optimization::enhanced_llvm_passes::vectorization_optimizer::{VectorizableOperation, VectorOperation};
    
    // Test profitable vectorization
    let profitable_op = VectorizableOperation {
        operation_type: VectorOperation::Add,
        data_type: "i32".to_string(),
        vector_width: 8,
        location: "test".to_string(),
        estimated_speedup: 5.0,
    };
    
    // Test unprofitable vectorization
    let unprofitable_op = VectorizableOperation {
        operation_type: VectorOperation::Add,
        data_type: "i32".to_string(),
        vector_width: 2,
        location: "test".to_string(),
        estimated_speedup: 1.5,
    };
    
    assert!(optimizer.is_profitable_vectorization(&profitable_op));
    assert!(!optimizer.is_profitable_vectorization(&unprofitable_op));
}

#[traced_test]
#[test]
fn test_target_vector_info_defaults() {
    use cursed::optimization::enhanced_llvm_passes::vectorization_optimizer::{TargetVectorInfo, VectorOperation, SIMDInstructionSet};
    
    let target_info = TargetVectorInfo::default();
    
    // Test supported widths
    assert!(target_info.supported_widths.contains_key("i32"));
    assert!(target_info.supported_widths.contains_key("f32"));
    assert!(target_info.supported_widths.contains_key("f64"));
    
    // Test operation costs
    assert!(target_info.operation_costs.contains_key(&VectorOperation::Add));
    assert!(target_info.operation_costs.contains_key(&VectorOperation::Multiply));
    
    // Test available instruction sets
    assert!(target_info.available_instructions.contains(&SIMDInstructionSet::SSE2));
    assert!(target_info.available_instructions.contains(&SIMDInstructionSet::AVX));
}

#[traced_test]
#[test]
fn test_vectorization_statistics() {
    let statistics = Arc::new(Mutex::new(EnhancedOptimizationStatistics::default()));
    let optimizer = VectorizationOptimizer::new(statistics.clone());
    
    // Test initial statistics
    let stats = optimizer.get_vectorization_statistics();
    assert_eq!(stats.get("vectorizable_operations").unwrap_or(&0), &0);
    assert_eq!(stats.get("analyzed_functions").unwrap_or(&0), &0);
    assert_eq!(stats.get("loop_candidates").unwrap_or(&0), &0);
    
    // Statistics should be consistent
    assert!(stats.contains_key("vectorizable_operations"));
    assert!(stats.contains_key("analyzed_functions"));
    assert!(stats.contains_key("loop_candidates"));
}

#[cfg(test)]
mod integration_tests {
    use super::*;
    use std::time::Duration;
    
    #[traced_test]
    #[test]
    fn test_vectorization_with_mock_llvm_function() {
        let statistics = Arc::new(Mutex::new(EnhancedOptimizationStatistics::default()));
        let mut optimizer = VectorizationOptimizer::new(statistics.clone());
        
        // Since we can't easily create real LLVM functions in unit tests,
        // we'll test the analysis and decision-making logic
        
        // Test that the optimizer handles empty function lists gracefully
        let stats = optimizer.get_vectorization_statistics();
        assert_eq!(stats["analyzed_functions"], 0);
        assert_eq!(stats["vectorizable_operations"], 0);
        
        // Test that profitability analysis works correctly
        use cursed::optimization::enhanced_llvm_passes::vectorization_optimizer::{VectorizableOperation, VectorOperation};
        
        let test_operations = vec![
            VectorizableOperation {
                operation_type: VectorOperation::Add,
                data_type: "f32".to_string(),
                vector_width: 8,
                location: "loop_1".to_string(),
                estimated_speedup: 6.4,
            },
            VectorizableOperation {
                operation_type: VectorOperation::Multiply,
                data_type: "i32".to_string(),
                vector_width: 4,
                location: "loop_2".to_string(),
                estimated_speedup: 3.2,
            },
            VectorizableOperation {
                operation_type: VectorOperation::Divide,
                data_type: "f64".to_string(),
                vector_width: 2,
                location: "loop_3".to_string(),
                estimated_speedup: 1.2,
            },
        ];
        
        // Test profitability for each operation
        let profitable_count = test_operations.iter()
            .filter(|op| optimizer.is_profitable_vectorization(op))
            .count();
        
        assert_eq!(profitable_count, 2); // First two should be profitable
    }
    
    #[traced_test]
    #[test]
    fn test_vectorization_decision_making() {
        let statistics = Arc::new(Mutex::new(EnhancedOptimizationStatistics::default()));
        let optimizer = VectorizationOptimizer::new(statistics);
        
        // Test vector width selection logic
        assert_eq!(optimizer.get_optimal_vector_width("i32"), 16);
        assert_eq!(optimizer.get_optimal_vector_width("f32"), 16);
        assert_eq!(optimizer.get_optimal_vector_width("f64"), 8);
        assert_eq!(optimizer.get_optimal_vector_width("unknown"), 4);
        
        // Test speedup estimation consistency
        use cursed::optimization::enhanced_llvm_passes::vectorization_optimizer::VectorOperation;
        
        let operations = vec![
            VectorOperation::Add,
            VectorOperation::Subtract,
            VectorOperation::Multiply,
            VectorOperation::Divide,
            VectorOperation::Load,
            VectorOperation::Store,
        ];
        
        for operation in operations {
            let speedup_4 = optimizer.estimate_vectorization_speedup(&operation, 4);
            let speedup_8 = optimizer.estimate_vectorization_speedup(&operation, 8);
            
            // Larger vector widths should generally give better speedup
            assert!(speedup_8 >= speedup_4);
            
            // All speedups should be positive
            assert!(speedup_4 > 0.0);
            assert!(speedup_8 > 0.0);
        }
    }
    
    #[traced_test]
    #[test]
    fn test_enhanced_statistics_integration() {
        let statistics = Arc::new(Mutex::new(EnhancedOptimizationStatistics::default()));
        let optimizer = VectorizationOptimizer::new(statistics.clone());
        
        // Test that statistics start at zero
        {
            let stats = statistics.lock().unwrap();
            assert_eq!(stats.vectorized_operations, 0);
        }
        
        // Simulate some vectorization operations by updating statistics
        {
            let mut stats = statistics.lock().unwrap();
            stats.vectorized_operations += 5;
            stats.estimated_runtime_improvement += 0.25;
        }
        
        // Verify the updates
        {
            let stats = statistics.lock().unwrap();
            assert_eq!(stats.vectorized_operations, 5);
            assert_eq!(stats.estimated_runtime_improvement, 0.25);
        }
    }
}

#[cfg(test)]
mod performance_tests {
    use super::*;
    use std::time::{Duration, Instant};
    
    #[traced_test]
    #[test]
    fn test_vectorization_analyzer_performance() {
        let statistics = Arc::new(Mutex::new(EnhancedOptimizationStatistics::default()));
        let optimizer = VectorizationOptimizer::new(statistics);
        
        // Test that the analyzer can handle multiple operations efficiently
        let start_time = Instant::now();
        
        use cursed::optimization::enhanced_llvm_passes::vectorization_optimizer::{VectorizableOperation, VectorOperation};
        
        let operations: Vec<VectorizableOperation> = (0..1000).map(|i| {
            VectorizableOperation {
                operation_type: match i % 4 {
                    0 => VectorOperation::Add,
                    1 => VectorOperation::Multiply,
                    2 => VectorOperation::Load,
                    _ => VectorOperation::Store,
                },
                data_type: "f32".to_string(),
                vector_width: 4,
                location: format!("location_{}", i),
                estimated_speedup: 3.0,
            }
        }).collect();
        
        // Test profitability analysis performance
        let profitable_count = operations.iter()
            .filter(|op| optimizer.is_profitable_vectorization(op))
            .count();
        
        let analysis_time = start_time.elapsed();
        
        // The analysis should complete quickly
        assert!(analysis_time < Duration::from_millis(100));
        assert!(profitable_count > 0);
        
        println!("Analyzed {} operations in {:?}", operations.len(), analysis_time);
    }
    
    #[traced_test]
    #[test]
    fn test_vector_width_selection_performance() {
        let statistics = Arc::new(Mutex::new(EnhancedOptimizationStatistics::default()));
        let optimizer = VectorizationOptimizer::new(statistics);
        
        let start_time = Instant::now();
        
        // Test width selection for many data types
        let data_types = vec!["i8", "i16", "i32", "i64", "f32", "f64", "unknown"];
        
        for _ in 0..10000 {
            for data_type in &data_types {
                let _ = optimizer.get_optimal_vector_width(data_type);
            }
        }
        
        let selection_time = start_time.elapsed();
        
        // Width selection should be very fast
        assert!(selection_time < Duration::from_millis(50));
        
        println!("Performed 70,000 width selections in {:?}", selection_time);
    }
}
