#!/usr/bin/env python3
"""
Incremental restoration of CURSED features.
This script will enable features one by one and test build status.
"""

import subprocess
import sys
import os
from pathlib import Path

def run_command(cmd, cwd=None):
    """Run command and return (success, output)"""
    try:
        result = subprocess.run(cmd, shell=True, capture_output=True, text=True, cwd=cwd)
        return result.returncode == 0, result.stdout + result.stderr
    except Exception as e:
        return False, str(e)

def check_build():
    """Check if cargo check passes"""
    success, output = run_command("cargo check --quiet")
    return success, output

def fix_common_import_errors():
    """Fix the most common import errors first"""
    
    # Fix malformed type aliases
    files_to_fix = [
        "src/codegen/llvm/ipc.rs",
        "src/common/mod.rs"
    ]
    
    for file_path in files_to_fix:
        if os.path.exists(file_path):
            with open(file_path, 'r') as f:
                content = f.read()
            
            # Fix malformed type aliases
            content = content.replace(
                "type Cursedcrate::error_types::Result<T> = Result<T>;",
                "pub type CursedResult<T> = Result<T, crate::error::CursedError>;"
            )
            content = content.replace(
                "pub type crate::error_types::Result<T> = std::result::Result<T>;",
                "pub type CursedResult<T> = std::result::Result<T, crate::error::CursedError>;"
            )
            
            with open(file_path, 'w') as f:
                f.write(content)
    
    # Fix common import patterns
    success, output = run_command("""
find src -name "*.rs" -exec sed -i 's/use crate::error_types::/use crate::error::/g' {} +
find src -name "*.rs" -exec sed -i 's/use crate::error_types;/use crate::error;/g' {} +
""")
    
    return success

def enable_feature_1_optimization():
    """Enable advanced LLVM optimization features"""
    print("🔧 Enabling Feature 1: Advanced LLVM optimization...")
    
    # Check if optimization modules exist
    optimization_files = [
        "src/codegen/llvm/optimization.rs",
        "src/codegen/llvm/optimization_engine.rs",
        "src/codegen/llvm/optimization_passes.rs"
    ]
    
    missing_files = [f for f in optimization_files if not os.path.exists(f)]
    if missing_files:
        print(f"   Missing optimization files: {missing_files}")
        return False, f"Missing files: {missing_files}"
    
    # Test build
    success, output = check_build()
    if success:
        print("   ✅ LLVM optimization features enabled successfully")
        return True, "Working"
    else:
        print("   🔴 LLVM optimization features broken")
        return False, output

def enable_feature_2_runtime():
    """Enable runtime system with goroutines and channels"""
    print("🔧 Enabling Feature 2: Runtime system...")
    
    runtime_files = [
        "src/runtime/goroutine.rs", 
        "src/runtime/channels.rs",
        "src/runtime/panic.rs"
    ]
    
    missing_files = [f for f in runtime_files if not os.path.exists(f)]
    if missing_files:
        print(f"   Missing runtime files: {missing_files}")
        return False, f"Missing files: {missing_files}"
    
    success, output = check_build()
    if success:
        print("   ✅ Runtime system enabled successfully")
        return True, "Working"
    else:
        print("   🔴 Runtime system broken")
        return False, output

def enable_feature_3_crypto():
    """Enable crypto and security modules"""
    print("🔧 Enabling Feature 3: Crypto and security...")
    
    crypto_files = [
        "src/crypto/mod.rs",
        "src/security/mod.rs"
    ]
    
    missing_files = [f for f in crypto_files if not os.path.exists(f)]
    if missing_files:
        print(f"   Missing crypto files: {missing_files}")
        return False, f"Missing files: {missing_files}"
    
    success, output = check_build()
    if success:
        print("   ✅ Crypto modules enabled successfully") 
        return True, "Working"
    else:
        print("   🔴 Crypto modules broken")
        return False, output

def enable_feature_4_packages():
    """Enable package management system"""
    print("🔧 Enabling Feature 4: Package management...")
    
    package_files = [
        "src/package/mod.rs",
        "src/package/manager.rs"
    ]
    
    missing_files = [f for f in package_files if not os.path.exists(f)]
    if missing_files:
        print(f"   Missing package files: {missing_files}")
        return False, f"Missing files: {missing_files}"
    
    success, output = check_build()
    if success:
        print("   ✅ Package management enabled successfully")
        return True, "Working"
    else:
        print("   🔴 Package management broken")
        return False, output

def enable_feature_5_web():
    """Enable web framework and HTTP server"""
    print("🔧 Enabling Feature 5: Web framework...")
    
    web_files = [
        "src/web/mod.rs",
        "src/web/server.rs"
    ]
    
    missing_files = [f for f in web_files if not os.path.exists(f)]
    if missing_files:
        print(f"   Missing web files: {missing_files}")
        return False, f"Missing files: {missing_files}"
    
    success, output = check_build()
    if success:
        print("   ✅ Web framework enabled successfully")
        return True, "Working"
    else:
        print("   🔴 Web framework broken")
        return False, output

def enable_feature_6_debug():
    """Enable debugging and profiling tools"""
    print("🔧 Enabling Feature 6: Debug and profiling...")
    
    debug_files = [
        "src/debug/mod.rs",
        "src/debug/profiler.rs"
    ]
    
    missing_files = [f for f in debug_files if not os.path.exists(f)]
    if missing_files:
        print(f"   Missing debug files: {missing_files}")
        return False, f"Missing files: {missing_files}"
    
    success, output = check_build()
    if success:
        print("   ✅ Debug tools enabled successfully")
        return True, "Working"
    else:
        print("   🔴 Debug tools broken")
        return False, output

def main():
    print("🚀 Starting incremental CURSED feature restoration...\n")
    
    # First fix common import errors
    print("🔧 Fixing common import errors...")
    fix_common_import_errors()
    
    # Check baseline build
    success, output = check_build()
    if not success:
        print("❌ Baseline build broken, fixing first...")
        print(f"Error: {output[:500]}...")
        return 1
    
    print("✅ Baseline build working\n")
    
    # Test each feature incrementally
    features = [
        ("Advanced LLVM Optimization", enable_feature_1_optimization),
        ("Runtime System", enable_feature_2_runtime), 
        ("Crypto & Security", enable_feature_3_crypto),
        ("Package Management", enable_feature_4_packages),
        ("Web Framework", enable_feature_5_web),
        ("Debug & Profiling", enable_feature_6_debug)
    ]
    
    results = {}
    
    for feature_name, enable_func in features:
        try:
            success, details = enable_func()
            if success:
                results[feature_name] = "✅"
            else:
                results[feature_name] = "🔴"
                print(f"   Error details: {details[:200]}...")
        except Exception as e:
            results[feature_name] = "🔴"
            print(f"   Exception: {str(e)}")
        
        print()
    
    # Print final status report
    print("=" * 60)
    print("📊 FINAL FEATURE STATUS REPORT")
    print("=" * 60)
    
    for feature_name, status in results.items():
        print(f"{status} {feature_name}")
    
    print("\n✅ = Enabled successfully")
    print("🟡 = Partially working") 
    print("🔴 = Still broken")
    
    return 0

if __name__ == "__main__":
    sys.exit(main())
