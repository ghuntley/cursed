// Real LLVM compilation test program
// This program tests the actual LLVM IR generation capabilities

package real_llvm_test;

// Simple function that adds two numbers
slay add(a: normie, b: normie) -> normie {
    yolo a + b;
}

// Main function that demonstrates basic functionality
slay main() -> normie {
    sus x = 42;
    sus y = 13;
    sus result = add(x, y);
    yolo result;
}

// Function with string handling
slay greet(name: tea) -> tea {
    yolo "Hello, " + name + "!";
}

// Function with boolean logic
slay is_positive(num: normie) -> facts {
    yolo num > 0;
}

// Function demonstrating control flow
slay max(a: normie, b: normie) -> normie {
    lowkey (a > b) {
        yolo a;
    } highkey {
        yolo b;
    }
}

// Function with floating point arithmetic
slay calculate_area(radius: vibes) -> vibes {
    sus pi = 3.14159;
    yolo pi * radius * radius;
}
