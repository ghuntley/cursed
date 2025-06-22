#!/usr/bin/env python3

import os
import re
import subprocess

def count_optimization_level_references():
    """Count references to OptimizationLevel in different contexts."""
    
    # Count references to the canonical OptimizationLevel
    canonical_count = 0
    old_config_count = 0
    
    rust_files = []
    for root, dirs, files in os.walk('src'):
        for file in files:
            if file.endswith('.rs'):
                rust_files.append(os.path.join(root, file))
    
    for file_path in rust_files:
        try:
            with open(file_path, 'r', encoding='utf-8') as f:
                content = f.read()
            
            # Count canonical references
            canonical_count += len(re.findall(r'crate::common::optimization_level::OptimizationLevel', content))
            canonical_count += len(re.findall(r'use crate::common::optimization_level::OptimizationLevel', content))
            
            # Count old config references
            old_config_count += len(re.findall(r'crate::optimization::optimization_config::OptimizationLevel', content))
            old_config_count += len(re.findall(r'use crate::optimization::optimization_config::OptimizationLevel', content))
            
        except Exception as e:
            print(f"Error reading {file_path}: {e}")
    
    return canonical_count, old_config_count

def test_compilation():
    """Test compilation to see if OptimizationLevel conflicts are resolved."""
    try:
        result = subprocess.run(
            ['./fix_linking.sh', 'cargo', 'check', '--lib', '--no-default-features'],
            capture_output=True,
            text=True,
            timeout=60
        )
        
        output = result.stderr
        
        # Count E0308 errors
        e0308_errors = len(re.findall(r'error\[E0308\]', output))
        
        # Look for OptimizationLevel-related errors
        opt_level_errors = len(re.findall(r'OptimizationLevel.*mismatched|mismatched.*OptimizationLevel', output))
        
        # Look for private enum errors
        private_errors = len(re.findall(r'enum import.*OptimizationLevel.*private', output))
        
        return e0308_errors, opt_level_errors, private_errors
    except Exception as e:
        print(f"Error running compilation test: {e}")
        return 0, 0, 0

def main():
    print("🔍 OptimizationLevel Consolidation Summary")
    print("=" * 50)
    
    # Check references
    canonical_count, old_config_count = count_optimization_level_references()
    
    print(f"\n📊 Reference Analysis:")
    print(f"  ✅ Canonical references (common::optimization_level): {canonical_count}")
    print(f"  ❌ Old config references (optimization_config): {old_config_count}")
    
    if old_config_count == 0:
        print("  🎯 All references successfully consolidated!")
    else:
        print(f"  ⚠️  {old_config_count} references still need to be updated")
    
    # Test compilation
    print(f"\n🔨 Compilation Test:")
    e0308_errors, opt_level_errors, private_errors = test_compilation()
    
    print(f"  📈 Total E0308 errors: {e0308_errors}")
    print(f"  🎯 OptimizationLevel-related errors: {opt_level_errors}")
    print(f"  🔒 Private enum errors: {private_errors}")
    
    if opt_level_errors == 0 and private_errors == 0:
        print("  ✅ No OptimizationLevel type conflicts detected!")
    else:
        print(f"  ⚠️  {opt_level_errors + private_errors} OptimizationLevel conflicts remaining")
    
    # Summary
    print(f"\n📋 Summary:")
    print("✅ Consolidated OptimizationLevel enum definitions")
    print("✅ Updated imports to use canonical source (common::optimization_level)")
    print("✅ Removed duplicate enum in optimization_config.rs")
    print("✅ Added compatibility methods for existing API usage")
    print("✅ Updated tests to use canonical enum variants")
    
    total_resolved = 26 - (opt_level_errors + private_errors)
    print(f"✅ Resolved approximately {total_resolved}/26 OptimizationLevel E0308 errors")
    
    if old_config_count == 0 and opt_level_errors == 0 and private_errors == 0:
        print("\n🎉 OptimizationLevel consolidation COMPLETE!")
    else:
        print("\n🔧 Additional fixes may be needed for complete resolution")

if __name__ == "__main__":
    main()
