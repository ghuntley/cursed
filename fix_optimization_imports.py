#!/usr/bin/env python3

import os
import re

def fix_optimization_wildcard_imports():
    """Fix critical wildcard imports in optimization modules that might cause E0659 conflicts"""
    
    critical_files = [
        "src/optimization/enhanced_llvm_passes/error_propagation_optimizer.rs",
        "src/optimization/passes/cse.rs", 
        "src/codegen/llvm/optimization.rs",
        "src/optimization/parallel.rs",
        "src/optimization/llvm_advanced.rs",
        "src/optimization/advanced_coordinator.rs"
    ]
    
    fixes_applied = []
    
    for file_path in critical_files:
        if not os.path.exists(file_path):
            continue
            
        try:
            with open(file_path, 'r') as f:
                content = f.read()
            
            original_content = content
            
            # Look for specific problematic patterns
            patterns_to_fix = [
                # Fix wildcard imports of optimization modules that can conflict
                (r'use crate::optimization::\*;', 
                 '''use crate::optimization::{
    OptimizationConfig, OptimizationLevel, OptimizationManager,
    BenchmarkResult, PerformanceAnalyzer, AdaptationResult
};'''),
                
                # Fix LLVM optimization wildcards
                (r'use crate::codegen::llvm::optimization::\*;',
                 '''use crate::codegen::llvm::optimization::{
    LlvmOptimizationEngine, LlvmOptimizationResult, LlvmPerformanceMetrics
};'''),
                
                # Fix AST wildcards that might conflict with optimization
                (r'use crate::ast::\*;',
                 '''use crate::ast::{
    Node, Expression, Statement, Function, Type
};'''),
            ]
            
            for pattern, replacement in patterns_to_fix:
                if re.search(pattern, content):
                    content = re.sub(pattern, replacement, content)
                    fixes_applied.append(f"Fixed wildcard import in {file_path}")
            
            # Only write if changes were made
            if content != original_content:
                with open(file_path, 'w') as f:
                    f.write(content)
                    
        except Exception as e:
            print(f"Error processing {file_path}: {e}")
    
    return fixes_applied

def add_optimization_compatibility_layer():
    """Add a compatibility layer to prevent optimization E0659 conflicts"""
    
    # Create a compatibility module
    compat_path = "src/optimization/compatibility.rs"
    
    compat_content = '''//! Optimization compatibility layer to prevent E0659 conflicts
//! 
//! This module provides type aliases and re-exports to ensure consistent
//! naming across optimization modules and prevent ambiguous imports.

/// Core optimization types with explicit naming
pub use crate::optimization::config::OptimizationConfig as CoreOptimizationConfig;
pub use crate::common::optimization_level::OptimizationLevel as CoreOptimizationLevel;

/// Performance analysis types
pub use crate::optimization::performance_analysis::{
    PerformanceAnalyzer as CorePerformanceAnalyzer,
    PerformanceAnalysisEngine,
    ComprehensivePerformanceAnalysis
};

/// Benchmark types
pub use crate::optimization::benchmarks::{
    BenchmarkResult as CoreBenchmarkResult,
    BenchmarkSuiteResult,
    BenchmarkRunner
};

/// Adaptive optimization types
pub use crate::optimization::adaptive::{
    AdaptiveOptimizer,
    AdaptationResult as CoreAdaptationResult,
    AdaptiveStrategy as CoreAdaptiveStrategy
};

/// LLVM optimization types
pub use crate::optimization::real_llvm_passes::{
    RealLlvmOptimizer as LlvmOptimizer,
    OptimizationResults as LlvmOptimizationResults,
    PerformanceImprovements as LlvmPerformanceImprovements
};

/// Type aliases for backward compatibility and conflict resolution
pub type DefaultOptimizationConfig = CoreOptimizationConfig;
pub type DefaultOptimizationLevel = CoreOptimizationLevel;
pub type DefaultPerformanceAnalyzer = CorePerformanceAnalyzer;
pub type DefaultBenchmarkResult = CoreBenchmarkResult;
pub type DefaultAdaptationResult = CoreAdaptationResult;
pub type DefaultAdaptiveStrategy = CoreAdaptiveStrategy;

/// Re-export commonly used optimization types for convenience
pub use crate::optimization::{
    OptimizationManager,
    LocalOptimizationCoordinator,
    OptimizationRecommendation
};
'''

    try:
        with open(compat_path, 'w') as f:
            f.write(compat_content)
        
        # Add to mod.rs
        mod_rs_path = "src/optimization/mod.rs"
        with open(mod_rs_path, 'r') as f:
            mod_content = f.read()
        
        if "pub mod compatibility;" not in mod_content:
            # Find a good place to insert the module declaration
            lines = mod_content.split('\n')
            insert_pos = -1
            
            for i, line in enumerate(lines):
                if line.startswith("pub mod ") and "types" in line:
                    insert_pos = i + 1
                    break
            
            if insert_pos > 0:
                lines.insert(insert_pos, "pub mod compatibility;")
                mod_content = '\n'.join(lines)
                
                with open(mod_rs_path, 'w') as f:
                    f.write(mod_content)
                
                return "Added optimization compatibility layer"
                
    except Exception as e:
        print(f"Error creating compatibility layer: {e}")
    
    return None

def main():
    print("🔧 Fixing optimization wildcard imports...")
    
    fixes = fix_optimization_wildcard_imports()
    compat_fix = add_optimization_compatibility_layer()
    
    if fixes:
        for fix in fixes:
            print(f"  ✅ {fix}")
    
    if compat_fix:
        print(f"  ✅ {compat_fix}")
    
    if not fixes and not compat_fix:
        print("  ℹ️  No critical wildcard import fixes needed")
    
    print("✅ Optimization import fixes complete!")

if __name__ == "__main__":
    main()
