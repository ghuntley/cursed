sus main() -> std_int {
    yap("=== Parser Comprehensive Test ===");
    
    // Variable declarations
    sus integer_val drip = 42;
    sus string_val drip = "Hello World";
    sus float_val drip = 3.14;
    
    // Binary expressions
    sus math_result drip = integer_val + 10;
    sus comparison_result drip = integer_val > 30;
    
    // Function calls
    yap("Testing function calls");
    yap(string_val);
    
    // Method calls (if stdlib is available)
    sus stdlib_result drip = mathz.add_two(5, 3);
    
    damn 0;
}
