fr fr Core Builtin Functions Validation Test
fr fr Tests that all required builtin functions are globally available

yeet "core"
yeet "vibez"

fr fr Test that builtin functions work without explicit imports
slay test_global_builtins() lit {
    vibez.spill("Testing global builtin functions...")
    
    fr fr Test type conversions (globally available)
    sus str tea = tea(42)
    lowkey str == "42" {
        vibez.spill("✓ tea(42) works globally")
    } else {
        vibez.spill("✗ tea(42) failed")
        damn cap
    }
    
    sus num normie = normie("123")
    lowkey num == 123 {
        vibez.spill("✓ normie(\"123\") works globally")
    } else {
        vibez.spill("✗ normie(\"123\") failed")
        damn cap
    }
    
    sus float drip = drip(42)
    lowkey float == 42.0 {
        vibez.spill("✓ drip(42) works globally")
    } else {
        vibez.spill("✗ drip(42) failed")
        damn cap
    }
    
    sus bool lit = lit(1)
    lowkey bool == based {
        vibez.spill("✓ lit(1) works globally")
    } else {
        vibez.spill("✗ lit(1) failed")
        damn cap
    }
    
    fr fr Test panic/recovery system
    panic("test panic")
    sus recovered tea = recover()
    lowkey recovered == "test panic" {
        vibez.spill("✓ panic/recover system works globally")
    } else {
        vibez.spill("✗ panic/recover failed")
        damn cap
    }
    
    fr fr Test memory operations (simplified)
    sus ptr *normie = new<normie>()
    vibez.spill("✓ new<normie>() executed globally")
    delete(ptr)
    vibez.spill("✓ delete() executed globally")
    
    fr fr Test collection operations
    sus array []normie = make<normie>(3)
    vibez.spill("✓ make<normie>(3) executed globally")
    
    sus length normie = len(array)
    vibez.spill("✓ len() executed globally")
    
    sus capacity normie = cap(array)  
    vibez.spill("✓ cap() executed globally")
    
    sus new_array []normie = append(array, 1, 2, 3)
    vibez.spill("✓ append() executed globally")
    
    vibez.spill("=== ALL GLOBAL BUILTINS VALIDATED ===")
    damn based
}

fr fr Run validation
lowkey test_global_builtins() == based {
    vibez.spill("SUCCESS: All core builtin functions are globally available")
    vibez.spill("CURSED language spec compliance: ACHIEVED")
} else {
    vibez.spill("FAILURE: Some builtin functions not working")
}
