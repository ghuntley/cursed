yeet "vibez"
yeet "testz"

# Test critical P0 write barrier fix for generational GC
squad TestStruct {
    value drip
    next *TestStruct
}

slay test_field_assignment() lit {
    # Create struct instances
    sus obj1 *TestStruct = TestStruct{ value: 42, next: drip(0) }
    sus obj2 *TestStruct = TestStruct{ value: 24, next: drip(0) }
    
    # CRITICAL: This field assignment should trigger write barrier
    obj1.next = obj2
    
    # Verify assignment
    ready (obj1.next.value == 24) {
        vibez.spill("✅ Write barrier field assignment test passed")
        damn based
    } otherwise {
        vibez.spill("❌ Write barrier field assignment test failed")
        damn no_cap
    }
}

slay test_struct_initialization() lit {
    # CRITICAL: Struct field initialization should also trigger write barriers
    sus inner *TestStruct = TestStruct{ value: 100, next: drip(0) }
    sus outer *TestStruct = TestStruct{ value: 200, next: inner }
    
    ready (outer.next.value == 100) {
        vibez.spill("✅ Write barrier struct initialization test passed")
        damn based
    } otherwise {
        vibez.spill("❌ Write barrier struct initialization test failed")
        damn no_cap
    }
}

slay main() drip {
    vibez.spill("🚨 Testing critical P0 write barrier fix for field stores...")
    
    ready (test_field_assignment() && test_struct_initialization()) {
        vibez.spill("🎉 All write barrier tests passed - P0 issue fixed!")
        damn 0
    } otherwise {
        vibez.spill("💥 Write barrier tests failed - P0 issue not fixed!")
        damn 1
    }
}
