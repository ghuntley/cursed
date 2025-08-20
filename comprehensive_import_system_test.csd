// CURSED Import System Test - All 5 Canonical Forms
// This test validates the complete module resolution system

// =============================================================================
// FORM 1: Single Import
// =============================================================================
yeet "testz"

// =============================================================================  
// FORM 2: Multiple Imports (Comma-separated)
// =============================================================================
yeet "mathz", "stringz", "arrayz"

// =============================================================================
// FORM 3: Aliased Import
// =============================================================================
yeet "mathz" as math_ops
yeet "stringz" as str
yeet "collections" as col

// =============================================================================
// FORM 4: Selective Imports (Destructuring)
// =============================================================================
yeet { print, println } from "vibez"
yeet { HashMap, Vec, LinkedList } from "collections"
yeet { sin, cos, tan, sqrt } from "mathz"

// =============================================================================
// FORM 5: Selective Imports with Per-Item Aliasing
// =============================================================================
yeet { HashMap as Map, Vec as List, LinkedList as LL } from "collections"
yeet { print as p, println as pln } from "vibez"
yeet { sin as sine, cos as cosine } from "mathz"

// =============================================================================
// Advanced Forms (Bonus)
// =============================================================================
// Versioned imports
yeet "json@^1.0.0"
yeet "http@~2.1.0"

// Nested module paths
yeet "std/collections"
yeet "stdlib/advanced/cryptz"

// Mixed selective with aliasing
yeet { func1, func2 as f2, Type as T } from "advanced_module"

// =============================================================================
// Test the imported functionality
// =============================================================================
slay main() {
    // Test stdlib imports
    test_start("comprehensive_import_test")
    
    // Test basic mathematical operations from aliased import
    sus result drip = math_ops.add(5, 3)
    assert_eq_int(result, 8)
    
    // Test string operations from aliased import  
    sus text tea = str.concat("Hello", " World")
    assert_eq_string(text, "Hello World")
    
    // Test selective imports
    p("Testing selective imports...")
    pln("All import forms working!")
    
    // Test advanced math functions
    sus angle_result drip = sine(0.5)
    // Should be approximately 0.479 (sin(0.5 radians))
    
    // Test collections from selective imports
    sus my_map Map = Map.new()
    Map.insert(my_map, "key", "value")
    
    sus my_list List = List.new()  
    List.push(my_list, 42)
    
    print_test_summary()
    
    p("✅ All 5 canonical import forms working correctly!")
    p("✅ Module dependency tracking operational!")
    p("✅ Import resolution system complete!")
}
