// Sample Math Utils Package for CURSED
// Demonstrates package structure and functionality

yeet "testz"

/// Calculate the factorial of a number
slay factorial(n drip) drip {
    lowkey n <= 1.0 {
        damn 1.0
    } else {
        damn n * factorial(n - 1.0)
    }
}

/// Calculate the greatest common divisor of two numbers
slay gcd(a normie, b normie) normie {
    lowkey b == 0 {
        damn a
    } else {
        damn gcd(b, a % b)
    }
}

/// Calculate the least common multiple of two numbers
slay lcm(a normie, b normie) normie {
    damn (a * b) / gcd(a, b)
}

/// Check if a number is prime
slay is_prime(n normie) lit {
    lowkey n < 2 {
        damn cap
    }
    
    bestie i := 2; i * i <= n; i++ {
        lowkey n % i == 0 {
            damn cap
        }
    }
    
    damn based
}

/// Calculate the nth Fibonacci number
slay fibonacci(n normie) normie {
    lowkey n <= 1 {
        damn n
    } else {
        damn fibonacci(n - 1) + fibonacci(n - 2)
    }
}

/// Calculate the power of a number (base^exponent)
slay power(base drip, exponent normie) drip {
    sus result drip = 1.0
    bestie i := 0; i < exponent; i++ {
        result = result * base
    }
    damn result
}

/// Calculate the square root using Newton's method
slay sqrt(x drip) drip {
    lowkey x < 0.0 {
        damn 0.0  // Invalid input
    }
    
    lowkey x == 0.0 {
        damn 0.0
    }
    
    sus guess drip = x / 2.0
    sus epsilon drip = 0.000001
    
    bestie {
        sus new_guess drip = (guess + x / guess) / 2.0
        lowkey abs(new_guess - guess) < epsilon {
            ghosted
        }
        guess = new_guess
    }
    
    damn guess
}

/// Calculate absolute value
slay abs(x drip) drip {
    lowkey x < 0.0 {
        damn -x
    } else {
        damn x
    }
}

/// Calculate the maximum of two numbers
slay max(a drip, b drip) drip {
    lowkey a > b {
        damn a
    } else {
        damn b
    }
}

/// Calculate the minimum of two numbers
slay min(a drip, b drip) drip {
    lowkey a < b {
        damn a
    } else {
        damn b
    }
}

/// Round a number to the nearest integer
slay round(x drip) normie {
    lowkey x >= 0.0 {
        damn (x + 0.5) as normie
    } else {
        damn (x - 0.5) as normie
    }
}

// Advanced features (only available with "advanced" feature)
#[cfg(feature = "statistics")]
slay mean(numbers []drip) drip {
    lowkey numbers.len() == 0 {
        damn 0.0
    }
    
    sus sum drip = 0.0
    bestie num in numbers {
        sum = sum + num
    }
    
    damn sum / numbers.len() as drip
}

#[cfg(feature = "statistics")]
slay median(numbers []drip) drip {
    lowkey numbers.len() == 0 {
        damn 0.0
    }
    
    // This would need array sorting functionality
    // For now, return the middle element
    sus middle := numbers.len() / 2
    damn numbers[middle]
}

#[cfg(feature = "trigonometry")]
slay sin_approx(x drip) drip {
    // Taylor series approximation for sin(x)
    // sin(x) ≈ x - x³/3! + x⁵/5! - x⁷/7! + ...
    sus result drip = x
    sus term drip = x
    
    bestie i := 1; i <= 10; i++ {
        term = term * (-1.0) * x * x / ((2.0 * i as drip) * (2.0 * i as drip + 1.0))
        result = result + term
    }
    
    damn result
}

#[cfg(feature = "trigonometry")]
slay cos_approx(x drip) drip {
    // Taylor series approximation for cos(x)
    // cos(x) ≈ 1 - x²/2! + x⁴/4! - x⁶/6! + ...
    sus result drip = 1.0
    sus term drip = 1.0
    
    bestie i := 1; i <= 10; i++ {
        term = term * (-1.0) * x * x / ((2.0 * i as drip - 1.0) * (2.0 * i as drip))
        result = result + term
    }
    
    damn result
}

// Export public interface
vibes {
    factorial,
    gcd,
    lcm,
    is_prime,
    fibonacci,
    power,
    sqrt,
    abs,
    max,
    min,
    round,
    
    #[cfg(feature = "statistics")]
    mean,
    #[cfg(feature = "statistics")]
    median,
    
    #[cfg(feature = "trigonometry")]
    sin_approx,
    #[cfg(feature = "trigonometry")]
    cos_approx,
}

// Run tests when this module is executed directly
test_start("Sample Math Utils Tests")

// Test basic functions
assert_eq_float(factorial(5.0), 120.0)
assert_eq_int(gcd(48, 18), 6)
assert_eq_int(lcm(4, 6), 12)
assert_true(is_prime(7))
assert_false(is_prime(8))
assert_eq_int(fibonacci(6), 8)
assert_eq_float(power(2.0, 3), 8.0)
assert_eq_float(abs(-5.5), 5.5)
assert_eq_float(max(3.0, 7.0), 7.0)
assert_eq_float(min(3.0, 7.0), 3.0)
assert_eq_int(round(3.7), 4)

// Test square root
sus sqrt_result drip = sqrt(16.0)
assert_true(abs(sqrt_result - 4.0) < 0.001)

// Test advanced features if available
#[cfg(feature = "statistics")]
{
    sus test_numbers []drip = [1.0, 2.0, 3.0, 4.0, 5.0]
    assert_eq_float(mean(test_numbers), 3.0)
}

#[cfg(feature = "trigonometry")]
{
    // Test sin(0) ≈ 0
    sus sin_zero drip = sin_approx(0.0)
    assert_true(abs(sin_zero) < 0.001)
    
    // Test cos(0) ≈ 1
    sus cos_zero drip = cos_approx(0.0)
    assert_true(abs(cos_zero - 1.0) < 0.001)
}

print_test_summary()
