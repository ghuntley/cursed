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
    }
    
    # For fractional exponents, use approximation
    lowkey base <= 0.0 {
        damn 0.0  # Safe fallback for negative bases
    }
    
    # Approximate implementation using exp(ln(base) * exp)
    sus ln_base meal = ln_meal(base)
    sus product meal = ln_base * exp
    damn exp_meal(product)
}

# Square Root (Newton's Method)
slay sqrt_meal(x meal) meal {
    lowkey x < 0.0 {
        damn 0.0  # Return 0 for negative input (safe fallback)
    }
    lowkey x == 0.0 {
        damn 0.0
    }
    
    sus guess meal = x / 2.0
    sus prev meal = 0.0
    sus diff meal = 1.0
    sus iterations normie = 0
    
    bestie (diff > EPSILON && iterations < 100) {
        prev = guess
        guess = (guess + (x / guess)) / 2.0
        diff = abs_meal(guess - prev)
        iterations = iterations + 1
    }
    
    damn guess
}

# Natural Logarithm (Taylor Series Approximation)
slay ln_meal(x meal) meal {
    lowkey x <= 0.0 {
        damn 0.0  # Safe fallback for non-positive input
    }
    lowkey x == 1.0 {
        damn 0.0
    }
    
    # Use ln(x) = 2 * sum((x-1)/(x+1))^(2n+1) / (2n+1))
    sus y meal = (x - 1.0) / (x + 1.0)
    sus y_squared meal = y * y
    sus term meal = y
    sus result meal = y
    sus n normie = 1
    
    bestie (n < 50 && abs_meal(term) > EPSILON) {
        term = term * y_squared / (2.0 * n + 1.0)
        result = result + term
        n = n + 1
    }
    
    damn 2.0 * result
}

# Exponential Function (Taylor Series)
slay exp_meal(x meal) meal {
    lowkey x == 0.0 {
        damn 1.0
    }
    
    sus result meal = 1.0
    sus term meal = 1.0
    sus n normie = 1
    
    bestie (n < 50 && abs_meal(term) > EPSILON) {
        term = term * x / n
        result = result + term
        n = n + 1
    }
    
    damn result
}

# Trigonometric Functions (Taylor Series)
slay sin_meal(x meal) meal {
    # Normalize angle to [-2π, 2π]
    sus normalized meal = x
    bestie (normalized > TAU) {
        normalized = normalized - TAU
    }
    bestie (normalized < -TAU) {
        normalized = normalized + TAU
    }
    
    sus result meal = normalized
    sus term meal = normalized
    sus x_squared meal = normalized * normalized
    sus n normie = 1
    
    bestie (n < 20 && abs_meal(term) > EPSILON) {
        term = -term * x_squared / ((2.0 * n) * (2.0 * n + 1.0))
        result = result + term
        n = n + 1
    }
    
    damn result
}

slay cos_meal(x meal) meal {
    # cos(x) = sin(π/2 - x)
    damn sin_meal(PI / 2.0 - x)
}

slay tan_meal(x meal) meal {
    sus cos_val meal = cos_meal(x)
    lowkey abs_meal(cos_val) < EPSILON {
        damn 0.0  # Safe fallback for division by zero
    }
    damn sin_meal(x) / cos_val
}

# Degree Conversion Functions
slay sin_deg(degrees meal) meal {
    damn sin_meal(degrees * DEGREES_TO_RADIANS)
}

slay cos_deg(degrees meal) meal {
    damn cos_meal(degrees * DEGREES_TO_RADIANS)
}

slay tan_deg(degrees meal) meal {
    damn tan_meal(degrees * DEGREES_TO_RADIANS)
}

# Angle Normalization
slay normalize_radians(angle meal) meal {
    sus result meal = angle
    bestie (result > TAU) {
        result = result - TAU
    }
    bestie (result < 0.0) {
        result = result + TAU
    }
    damn result
}

slay normalize_degrees(angle meal) meal {
    sus result meal = angle
    bestie (result > 360.0) {
        result = result - 360.0
    }
    bestie (result < 0.0) {
        result = result + 360.0
    }
    damn result
}

# Utility Functions
slay is_approximately_equal(a meal, b meal, epsilon meal) lit {
    damn abs_meal(a - b) < epsilon
}

slay is_zero(x meal) lit {
    damn abs_meal(x) < EPSILON
}

slay is_positive_meal(x meal) lit {
    damn x > 0.0
}

slay is_negative_meal(x meal) lit {
    damn x < 0.0
}

slay is_even(x normie) lit {
    damn (x % 2) == 0
}

slay is_odd(x normie) lit {
    damn (x % 2) == 1
}

# Factorial Function
slay factorial(n normie) normie {
    lowkey n <= 1 {
        damn 1
    }
    sus result normie = 1
    sus i normie = 2
    bestie (i <= n) {
        result = result * i
        i = i + 1
    }
    damn result
}

# Greatest Common Divisor
slay gcd(a normie, b normie) normie {
    sus x normie = abs_normie(a)
    sus y normie = abs_normie(b)
    
    bestie (y != 0) {
        sus temp normie = y
        y = x % y
        x = temp
    }
    
    damn x
}

# Least Common Multiple
slay lcm(a normie, b normie) normie {
    lowkey a == 0 || b == 0 {
        damn 0
    }
    damn abs_normie(a * b) / gcd(a, b)
}

# Random Number Generation (Linear Congruential Generator)
sus random_seed normie = 1

slay set_random_seed(seed normie) cringe {
    random_seed = seed
}

slay random_int() normie {
    random_seed = (random_seed * 1103515245 + 12345) % 2147483648
    damn random_seed
}

slay random_meal() meal {
    damn random_int() / 2147483647.0
}

slay random_range(min_val normie, max_val normie) normie {
    lowkey min_val >= max_val {
        damn min_val
    }
    sus range normie = max_val - min_val
    damn min_val + (random_int() % range)
}

# Statistics Functions
slay mean_array(values []meal, count normie) meal {
    lowkey count == 0 {
        damn 0.0
    }
    
    sus sum meal = 0.0
    sus i normie = 0
    bestie (i < count) {
        sum = sum + values[i]
        i = i + 1
    }
    
    damn sum / count
}

slay sum_array(values []meal, count normie) meal {
    sus sum meal = 0.0
    sus i normie = 0
    bestie (i < count) {
        sum = sum + values[i]
        i = i + 1
    }
    damn sum
}

slay max_array(values []meal, count normie) meal {
    lowkey count == 0 {
        damn 0.0
    }
    
    sus max_val meal = values[0]
    sus i normie = 1
    bestie (i < count) {
        lowkey values[i] > max_val {
            max_val = values[i]
        }
        i = i + 1
    }
    damn max_val
}

slay min_array(values []meal, count normie) meal {
    lowkey count == 0 {
        damn 0.0
    }
    
    sus min_val meal = values[0]
    sus i normie = 1
    bestie (i < count) {
        lowkey values[i] < min_val {
            min_val = values[i]
        }
        i = i + 1
    }
    damn min_val
}

# Complex number support (basic operations)
collab Complex {
    real meal
    imag meal
}

slay complex_new(real meal, imag meal) Complex {
    damn Complex{real: real, imag: imag}
}

slay complex_add(a Complex, b Complex) Complex {
    damn Complex{real: a.real + b.real, imag: a.imag + b.imag}
}

slay complex_multiply(a Complex, b Complex) Complex {
    sus real_part meal = a.real * b.real - a.imag * b.imag
    sus imag_part meal = a.real * b.imag + a.imag * b.real
    damn Complex{real: real_part, imag: imag_part}
}

slay complex_magnitude(c Complex) meal {
    damn sqrt_meal(c.real * c.real + c.imag * c.imag)
}

# Advanced Mathematical Functions
slay fibonacci(n normie) normie {
    lowkey n <= 1 {
        damn n
    }
    
    sus a normie = 0
    sus b normie = 1
    sus i normie = 2
    
    bestie (i <= n) {
        sus temp normie = a + b
        a = b
        b = temp
        i = i + 1
    }
    
    damn b
}

# Matrix operations (basic 2x2)
collab Matrix2x2 {
    a00 meal
    a01 meal
    a10 meal
    a11 meal
}

slay matrix_new(a00 meal, a01 meal, a10 meal, a11 meal) Matrix2x2 {
    damn Matrix2x2{a00: a00, a01: a01, a10: a10, a11: a11}
}

slay matrix_add(m1 Matrix2x2, m2 Matrix2x2) Matrix2x2 {
    damn Matrix2x2{
        a00: m1.a00 + m2.a00,
        a01: m1.a01 + m2.a01,
        a10: m1.a10 + m2.a10,
        a11: m1.a11 + m2.a11
    }
}

slay matrix_multiply(m1 Matrix2x2, m2 Matrix2x2) Matrix2x2 {
    damn Matrix2x2{
        a00: m1.a00 * m2.a00 + m1.a01 * m2.a10,
        a01: m1.a00 * m2.a01 + m1.a01 * m2.a11,
        a10: m1.a10 * m2.a00 + m1.a11 * m2.a10,
        a11: m1.a10 * m2.a01 + m1.a11 * m2.a11
    }
}

slay matrix_determinant(m Matrix2x2) meal {
    damn m.a00 * m.a11 - m.a01 * m.a10
}
