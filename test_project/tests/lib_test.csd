// test_project - Test module
//
// This file contains tests for your CURSED package.
// Run tests with: cursed test

import "test_project/src/lib" as lib;

func test_greet() {
    let result = lib.greet("World");
    assert_eq(result, "Hello, World!");
    puts("✓ test_greet passed");
}

func test_greeter_interface() {
    let greeter = lib.SimpleGreeter{};
    let result = greeter.greet("CURSED");
    assert_eq(result, "Hello, CURSED from SimpleGreeter!");
    puts("✓ test_greeter_interface passed");
}

func main() {
    puts("Running tests for test_project...");
    
    test_greet();
    test_greeter_interface();
    
    puts("All tests passed! 🎉");
}
