// mathz module - Pure CURSED mathematical functions
// Provides comprehensive mathematical operations and constants
// Self-contained module for core mathematical operations

// Mathematical constants - using meal (f64) for better precision
sus Pi meal = 3.14159265358979323846
sus E meal = 2.71828182845904523536
sus MaxFloat32 drip = 34028235000000000000000000000000000000.0
sus MaxFloat64 meal = 179769313486231570000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000.0
sus MinFloat32 drip = 0.0 - 34028235000000000000000000000000000000.0
sus MinFloat64 meal = 0.0 - 179769313486231570000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000.0
sus Epsilon meal = 0.00000011920929
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
    } lowkey exp >= 0 {
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

vibez.spill("Mathz first 70 lines test passed")
