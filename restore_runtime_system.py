#!/usr/bin/env python3
"""
Quick restoration of the runtime system from runtime_disabled
"""

import os
import shutil
import subprocess

def restore_runtime():
    """Restore runtime from disabled state"""
    print("🔧 Restoring runtime system...")
    
    if os.path.exists("src/runtime_disabled"):
        if os.path.exists("src/runtime"):
            print("   Runtime directory already exists, backing up...")
            if os.path.exists("src/runtime_backup"):
                shutil.rmtree("src/runtime_backup")
            shutil.move("src/runtime", "src/runtime_backup")
        
        print("   Moving runtime_disabled to runtime...")
        shutil.move("src/runtime_disabled", "src/runtime")
        
        # Update lib.rs to include runtime module
        with open("src/lib.rs", "r") as f:
            content = f.read()
        
        if "pub mod runtime;" not in content:
            content += "\npub mod runtime;\n"
            with open("src/lib.rs", "w") as f:
                f.write(content)
            print("   Added runtime module to lib.rs")
        
        print("   ✅ Runtime system restored")
        return True
    else:
        print("   ❌ No runtime_disabled directory found")
        return False

def test_runtime():
    """Test if runtime compiles"""
    print("🧪 Testing runtime compilation...")
    
    try:
        result = subprocess.run(['cargo', 'check', '--quiet'], 
                              capture_output=True, text=True)
        if result.returncode == 0:
            print("   ✅ Runtime compiles successfully")
            return True
        else:
            print("   🔴 Runtime compilation issues:")
            print(f"   {result.stderr[:300]}...")
            return False
    except Exception as e:
        print(f"   🔴 Error testing runtime: {e}")
        return False

def main():
    print("🚀 CURSED Runtime System Restoration\n")
    
    # Step 1: Restore runtime
    if not restore_runtime():
        print("❌ Could not restore runtime system")
        return 1
    
    # Step 2: Test compilation
    if test_runtime():
        print("\n🎉 Runtime system successfully restored and working!")
        print("\nNext steps:")
        print("- Run: cargo test runtime")
        print("- Test goroutine functionality")
        print("- Verify channel operations")
        print("- Check async/await support")
    else:
        print("\n⚠️  Runtime restored but has compilation issues")
        print("Manual fixes may be needed for import errors")
    
    return 0

if __name__ == "__main__":
    exit(main())
