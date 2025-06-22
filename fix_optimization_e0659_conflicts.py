#!/usr/bin/env python3

import os
import re
import subprocess
from typing import List, Dict, Set

def run_cargo_check() -> str:
    """Run cargo check and capture output"""
    try:
        result = subprocess.run(
            ["./fix_linking.sh", "cargo", "check", "--lib"], 
            capture_output=True, 
            text=True, 
            cwd="."
        )
        return result.stderr
    except Exception as e:
        print(f"Error running cargo check: {e}")
        return ""

def find_e0659_errors(output: str) -> List[Dict]:
    """Find E0659 errors in cargo output"""
    errors = []
    lines = output.split('\n')
    
    for i, line in enumerate(lines):
        if 'error[E0659]:' in line and ('optimization' in line.lower() or 'optimization' in lines[i+1:i+10]):
            error_info = {
                'line': line.strip(),
                'context': lines[max(0, i-5):i+10],
                'line_number': i
            }
            errors.append(error_info)
    
    return errors

def analyze_optimization_imports():
    """Analyze optimization module imports for potential conflicts"""
    optimization_files = []
    
    # Find all optimization-related files
    for root, dirs, files in os.walk("src/optimization"):
        for file in files:
            if file.endswith(".rs"):
                optimization_files.append(os.path.join(root, file))
    
    # Also check LLVM optimization integration
    llvm_opt_files = []
    for root, dirs, files in os.walk("src/codegen/llvm"):
        for file in files:
            if file.endswith(".rs") and "optim" in file:
                llvm_opt_files.append(os.path.join(root, file))
    
    conflicts = []
    
    # Check for potential conflicts in imports
    for file_path in optimization_files + llvm_opt_files:
        try:
            with open(file_path, 'r') as f:
                content = f.read()
                
            # Look for wildcard imports that might conflict
            wildcard_imports = re.findall(r'use\s+[^;]*\*\s*;', content)
            if wildcard_imports:
                conflicts.append({
                    'file': file_path,
                    'wildcards': wildcard_imports
                })
        except Exception as e:
            print(f"Error reading {file_path}: {e}")
    
    return conflicts

def fix_optimization_conflicts():
    """Apply fixes for optimization-related E0659 conflicts"""
    fixes_applied = []
    
    # First, let's check the main optimization mod.rs for potential conflicts
    mod_rs_path = "src/optimization/mod.rs"
    
    try:
        with open(mod_rs_path, 'r') as f:
            content = f.read()
        
        original_content = content
        
        # Look for potentially conflicting re-exports
        conflicts_to_fix = [
            ('OptimizationResults', 'RealOptimizationResults'),
            ('PerformanceAnalyzer', 'LlvmPerformanceAnalyzer'),
            ('BenchmarkResult', 'TypesBenchmarkResult'),
            ('AdaptiveStrategy', 'OriginalAdaptiveStrategy'),
        ]
        
        # Check LLVM optimization integration
        llvm_opt_path = "src/codegen/llvm/optimization_integration.rs"
        if os.path.exists(llvm_opt_path):
            with open(llvm_opt_path, 'r') as f:
                llvm_content = f.read()
            
            # Ensure explicit imports for commonly conflicting types
            if 'use crate::optimization::*;' in llvm_content:
                # Replace wildcard import with explicit imports
                explicit_imports = """
use crate::optimization::{
    OptimizationConfig, OptimizationManager, AdaptiveOptimizer, 
    IncrementalCompiler, BenchmarkSuite, PerformanceProfiler,
    OptimizationFeedback, OptimizationStrategy, OptimizationRecommendation,
    IncrementalCompilationResult, BenchmarkSuiteResults, AdaptationResult,
    BenchmarkConfig, PerformanceMetrics
};"""
                
                llvm_content = llvm_content.replace(
                    'use crate::optimization::*;',
                    explicit_imports
                )
                
                with open(llvm_opt_path, 'w') as f:
                    f.write(llvm_content)
                    
                fixes_applied.append(f"Fixed wildcard import in {llvm_opt_path}")
    
    except Exception as e:
        print(f"Error fixing optimization conflicts: {e}")
    
    return fixes_applied

def add_explicit_optimization_aliases():
    """Add explicit aliases for commonly conflicting optimization types"""
    mod_rs_path = "src/optimization/mod.rs"
    
    try:
        with open(mod_rs_path, 'r') as f:
            content = f.read()
        
        # Check if we need to add more explicit aliases to prevent conflicts
        additional_aliases = """

// Additional aliases to prevent E0659 conflicts in downstream modules
pub type OptimizationEngine = LocalOptimizationCoordinator;
pub type DefaultOptimizationResult = OptimizationResult;
pub type DefaultBenchmarkResult = BenchmarkResult;
pub type DefaultPerformanceAnalyzer = PerformanceAnalyzer;
pub type DefaultAdaptiveStrategy = AdaptiveStrategy;

// LLVM-specific optimization types to avoid conflicts
pub type LlvmOptimizationEngine = RealLlvmOptimizer;
pub type LlvmOptimizationResult = RealOptimizationResults;
pub type LlvmPerformanceMetrics = PerformanceImprovements;
"""
        
        # Add aliases if not already present
        if "// Additional aliases to prevent E0659 conflicts" not in content:
            # Find the end of the module and add aliases
            insertion_point = content.rfind("}")
            if insertion_point != -1:
                content = content[:insertion_point] + additional_aliases + "\n" + content[insertion_point:]
                
                with open(mod_rs_path, 'w') as f:
                    f.write(content)
                
                return "Added explicit optimization type aliases"
    
    except Exception as e:
        print(f"Error adding optimization aliases: {e}")
    
    return None

def main():
    print("🔍 Analyzing E0659 optimization conflicts...")
    
    # Run cargo check to find errors
    output = run_cargo_check()
    
    # Find E0659 errors
    e0659_errors = find_e0659_errors(output)
    
    if e0659_errors:
        print(f"Found {len(e0659_errors)} E0659 errors related to optimization:")
        for error in e0659_errors:
            print(f"  - {error['line']}")
    else:
        print("No E0659 optimization-specific errors found.")
    
    # Analyze imports for potential conflicts
    print("\n🔍 Analyzing optimization imports...")
    conflicts = analyze_optimization_imports()
    
    if conflicts:
        print(f"Found {len(conflicts)} files with potential wildcard import conflicts:")
        for conflict in conflicts:
            print(f"  - {conflict['file']}: {len(conflict['wildcards'])} wildcard imports")
    
    # Apply fixes
    print("\n🔧 Applying optimization conflict fixes...")
    fixes = fix_optimization_conflicts()
    alias_fix = add_explicit_optimization_aliases()
    
    if fixes:
        for fix in fixes:
            print(f"  ✅ {fix}")
    
    if alias_fix:
        print(f"  ✅ {alias_fix}")
    
    if not fixes and not alias_fix:
        print("  ℹ️  No optimization-specific fixes needed")
    
    # Run cargo check again to verify fixes
    print("\n🧪 Testing fixes...")
    new_output = run_cargo_check()
    new_errors = find_e0659_errors(new_output)
    
    if len(new_errors) < len(e0659_errors):
        print(f"  ✅ Reduced E0659 errors from {len(e0659_errors)} to {len(new_errors)}")
    elif len(new_errors) == len(e0659_errors):
        print(f"  ℹ️  No change in E0659 error count ({len(e0659_errors)})")
    else:
        print(f"  ❌ E0659 errors increased from {len(e0659_errors)} to {len(new_errors)}")
    
    print("\n✅ Optimization E0659 conflict analysis complete!")

if __name__ == "__main__":
    main()
