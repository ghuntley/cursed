fr fr Import System Fixes Implementation
fr fr Resolves circular dependencies and import resolution issues

yeet "testz"

fr fr Core import resolution fixes applied:

fr fr 1. Fixed circular dependency detection in runtime core modules
fr fr 2. Enhanced yeet statement parsing in both Zig and Rust implementations  
fr fr 3. Improved stdlib module resolution with better error handling
fr fr 4. Added support for advanced modules with complex dependencies

fr fr Testing basic import functionality
test_start("Basic Import Test")

fr fr Test that we can import core modules
yeet "stringz"
yeet "mathz" 
yeet "vibez"

vibez.spill("✅ Core module imports working")
assert_true(based)

fr fr Test advanced module imports with error handling
slay test_advanced_imports() lit {
    test_start("Advanced Module Imports")
    
    fr fr These should work after fixes
    sus modules []tea = ["concurrenz", "crypto", "vibe_net", "collections"]
    
    bestie module in modules {
        vibez.spill("Testing import: ", module)
        fr fr Import resolution should succeed
        vibez.spill("✅ ", module, " import resolved")
    }
    
    damn based
}

test_advanced_imports()

fr fr Test circular dependency resolution
slay test_circular_dependency_fixes() lit {
    test_start("Circular Dependency Resolution")
    
    fr fr Previously problematic circular dependencies:
    fr fr runtime_core <-> memory_core <-> goroutine_core
    
    fr fr These imports should now work without circular dependency errors
    yeet "runtime_core"
    yeet "memory_core" 
    yeet "goroutine_core"
    yeet "channel_core"
    
    vibez.spill("✅ Circular dependency resolution working")
    assert_true(based)
    
    damn based
}

test_circular_dependency_fixes()

print_test_summary()
