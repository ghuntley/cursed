// Basic Standard Library Test
// Testing core functionality without imports

fn test_basic_operations() {
    println("=== Basic Operations Test ===");
    
    // Test basic printing
    println("Hello from Cursed!");
    
    // Test basic arithmetic
    let a = 10;
    let b = 5;
    let sum = a + b;
    let product = a * b;
    
    println("10 + 5 = " + to_string(sum));
    println("10 * 5 = " + to_string(product));
    
    // Test string operations
    let text = "Hello, World!";
    println("Text: " + text);
    
    // Test basic conditionals
    if (sum > 10) {
        println("Sum is greater than 10");
    }
    
    println("✓ Basic operations working");
}

fn test_collections() {
    println("=== Collections Test ===");
    
    // Test arrays (if available)
    let arr = [];
    println("Empty array created");
    
    // Test basic loops
    let i = 0;
    while (i < 3) {
        println("Loop iteration: " + to_string(i));
        i = i + 1;
    }
    
    println("✓ Basic collections working");
}

fn test_functions() {
    println("=== Functions Test ===");
    
    fn add(x, y) {
        return x + y;
    }
    
    let result = add(20, 30);
    println("Function result: " + to_string(result));
    
    println("✓ Functions working");
}

fn main() {
    println("Cursed Standard Library Basic Test Suite");
    println("=======================================");
    
    test_basic_operations();
    test_collections();
    test_functions();
    
    println("\n=== Summary ===");
    println("✓ Basic language features working");
    println("! Import system needs investigation");
    println("! Stdlib modules need runtime integration");
}
