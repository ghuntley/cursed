// Test interface dispatch system implementation
yeet "testz"

// Define a simple interface
collab TestInterface {
    slay test_method(param normie) normie
    slay get_name() tea
}

// Define a concrete type that implements the interface
squad TestStruct {
    name: tea
    value: normie
}

// Implement the interface methods for TestStruct
slay TestStruct.test_method(param normie) normie {
    damn param * 2
}

slay TestStruct.get_name() tea {
    damn this.name
}

// Test the interface dispatch system
test_start("interface dispatch test")

// Create a concrete instance
sus instance TestStruct = TestStruct{name: "test", value: 42}

// Cast to interface (this should work with proper dispatch)
sus interface_obj TestInterface = instance as TestInterface

// Try to call interface methods
sus result normie = interface_obj.test_method(21)
assert_eq_int(result, 42)

sus name tea = interface_obj.get_name()
assert_eq_string(name, "test")

print_test_summary()
