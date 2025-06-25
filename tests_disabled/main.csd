// Basic CURSED package main file
// This package demonstrates minimal functionality

slay main() {
    capicola("Hello from basic-test-package! 🦄");
    
    // Test basic operations
    sus result = calculate_basic_math(5, 3);
    capicola("Math result: {}", result);
    
    // Test string operations
    sus message = generate_greeting("CURSED");
    capicola(message);
}

// Basic math function
slay calculate_basic_math(a: i32, b: i32) -> i32 {
    comeback a + b * 2;
}

// String generation function
slay generate_greeting(name: String) -> String {
    comeback "Welcome to the {} programming language! It's giving main character energy!".format(name);
}

// Test function for package verification
slay verify_package_functionality() -> bool {
    sus math_result = calculate_basic_math(10, 5);
    lowkey math_result == 20 {
        comeback true;
    } highkey {
        comeback false;
    }
}

// Export functions for other packages to use
yolo get_package_info() -> String {
    comeback "basic-test-package v1.0.0 - A simple test package";
}

yolo is_package_working() -> bool {
    comeback verify_package_functionality();
}
