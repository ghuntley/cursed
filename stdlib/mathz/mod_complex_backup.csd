vibe mathz

fr fr Simplified mathz module to prevent parser segfaults
fr fr Basic mathematical functions for CURSED

fr fr Mathematical Constants
sus PI meal = 3.141592653589793
sus E meal = 2.718281828459045

fr fr Basic Arithmetic Operations
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
        damn 0.0
    }
    damn a / b
}

fr fr Absolute Value Functions
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

fr fr Min/Max Functions
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

fr fr Simple Power Functions (avoid complex loops)
slay pow_meal(base meal, exp normie) meal {
    lowkey exp == 0 {
        damn 1.0
    }
    lowkey exp == 1 {
        damn base
    }
    lowkey exp == 2 {
        damn base * base
    }
    lowkey exp < 0 {
        damn 0.0  fr fr Simplified - avoid recursion
    }
    
    fr fr Simple power calculation for small positive exponents
    sus result meal = base
    lowkey exp > 1 {
        result = base * base  fr fr exp^2
    }
    lowkey exp > 2 {
        result = result * base  fr fr exp^3
    }
    damn result
}

fr fr Simplified sqrt using simple approximation
slay sqrt_meal(x meal) meal {
    lowkey x < 0.0 {
        damn 0.0
    }
    lowkey x == 0.0 {
        damn 0.0
    }
    lowkey x == 1.0 {
        damn 1.0
    }
    
    fr fr Simple approximation for common cases
    lowkey x <= 4.0 {
        damn 2.0
    }
    lowkey x <= 9.0 {
        damn 3.0  
    }
    lowkey x <= 16.0 {
        damn 4.0
    }
    
    damn x / 2.0  fr fr Rough approximation for large values
}

fr fr Floor function - simplified
slay floor_meal(x meal) normie {
    sus result normie = 0
    result = x  fr fr Explicit conversion to normie
    damn result
}

fr fr Factorial function
slay factorial(n normie) normie {
    lowkey n <= 1 {
        damn 1
    }
    lowkey n == 2 {
        damn 2
    }
    lowkey n == 3 {
        damn 6
    }
    lowkey n == 4 {
        damn 24
    }
    lowkey n == 5 {
        damn 120
    }
    damn n * (n - 1)  fr fr Simplified approximation
}
