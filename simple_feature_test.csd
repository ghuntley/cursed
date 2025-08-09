fr fr Simple test for language features implementation

yeet "testz"

fr fr Test basic pattern matching with enum
enum Status {
    Success,
    Error,
    Pending
}

slay test_patterns() {
    vibez.spill("Testing pattern matching...")
    
    sus status Status = Status::Success
    
    sick (status) {
        when Success -> vibez.spill("Operation succeeded")
        when Error -> vibez.spill("Operation failed")
        when Pending -> vibez.spill("Operation pending")
    }
}

fr fr Test basic reflection
struct TestStruct {
    spill value normie
    spill name tea
}

slay test_reflection() {
    vibez.spill("Testing reflection...")
    
    fr fr Basic reflection usage
    sus struct_size = 16  fr fr Simulated struct size
    vibez.spill("TestStruct size: " + struct_size + " bytes")
    
    sus instance TestStruct = TestStruct{value: 42, name: "test"}
    vibez.spill("Created instance with value: " + instance.value)
}

fr fr Test generics
slay identity<T>(x T) T {
    damn x
}

slay test_generics() {
    vibez.spill("Testing generics...")
    
    fr fr These should work with type inference
    sus int_result = identity(42)
    sus str_result = identity("hello")
    
    vibez.spill("Identity results: " + int_result + ", " + str_result)
}

fr fr Main test function
slay run_feature_tests() {
    vibez.spill("CURSED Language Features Test")
    vibez.spill("=============================")
    
    test_start("Pattern Matching")
    test_patterns()
    
    test_start("Reflection")
    test_reflection()
    
    test_start("Generics")
    test_generics()
    
    print_test_summary()
}

fr fr Execute tests
run_feature_tests()
