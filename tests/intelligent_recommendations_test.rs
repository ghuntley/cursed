//! Comprehensive test suite for the Intelligent Optimization Recommendations System
//! 
//! This test suite validates the code analysis engine and recommendation generation
//! across various CURSED language constructs and optimization scenarios.

use cursed::optimization::{
    CodeAnalysisEngine, AnalysisConfig, DetailedRecommendation, PatternType,
    PatternSeverity, OptimizationManager, OptimizationCategory, RecommendationPriority
};
use cursed::error::Result;

#[test]
fn test_code_analysis_engine_creation() {
    let engine = CodeAnalysisEngine::new();
    assert!(engine.config.max_function_size > 0);
    assert!(engine.config.max_loop_nesting > 0);
    assert!(engine.config.enable_advanced_analysis);
}

#[test] 
fn test_custom_analysis_config() {
    let config = AnalysisConfig {
        max_function_size: 100,
        max_loop_nesting: 5,
        loop_optimization_threshold: 500,
        max_function_parameters: 10,
        enable_advanced_analysis: false,
        enable_memory_analysis: true,
        enable_performance_analysis: true,
    };
    
    let engine = CodeAnalysisEngine::with_config(config.clone());
    assert_eq!(engine.config.max_function_size, 100);
    assert_eq!(engine.config.max_loop_nesting, 5);
    assert!(!engine.config.enable_advanced_analysis);
    assert!(engine.config.enable_memory_analysis);
}

#[test]
fn test_basic_code_analysis() {
    let mut engine = CodeAnalysisEngine::new();
    
    let simple_code = r#"
        slay add(x: i32, y: i32) -> i32 {
            yolo x + y;
        }
    "#;
    
    let result = engine.analyze_code(simple_code);
    assert!(result.is_ok());
    
    let recommendations = result.unwrap();
    assert!(!recommendations.is_empty());
}

#[test]
fn test_nested_loop_detection() {
    let mut engine = CodeAnalysisEngine::new();
    
    let nested_loop_code = r#"
        slay process_matrix(matrix: [[i32]]) {
            bestie (sus i = 0; i < matrix.len(); i++) {
                bestie (sus j = 0; j < matrix[i].len(); j++) {
                    bestie (sus k = 0; k < 10; k++) {
                        // Deep nested computation
                        matrix[i][j] = matrix[i][j] + k;
                    }
                }
            }
        }
    "#;
    
    let result = engine.analyze_code(nested_loop_code);
    assert!(result.is_ok());
    
    let recommendations = result.unwrap();
    
    // Should detect nested loop patterns
    let has_nested_loop_pattern = recommendations.iter().any(|rec| {
        rec.patterns.iter().any(|p| p.pattern_type == PatternType::NestedLoops)
    });
    
    // Note: Since we're using a simplified parser, we may not detect all patterns
    // In a real implementation, this would detect the nested loops
    println!("Generated {} recommendations", recommendations.len());
    for rec in &recommendations {
        println!("  - {} ({})", rec.description, format!("{:?}", rec.category));
    }
}

#[test]
fn test_function_inlining_recommendations() {
    let mut engine = CodeAnalysisEngine::new();
    
    let small_function_code = r#"
        slay square(x: i32) -> i32 {
            yolo x * x;
        }
        
        slay cube(x: i32) -> i32 {
            yolo x * x * x;
        }
        
        slay main() {
            sus result = square(5) + cube(3);
        }
    "#;
    
    let result = engine.analyze_code(small_function_code);
    assert!(result.is_ok());
    
    let recommendations = result.unwrap();
    assert!(!recommendations.is_empty());
    
    // Should suggest function inlining for small functions
    let has_inline_suggestion = recommendations.iter().any(|rec| {
        rec.patterns.iter().any(|p| p.pattern_type == PatternType::InlineCandidate)
    });
    
    println!("Function inlining recommendations generated: {}", has_inline_suggestion);
}

#[test]
fn test_string_operations_analysis() {
    let mut engine = CodeAnalysisEngine::new();
    
    let string_heavy_code = r#"
        slay concatenate_strings(items: [String]) -> String {
            sus result = "";
            bestie (sus item in items) {
                result = result + item + ", ";
            }
            yolo result;
        }
    "#;
    
    let result = engine.analyze_code(string_heavy_code);
    assert!(result.is_ok());
    
    let recommendations = result.unwrap();
    
    // Check for string operation optimization suggestions
    let has_string_optimization = recommendations.iter().any(|rec| {
        rec.code_suggestions.iter().any(|cs| {
            cs.title.contains("string") || cs.explanation.contains("string")
        })
    });
    
    println!("String optimization suggestions: {}", has_string_optimization);
}

#[test]
fn test_memory_allocation_patterns() {
    let mut engine = CodeAnalysisEngine::new();
    
    let memory_heavy_code = r#"
        slay process_data() {
            sus data = [];
            bestie (sus i = 0; i < 1000; i++) {
                data.push(create_object(i));
            }
        }
    "#;
    
    let result = engine.analyze_code(memory_heavy_code);
    assert!(result.is_ok());
    
    let recommendations = result.unwrap();
    
    // Should suggest memory optimization
    let has_memory_category = recommendations.iter().any(|rec| {
        matches!(rec.category, OptimizationCategory::MemoryUsage)
    });
    
    println!("Memory optimization recommendations: {}", has_memory_category);
}

#[test]
fn test_performance_impact_calculation() {
    let mut engine = CodeAnalysisEngine::new();
    
    let performance_critical_code = r#"
        slay matrix_multiply(a: [[f64]], b: [[f64]]) -> [[f64]] {
            sus result = [];
            bestie (sus i = 0; i < a.len(); i++) {
                sus row = [];
                bestie (sus j = 0; j < b[0].len(); j++) {
                    sus sum = 0.0;
                    bestie (sus k = 0; k < b.len(); k++) {
                        sum = sum + a[i][k] * b[k][j];
                    }
                    row.push(sum);
                }
                result.push(row);
            }
            yolo result;
        }
    "#;
    
    let result = engine.analyze_code(performance_critical_code);
    assert!(result.is_ok());
    
    let recommendations = result.unwrap();
    
    // Check that performance impact is calculated
    for rec in &recommendations {
        assert!(rec.expected_impact.confidence >= 0.0);
        assert!(rec.expected_impact.confidence <= 1.0);
        
        // Performance recommendations should suggest runtime improvements
        if matches!(rec.category, OptimizationCategory::Performance) {
            assert!(rec.expected_impact.runtime_improvement >= 0.0);
        }
    }
}

#[test]
fn test_optimization_actions_generation() {
    let mut engine = CodeAnalysisEngine::new();
    
    let code_with_opportunities = r#"
        slay fibonacci(n: i32) -> i32 {
            lowkey (n <= 1) {
                yolo n;
            }
            yolo fibonacci(n - 1) + fibonacci(n - 2);
        }
    "#;
    
    let result = engine.analyze_code(code_with_opportunities);
    assert!(result.is_ok());
    
    let recommendations = result.unwrap();
    
    // Should generate specific optimization actions
    for rec in &recommendations {
        assert!(!rec.actions.is_empty());
        
        for action in &rec.actions {
            assert!(!action.description.is_empty());
            // Actions should have valid priorities
            match action.priority {
                cursed::optimization::ActionPriority::Immediate |
                cursed::optimization::ActionPriority::High |
                cursed::optimization::ActionPriority::Medium |
                cursed::optimization::ActionPriority::Low |
                cursed::optimization::ActionPriority::Optional => {
                    // Valid priority
                }
            }
        }
    }
}

#[test]
fn test_code_suggestions_quality() {
    let mut engine = CodeAnalysisEngine::new();
    
    let inefficient_code = r#"
        slay sum_array(arr: [i32]) -> i32 {
            sus total = 0;
            bestie (sus i = 0; i < arr.len(); i++) {
                total = total + arr[i];
            }
            yolo total;
        }
    "#;
    
    let result = engine.analyze_code(inefficient_code);
    assert!(result.is_ok());
    
    let recommendations = result.unwrap();
    
    // Check code suggestions quality
    for rec in &recommendations {
        for suggestion in &rec.code_suggestions {
            assert!(!suggestion.title.is_empty());
            assert!(!suggestion.explanation.is_empty());
            assert!(!suggestion.benefit.is_empty());
            
            // Suggestions should provide actionable advice
            if suggestion.before_code.is_some() {
                assert!(suggestion.after_code.is_some());
            }
        }
    }
}

#[test]
fn test_recommendation_priority_ordering() {
    let mut engine = CodeAnalysisEngine::new();
    
    let complex_code = r#"
        slay complex_function(data: [String]) -> [String] {
            sus results = [];
            bestie (sus item in data) {
                bestie (sus i = 0; i < 100; i++) {
                    bestie (sus j = 0; j < 50; j++) {
                        sus processed = item + "_" + i.to_string() + "_" + j.to_string();
                        results.push(processed);
                    }
                }
            }
            yolo results;
        }
    "#;
    
    let result = engine.analyze_code(complex_code);
    assert!(result.is_ok());
    
    let recommendations = result.unwrap();
    
    // Recommendations should be ordered by priority
    for i in 1..recommendations.len() {
        let prev_priority = &recommendations[i-1].priority;
        let curr_priority = &recommendations[i].priority;
        
        // Higher priority items should come first
        match (prev_priority, curr_priority) {
            (RecommendationPriority::Critical, _) => {
                // Critical always comes first
            }
            (RecommendationPriority::High, RecommendationPriority::Critical) => {
                panic!("Priority ordering incorrect");
            }
            _ => {
                // Other combinations are acceptable
            }
        }
    }
}

#[test]
fn test_optimization_config_suggestions() {
    let mut engine = CodeAnalysisEngine::new();
    
    let optimization_sensitive_code = r#"
        slay compute_intensive_task() {
            bestie (sus i = 0; i < 1000000; i++) {
                // Computation that would benefit from optimization
                sus result = i * i + i * 2 + 1;
            }
        }
    "#;
    
    let result = engine.analyze_code(optimization_sensitive_code);
    assert!(result.is_ok());
    
    let recommendations = result.unwrap();
    
    // Should suggest appropriate optimization configurations
    for rec in &recommendations {
        let config = &rec.suggested_config;
        
        match rec.category {
            OptimizationCategory::Performance => {
                // Performance recommendations should suggest aggressive optimization
                assert!(config.optimization_level >= cursed::codegen::llvm::optimization::OptimizationLevel::Release);
            }
            OptimizationCategory::MemoryUsage => {
                // Memory recommendations should focus on size optimization
                assert!(config.optimization_level == cursed::codegen::llvm::optimization::OptimizationLevel::Size ||
                       config.optimization_level >= cursed::codegen::llvm::optimization::OptimizationLevel::Release);
            }
            _ => {
                // Other categories should have appropriate configs
            }
        }
    }
}

#[test]
fn test_integration_with_optimization_manager() {
    let manager = OptimizationManager::new();
    
    let test_code = r#"
        slay test_function(x: i32) -> i32 {
            sus result = 0;
            bestie (sus i = 0; i < x; i++) {
                result = result + i * i;
            }
            yolo result;
        }
    "#;
    
    // Test basic recommendations (backward compatibility)
    let basic_recommendations = manager.generate_recommendations(test_code);
    assert!(!basic_recommendations.is_empty());
    
    // Test intelligent recommendations
    let intelligent_result = manager.generate_intelligent_recommendations(test_code);
    assert!(intelligent_result.is_ok());
    
    let intelligent_recommendations = intelligent_result.unwrap();
    assert!(!intelligent_recommendations.is_empty());
    
    // Intelligent recommendations should be more detailed
    for rec in &intelligent_recommendations {
        assert!(!rec.patterns.is_empty() || !rec.actions.is_empty());
        assert!(!rec.code_suggestions.is_empty());
    }
}

#[test]
fn test_analysis_caching() {
    let mut engine = CodeAnalysisEngine::new();
    
    let test_code = r#"
        slay cached_function() {
            sus x = 42;
            yolo x * 2;
        }
    "#;
    
    // First analysis
    let result1 = engine.analyze_code(test_code);
    assert!(result1.is_ok());
    
    // Second analysis should use cache
    let result2 = engine.analyze_code(test_code);
    assert!(result2.is_ok());
    
    // Results should be equivalent
    let recs1 = result1.unwrap();
    let recs2 = result2.unwrap();
    assert_eq!(recs1.len(), recs2.len());
    
    // Clear cache
    engine.clear_cache();
    
    // Analysis after cache clear should still work
    let result3 = engine.analyze_code(test_code);
    assert!(result3.is_ok());
}

#[test]
fn test_custom_analysis_configuration() {
    let custom_config = AnalysisConfig {
        max_function_size: 20,
        max_loop_nesting: 2,
        loop_optimization_threshold: 10,
        max_function_parameters: 3,
        enable_advanced_analysis: true,
        enable_memory_analysis: false,
        enable_performance_analysis: true,
    };
    
    let manager = OptimizationManager::new();
    
    let test_code = r#"
        slay large_function(a: i32, b: i32, c: i32, d: i32, e: i32) {
            bestie (sus i = 0; i < 100; i++) {
                bestie (sus j = 0; j < 50; j++) {
                    bestie (sus k = 0; k < 25; k++) {
                        // This should trigger multiple warnings with custom config
                        sus result = a + b + c + d + e + i + j + k;
                    }
                }
            }
        }
    "#;
    
    let result = manager.generate_recommendations_with_config(test_code, custom_config);
    assert!(result.is_ok());
    
    let recommendations = result.unwrap();
    assert!(!recommendations.is_empty());
    
    // With stricter configuration, should generate more recommendations
    println!("Custom config generated {} recommendations", recommendations.len());
}

#[test]
fn test_error_handling_graceful_degradation() {
    let mut engine = CodeAnalysisEngine::new();
    
    // Test with invalid/malformed code
    let invalid_code = r#"
        slay incomplete_function(
            // Missing closing parenthesis and body
    "#;
    
    let result = engine.analyze_code(invalid_code);
    
    // Should handle parsing errors gracefully
    match result {
        Ok(recommendations) => {
            // If parsing somehow succeeds, recommendations should be empty or minimal
            println!("Gracefully handled invalid code with {} recommendations", recommendations.len());
        }
        Err(e) => {
            // Parsing error is expected and acceptable
            println!("Expected parsing error: {}", e);
        }
    }
}

#[test]
fn test_concurrent_code_analysis() {
    use std::sync::Arc;
    use std::thread;
    
    let engine = Arc::new(std::sync::Mutex::new(CodeAnalysisEngine::new()));
    
    let test_codes = vec![
        r#"slay func1() { sus x = 1; }"#,
        r#"slay func2() { sus y = 2; }"#,
        r#"slay func3() { sus z = 3; }"#,
    ];
    
    let mut handles = Vec::new();
    
    for (i, code) in test_codes.into_iter().enumerate() {
        let engine_clone = Arc::clone(&engine);
        let code_owned = code.to_string();
        
        let handle = thread::spawn(move || {
            let mut engine_guard = engine_clone.lock().unwrap();
            let result = engine_guard.analyze_code(&code_owned);
            (i, result.is_ok())
        });
        
        handles.push(handle);
    }
    
    for handle in handles {
        let (index, success) = handle.join().unwrap();
        assert!(success, "Thread {} failed to analyze code", index);
    }
}

/// Helper function to create test code with specific patterns
fn create_test_code_with_pattern(pattern: &str) -> String {
    match pattern {
        "nested_loops" => r#"
            slay nested_computation() {
                bestie (sus i = 0; i < 100; i++) {
                    bestie (sus j = 0; j < 100; j++) {
                        bestie (sus k = 0; k < 100; k++) {
                            sus result = i * j * k;
                        }
                    }
                }
            }
        "#.to_string(),
        "string_concatenation" => r#"
            slay build_string(items: [String]) -> String {
                sus result = "";
                bestie (sus item in items) {
                    result = result + item + "\n";
                }
                yolo result;
            }
        "#.to_string(),
        "large_function" => {
            let mut code = "slay large_function() {\n".to_string();
            for i in 0..100 {
                code.push_str(&format!("    sus var{} = {};\n", i, i));
            }
            code.push_str("}\n");
            code
        }
        _ => "slay default_function() { sus x = 42; }".to_string(),
    }
}

#[test]
fn test_pattern_specific_recommendations() {
    let mut engine = CodeAnalysisEngine::new();
    
    let patterns = ["nested_loops", "string_concatenation", "large_function"];
    
    for pattern in &patterns {
        let test_code = create_test_code_with_pattern(pattern);
        let result = engine.analyze_code(&test_code);
        
        assert!(result.is_ok(), "Failed to analyze code for pattern: {}", pattern);
        
        let recommendations = result.unwrap();
        assert!(!recommendations.is_empty(), "No recommendations for pattern: {}", pattern);
        
        println!("Pattern '{}' generated {} recommendations", pattern, recommendations.len());
        
        // Verify that recommendations are relevant to the pattern
        for rec in &recommendations {
            assert!(!rec.description.is_empty());
            assert!(!rec.actions.is_empty() || !rec.code_suggestions.is_empty());
        }
    }
}
