// mathz module - Pure CURSED mathematical functions
// Provides comprehensive mathematical operations and constants
// Self-contained module for core mathematical operations

// Mathematical constants - using meal (f64) for better precision
sus Pi meal = 3.14159265358979323846
sus E meal = 2.71828182845904523536
sus MaxFloat32 drip = 3.4028235e38
sus MaxFloat64 meal = 1.7976931348623157e308
sus MinFloat32 drip = 0.0 - 3.4028235e38
sus MinFloat64 meal = 0.0 - 1.7976931348623157e308
sus Epsilon meal = 1.19209290e-07
sus Tau meal = 6.28318530717958647692

// Absolute value function
slay Abs(x meal) meal {
    lowkey x < 0.0 {
        damn 0.0 - x
    }
    damn x
}

// Absolute value for integers
slay AbsInt(x normie) normie {
    lowkey x < 0 {
        damn 0 - x
    }
    damn x
}

// Maximum of two values
slay Max(x meal, y meal) meal {
    lowkey x > y {
        damn x
    }
    damn y
}

// Minimum of two values
slay Min(x meal, y meal) meal {
    lowkey x < y {
        damn x
    }
    damn y
}

// Power function using iterative approach
slay Pow(base meal, exp normie) meal {
    lowkey exp == 0 {
        damn 1.0
    }
    
    sus result meal = 1.0
    sus absExp normie = 0
    
    lowkey exp < 0 {
        absExp = 0 - exp
    } lowkey {
        absExp = exp
    }
    
    bestie i := 0; i < absExp; i++ {
        result = result * base
    }
    
    lowkey exp < 0 {
        damn 1.0 / result
    }
    
    damn result
}

// Square root using Newton's method
slay Sqrt(x meal) meal {
    lowkey x < 0.0 {
        damn 0.0  // Return 0 for negative inputs (simplified)
    }
    lowkey x == 0.0 {
        damn 0.0
    }
    
    sus guess meal = x / 2.0
    sus prev meal = 0.0
    
    bestie iterations := 0; iterations < 10; iterations++ {
        prev = guess
        guess = (guess + x / guess) / 2.0
        sus diff meal = Abs(guess - prev)
        lowkey diff <= Epsilon {
            ghosted
        }
    }
    
    damn guess
}

// Ceiling function
slay Ceil(x meal) meal {
    sus intPart normie = x.(normie)
    sus intPartFloat meal = intPart.(meal)
    lowkey x > intPartFloat {
        damn intPartFloat + 1.0
    }
    damn intPartFloat
}

// Floor function
slay Floor(x meal) meal {
    sus intPart normie = x.(normie)
    sus intPartFloat meal = intPart.(meal)
    lowkey x < intPartFloat {
        damn intPartFloat - 1.0
    }
    damn intPartFloat
}

// Rounding function
slay Round(x meal) meal {
    sus floor meal = Floor(x)
    lowkey x - floor >= 0.5 {
        damn floor + 1.0
    }
    damn floor
}

// Sign function
slay Sign(x meal) normie {
    lowkey x > 0.0 {
        damn 1
    }
    lowkey x < 0.0 {
        damn 0 - 1
    }
    damn 0
}

// Clamp function
slay Clamp(x meal, min meal, max meal) meal {
    lowkey x < min {
        damn min
    }
    lowkey x > max {
        damn max
    }
    damn x
}

// Linear interpolation
slay Lerp(a meal, b meal, t meal) meal {
    damn a + t * (b - a)
}

// Radians to degrees conversion
slay RadToDeg(rad meal) meal {
    damn rad * 180.0 / Pi
}

// Degrees to radians conversion
slay DegToRad(deg meal) meal {
    damn deg * Pi / 180.0
}

// Factorial function
slay Factorial(n normie) normie {
    lowkey n <= 1 {
        damn 1
    }
    
    sus result normie = 1
    
    bestie i := 2; i <= n; i++ {
        result = result * i
    }
    
    damn result
}

// Check if number is prime
slay IsPrime(n normie) lit {
    lowkey n <= 1 {
        damn cap
    }
    lowkey n <= 3 {
        damn based
    }
    lowkey n % 2 == 0 || n % 3 == 0 {
        damn cap
    }
    
    bestie i := 5; (i * i) <= n; i = i + 6 {
        lowkey n % i == 0 || n % (i + 2) == 0 {
            damn cap
        }
    }
    
    damn based
}

// Calculate hypotenuse
slay Hypot(x meal, y meal) meal {
    damn Sqrt(x * x + y * y)
}

// Distance between two points
slay Distance(x1 meal, y1 meal, x2 meal, y2 meal) meal {
    sus dx meal = x2 - x1
    sus dy meal = y2 - y1
    damn Sqrt(dx * dx + dy * dy)
}
