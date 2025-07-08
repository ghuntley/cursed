// Demo of pure CURSED math implementation working

slay main() {
    vibez.spill("=== Pure CURSED Math Demo ===");
    
    # Test constants
    vibez.spill("Testing constants:");
    vibez.spill("PI = ");
    vibez.spill(math_pi());
    vibez.spill("E = ");
    vibez.spill(math_e());
    
    # Test basic operations
    vibez.spill("\nTesting basic operations:");
    vibez.spill("abs(-10) = ");
    vibez.spill(math_abs(-10.0));
    
    vibez.spill("min(5, 3) = ");
    vibez.spill(math_min(5.0, 3.0));
    
    vibez.spill("max(5, 3) = ");
    vibez.spill(math_max(5.0, 3.0));
    
    # Test power functions
    vibez.spill("\nTesting power functions:");
    vibez.spill("sqrt(16) = ");
    vibez.spill(math_sqrt(16.0));
    
    vibez.spill("pow(2, 3) = ");
    vibez.spill(math_pow(2.0, 3.0));
    
    # Test trigonometric functions
    vibez.spill("\nTesting trigonometric functions:");
    vibez.spill("sin(0) = ");
    vibez.spill(math_sin(0.0));
    
    vibez.spill("cos(0) = ");
    vibez.spill(math_cos(0.0));
    
    # Test rounding functions
    vibez.spill("\nTesting rounding functions:");
    vibez.spill("floor(3.7) = ");
    vibez.spill(math_floor(3.7));
    
    vibez.spill("ceil(3.2) = ");
    vibez.spill(math_ceil(3.2));
    
    vibez.spill("round(3.6) = ");
    vibez.spill(math_round(3.6));
    
    # Test utility functions
    vibez.spill("\nTesting utility functions:");
    vibez.spill("gcd(48, 18) = ");
    vibez.spill(math_gcd(48, 18));
    
    vibez.spill("factorial(5) = ");
    vibez.spill(math_factorial(5));
    
    vibez.spill("fibonacci(10) = ");
    vibez.spill(math_fibonacci(10));
    
    # Test geometry functions
    vibez.spill("\nTesting geometry functions:");
    vibez.spill("distance_2d(0,0,3,4) = ");
    vibez.spill(math_distance_2d(0.0, 0.0, 3.0, 4.0));
    
    vibez.spill("\n=== All pure CURSED math functions working! ===");
}
