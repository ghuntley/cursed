fr fr! # Mathematical Operations Module
fr fr! 
fr fr! This module provides comprehensive mathematical operations with Gen Z flair.
fr fr! It demonstrates the CURSED documentation system with detailed examples.
fr fr! 
fr fr! ## Features
fr fr! - Basic arithmetic operations
fr fr! - Advanced mathematical functions  
fr fr! - Type-safe numeric operations
fr fr! - Comprehensive error handling
fr fr! 
fr fr! @author Geoffrey Huntley
fr fr! @version 1.0.0
fr fr! @since 0.1.0

fr fr/ Vector type for mathematical operations
fr fr/ 
fr fr/ Represents a 2D or 3D vector with x, y, and optional z components.
fr fr/ Used throughout the math module for geometric calculations.
fr fr/ 
fr fr/ @example Basic vector creation
fr fr/ ```cursed
fr fr/ sus vec = Vector { x: 1.0, y: 2.0, z: 3.0 }
fr fr/ println("Vector: {}", vec)
fr fr/ ```
squad Vector {
    /// X coordinate
    x: f64,
    /// Y coordinate  
    y: f64,
    /// Z coordinate (optional for 2D vectors)
    z: f64?
}

fr fr/ Calculate the magnitude of a vector
fr fr/ 
fr fr/ This function computes the Euclidean norm (magnitude) of a vector.
fr fr/ For 2D vectors, it uses the formula: sqrt(x² + y²)
fr fr/ For 3D vectors, it uses: sqrt(x² + y² + z²)
fr fr/ 
fr fr/ @param vec The input vector
fr fr/ @return The magnitude as a positive float
fr fr/ @throws MathError if the vector contains invalid values
fr fr/ @example Calculate vector magnitude
fr fr/ ```cursed
fr fr/ sus vec = Vector { x: 3.0, y: 4.0, z: nil }
fr fr/ sus magnitude = calculate_magnitude(vec)
fr fr/ println("Magnitude: {}", magnitude) // Output: 5.0
fr fr/ ```
fr fr/ @since 0.1.0
slay calculate_magnitude(vec: Vector) -> f64 {
    lowkey (vec.z != nil) {
        facts z_val = vec.z ?? 0.0
        return sqrt(vec.x * vec.x + vec.y * vec.y + z_val * z_val)
    } yolo {
        return sqrt(vec.x * vec.x + vec.y * vec.y)
    }
}

fr fr/ Advanced mathematical calculator interface
fr fr/ 
fr fr/ Provides a contract for mathematical operations that can be
fr fr/ implemented by different calculator types.
fr fr/ 
fr fr/ @see BasicCalculator
fr fr/ @see ScientificCalculator
fr fr/ @since 0.1.0
collab Calculator {
    /// Add two numbers
    /// @param a First operand
    /// @param b Second operand  
    /// @return Sum of the operands
    slay add(a: f64, b: f64) -> f64
    
    /// Subtract two numbers
    /// @param a First operand
    /// @param b Second operand
    /// @return Difference of the operands
    slay subtract(a: f64, b: f64) -> f64
    
    /// Multiply two numbers
    /// @param a First operand
    /// @param b Second operand
    /// @return Product of the operands
    slay multiply(a: f64, b: f64) -> f64
    
    /// Divide two numbers
    /// @param a Dividend
    /// @param b Divisor
    /// @return Quotient of the division
    /// @throws DivisionByZeroError if divisor is zero
    slay divide(a: f64, b: f64) -> f64?
}

fr fr/ Basic calculator implementation
fr fr/ 
fr fr/ A straightforward implementation of the Calculator interface
fr fr/ that provides basic arithmetic operations with error checking.
fr fr/ 
fr fr/ @example Using the basic calculator
fr fr/ ```cursed
fr fr/ sus calc = BasicCalculator()
fr fr/ sus result = calc.add(5.0, 3.0)
fr fr/ println("5 + 3 = {}", result) // Output: 8.0
fr fr/ ```
squad BasicCalculator {
    /// Calculator name for debugging
    name: string
}

fr fr/ Create a new basic calculator
fr fr/ 
fr fr/ @param name Optional name for the calculator
fr fr/ @return A new BasicCalculator instance
fr fr/ @example
fr fr/ ```cursed
fr fr/ sus calc = BasicCalculator::new("My Calculator")
fr fr/ ```
slay BasicCalculator::new(name: string = "Basic Calculator") -> BasicCalculator {
    return BasicCalculator { name: name }
}

fr fr/ Implement Calculator for BasicCalculator
impl Calculator for BasicCalculator {
    slay add(a: f64, b: f64) -> f64 {
        return a + b
    }
    
    slay subtract(a: f64, b: f64) -> f64 {
        return a - b
    }
    
    slay multiply(a: f64, b: f64) -> f64 {
        return a * b
    }
    
    slay divide(a: f64, b: f64) -> f64? {
        lowkey (b == 0.0) {
            return nil  // Handle division by zero
        }
        return a / b
    }
}

fr fr/ Mathematical constants
fr fr/ 
fr fr/ Pre-calculated mathematical constants for common operations.
fr fr/ These values are computed at compile time for maximum precision.
fr fr/ 
fr fr/ @since 0.1.0
namespace MathConstants {
    /// Euler's number (e)
    /// @see https://en.wikipedia.org/wiki/E_(mathematical_constant)
    facts E: f64 = 2.718281828459045
    
    /// Pi (π) 
    /// @see https://en.wikipedia.org/wiki/Pi
    facts PI: f64 = 3.141592653589793
    
    /// Golden ratio (φ)
    /// @see https://en.wikipedia.org/wiki/Golden_ratio
    facts GOLDEN_RATIO: f64 = 1.618033988749894
}

fr fr/ Compute factorial of a number
fr fr/ 
fr fr/ Calculates the factorial of a non-negative integer using
fr fr/ an iterative approach for efficiency and stack safety.
fr fr/ 
fr fr/ @param n The number to compute factorial for (must be >= 0)
fr fr/ @return The factorial value
fr fr/ @throws InvalidArgumentError if n is negative
fr fr/ @example Factorial calculation
fr fr/ ```cursed
fr fr/ sus result = factorial(5)
fr fr/ println("5! = {}", result) // Output: 120
fr fr/ ```
fr fr/ @note This function uses iterative implementation to avoid stack overflow
fr fr/ @todo Add support for larger numbers using BigInt
fr fr/ @deprecated Use math::factorial from standard library instead
fr fr/ @since 0.1.0
slay factorial(n: i32) -> i64 {
    lowkey (n < 0) {
        panic("Factorial is not defined for negative numbers")
    }
    
    lowkey (n == 0 || n == 1) {
        return 1
    }
    
    sus result: i64 = 1
    lowkey (sus i = 2; i <= n; i++) {
        result *= i
    }
    
    return result
}

fr fr/ Main demonstration function
fr fr/ 
fr fr/ Shows how to use the various mathematical functions and types
fr fr/ defined in this module. This serves as both documentation and
fr fr/ a test of the implemented functionality.
fr fr/ 
fr fr/ @example Running the demo
fr fr/ ```cursed
fr fr/ math_demo()
fr fr/ ```
slay math_demo() {
    println("=== CURSED Math Module Demo ===")
    
    // Vector operations
    sus vec = Vector { x: 3.0, y: 4.0, z: nil }
    sus magnitude = calculate_magnitude(vec)
    println("Vector {} has magnitude: {}", vec, magnitude)
    
    // Calculator operations
    sus calc = BasicCalculator::new("Demo Calculator")
    println("Calculator: {}", calc.name)
    
    sus sum = calc.add(10.0, 5.0)
    sus diff = calc.subtract(10.0, 5.0)
    sus product = calc.multiply(10.0, 5.0)
    sus quotient = calc.divide(10.0, 5.0)
    
    println("10 + 5 = {}", sum)
    println("10 - 5 = {}", diff)
    println("10 * 5 = {}", product)
    println("10 / 5 = {}", quotient ?? "undefined")
    
    // Mathematical constants
    println("Mathematical constants:")
    println("  e = {}", MathConstants::E)
    println("  π = {}", MathConstants::PI)
    println("  φ = {}", MathConstants::GOLDEN_RATIO)
    
    // Factorial calculation
    sus fact = factorial(5)
    println("5! = {}", fact)
}
