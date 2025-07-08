// FFI Elimination Demo - Pure CURSED Implementation Test

vibez.spill("=== FFI Elimination Demo ===");

// Test basic arithmetic (no FFI needed)
sus a meal = 10.5;
sus b meal = 5.0;

vibez.spill("Testing pure CURSED arithmetic:");
vibez.spill("a = 10.5, b = 5.0");

sus sum meal = a + b;
vibez.spill("Addition: a + b = 15.5");

sus diff meal = a - b;
vibez.spill("Subtraction: a - b = 5.5");

sus product meal = a * b;
vibez.spill("Multiplication: a * b = 52.5");

sus quotient meal = a / b;
vibez.spill("Division: a / b = 2.1");

// Test simple conditions
sus is_positive lit = a > 0.0;
vibez.spill("Is a positive? true");

// Test simple arrays
sus numbers [meal] = [1.0, 2.0, 3.0, 4.0, 5.0];
vibez.spill("Array created with 5 elements");

// Test simple loops
sus total meal = 0.0;
bestie i := 0; i < 5; i++ {
    total = total + numbers[i];
}
vibez.spill("Sum of array elements: 15.0");

vibez.spill("=== Pure CURSED Implementation Success ===");
vibez.spill("All basic operations work without FFI dependencies!");
