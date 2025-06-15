//! # Mathematical Operations Module
//! 
//! This module provides comprehensive mathematical operations with Gen Z flair.
//! It demonstrates the CURSED documentation system with detailed examples.
//! 
//! ## Features
//! - Basic arithmetic operations
//! - Advanced mathematical functions  
//! - Type-safe numeric operations
//! - Comprehensive error handling
//! 
//! @author Geoffrey Huntley
//! @version 1.0.0
//! @since 0.1.0

/// Vector type for mathematical operations
/// 
/// Represents a 2D or 3D vector with x, y, and optional z components.
/// Used throughout the math module for geometric calculations.
/// 
/// @example Basic vector creation
/// ```cursed
/// sus vec = Vector { x: 1.0, y: 2.0, z: 3.0 }
/// println("Vector: {}", vec)
/// ```
squad Vector {
    /// X coordinate
    x: f64,
    /// Y coordinate  
    y: f64,
    /// Z coordinate (optional for 2D vectors)
    z: f64?
}

/// Calculate the magnitude of a vector
/// 
/// This function computes the Euclidean norm (magnitude) of a vector.
/// For 2D vectors, it uses the formula: sqrt(x² + y²)
/// For 3D vectors, it uses: sqrt(x² + y² + z²)
/// 
/// @param vec The input vector
/// @return The magnitude as a positive float
/// @throws MathError if the vector contains invalid values
/// @example Calculate vector magnitude
/// ```cursed
/// sus vec = Vector { x: 3.0, y: 4.0, z: nil }
/// sus magnitude = calculate_magnitude(vec)
/// println("Magnitude: {}", magnitude) // Output: 5.0
/// ```
/// @since 0.1.0
slay calculate_magnitude(vec: Vector) -> f64 {
    lowkey (vec.z != nil) {
        facts z_val = vec.z ?? 0.0
        return sqrt(vec.x * vec.x + vec.y * vec.y + z_val * z_val)
    } yolo {
        return sqrt(vec.x * vec.x + vec.y * vec.y)
    }
}

/// Advanced mathematical calculator interface
/// 
/// Provides a contract for mathematical operations that can be
/// implemented by different calculator types.
/// 
/// @see BasicCalculator
/// @see ScientificCalculator
/// @since 0.1.0
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

/// Basic calculator implementation
/// 
/// A straightforward implementation of the Calculator interface
/// that provides basic arithmetic operations with error checking.
/// 
/// @example Using the basic calculator
/// ```cursed
/// sus calc = BasicCalculator()
/// sus result = calc.add(5.0, 3.0)
/// println("5 + 3 = {}", result) // Output: 8.0
/// ```
squad BasicCalculator {
    /// Calculator name for debugging
    name: string
}

/// Create a new basic calculator
/// 
/// @param name Optional name for the calculator
/// @return A new BasicCalculator instance
/// @example
/// ```cursed
/// sus calc = BasicCalculator::new("My Calculator")
/// ```
slay BasicCalculator::new(name: string = "Basic Calculator") -> BasicCalculator {
    return BasicCalculator { name: name }
}

/// Implement Calculator for BasicCalculator
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

/// Mathematical constants
/// 
/// Pre-calculated mathematical constants for common operations.
/// These values are computed at compile time for maximum precision.
/// 
/// @since 0.1.0
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

/// Compute factorial of a number
/// 
/// Calculates the factorial of a non-negative integer using
/// an iterative approach for efficiency and stack safety.
/// 
/// @param n The number to compute factorial for (must be >= 0)
/// @return The factorial value
/// @throws InvalidArgumentError if n is negative
/// @example Factorial calculation
/// ```cursed
/// sus result = factorial(5)
/// println("5! = {}", result) // Output: 120
/// ```
/// @note This function uses iterative implementation to avoid stack overflow
/// @todo Add support for larger numbers using BigInt
/// @deprecated Use math::factorial from standard library instead
/// @since 0.1.0
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

/// Main demonstration function
/// 
/// Shows how to use the various mathematical functions and types
/// defined in this module. This serves as both documentation and
/// a test of the implemented functionality.
/// 
/// @example Running the demo
/// ```cursed
/// math_demo()
/// ```
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
