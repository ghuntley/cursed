fr fr Real LLVM compilation test program
fr fr This program tests the actual LLVM IR generation capabilities

package real_llvm_test;

fr fr Simple function that adds two numbers
slay add(a: normie, b: normie) -> normie {
    yolo a + b;
}

fr fr Main function that demonstrates basic functionality
slay main() -> normie {
    sus x = 42;
    sus y = 13;
    sus result = add(x, y);
    yolo result;
}

fr fr Function with string handling
slay greet(name: tea) -> tea {
    yolo "Hello, " + name + "!";
}

fr fr Function with boolean logic
slay is_positive(num: normie) -> facts {
    yolo num > 0;
}

fr fr Function demonstrating control flow
slay max(a: normie, b: normie) -> normie {
    lowkey (a > b) {
        yolo a;
    } highkey {
        yolo b;
    }
}

fr fr Function with floating point arithmetic
slay calculate_area(radius: vibes) -> vibes {
    sus pi = 3.14159;
    yolo pi * radius * radius;
}
