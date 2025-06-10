#!/usr/bin/env python3
"""
Count how many test files are now compiling vs failing.
"""

import subprocess
import re
import glob
import os

def test_individual_files():
    """Test individual test files to see which ones compile."""
    test_files = glob.glob('tests/*.rs')
    
    # Skip non-test files
    excluded = ['ast_factory.rs', 'tracing_setup.rs', 'test_helpers.rs', 'token_helper.rs', 'common.rs']
    test_files = [f for f in test_files if not any(ex in f for ex in excluded)]
    
    working = []
    failing = []
    
    print(f"Testing {len(test_files)} individual test files...")
    
    for test_file in test_files[:50]:  # Test first 50 to get a sample
        basename = os.path.basename(test_file).replace('.rs', '')
        
        try:
            result = subprocess.run(['./fix_linking.sh', 'cargo', 'test', '--test', basename], 
                                  capture_output=True, text=True, timeout=10)
            
            if result.returncode == 0:
                working.append(basename)
                print(f"✅ {basename}")
            else:
                failing.append(basename)
                print(f"❌ {basename}")
                
        except subprocess.TimeoutExpired:
            failing.append(basename)
            print(f"⏰ {basename} (timeout)")
        except Exception as e:
            failing.append(basename)
            print(f"💥 {basename} (error)")
    
    return working, failing

def main():
    """Count working vs failing tests."""
    print("Analyzing test compilation status...")
    
    working, failing = test_individual_files()
    
    print(f"\n📊 RESULTS:")
    print(f"✅ Working tests: {len(working)}")
    print(f"❌ Failing tests: {len(failing)}")
    print(f"📈 Success rate: {len(working)/(len(working)+len(failing))*100:.1f}%")
    
    if working:
        print(f"\n✅ Working tests:")
        for test in sorted(working):
            print(f"  - {test}")

if __name__ == "__main__":
    main()
