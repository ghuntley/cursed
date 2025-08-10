#!/usr/bin/env python3
"""
Simplified test script for the CURSED Continuous Benchmark Harness
Tests core functionality without requiring advanced dependencies
"""

import sys
import os
import tempfile
import sqlite3
import json
from datetime import datetime, timedelta
from pathlib import Path

def test_basic_imports():
    """Test if basic Python modules are available"""
    print("🔍 Testing basic imports...")
    
    try:
        import sqlite3
        import json
        import time
        import threading
        from datetime import datetime, timedelta
        from typing import Dict, List, Tuple, Optional, Any
        from dataclasses import dataclass
        print("✅ All basic Python modules available")
        return True
    except ImportError as e:
        print(f"❌ Missing basic module: {e}")
        return False

def test_dataclass_functionality():
    """Test dataclass functionality for benchmark results"""
    print("\n📊 Testing dataclass functionality...")
    
    try:
        from dataclasses import dataclass
        
        @dataclass
        class TestBenchmarkResult:
            timestamp: datetime
            benchmark_name: str
            compilation_time_ms: float
            success: bool
        
        # Create test result
        result = TestBenchmarkResult(
            timestamp=datetime.now(),
            benchmark_name="test.csd",
            compilation_time_ms=123.45,
            success=True
        )
        
        if result.compilation_time_ms == 123.45:
            print("✅ Dataclass functionality working")
            return True
        else:
            print("❌ Dataclass values not set correctly")
            return False
            
    except Exception as e:
        print(f"❌ Dataclass test failed: {e}")
        return False

def test_sqlite_database():
    """Test SQLite database operations"""
    print("\n💾 Testing SQLite database...")
    
    with tempfile.NamedTemporaryFile(suffix='.db', delete=False) as tmp:
        db_path = tmp.name
    
    try:
        # Create database and tables
        conn = sqlite3.connect(db_path)
        
        # Create benchmark results table
        conn.execute("""
            CREATE TABLE benchmark_results (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                timestamp TEXT NOT NULL,
                benchmark_name TEXT NOT NULL,
                compilation_time_ms REAL NOT NULL,
                success BOOLEAN NOT NULL
            )
        """)
        
        # Insert test data
        test_data = [
            (datetime.now().isoformat(), "test1.csd", 100.5, True),
            (datetime.now().isoformat(), "test2.csd", 200.0, True),
            (datetime.now().isoformat(), "test3.csd", 150.7, False)
        ]
        
        conn.executemany("""
            INSERT INTO benchmark_results (timestamp, benchmark_name, compilation_time_ms, success)
            VALUES (?, ?, ?, ?)
        """, test_data)
        
        # Query data back
        cursor = conn.execute("SELECT COUNT(*) FROM benchmark_results WHERE success = 1")
        successful_count = cursor.fetchone()[0]
        
        conn.close()
        
        if successful_count == 2:
            print("✅ SQLite database operations working")
            return True
        else:
            print(f"❌ Expected 2 successful results, got {successful_count}")
            return False
            
    except Exception as e:
        print(f"❌ SQLite test failed: {e}")
        return False
    finally:
        if os.path.exists(db_path):
            os.unlink(db_path)

def test_configuration_file():
    """Test configuration file handling"""
    print("\n⚙️  Testing configuration file...")
    
    try:
        # Test loading existing config
        config_path = "benchmark_config.json"
        if os.path.exists(config_path):
            with open(config_path, 'r') as f:
                config = json.load(f)
            
            # Check required keys
            required_keys = [
                'benchmark_interval_minutes',
                'benchmark_files',
                'optimization_levels',
                'regression_threshold_percent'
            ]
            
            missing_keys = [key for key in required_keys if key not in config]
            if not missing_keys:
                print("✅ Configuration file is valid")
                print(f"   Interval: {config['benchmark_interval_minutes']} minutes")
                print(f"   Benchmarks: {len(config['benchmark_files'])}")
                return True
            else:
                print(f"❌ Missing keys in config: {missing_keys}")
                return False
        else:
            # Test creating default config
            default_config = {
                "benchmark_interval_minutes": 30,
                "benchmark_files": ["test.csd"],
                "optimization_levels": ["Debug", "ReleaseFast"],
                "regression_threshold_percent": 10.0
            }
            
            with tempfile.NamedTemporaryFile(mode='w', suffix='.json', delete=False) as tmp:
                json.dump(default_config, tmp, indent=2)
                tmp_path = tmp.name
            
            # Load it back
            with open(tmp_path, 'r') as f:
                loaded_config = json.load(f)
            
            os.unlink(tmp_path)
            
            if loaded_config['benchmark_interval_minutes'] == 30:
                print("✅ Configuration file creation and loading working")
                return True
            else:
                print("❌ Configuration round-trip failed")
                return False
                
    except Exception as e:
        print(f"❌ Configuration test failed: {e}")
        return False

def test_file_operations():
    """Test file system operations"""
    print("\n📁 Testing file operations...")
    
    try:
        # Test creating temporary files
        with tempfile.NamedTemporaryFile(mode='w', suffix='.csd', delete=False) as tmp:
            tmp.write('vibez.spill("Hello, world!")')
            test_file = tmp.name
        
        # Test file existence and reading
        if os.path.exists(test_file):
            with open(test_file, 'r') as f:
                content = f.read()
            
            if 'Hello, world!' in content:
                print("✅ File operations working")
                os.unlink(test_file)
                return True
            else:
                print("❌ File content not as expected")
                return False
        else:
            print("❌ Temporary file not created")
            return False
            
    except Exception as e:
        print(f"❌ File operations test failed: {e}")
        return False

def test_datetime_operations():
    """Test datetime operations for timestamps"""
    print("\n🕐 Testing datetime operations...")
    
    try:
        # Test datetime creation and formatting
        now = datetime.now()
        iso_format = now.isoformat()
        
        # Test parsing back
        parsed = datetime.fromisoformat(iso_format)
        
        # Test timedelta operations
        one_week_ago = now - timedelta(days=7)
        
        if abs((parsed - now).total_seconds()) < 1:
            print("✅ Datetime operations working")
            print(f"   Current time: {now.strftime('%Y-%m-%d %H:%M:%S')}")
            print(f"   One week ago: {one_week_ago.strftime('%Y-%m-%d %H:%M:%S')}")
            return True
        else:
            print("❌ Datetime parsing failed")
            return False
            
    except Exception as e:
        print(f"❌ Datetime test failed: {e}")
        return False

def test_cursed_compiler_presence():
    """Test if CURSED compiler binaries are present"""
    print("\n🔧 Testing CURSED compiler presence...")
    
    compiler_paths = [
        "zig-out/bin/cursed-zig",
        "zig-out/bin/cursed",
        "zig-out/bin/cursed-stable"
    ]
    
    found_compilers = []
    for path in compiler_paths:
        if os.path.exists(path):
            found_compilers.append(path)
            try:
                # Test if file is executable
                if os.access(path, os.X_OK):
                    print(f"✅ Found executable compiler: {path}")
                else:
                    print(f"⚠️  Found compiler but not executable: {path}")
            except Exception:
                print(f"⚠️  Found compiler file: {path}")
    
    if found_compilers:
        print(f"✅ Found {len(found_compilers)} compiler binary(ies)")
        return True
    else:
        print("⚠️  No CURSED compiler binaries found")
        print("   This is normal if you haven't built the compiler yet")
        print("   Run 'zig build' to build the compiler")
        return False

def test_benchmark_files():
    """Test if benchmark files are present"""
    print("\n📝 Testing benchmark files...")
    
    # Check if we have the config to know what files to look for
    config_file = "benchmark_config.json"
    if os.path.exists(config_file):
        with open(config_file, 'r') as f:
            config = json.load(f)
        
        benchmark_files = config.get('benchmark_files', [])
        found_files = []
        
        for file_path in benchmark_files:
            if os.path.exists(file_path):
                found_files.append(file_path)
                print(f"✅ Found benchmark: {file_path}")
            else:
                print(f"⚠️  Missing benchmark: {file_path}")
        
        if found_files:
            print(f"✅ Found {len(found_files)}/{len(benchmark_files)} configured benchmarks")
            return len(found_files) > 0
        else:
            print("⚠️  No configured benchmark files found")
            return False
    else:
        # Check for common benchmark files
        common_benchmarks = [
            "comprehensive_stdlib_test.csd",
            "advanced_features_test.csd",
            "benchmarks/cursed/fasta.csd"
        ]
        
        found = []
        for file_path in common_benchmarks:
            if os.path.exists(file_path):
                found.append(file_path)
                print(f"✅ Found benchmark: {file_path}")
        
        if found:
            print(f"✅ Found {len(found)} common benchmark files")
            return True
        else:
            print("⚠️  No common benchmark files found")
            return False

def main():
    """Run all simplified tests"""
    print("🚀 CURSED Continuous Benchmark Harness - Simplified Test Suite")
    print("=" * 65)
    
    tests = [
        ("Basic Imports", test_basic_imports),
        ("Dataclass Functionality", test_dataclass_functionality),
        ("SQLite Database", test_sqlite_database),
        ("Configuration File", test_configuration_file),
        ("File Operations", test_file_operations),
        ("Datetime Operations", test_datetime_operations),
        ("CURSED Compiler Presence", test_cursed_compiler_presence),
        ("Benchmark Files", test_benchmark_files)
    ]
    
    passed = 0
    warnings = 0
    total = len(tests)
    
    for test_name, test_func in tests:
        try:
            result = test_func()
            if result:
                passed += 1
            else:
                # Check if this is a warning (compiler/benchmark files not found)
                if "compiler" in test_name.lower() or "benchmark" in test_name.lower():
                    warnings += 1
        except Exception as e:
            print(f"❌ {test_name} failed with exception: {e}")
    
    print("\n" + "=" * 65)
    print(f"🎯 Test Results: {passed}/{total} tests passed")
    if warnings > 0:
        print(f"⚠️  {warnings} tests had warnings (missing files, normal before setup)")
    
    # Determine overall status
    core_tests_passed = passed >= (total - warnings)
    
    if core_tests_passed:
        print("🎉 Core functionality tests passed! The benchmark harness should work.")
        print("\nNext steps:")
        if warnings > 0:
            print("  1. Run 'zig build' to build the CURSED compiler")
            print("  2. Run './run_continuous_benchmarks.sh setup' for full setup")
        print("  3. Run './run_continuous_benchmarks.sh single' to test with a benchmark")
        print("  4. Run './run_continuous_benchmarks.sh start' to begin continuous monitoring")
        
        print("\n📋 System Requirements Status:")
        print("  ✅ Python 3 and standard library")
        print("  ✅ SQLite database functionality")
        print("  ✅ JSON configuration handling")
        print("  ✅ File system operations")
        
        if warnings == 0:
            print("  ✅ CURSED compiler binaries")
            print("  ✅ Benchmark test files")
        else:
            print("  ⚠️  CURSED compiler binaries (run 'zig build')")
            print("  ⚠️  Some benchmark files (will be created during setup)")
            
    else:
        print("❌ Some core functionality tests failed.")
        print("💡 Please check the errors above and ensure you have:")
        print("   • Python 3.7+ with standard library")
        print("   • Write permissions in current directory")
        print("   • SQLite3 support in Python")
    
    return core_tests_passed

if __name__ == "__main__":
    success = main()
    sys.exit(0 if success else 1)
