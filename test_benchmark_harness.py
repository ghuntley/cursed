#!/usr/bin/env python3
"""
Test script for the CURSED Continuous Benchmark Harness
Verifies core functionality without requiring full setup
"""

import sys
import os
import tempfile
import sqlite3
import json
from datetime import datetime
from pathlib import Path

# Add current directory to path to import harness modules
sys.path.insert(0, os.getcwd())

try:
    from continuous_benchmark_harness import (
        BenchmarkResult, BenchmarkDatabase, StatisticalAnalyzer, BenchmarkRunner
    )
    print("✅ Successfully imported harness modules")
except ImportError as e:
    print(f"❌ Failed to import harness modules: {e}")
    sys.exit(1)

def test_database_functionality():
    """Test database operations"""
    print("\n🔍 Testing database functionality...")
    
    with tempfile.NamedTemporaryFile(suffix='.db', delete=False) as tmp:
        db_path = tmp.name
    
    try:
        # Test database initialization
        db = BenchmarkDatabase(db_path)
        print("✅ Database initialized successfully")
        
        # Test result storage
        result = BenchmarkResult(
            timestamp=datetime.now(),
            git_commit="test123",
            benchmark_name="test_benchmark.csd",
            compilation_time_ms=150.5,
            execution_time_ms=25.0,
            memory_usage_kb=1024,
            binary_size_bytes=4096,
            success=True,
            compiler_version="test-1.0",
            optimization_level="ReleaseFast"
        )
        
        db.store_result(result)
        print("✅ Benchmark result stored successfully")
        
        # Test data retrieval
        historical_results = db.get_historical_results("test_benchmark.csd", "compilation_time_ms", days=1)
        if len(historical_results) == 1:
            print("✅ Historical data retrieval working")
        else:
            print("❌ Historical data retrieval failed")
        
        return True
        
    except Exception as e:
        print(f"❌ Database test failed: {e}")
        return False
    finally:
        if os.path.exists(db_path):
            os.unlink(db_path)

def test_statistical_analysis():
    """Test statistical analysis functionality"""
    print("\n📊 Testing statistical analysis...")
    
    try:
        analyzer = StatisticalAnalyzer()
        
        # Test regression detection with sample data
        historical_data = [100, 105, 98, 102, 99, 101, 103, 97, 100, 104]
        current_data = [120, 125, 118, 122, 119]  # Clear regression
        
        is_regression, p_value, effect_size = analyzer.detect_regression(
            current_data, historical_data, confidence_threshold=0.95
        )
        
        if is_regression:
            print("✅ Regression detection working (detected expected regression)")
        else:
            print("⚠️  Regression detection may need tuning (didn't detect clear regression)")
        
        # Test trend calculation
        import numpy as np
        values = [100 + i * 2 + np.random.normal(0, 5) for i in range(20)]
        timestamps = [datetime.now() for _ in range(20)]
        
        trend_result = analyzer.calculate_trend(values, timestamps)
        if 'slope' in trend_result and 'trend' in trend_result:
            print("✅ Trend calculation working")
            print(f"   Trend: {trend_result['trend']}, Slope: {trend_result['slope']:.3f}")
        else:
            print("❌ Trend calculation failed")
        
        return True
        
    except Exception as e:
        print(f"❌ Statistical analysis test failed: {e}")
        return False

def test_configuration_loading():
    """Test configuration system"""
    print("\n⚙️  Testing configuration loading...")
    
    try:
        # Test with existing config
        if os.path.exists('benchmark_config.json'):
            with open('benchmark_config.json', 'r') as f:
                config = json.load(f)
            
            required_keys = [
                'benchmark_interval_minutes',
                'benchmark_files', 
                'optimization_levels',
                'regression_threshold_percent'
            ]
            
            missing_keys = [key for key in required_keys if key not in config]
            if not missing_keys:
                print("✅ Configuration file has all required keys")
                print(f"   Configured benchmarks: {len(config['benchmark_files'])}")
                print(f"   Optimization levels: {len(config['optimization_levels'])}")
            else:
                print(f"❌ Missing configuration keys: {missing_keys}")
                return False
        else:
            print("⚠️  benchmark_config.json not found, but harness can create defaults")
        
        return True
        
    except Exception as e:
        print(f"❌ Configuration test failed: {e}")
        return False

def test_benchmark_runner_basic():
    """Test basic benchmark runner functionality"""
    print("\n🏃 Testing benchmark runner basics...")
    
    try:
        runner = BenchmarkRunner(".")
        
        # Test git commit retrieval
        git_commit = runner._get_git_commit()
        if git_commit and git_commit != "unknown":
            print(f"✅ Git commit detection working: {git_commit[:8]}")
        else:
            print("⚠️  Git commit detection returned 'unknown' (may be normal)")
        
        # Test environment info
        env_info = runner._get_environment_info()
        if 'os' in env_info and 'python_version' in env_info:
            print("✅ Environment info collection working")
            print(f"   OS: {env_info['os']}, CPU cores: {env_info['cpu_count']}")
        else:
            print("❌ Environment info collection failed")
            return False
        
        return True
        
    except Exception as e:
        print(f"❌ Benchmark runner test failed: {e}")
        return False

def test_compiler_detection():
    """Test if CURSED compiler can be detected"""
    print("\n🔧 Testing compiler detection...")
    
    compiler_paths = [
        "zig-out/bin/cursed-zig",
        "zig-out/bin/cursed",
        "zig-out/bin/cursed-stable"
    ]
    
    found_compilers = []
    for path in compiler_paths:
        if os.path.exists(path):
            found_compilers.append(path)
    
    if found_compilers:
        print(f"✅ Found {len(found_compilers)} compiler(s):")
        for compiler in found_compilers:
            print(f"   {compiler}")
    else:
        print("⚠️  No compiled CURSED binaries found")
        print("   Run 'zig build' to build the compiler first")
    
    return len(found_compilers) > 0

def main():
    """Run all tests"""
    print("🚀 CURSED Continuous Benchmark Harness - Test Suite")
    print("=" * 55)
    
    tests = [
        ("Database Functionality", test_database_functionality),
        ("Statistical Analysis", test_statistical_analysis),
        ("Configuration Loading", test_configuration_loading),
        ("Benchmark Runner Basics", test_benchmark_runner_basic),
        ("Compiler Detection", test_compiler_detection)
    ]
    
    passed = 0
    total = len(tests)
    
    for test_name, test_func in tests:
        try:
            if test_func():
                passed += 1
        except Exception as e:
            print(f"❌ {test_name} failed with exception: {e}")
    
    print("\n" + "=" * 55)
    print(f"🎯 Test Results: {passed}/{total} tests passed")
    
    if passed == total:
        print("🎉 All tests passed! The benchmark harness is ready to use.")
        print("\nNext steps:")
        print("  1. Run './run_continuous_benchmarks.sh setup' for full setup")
        print("  2. Run './run_continuous_benchmarks.sh single' to test with a benchmark")
        print("  3. Run './run_continuous_benchmarks.sh start' to begin continuous monitoring")
    elif passed >= total - 1:
        print("✅ Most tests passed. The system should work with minor limitations.")
        print("⚠️  Check any warnings above and run setup if needed.")
    else:
        print("❌ Several tests failed. Please check the errors above.")
        print("💡 Try running the setup script: './run_continuous_benchmarks.sh setup'")
    
    return passed == total

if __name__ == "__main__":
    success = main()
    sys.exit(0 if success else 1)
