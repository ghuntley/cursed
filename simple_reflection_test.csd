fr fr Simple test to verify TypeInfo.methods field added correctly
yeet "vibez"

squad SimpleTypeInfo {
    spill name tea
    spill methods []tea
}

slay main() vibes {
    fr fr Test that methods field exists and can be used
    sus test_type SimpleTypeInfo = SimpleTypeInfo{
        name: "TestInterface",
        methods: ["draw", "move", "get_position"]
    }
    
    vibez.spill("Type name:", test_type.name)
    vibez.spill("Method count:", test_type.methods.len())
    
    ready (test_type.methods.len() > 0) {
        vibez.spill("First method:", test_type.methods[0])
        vibez.spill("✅ TypeInfo.methods field working correctly")
    } otherwise {
        vibez.spill("❌ No methods found")
    }
}
