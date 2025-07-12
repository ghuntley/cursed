// mathz module - Pure CURSED mathematical functions
// Provides comprehensive mathematical operations and constants

// Mathematical constants
sus Pi drip = 3.14159265358979323846
sus E drip = 2.71828182845904523536
sus MaxFloat32 drip = 3.4028235e38
sus MaxFloat64 meal = 1.7976931348623157e308
sus MinFloat32 drip = 0.0 - 3.4028235e38
sus MinFloat64 meal = 0.0 - 1.7976931348623157e308
sus Epsilon drip = 1.19209290e-07
sus Tau drip = 6.28318530717958647692

// Absolute value function
slay Abs(x meal) meal {
    bestie x < 0.0 {
        damn 0.0 - x
    }
    damn x
}

// Absolute value for integers
slay AbsInt(x normie) normie {
    bestie x < 0 {
        damn 0 - x
    }
    damn x
}

// Maximum of two values
slay Max(x meal, y meal) meal {
    bestie x > y {
        damn x
    }
    damn y
}

// Minimum of two values
slay Min(x meal, y meal) meal {
    bestie x < y {
        damn x
    }
    damn y
}

// Power function using iterative approach
slay Pow(base meal, exp normie) meal {
    bestie exp == 0 {
        damn 1.0
    }
    
    sus result meal = 1.0
    sus absExp normie = AbsInt(exp)
    sus i normie = 0
    
    bestie i < absExp; i++ {
        result = result * base
    }
    
    bestie exp < 0 {
        damn 1.0 / result
    }
    
    damn result
}

// Square root using Newton's method
slay Sqrt(x meal) meal {
    bestie x < 0.0 {
        damn 0.0  // Return 0 for negative inputs (simplified)
    }
    bestie x == 0.0 {
        damn 0.0
    }
    
    sus guess meal = x / 2.0
    sus prev meal = 0.0
    sus iterations normie = 0
    
    bestie iterations < 10 && Abs(guess - prev) > Epsilon; iterations++ {
        prev = guess
        guess = (guess + x / guess) / 2.0
    }
    
    damn guess
}

// Ceiling function
slay Ceil(x meal) meal {
    sus intPart normie = x.(normie)
    bestie x > intPart.(meal) {
        damn intPart.(meal) + 1.0
    }
    damn intPart.(meal)
}

// Floor function
slay Floor(x meal) meal {
    sus intPart normie = x.(normie)
    bestie x < intPart.(meal) {
        damn intPart.(meal) - 1.0
    }
    damn intPart.(meal)
}

// Rounding function
slay Round(x meal) meal {
    sus floor meal = Floor(x)
    bestie x - floor >= 0.5 {
        damn floor + 1.0
    }
    damn floor
}

// Sign function
slay Sign(x meal) normie {
    bestie x > 0.0 {
        damn 1
    }
    bestie x < 0.0 {
        damn 0 - 1
    }
    damn 0
}

// Clamp function
slay Clamp(x meal, min meal, max meal) meal {
    bestie x < min {
        damn min
    }
    bestie x > max {
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
    bestie n <= 1 {
        damn 1
    }
    
    sus result normie = 1
    sus i normie = 2
    
    bestie i <= n; i++ {
        result = result * i
    }
    
    damn result
}

// Check if number is prime
slay IsPrime(n normie) lit {
    bestie n <= 1 {
        damn cap
    }
    bestie n <= 3 {
        damn based
    }
    bestie n % 2 == 0 || n % 3 == 0 {
        damn cap
    }
    
    sus i normie = 5
    bestie i * i <= n; i = i + 6 {
        bestie n % i == 0 || n % (i + 2) == 0 {
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
