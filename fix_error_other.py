#!/usr/bin/env python3

import os
import re

def fix_error_other_in_file(filepath):
    """Fix Error::Other to Error::General in a single file"""
    try:
        with open(filepath, 'r', encoding='utf-8') as f:
            content = f.read()
        
        original_content = content
        
        # Replace all variants of Error::Other with Error::General
        content = re.sub(r'\bError::Other\b', 'Error::General', content)
        content = re.sub(r'\berror::Error::Other\b', 'error::Error::General', content)
        content = re.sub(r'\bCursedError::Other\b', 'CursedError::General', content)
        
        if content != original_content:
            with open(filepath, 'w', encoding='utf-8') as f:
                f.write(content)
            print(f"Fixed Error::Other in {filepath}")
            return True
        
    except Exception as e:
        print(f"Error processing {filepath}: {e}")
        return False
    
    return False

def main():
    # List of files that have Error::Other
    files_to_fix = [
        "src/codegen/llvm/optimization.rs",
        "src/codegen/llvm/optimization_pipeline.rs", 
        "src/codegen/llvm/lto_integration.rs",
        "src/codegen/llvm/optimization_passes.rs",
        "src/codegen/llvm/performance_monitor.rs",
        "src/codegen/llvm/real_optimization_integration.rs",
        "src/optimization/config.rs",
        "src/optimization/llvm_passes.rs",
        "src/optimization/benchmarks.rs",
        "src/optimization/performance_integration.rs",
        "src/optimization/passes.rs",
        "src/optimization/pgo/profile_collector.rs",
        "src/optimization/pgo/profile_storage.rs",
        "src/optimization/pgo/collector.rs",
        "src/optimization/pgo/profile_manager.rs",
        "src/optimization/pgo/pgo_passes.rs",
        "src/optimization/pgo/mod.rs",
        "src/optimization/pgo/llvm_integration.rs",
        "src/optimization/lto.rs",
        "src/optimization/system.rs",
        "src/optimization/baseline_storage.rs",
        "src/optimization/comprehensive_performance_system.rs",
        "src/optimization/cache.rs",
        "src/optimization/parallel.rs",
        "src/optimization/optimization_levels.rs",
        "src/cli/pgo.rs",
        "src/cli/pgo_commands.rs",
        "src/build_system/lto_integration.rs",
        "src/core/performance_pipeline.rs",
        "src/stdlib/web_vibez/client.rs",
        "src/stdlib/glowup_http/client.rs",
        "src/stdlib/database/mysql/error.rs",
        "src/stdlib/async/mod.rs",
        "src/profiling/performance.rs",
        "src/bin/cursed_baseline.rs",
        "src/bin/cursed_optimize.rs"
    ]
    
    fixed_count = 0
    for filepath in files_to_fix:
        if os.path.exists(filepath):
            if fix_error_other_in_file(filepath):
                fixed_count += 1
        else:
            print(f"File not found: {filepath}")
    
    print(f"\nFixed Error::Other in {fixed_count} files")

if __name__ == "__main__":
    main()
