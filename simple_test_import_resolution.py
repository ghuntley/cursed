#!/usr/bin/env python3
"""
Simple test to verify import resolution system for stdlib testz module
"""

import subprocess
import sys
import os

def test_import_resolution():
    """Test if testz import can be resolved"""
    
    # Create a simple test program
    test_program = '''yeet "testz"

test_start("Simple test")
assert_true(based)
print_test_summary()
'''
    
    # Write test program
    with open('test_simple_import.csd', 'w') as f:
        f.write(test_program)
    
    print("🔍 Testing 'yeet testz' import resolution...")
    
    # Test with interpretation mode first
    try:
        result = subprocess.run(['cargo', 'run', '--bin', 'cursed', 'test_simple_import.csd'], 
                              capture_output=True, text=True, timeout=30)
        
        if result.returncode == 0:
            print("✅ Import resolution works!")
            print("Output:", result.stdout)
            return True
        else:
            print("❌ Import resolution failed")
            print("Error:", result.stderr)
            
            # Check if it's a module resolution issue specifically
            if "Module not found" in result.stderr or "testz" in result.stderr:
                print("🎯 Identified: testz module resolution issue")
                return False
            else:
                print("🔧 Different issue - not import resolution")
                return False
                
    except subprocess.TimeoutExpired:
        print("⏰ Test timed out - possible hanging issue")
        return False
    except Exception as e:
        print(f"💥 Test crashed: {e}")
        return False
    finally:
        # Cleanup
        if os.path.exists('test_simple_import.csd'):
            os.remove('test_simple_import.csd')

if __name__ == "__main__":
    success = test_import_resolution()
    sys.exit(0 if success else 1)
