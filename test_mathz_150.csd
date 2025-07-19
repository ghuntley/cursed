# mathz - Comprehensive Mathematics Module for CURSED
# Pure CURSED implementation without FFI dependencies
# Migrated from Rust stdlib/math modules

yeet "testz"

# Mathematical Constants
sus PI meal = 3.141592653589793
sus E meal = 2.718281828459045
sus TAU meal = 6.283185307179586
sus SQRT_2 meal = 1.4142135623730951
sus SQRT_3 meal = 1.7320508075688772
sus LN_2 meal = 0.6931471805599453
sus LN_10 meal = 2.302585092994046
sus LOG2_E meal = 1.4426950408889634
sus LOG10_E meal = 0.4342944819032518
sus GOLDEN_RATIO meal = 1.618033988749895
sus EULER_MASCHERONI meal = 0.5772156649015329
sus DEGREES_TO_RADIANS meal = 0.017453292519943295
sus RADIANS_TO_DEGREES meal = 57.29577951308232
sus EPSILON meal = 0.00000000000000022204460492503131

# Basic Arithmetic Operations
slay math_add(a meal, b meal) meal {
    damn a + b
}

slay math_subtract(a meal, b meal) meal {
    damn a - b
}

slay math_multiply(a meal, b meal) meal {
    damn a * b
}

slay math_divide(a meal, b meal) meal {
    lowkey b == 0.0 {
        damn 0.0  # Return 0 for division by zero (safe fallback)
    }
    damn a / b
}

# Absolute Value Functions
slay abs_meal(x meal) meal {
    lowkey x < 0.0 {
        damn -x
    }
    damn x
}

slay abs_normie(x normie) normie {
    lowkey x < 0 {
        damn -x
    }
    damn x
}

# Min/Max Functions
slay max_meal(a meal, b meal) meal {
    lowkey a > b {
        damn a
    }
    damn b
}

slay max_normie(a normie, b normie) normie {
    lowkey a > b {
        damn a
    }
    damn b
}

slay min_meal(a meal, b meal) meal {
    lowkey a < b {
        damn a
    }
    damn b
}

slay min_normie(a normie, b normie) normie {
    lowkey a < b {
        damn a
    }
    damn b
}

# Floor, Ceiling, and Rounding
slay floor_meal(x meal) normie {
    sus result normie = 0
    lowkey x >= 0.0 {
        # For positive numbers, truncate
        result = x  # Implicit conversion to int truncates
    } {
        # For negative numbers, subtract 1 if not exact
        result = x  # Truncate
        lowkey x < result {
            result = result - 1
        }
    }
    damn result
}

slay ceil_meal(x meal) normie {
    sus result normie = 0
    lowkey x <= 0.0 {
        # For negative numbers, truncate
        result = x  # Implicit conversion to int truncates
    } {
        # For positive numbers, add 1 if not exact
        result = x  # Truncate
        lowkey x > result {
            result = result + 1
        }
    }
    damn result
}

slay round_meal(x meal) normie {
    lowkey x >= 0.0 {
        damn floor_meal(x + 0.5)
    } {
        damn ceil_meal(x - 0.5)
    }
}

# Power Functions
slay pow_meal(base meal, exp normie) meal {
    lowkey exp == 0 {
        damn 1.0
    }
    lowkey exp == 1 {
        damn base
    }
    lowkey exp < 0 {
        damn 1.0 / pow_meal(base, -exp)
    }
    
    sus result meal = 1.0
    sus i normie = 0
    bestie (i < exp) {
        result = result * base
        i = i + 1
    }
    damn result
}

slay pow_meal_meal(base meal, exp meal) meal {
    lowkey abs_meal(exp - round_meal(exp)) < EPSILON {
        # If exponent is effectively an integer
        damn pow_meal(base, round_meal(exp))
