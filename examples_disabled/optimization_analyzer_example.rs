#!/usr/bin/env cargo

//! Example of using the CURSED Intelligent Optimization Recommendations System
//! 
//! This example demonstrates how to analyze CURSED source code and generate
//! intelligent optimization recommendations using the analysis engine.

use cursed::optimization::{
    OptimizationManager, CodeAnalysisEngine, AnalysisConfig,
    PatternType, PatternSeverity, OptimizationCategory
};
use std::fs;
use std::io::{self, Write};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing for better output
    tracing_subscriber::fmt::init();

    println!("🚀 CURSED Intelligent Optimization Recommendations Demo");
    println!("========================================================\n");

    // Load the demo CURSED code
    let demo_file = "examples/optimization_recommendations_demo.csd";
    let source_code = match fs::read_to_string(demo_file) {
        Ok(content) => content,
        Err(_) => {
            // Fallback to inline demo code if file not found
            create_inline_demo_code()
        }
    };

    println!("📝 Analyzing CURSED source code ({} chars)...\n", source_code.len());

    // Example 1: Basic optimization recommendations
    println!("🔍 Basic Optimization Analysis");
    println!("------------------------------");
    
    let manager = OptimizationManager::new();
    let basic_recommendations = manager.generate_recommendations(&source_code);
    
    for (i, rec) in basic_recommendations.iter().enumerate() {
        println!("{}. [{}] {}", 
                 i + 1,
                 format!("{:?}", rec.priority),
                 rec.description);
        println!("   Category: {:?}", rec.category);
        println!("   Optimization Level: {:?}\n", rec.suggested_config.optimization_level);
    }

    // Example 2: Detailed intelligent recommendations
    println!("🧠 Intelligent Analysis with Pattern Detection");
    println!("----------------------------------------------");

    match manager.generate_intelligent_recommendations(&source_code) {
        Ok(detailed_recommendations) => {
            for (i, rec) in detailed_recommendations.iter().enumerate() {
                println!("{}. {} [Priority: {:?}]", 
                         i + 1, 
                         rec.description,
                         rec.priority);
                
                println!("   Category: {:?}", rec.category);
                
                // Show detected patterns
                if !rec.patterns.is_empty() {
                    println!("   Detected Patterns:");
                    for pattern in &rec.patterns {
                        println!("     • {:?} ({}): {}", 
                                 pattern.pattern_type,
                                 format!("{:?}", pattern.severity),
                                 pattern.description);
                    }
                }
                
                // Show expected performance impact
                let impact = &rec.expected_impact;
                println!("   Expected Impact:");
                println!("     • Runtime: {:.1}% improvement", impact.runtime_improvement);
                println!("     • Memory: {:.1}% improvement", impact.memory_improvement);
                println!("     • Compile time: {:.1}% change", impact.compile_time_impact);
                println!("     • Confidence: {:.1}%", impact.confidence * 100.0);
                
                // Show optimization actions
                if !rec.actions.is_empty() {
                    println!("   Recommended Actions:");
                    for action in &rec.actions {
                        println!("     • {} (Priority: {:?})", 
                                 action.description,
                                 action.priority);
                        
                        if !action.config_changes.is_empty() {
                            for change in &action.config_changes {
                                println!("       - {}: {} → {}", 
                                         change.setting,
                                         change.current_value,
                                         change.recommended_value);
                            }
                        }
                    }
                }
                
                // Show code suggestions
                if !rec.code_suggestions.is_empty() {
                    println!("   Code Suggestions:");
                    for suggestion in &rec.code_suggestions {
                        println!("     • {}: {}", suggestion.title, suggestion.explanation);
                        if let Some(ref before) = suggestion.before_code {
                            println!("       Before: {}", before.lines().next().unwrap_or(""));
                        }
                        if let Some(ref after) = suggestion.after_code {
                            println!("       After:  {}", after.lines().next().unwrap_or(""));
                        }
                        println!("       Benefit: {}", suggestion.benefit);
                    }
                }
                
                println!();
            }
        }
        Err(e) => {
            println!("❌ Error generating intelligent recommendations: {}", e);
        }
    }

    // Example 3: Custom analysis configuration
    println!("⚙️  Custom Analysis Configuration");
    println!("--------------------------------");

    let custom_config = AnalysisConfig {
        max_function_size: 30,      // Stricter function size limit
        max_loop_nesting: 2,        // Lower loop nesting tolerance
        loop_optimization_threshold: 50,
        max_function_parameters: 4,
        enable_advanced_analysis: true,
        enable_memory_analysis: true,
        enable_performance_analysis: true,
    };

    match manager.generate_recommendations_with_config(&source_code, custom_config) {
        Ok(custom_recommendations) => {
            println!("Custom analysis generated {} recommendations:", custom_recommendations.len());
            
            for (i, rec) in custom_recommendations.iter().take(5).enumerate() {
                println!("{}. {}", i + 1, rec.description);
                println!("   Expected runtime improvement: {:.1}%", 
                         rec.expected_impact.runtime_improvement);
            }
            
            if custom_recommendations.len() > 5 {
                println!("   ... and {} more recommendations", custom_recommendations.len() - 5);
            }
        }
        Err(e) => {
            println!("❌ Error with custom configuration: {}", e);
        }
    }

    // Example 4: Pattern-specific analysis
    println!("\n🎯 Pattern-Specific Analysis");
    println!("----------------------------");

    analyze_specific_patterns(&source_code)?;

    // Example 5: Interactive mode
    println!("\n🔧 Interactive Analysis Mode");
    println!("----------------------------");
    interactive_analysis_mode()?;

    println!("\n✅ Demo completed successfully!");
    println!("💡 Try modifying the demo code and running the analysis again!");

    Ok(())
}

fn create_inline_demo_code() -> String {
    r#"
// Inline demo code with optimization opportunities
slay nested_loop_example(data: [[i32]]) {
    bestie (sus i = 0; i < data.len(); i++) {
        bestie (sus j = 0; j < data[i].len(); j++) {
            bestie (sus k = 0; k < 10; k++) {
                sus result = data[i][j] * k;
            }
        }
    }
}

slay small_function(x: i32) -> i32 {
    yolo x * 2;
}

slay string_concatenation(items: [String]) -> String {
    sus result = "";
    bestie (sus item in items) {
        result = result + item + ", ";
    }
    yolo result;
}
    "#.to_string()
}

fn analyze_specific_patterns(source_code: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut engine = CodeAnalysisEngine::new();
    
    let recommendations = engine.analyze_code(source_code)?;
    
    // Group recommendations by pattern type
    let mut pattern_counts = std::collections::HashMap::new();
    
    for rec in &recommendations {
        for pattern in &rec.patterns {
            *pattern_counts.entry(pattern.pattern_type.clone()).or_insert(0) += 1;
        }
    }
    
    println!("Pattern Detection Summary:");
    for (pattern_type, count) in &pattern_counts {
        println!("  • {:?}: {} occurrences", pattern_type, count);
    }

    // Show most critical patterns
    let mut critical_patterns: Vec<_> = recommendations.iter()
        .flat_map(|r| &r.patterns)
        .filter(|p| p.severity == PatternSeverity::Critical || p.severity == PatternSeverity::High)
        .collect();
    
    critical_patterns.sort_by(|a, b| b.severity.partial_cmp(&a.severity).unwrap_or(std::cmp::Ordering::Equal));
    
    if !critical_patterns.is_empty() {
        println!("\nCritical Optimization Opportunities:");
        for (i, pattern) in critical_patterns.iter().take(3).enumerate() {
            println!("{}. {} (Line: {}, Column: {})", 
                     i + 1,
                     pattern.description,
                     pattern.location.line,
                     pattern.location.column);
            println!("   Expected improvement: {:.1}% runtime, {:.1}% memory",
                     pattern.performance_impact.runtime_improvement,
                     pattern.performance_impact.memory_improvement);
        }
    }

    Ok(())
}

fn interactive_analysis_mode() -> Result<(), Box<dyn std::error::Error>> {
    println!("Enter CURSED code to analyze (type 'exit' to quit):");
    
    loop {
        print!("📝 Enter code: ");
        io::stdout().flush()?;
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        
        let input = input.trim();
        if input == "exit" || input.is_empty() {
            break;
        }
        
        // Simple multi-line input (very basic)
        let mut code = input.to_string();
        if !code.contains('{') {
            code = format!("slay example() {{ {} }}", code);
        }
        
        let mut engine = CodeAnalysisEngine::new();
        match engine.analyze_code(&code) {
            Ok(recommendations) => {
                if recommendations.is_empty() {
                    println!("✅ No optimization opportunities detected.");
                } else {
                    println!("🔍 Found {} optimization opportunities:", recommendations.len());
                    for (i, rec) in recommendations.iter().take(3).enumerate() {
                        println!("  {}. {}", i + 1, rec.description);
                    }
                }
            }
            Err(e) => {
                println!("❌ Analysis error: {}", e);
            }
        }
        
        println!();
    }
    
    Ok(())
}
