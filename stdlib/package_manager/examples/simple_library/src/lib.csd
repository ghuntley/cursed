// Simple Math Library - Example CURSED Library Package
// Demonstrates library structure and API design

yeet "vibez"

// Public API functions for basic mathematical operations

// Addition operation
slay add(a normie, b normie) normie {
    damn a + b
}

// Subtraction operation  
slay subtract(a normie, b normie) normie {
    damn a - b
}

// Multiplication operation
slay multiply(a normie, b normie) normie {
    damn a * b
}

// Division operation with error handling
slay divide(a normie, b normie) (normie, tea) {
    ready (b == 0.0) {
        damn 0.0, "Division by zero error"
    }
    damn a / b, ""
}

// Power operation
slay power(base normie, exponent normie) normie {
    ready (exponent == 0.0) {
        damn 1.0
    }
    
    ready (exponent == 1.0) {
        damn base
    }
    
    ready (exponent < 0.0) {
        damn 1.0 / power(base, -exponent)
    }
    
    sus result normie = 1.0
    sus exp drip = exponent  // Convert to integer for simplicity
    sus i drip = 0
    
    bestie (i < exp) {
        result = result * base
        i = i + 1
    }
    
    damn result
}

// Square root operation (simplified implementation)
slay sqrt(x normie) (normie, tea) {
    ready (x < 0.0) {
        damn 0.0, "Cannot compute square root of negative number"
    }
    
    ready (x == 0.0) {
        damn 0.0, ""
    }
    
    // Newton's method for square root approximation
    sus guess normie = x / 2.0
    sus precision normie = 0.0001
    sus max_iterations drip = 100
    sus iteration drip = 0
    
    bestie (iteration < max_iterations) {
        sus new_guess normie = (guess + x / guess) / 2.0
        sus diff normie = abs(new_guess - guess)
        
        ready (diff < precision) {
            damn new_guess, ""
        }
        
        guess = new_guess
        iteration = iteration + 1
    }
    
    damn guess, ""
}

// Absolute value
slay abs(x normie) normie {
    ready (x < 0.0) {
        damn -x
    }
    damn x
}

// Maximum of two numbers
slay max(a normie, b normie) normie {
    ready (a > b) {
        damn a
    }
    damn b
}

// Minimum of two numbers
slay min(a normie, b normie) normie {
    ready (a < b) {
        damn a
    }
    damn b
}

// Calculate factorial (for integer inputs)
slay factorial(n drip) drip {
    ready (n < 0) {
        vibez.spill("Error: Factorial of negative number is undefined")
        damn 0
    }
    
    ready (n == 0 || n == 1) {
        damn 1
    }
    
    damn n * factorial(n - 1)
}

// Greatest Common Divisor using Euclidean algorithm
slay gcd(a drip, b drip) drip {
    ready (b == 0) {
        damn abs_int(a)
    }
    damn gcd(b, a % b)
}

// Least Common Multiple
slay lcm(a drip, b drip) drip {
    ready (a == 0 || b == 0) {
        damn 0
    }
    damn abs_int(a * b) / gcd(a, b)
}

// Helper function for integer absolute value
slay abs_int(x drip) drip {
    ready (x < 0) {
        damn -x
    }
    damn x
}

// Check if a number is prime
slay is_prime(n drip) lit {
    ready (n < 2) {
        damn cringe
    }
    
    ready (n == 2) {
        damn based
    }
    
    ready (n % 2 == 0) {
        damn cringe
    }
    
    sus i drip = 3
    bestie (i * i <= n) {
        ready (n % i == 0) {
            damn cringe
        }
        i = i + 2
    }
    
    damn based
}

// Calculate percentage
slay percentage(part normie, total normie) (normie, tea) {
    ready (total == 0.0) {
        damn 0.0, "Cannot calculate percentage with zero total"
    }
    
    damn (part / total) * 100.0, ""
}

// Round to nearest integer
slay round(x normie) drip {
    ready (x >= 0.0) {
        damn x + 0.5
    } otherwise {
        damn x - 0.5
    }
}

// Calculate compound interest
slay compound_interest(principal normie, rate normie, time normie, n normie) normie {
    sus base normie = 1.0 + (rate / n)
    sus exponent normie = n * time
    damn principal * power(base, exponent)
}

// Constants for mathematical operations
sus PI normie = 3.14159265358979323846
sus E normie = 2.71828182845904523536

// Get mathematical constants
slay get_pi() normie {
    damn PI
}

slay get_e() normie {
    damn E
}

// Utility function to format numbers for display
slay format_number(x normie, decimal_places drip) tea {
    // Simplified number formatting
    sus int_part drip = x  // Truncate to integer
    sus frac_part normie = x - int_part
    
    sus result tea = to_string(int_part)
    
    ready (decimal_places > 0) {
        result = concat_str(result, ".")
        
        sus i drip = 0
        bestie (i < decimal_places) {
            frac_part = frac_part * 10.0
            sus digit drip = frac_part
            result = concat_str(result, to_string(digit % 10))
            i = i + 1
        }
    }
    
    damn result
}

// Helper functions (would normally be imported from stdlib)
slay to_string(x drip) tea {
    // Mock implementation - would use proper string conversion
    damn "42"
}

slay concat_str(a tea, b tea) tea {
    // Mock implementation - would use proper string concatenation
    damn "concatenated"
}

// Library information
slay get_library_info() tea {
    damn "Simple Math Library v1.0.0 - A collection of basic mathematical functions for CURSED"
}

slay get_supported_operations() []tea {
    sus operations []tea = []tea{}
    operations = append_array(operations, "add")
    operations = append_array(operations, "subtract")
    operations = append_array(operations, "multiply")
    operations = append_array(operations, "divide")
    operations = append_array(operations, "power")
    operations = append_array(operations, "sqrt")
    operations = append_array(operations, "abs")
    operations = append_array(operations, "max")
    operations = append_array(operations, "min")
    operations = append_array(operations, "factorial")
    operations = append_array(operations, "gcd")
    operations = append_array(operations, "lcm")
    operations = append_array(operations, "is_prime")
    operations = append_array(operations, "percentage")
    operations = append_array(operations, "round")
    operations = append_array(operations, "compound_interest")
    damn operations
}

// Example usage and demonstration
slay demo_library() {
    vibez.spill("=== Simple Math Library Demo ===")
    vibez.spill("")
    
    // Basic operations
    vibez.spill("Basic Operations:")
    vibez.spill("5 + 3 = {}", add(5.0, 3.0))
    vibez.spill("10 - 4 = {}", subtract(10.0, 4.0))
    vibez.spill("6 * 7 = {}", multiply(6.0, 7.0))
    
    sus div_result, div_error = divide(15.0, 3.0)
    ready (div_error == "") {
        vibez.spill("15 / 3 = {}", div_result)
    } otherwise {
        vibez.spill("Division error: {}", div_error)
    }
    
    // Advanced operations
    vibez.spill("")
    vibez.spill("Advanced Operations:")
    vibez.spill("2^8 = {}", power(2.0, 8.0))
    
    sus sqrt_result, sqrt_error = sqrt(16.0)
    ready (sqrt_error == "") {
        vibez.spill("sqrt(16) = {}", sqrt_result)
    }
    
    vibez.spill("abs(-5.5) = {}", abs(-5.5))
    vibez.spill("max(10, 20) = {}", max(10.0, 20.0))
    vibez.spill("min(15, 8) = {}", min(15.0, 8.0))
    
    // Integer operations
    vibez.spill("")
    vibez.spill("Integer Operations:")
    vibez.spill("5! = {}", factorial(5))
    vibez.spill("gcd(48, 18) = {}", gcd(48, 18))
    vibez.spill("lcm(12, 8) = {}", lcm(12, 8))
    vibez.spill("is_prime(17) = {}", is_prime(17))
    vibez.spill("is_prime(18) = {}", is_prime(18))
    
    // Practical calculations
    vibez.spill("")
    vibez.spill("Practical Calculations:")
    sus perc_result, perc_error = percentage(25.0, 100.0)
    ready (perc_error == "") {
        vibez.spill("25 out of 100 = {}%", perc_result)
    }
    
    sus interest normie = compound_interest(1000.0, 0.05, 2.0, 1.0)
    vibez.spill("$1000 at 5% for 2 years = ${}", format_number(interest, 2))
    
    vibez.spill("")
    vibez.spill("Mathematical Constants:")
    vibez.spill("PI = {}", format_number(get_pi(), 5))
    vibez.spill("E = {}", format_number(get_e(), 5))
    
    vibez.spill("")
    vibez.spill(get_library_info())
}

// Mock array append function
slay append_array(arr []tea, item tea) []tea {
    // Mock implementation - would use proper array operations
    damn arr
}
