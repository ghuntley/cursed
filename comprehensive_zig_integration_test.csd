fr fr CURSED Zig Integration Test - Advanced Features
fr fr Test all advanced language constructs with full parser

fr fr Import testing framework
yeet "testz"

fr fr Test struct declarations
squad TestStruct {
    x drip,
    y drip,
    name tea
}

fr fr Test interface declarations  
collab TestInterface {
    slay test_method(param drip) drip
    slay get_name() tea
}

fr fr Test function with generics syntax
slay generic_function<T>(value T) T {
    damn value
}

fr fr Test for loop syntax  
slay test_for_loop() {
    bestie i drip = 0; i < 5; i++ {
        vibez.spill(i)
    }
}

fr fr Test match statement
slay test_match(value drip) tea {
    match value {
        1 => damn "one"
        2 => damn "two"
        _ => damn "other"
    }
}

fr fr Test tuple and member access
slay test_advanced_syntax() {
    sus data tuple = (42, "test", based)
    sus struct_instance TestStruct = TestStruct{x: 10, y: 20, name: "test"}
    
    vibez.spill(data.0)
    vibez.spill(struct_instance.name)
}

fr fr Test error handling
slay test_error_handling() {
    sus result shook = might_fail("test")
    match result {
        Ok(value) => vibez.spill(value)
        Err(error) => vibez.spill("Error occurred")
    }
}

fr fr Main test execution
test_start("CURSED Zig Advanced Features Test")

test_for_loop()
test_advanced_syntax()
test_error_handling()

sus match_result tea = test_match(1)
assert_eq_string(match_result, "one")

print_test_summary()
