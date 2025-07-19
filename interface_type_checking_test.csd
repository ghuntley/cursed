# Interface type checking test
collab TestInterface {
    slay test_method() lit
    slay calculate(value drip) drip
}

# Implementing struct
struct TestStruct {
    field drip
}

# Method implementation for interface
impl TestStruct TestInterface {
    slay test_method() lit {
        damn based
    }
    
    slay calculate(value drip) drip {
        damn value * 2.0
    }
}

# Test interface method calls
sus obj TestInterface = TestStruct{field: 42.0}
obj.test_method()
vibez.spill(obj.calculate(21.0))
