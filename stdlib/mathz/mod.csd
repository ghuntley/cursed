yeet "testz"

fr fr Mathematical Constants
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
        damn 0.0 fr fr Return 0 for division by zero (safe fallback)
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

fr fr Floor, Ceiling, and Rounding
slay floor_meal(x meal) normie {
    sus result normie = 0
    lowkey x >= 0.0 { fr fr For positive numbers, truncate
        result = x fr fr Implicit conversion to int truncates
    } { fr fr For negative numbers, subtract 1 if not exact
        result = x fr fr Truncate
        lowkey x < result {
            result = result - 1
        }
    }
    damn result
}

slay ceil_meal(x meal) normie {
    sus result normie = 0
    lowkey x <= 0.0 { fr fr For negative numbers, truncate
        result = x fr fr Implicit conversion to int truncates
    } { fr fr For positive numbers, add 1 if not exact
        result = x fr fr Truncate
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

fr fr Power Functions
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
    bestie i := 0; i < exp; i++ {
        result = result * base
    }
    damn result
}

slay pow_meal_meal(base meal, exp meal) meal {
    lowkey abs_meal(exp - round_meal(exp)) < EPSILON { fr fr If exponent is effectively an integer
        damn pow_meal(base, round_meal(exp))
    } fr fr For fractional exponents, use approximation
    lowkey base <= 0.0 {
        damn 0.0 fr fr Safe fallback for negative bases
    } fr fr Approximate implementation using exp(ln(base) * exp)
    sus ln_base meal = ln_meal(base)
    sus product meal = ln_base * exp
    damn exp_meal(product)
}

fr fr Square Root (Newton's Method)
slay sqrt_meal(x meal) meal {
    lowkey x < 0.0 {
        damn 0.0 fr fr Return 0 for negative input (safe fallback)
    }
    lowkey x == 0.0 {
        damn 0.0
    }
    
    sus guess meal = x / 2.0
    sus prev meal = 0.0
    sus diff meal = 1.0
    sus iterations normie = 0
    
    bestie diff > EPSILON {
        lowkey iterations >= 100 {
            ghosted
        }
        prev = guess
        guess = (guess + (x / guess)) / 2.0
        diff = abs_meal(guess - prev)
        iterations = iterations + 1
    }
    
    damn guess
}

fr fr Natural Logarithm (Taylor Series Approximation)
slay ln_meal(x meal) meal {
    lowkey x <= 0.0 {
        damn 0.0 fr fr Safe fallback for non-positive input
    }
    lowkey x == 1.0 {
        damn 0.0
    } fr fr Use ln(x) = 2 * sum((x-1)/(x+1))^(2n+1) / (2n+1))
    sus y meal = (x - 1.0) / (x + 1.0)
    sus y_squared meal = y * y
    sus term meal = y
    sus result meal = y
    sus n normie = 1
    
    bestie n < 50 {
        lowkey abs_meal(term) <= EPSILON {
            ghosted
        }
        term = term * y_squared / (2.0 * n + 1.0)
        result = result + term
        n = n + 1
    }
    
    damn 2.0 * result
}

fr fr Exponential Function (Taylor Series)
slay exp_meal(x meal) meal {
    lowkey x == 0.0 {
        damn 1.0
    }
    
    sus result meal = 1.0
    sus term meal = 1.0
    sus n normie = 1
    
    bestie n < 50 {
        lowkey abs_meal(term) <= EPSILON {
            ghosted
        }
        term = term * x / n
        result = result + term
        n = n + 1
    }
    
    damn result
}

fr fr Trigonometric Functions (Taylor Series)
slay sin_meal(x meal) meal { fr fr Normalize angle to [-2π, 2π]
    sus normalized meal = x
    bestie normalized > TAU {
        normalized = normalized - TAU
    }
    bestie normalized < -TAU {
        normalized = normalized + TAU
    }
    
    sus result meal = normalized
    sus term meal = normalized
    sus x_squared meal = normalized * normalized
    sus n normie = 1
    
    bestie n < 20 {
        lowkey abs_meal(term) <= EPSILON {
            ghosted
        }
        term = -term * x_squared / ((2.0 * n) * (2.0 * n + 1.0))
        result = result + term
        n = n + 1
    }
    
    damn result
}

slay cos_meal(x meal) meal { fr fr cos(x) = sin(π/2 - x)
    damn sin_meal(PI / 2.0 - x)
}

slay tan_meal(x meal) meal {
    sus cos_val meal = cos_meal(x)
    lowkey abs_meal(cos_val) < EPSILON {
        damn 0.0 fr fr Safe fallback for division by zero
    }
    damn sin_meal(x) / cos_val
}

fr fr Degree Conversion Functions
slay sin_deg(degrees meal) meal {
    damn sin_meal(degrees * DEGREES_TO_RADIANS)
}

slay cos_deg(degrees meal) meal {
    damn cos_meal(degrees * DEGREES_TO_RADIANS)
}

slay tan_deg(degrees meal) meal {
    damn tan_meal(degrees * DEGREES_TO_RADIANS)
}

fr fr Angle Normalization
slay normalize_radians(angle meal) meal {
    sus result meal = angle
    bestie result > TAU {
        result = result - TAU
    }
    bestie result < 0.0 {
        result = result + TAU
    }
    damn result
}

slay normalize_degrees(angle meal) meal {
    sus result meal = angle
    bestie result > 360.0 {
        result = result - 360.0
    }
    bestie result < 0.0 {
        result = result + 360.0
    }
    damn result
}

fr fr Utility Functions
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

fr fr Factorial Function
slay factorial(n normie) normie {
    lowkey n <= 1 {
        damn 1
    }
    sus result normie = 1
    bestie i := 2; i <= n; i++ {
        result = result * i
    }
    damn result
}

fr fr Greatest Common Divisor
slay gcd(a normie, b normie) normie {
    sus x normie = abs_normie(a)
    sus y normie = abs_normie(b)
    
    bestie y != 0 {
        sus temp normie = y
        y = x % y
        x = temp
    }
    
    damn x
}

fr fr Least Common Multiple
slay lcm(a normie, b normie) normie {
    lowkey a == 0 || b == 0 {
        damn 0
    }
    damn abs_normie(a * b) / gcd(a, b)
}

fr fr Random Number Generation (Linear Congruential Generator)
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

fr fr Logarithmic Functions
slay log10_meal(x meal) meal {
    lowkey x <= 0.0 {
        damn 0.0 fr fr Safe fallback
    }
    damn ln_meal(x) / LN_10
}

slay log2_meal(x meal) meal {
    lowkey x <= 0.0 {
        damn 0.0 fr fr Safe fallback
    }
    damn ln_meal(x) / LN_2
}

fr fr Inverse Trigonometric Functions (using Taylor series)
slay asin_meal(x meal) meal {
    lowkey x < -1.0 || x > 1.0 {
        damn 0.0 fr fr Invalid input, return 0
    }
    lowkey x == 0.0 {
        damn 0.0
    }
    lowkey x == 1.0 {
        damn PI / 2.0
    }
    lowkey x == -1.0 {
        damn -PI / 2.0
    }
    
    fr fr Use arcsin(x) = x + x³/6 + 3x⁵/40 + 5x⁷/112 + ...
    sus result meal = x
    sus x_squared meal = x * x
    sus term meal = x
    sus n normie = 1
    
    bestie n < 20 {
        lowkey abs_meal(term) <= EPSILON {
            ghosted
        }
        term = term * x_squared * (2.0 * n - 1.0) * (2.0 * n - 1.0) / ((2.0 * n) * (2.0 * n + 1.0))
        result = result + term
        n = n + 1
    }
    
    damn result
}

slay acos_meal(x meal) meal {
    lowkey x < -1.0 || x > 1.0 {
        damn 0.0 fr fr Invalid input
    }
    damn PI / 2.0 - asin_meal(x)
}

slay atan_meal(x meal) meal {
    lowkey x == 0.0 {
        damn 0.0
    }
    lowkey x > 1.0 {
        damn PI / 2.0 - atan_meal(1.0 / x)
    }
    lowkey x < -1.0 {
        damn -PI / 2.0 - atan_meal(1.0 / x)
    }
    
    fr fr Use arctan(x) = x - x³/3 + x⁵/5 - x⁷/7 + ...
    sus result meal = x
    sus x_squared meal = x * x
    sus term meal = x
    sus n normie = 1
    sus sign normie = -1
    
    bestie n < 50 {
        lowkey abs_meal(term) <= EPSILON {
            ghosted
        }
        term = term * x_squared
        result = result + sign * term / (2.0 * n + 1.0)
        sign = -sign
        n = n + 1
    }
    
    damn result
}

fr fr Hyperbolic Functions
slay sinh_meal(x meal) meal {
    sus exp_pos meal = exp_meal(x)
    sus exp_neg meal = exp_meal(-x)
    damn (exp_pos - exp_neg) / 2.0
}

slay cosh_meal(x meal) meal {
    sus exp_pos meal = exp_meal(x)
    sus exp_neg meal = exp_meal(-x)
    damn (exp_pos + exp_neg) / 2.0
}

slay tanh_meal(x meal) meal {
    sus sinh_val meal = sinh_meal(x)
    sus cosh_val meal = cosh_meal(x)
    lowkey abs_meal(cosh_val) < EPSILON {
        damn 0.0
    }
    damn sinh_val / cosh_val
}

fr fr Additional Utility Functions
slay clamp_meal(value meal, min_val meal, max_val meal) meal {
    lowkey value < min_val {
        damn min_val
    }
    lowkey value > max_val {
        damn max_val
    }
    damn value
}

slay clamp_normie(value normie, min_val normie, max_val normie) normie {
    lowkey value < min_val {
        damn min_val
    }
    lowkey value > max_val {
        damn max_val
    }
    damn value
}

slay lerp_meal(a meal, b meal, t meal) meal {
    damn a + t * (b - a)
}

slay sign_meal(x meal) meal {
    lowkey x > 0.0 {
        damn 1.0
    }
    lowkey x < 0.0 {
        damn -1.0
    }
    damn 0.0
}

slay sign_normie(x normie) normie {
    lowkey x > 0 {
        damn 1
    }
    lowkey x < 0 {
        damn -1
    }
    damn 0
}

fr fr Special Constants
sus INFINITY meal = 1.0 / 0.0
sus NAN meal = 0.0 / 0.0

fr fr NaN and Infinity checks
slay is_nan(x meal) lit {
    damn x != x
}

slay is_infinite(x meal) lit {
    damn x == INFINITY || x == -INFINITY
}

slay is_finite(x meal) lit {
    damn !is_nan(x) && !is_infinite(x)
}

fr fr Truncation function
slay trunc_meal(x meal) normie {
    lowkey x >= 0.0 {
        damn floor_meal(x)
    } {
        damn ceil_meal(x)
    }
}

fr fr Fractional part
slay frac_meal(x meal) meal {
    damn x - trunc_meal(x)
}

fr fr Modulo for floats
slay fmod_meal(x meal, y meal) meal {
    lowkey y == 0.0 {
        damn 0.0
    }
    sus quotient normie = trunc_meal(x / y)
    damn x - quotient * y
}

fr fr Advanced Random Functions
slay random_meal_range(min_val meal, max_val meal) meal {
    lowkey min_val >= max_val {
        damn min_val
    }
    sus range meal = max_val - min_val
    damn min_val + random_meal() * range
}

slay random_gaussian() meal {
    fr fr Box-Muller transform for Gaussian distribution
    sus u1 meal = random_meal()
    sus u2 meal = random_meal()
    
    lowkey u1 <= 0.0 {
        u1 = EPSILON
    }
    
    sus z0 meal = sqrt_meal(-2.0 * ln_meal(u1)) * cos_meal(TAU * u2)
    damn z0
}

fr fr Prime checking
slay is_prime(n normie) lit {
    lowkey n <= 1 {
        damn cringe
    }
    lowkey n <= 3 {
        damn based
    }
    lowkey n % 2 == 0 || n % 3 == 0 {
        damn cringe
    }
    
    sus i normie = 5
    bestie i * i <= n {
        lowkey n % i == 0 || n % (i + 2) == 0 {
            damn cringe
        }
        i = i + 6
    }
    
    damn based
}

fr fr Advanced Mathematical Functions
slay fibonacci(n normie) normie {
    lowkey n <= 1 {
        damn n
    }
    
    sus a normie = 0
    sus b normie = 1
    
    bestie i := 2; i <= n; i++ {
        sus temp normie = a + b
        a = b
        b = temp
    }
    
    damn b
}

fr fr Sum of arithmetic series
slay arithmetic_sum(first normie, last normie, count normie) normie {
    lowkey count <= 0 {
        damn 0
    }
    damn count * (first + last) / 2
}

fr fr Sum of geometric series
slay geometric_sum(first meal, ratio meal, count normie) meal {
    lowkey count <= 0 {
        damn 0.0
    }
    lowkey abs_meal(ratio - 1.0) < EPSILON {
        damn first * count
    }
    damn first * (1.0 - pow_meal_meal(ratio, count)) / (1.0 - ratio)
}

fr fr Distance functions
slay distance_2d(x1 meal, y1 meal, x2 meal, y2 meal) meal {
    sus dx meal = x2 - x1
    sus dy meal = y2 - y1
    damn sqrt_meal(dx * dx + dy * dy)
}

slay distance_3d(x1 meal, y1 meal, z1 meal, x2 meal, y2 meal, z2 meal) meal {
    sus dx meal = x2 - x1
    sus dy meal = y2 - y1
    sus dz meal = z2 - z1
    damn sqrt_meal(dx * dx + dy * dy + dz * dz)
}
