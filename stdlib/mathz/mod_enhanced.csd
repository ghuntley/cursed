fr fr Enhanced MATHZ Module - Comprehensive Mathematical Operations for CURSED
fr fr Production-ready implementation with advanced mathematical functions

fr fr ===== MATHEMATICAL CONSTANTS =====

sus PI meal = 3.141592653589793
sus E meal = 2.718281828459045
sus TAU meal = 6.283185307179586
sus SQRT_2 meal = 1.4142135623730951
sus SQRT_3 meal = 1.7320508075688772
sus SQRT_5 meal = 2.23606797749979
sus LN_2 meal = 0.6931471805599453
sus LN_10 meal = 2.302585092994046
sus LOG2_E meal = 1.4426950408889634
sus LOG10_E meal = 0.4342944819032518
sus GOLDEN_RATIO meal = 1.618033988749895
sus EULER_MASCHERONI meal = 0.5772156649015329
sus DEGREES_TO_RADIANS meal = 0.017453292519943295
sus RADIANS_TO_DEGREES meal = 57.29577951308232
sus EPSILON meal = 0.00000000000000022204460492503131
sus INFINITY meal = 1.0e308
sus NEG_INFINITY meal = -1.0e308
sus NAN meal = 0.0 / 0.0

fr fr Physical and Mathematical Constants
sus SPEED_OF_LIGHT meal = 299792458.0          fr fr m/s
sus PLANCK_CONSTANT meal = 6.62607015e-34       fr fr J⋅s
sus AVOGADRO_NUMBER meal = 6.02214076e23        fr fr mol⁻¹
sus BOLTZMANN_CONSTANT meal = 1.380649e-23      fr fr J/K
sus GRAVITATIONAL_CONSTANT meal = 6.67430e-11   fr fr m³/kg⋅s²

fr fr ===== BASIC ARITHMETIC OPERATIONS =====

slay add(a meal, b meal) meal {
    damn a + b
}

slay subtract(a meal, b meal) meal {
    damn a - b
}

slay multiply(a meal, b meal) meal {
    damn a * b
}

slay divide(a meal, b meal) meal {
    check abs_meal(b) < EPSILON {
        damn INFINITY fr fr Return infinity for division by zero
    }
    damn a / b
}

slay safe_divide(a meal, b meal) (meal, lit) {
    check abs_meal(b) < EPSILON {
        damn (0.0, cringe) fr fr Return error flag
    }
    damn (a / b, based)
}

slay mod_meal(a meal, b meal) meal {
    check abs_meal(b) < EPSILON {
        damn NAN
    }
    sus quotient meal = floor_meal(a / b)
    damn a - quotient * b
}

fr fr ===== ABSOLUTE VALUE AND SIGN FUNCTIONS =====

slay abs_meal(x meal) meal {
    check x < 0.0 {
        damn -x
    }
    damn x
}

slay abs_normie(x normie) normie {
    check x < 0 {
        damn -x
    }
    damn x
}

slay sign_meal(x meal) meal {
    check x > EPSILON {
        damn 1.0
    } highkey x < -EPSILON {
        damn -1.0
    }
    damn 0.0
}

slay sign_normie(x normie) normie {
    check x > 0 {
        damn 1
    } highkey x < 0 {
        damn -1
    }
    damn 0
}

slay copysign(magnitude meal, sign meal) meal {
    sus abs_mag meal = abs_meal(magnitude)
    check sign < 0.0 {
        damn -abs_mag
    }
    damn abs_mag
}

fr fr ===== MIN/MAX OPERATIONS =====

slay max_meal(a meal, b meal) meal {
    check is_nan(a) || is_nan(b) {
        damn NAN
    }
    check a > b {
        damn a
    }
    damn b
}

slay min_meal(a meal, b meal) meal {
    check is_nan(a) || is_nan(b) {
        damn NAN
    }
    check a < b {
        damn a
    }
    damn b
}

slay max_normie(a normie, b normie) normie {
    check a > b {
        damn a
    }
    damn b
}

slay min_normie(a normie, b normie) normie {
    check a < b {
        damn a
    }
    damn b
}

slay max_array(values []meal) meal {
    check len_array(values) == 0 {
        damn NAN
    }
    
    sus max_val meal = values[0]
    sus i normie = 1
    bestie i < len_array(values) {
        max_val = max_meal(max_val, values[i])
        i = i + 1
    }
    damn max_val
}

slay min_array(values []meal) meal {
    check len_array(values) == 0 {
        damn NAN
    }
    
    sus min_val meal = values[0]
    sus i normie = 1
    bestie i < len_array(values) {
        min_val = min_meal(min_val, values[i])
        i = i + 1
    }
    damn min_val
}

fr fr ===== ROUNDING AND TRUNCATION =====

slay floor_meal(x meal) normie {
    check is_nan(x) || is_infinite(x) {
        damn 0 fr fr Safe fallback
    }
    
    sus result normie = x fr fr Truncate to integer
    check x < 0.0 && x != result {
        result = result - 1 fr fr Adjust for negative non-integers
    }
    damn result
}

slay ceil_meal(x meal) normie {
    check is_nan(x) || is_infinite(x) {
        damn 0 fr fr Safe fallback
    }
    
    sus result normie = x fr fr Truncate to integer
    check x > 0.0 && x != result {
        result = result + 1 fr fr Adjust for positive non-integers
    }
    damn result
}

slay round_meal(x meal) normie {
    check is_nan(x) || is_infinite(x) {
        damn 0 fr fr Safe fallback
    }
    
    check x >= 0.0 {
        damn floor_meal(x + 0.5)
    }
    damn ceil_meal(x - 0.5)
}

slay trunc_meal(x meal) normie {
    check is_nan(x) || is_infinite(x) {
        damn 0
    }
    damn x fr fr Implicit truncation
}

slay frac_meal(x meal) meal {
    sus truncated normie = trunc_meal(x)
    damn x - truncated
}

slay round_to_places(x meal, places normie) meal {
    check places < 0 {
        damn x
    }
    sus multiplier meal = pow_meal(10.0, places)
    damn round_meal(x * multiplier) / multiplier
}

fr fr ===== POWER AND ROOT FUNCTIONS =====

slay pow_meal(base meal, exp normie) meal {
    check exp == 0 {
        damn 1.0
    }
    check exp == 1 {
        damn base
    }
    check base == 0.0 {
        check exp > 0 {
            damn 0.0
        }
        damn INFINITY
    }
    check exp < 0 {
        damn 1.0 / pow_meal(base, -exp)
    }
    
    sus result meal = 1.0
    sus i normie = 0
    bestie i < exp {
        result = result * base
        i = i + 1
    }
    damn result
}

slay pow_meal_meal(base meal, exp meal) meal {
    check is_nan(base) || is_nan(exp) {
        damn NAN
    }
    check base == 0.0 && exp == 0.0 {
        damn 1.0 fr fr Mathematical convention
    }
    check base == 0.0 {
        check exp > 0.0 {
            damn 0.0
        }
        damn INFINITY
    }
    check base < 0.0 && !is_integer_float(exp) {
        damn NAN fr fr Complex result
    }
    
    fr fr For integer exponents, use integer power
    check is_integer_float(exp) {
        damn pow_meal(base, trunc_meal(exp))
    }
    
    fr fr For fractional exponents: x^y = exp(ln(x) * y)
    check base <= 0.0 {
        damn NAN
    }
    
    sus ln_base meal = ln_meal(base)
    sus product meal = ln_base * exp
    damn exp_meal(product)
}

slay sqrt_meal(x meal) meal {
    check x < 0.0 {
        damn NAN
    }
    check x == 0.0 {
        damn 0.0
    }
    check x == 1.0 {
        damn 1.0
    }
    
    fr fr Newton's method for square root
    sus guess meal = x / 2.0
    sus prev meal = 0.0
    sus iterations normie = 0
    
    bestie iterations < 50 {
        prev = guess
        guess = (guess + x / guess) / 2.0
        
        check abs_meal(guess - prev) < EPSILON {
            ghosted
        }
        iterations = iterations + 1
    }
    
    damn guess
}

slay cbrt_meal(x meal) meal {
    check x == 0.0 {
        damn 0.0
    }
    
    sus sign meal = 1.0
    sus abs_x meal = x
    
    check x < 0.0 {
        sign = -1.0
        abs_x = -x
    }
    
    fr fr Newton's method for cube root
    sus guess meal = abs_x / 3.0
    sus prev meal = 0.0
    sus iterations normie = 0
    
    bestie iterations < 50 {
        prev = guess
        guess = (2.0 * guess + abs_x / (guess * guess)) / 3.0
        
        check abs_meal(guess - prev) < EPSILON {
            ghosted
        }
        iterations = iterations + 1
    }
    
    damn sign * guess
}

slay nth_root(x meal, n normie) meal {
    check n == 0 {
        damn NAN
    }
    check n == 1 {
        damn x
    }
    check n == 2 {
        damn sqrt_meal(x)
    }
    check n == 3 {
        damn cbrt_meal(x)
    }
    
    check x < 0.0 && n % 2 == 0 {
        damn NAN fr fr Even root of negative number
    }
    
    damn pow_meal_meal(x, 1.0 / n)
}

fr fr ===== EXPONENTIAL AND LOGARITHMIC FUNCTIONS =====

slay exp_meal(x meal) meal {
    check is_nan(x) {
        damn NAN
    }
    check x == 0.0 {
        damn 1.0
    }
    check x > 700.0 {
        damn INFINITY fr fr Prevent overflow
    }
    check x < -700.0 {
        damn 0.0 fr fr Underflow to zero
    }
    
    fr fr Taylor series: e^x = 1 + x + x²/2! + x³/3! + ...
    sus result meal = 1.0
    sus term meal = 1.0
    sus n normie = 1
    
    bestie n < 100 {
        term = term * x / n
        result = result + term
        
        check abs_meal(term) < EPSILON {
            ghosted
        }
        n = n + 1
    }
    
    damn result
}

slay ln_meal(x meal) meal {
    check is_nan(x) || x < 0.0 {
        damn NAN
    }
    check x == 0.0 {
        damn NEG_INFINITY
    }
    check x == 1.0 {
        damn 0.0
    }
    check x == E {
        damn 1.0
    }
    
    fr fr For values close to 1, use Taylor series around 1
    check abs_meal(x - 1.0) < 0.5 {
        sus y meal = x - 1.0
        sus result meal = y
        sus term meal = y
        sus n normie = 2
        sus sign meal = -1.0
        
        bestie n < 100 {
            term = term * y
            result = result + sign * term / n
            sign = -sign
            
            check abs_meal(term / n) < EPSILON {
                ghosted
            }
            n = n + 1
        }
        
        damn result
    }
    
    fr fr For other values, use the formula ln(x) = 2 * artanh((x-1)/(x+1))
    sus y meal = (x - 1.0) / (x + 1.0)
    sus y_squared meal = y * y
    sus term meal = y
    sus result meal = y
    sus n normie = 1
    
    bestie n < 50 {
        term = term * y_squared
        result = result + term / (2.0 * n + 1.0)
        
        check abs_meal(term / (2.0 * n + 1.0)) < EPSILON {
            ghosted
        }
        n = n + 1
    }
    
    damn 2.0 * result
}

slay log10_meal(x meal) meal {
    check x <= 0.0 {
        damn NAN
    }
    damn ln_meal(x) / LN_10
}

slay log2_meal(x meal) meal {
    check x <= 0.0 {
        damn NAN
    }
    damn ln_meal(x) / LN_2
}

slay log_base(x meal, base meal) meal {
    check x <= 0.0 || base <= 0.0 || base == 1.0 {
        damn NAN
    }
    damn ln_meal(x) / ln_meal(base)
}

fr fr ===== TRIGONOMETRIC FUNCTIONS =====

slay normalize_angle(angle meal) meal {
    sus normalized meal = angle
    bestie normalized > PI {
        normalized = normalized - TAU
    }
    bestie normalized <= -PI {
        normalized = normalized + TAU
    }
    damn normalized
}

slay sin_meal(x meal) meal {
    check is_nan(x) || is_infinite(x) {
        damn NAN
    }
    
    sus normalized meal = normalize_angle(x)
    
    fr fr Taylor series: sin(x) = x - x³/3! + x⁵/5! - x⁷/7! + ...
    sus result meal = normalized
    sus term meal = normalized
    sus x_squared meal = normalized * normalized
    sus n normie = 1
    
    bestie n < 20 {
        term = -term * x_squared / ((2.0 * n) * (2.0 * n + 1.0))
        result = result + term
        
        check abs_meal(term) < EPSILON {
            ghosted
        }
        n = n + 1
    }
    
    damn result
}

slay cos_meal(x meal) meal {
    check is_nan(x) || is_infinite(x) {
        damn NAN
    }
    
    sus normalized meal = normalize_angle(x)
    
    fr fr Taylor series: cos(x) = 1 - x²/2! + x⁴/4! - x⁶/6! + ...
    sus result meal = 1.0
    sus term meal = 1.0
    sus x_squared meal = normalized * normalized
    sus n normie = 1
    
    bestie n < 20 {
        term = -term * x_squared / ((2.0 * n - 1.0) * (2.0 * n))
        result = result + term
        
        check abs_meal(term) < EPSILON {
            ghosted
        }
        n = n + 1
    }
    
    damn result
}

slay tan_meal(x meal) meal {
    sus cos_val meal = cos_meal(x)
    check abs_meal(cos_val) < EPSILON {
        damn INFINITY
    }
    damn sin_meal(x) / cos_val
}

slay cot_meal(x meal) meal {
    sus sin_val meal = sin_meal(x)
    check abs_meal(sin_val) < EPSILON {
        damn INFINITY
    }
    damn cos_meal(x) / sin_val
}

slay sec_meal(x meal) meal {
    sus cos_val meal = cos_meal(x)
    check abs_meal(cos_val) < EPSILON {
        damn INFINITY
    }
    damn 1.0 / cos_val
}

slay csc_meal(x meal) meal {
    sus sin_val meal = sin_meal(x)
    check abs_meal(sin_val) < EPSILON {
        damn INFINITY
    }
    damn 1.0 / sin_val
}

fr fr ===== INVERSE TRIGONOMETRIC FUNCTIONS =====

slay asin_meal(x meal) meal {
    check x < -1.0 || x > 1.0 {
        damn NAN
    }
    check x == -1.0 {
        damn -PI / 2.0
    }
    check x == 1.0 {
        damn PI / 2.0
    }
    check x == 0.0 {
        damn 0.0
    }
    
    fr fr Use Taylor series for |x| < 0.5, otherwise use identity
    check abs_meal(x) < 0.5 {
        sus result meal = x
        sus term meal = x
        sus x_squared meal = x * x
        sus n normie = 1
        
        bestie n < 20 {
            term = term * x_squared * (2.0 * n - 1.0) * (2.0 * n - 1.0) / ((2.0 * n) * (2.0 * n + 1.0))
            result = result + term
            
            check abs_meal(term) < EPSILON {
                ghosted
            }
            n = n + 1
        }
        
        damn result
    }
    
    fr fr For larger values: arcsin(x) = π/2 - arcsin(√(1-x²))
    sus sqrt_term meal = sqrt_meal(1.0 - x * x)
    check x > 0.0 {
        damn PI / 2.0 - asin_meal(sqrt_term)
    }
    damn -PI / 2.0 + asin_meal(sqrt_term)
}

slay acos_meal(x meal) meal {
    check x < -1.0 || x > 1.0 {
        damn NAN
    }
    damn PI / 2.0 - asin_meal(x)
}

slay atan_meal(x meal) meal {
    check is_nan(x) {
        damn NAN
    }
    check is_infinite(x) {
        check x > 0.0 {
            damn PI / 2.0
        }
        damn -PI / 2.0
    }
    check x == 0.0 {
        damn 0.0
    }
    check x == 1.0 {
        damn PI / 4.0
    }
    check x == -1.0 {
        damn -PI / 4.0
    }
    
    fr fr For |x| > 1, use identity: arctan(x) = π/2 - arctan(1/x)
    check abs_meal(x) > 1.0 {
        check x > 0.0 {
            damn PI / 2.0 - atan_meal(1.0 / x)
        }
        damn -PI / 2.0 - atan_meal(1.0 / x)
    }
    
    fr fr Taylor series: arctan(x) = x - x³/3 + x⁵/5 - x⁷/7 + ...
    sus result meal = x
    sus term meal = x
    sus x_squared meal = x * x
    sus n normie = 1
    sus sign meal = -1.0
    
    bestie n < 50 {
        term = term * x_squared
        result = result + sign * term / (2.0 * n + 1.0)
        sign = -sign
        
        check abs_meal(term / (2.0 * n + 1.0)) < EPSILON {
            ghosted
        }
        n = n + 1
    }
    
    damn result
}

slay atan2_meal(y meal, x meal) meal {
    check is_nan(x) || is_nan(y) {
        damn NAN
    }
    check x == 0.0 && y == 0.0 {
        damn 0.0 fr fr Convention
    }
    check x == 0.0 {
        check y > 0.0 {
            damn PI / 2.0
        }
        damn -PI / 2.0
    }
    check x > 0.0 {
        damn atan_meal(y / x)
    }
    check y >= 0.0 {
        damn atan_meal(y / x) + PI
    }
    damn atan_meal(y / x) - PI
}

fr fr ===== HYPERBOLIC FUNCTIONS =====

slay sinh_meal(x meal) meal {
    check is_nan(x) {
        damn NAN
    }
    check abs_meal(x) > 700.0 {
        check x > 0.0 {
            damn INFINITY
        }
        damn NEG_INFINITY
    }
    
    sus exp_pos meal = exp_meal(x)
    sus exp_neg meal = exp_meal(-x)
    damn (exp_pos - exp_neg) / 2.0
}

slay cosh_meal(x meal) meal {
    check is_nan(x) {
        damn NAN
    }
    check abs_meal(x) > 700.0 {
        damn INFINITY
    }
    
    sus exp_pos meal = exp_meal(x)
    sus exp_neg meal = exp_meal(-x)
    damn (exp_pos + exp_neg) / 2.0
}

slay tanh_meal(x meal) meal {
    check is_nan(x) {
        damn NAN
    }
    check abs_meal(x) > 20.0 {
        check x > 0.0 {
            damn 1.0
        }
        damn -1.0
    }
    
    sus sinh_val meal = sinh_meal(x)
    sus cosh_val meal = cosh_meal(x)
    damn sinh_val / cosh_val
}

fr fr ===== SPECIAL FUNCTIONS =====

slay factorial(n normie) normie {
    check n < 0 {
        damn 0 fr fr Undefined for negative numbers
    }
    check n <= 1 {
        damn 1
    }
    check n > 20 {
        damn 0 fr fr Overflow protection
    }
    
    sus result normie = 1
    sus i normie = 2
    bestie i <= n {
        result = result * i
        i = i + 1
    }
    damn result
}

slay gamma_approximation(x meal) meal {
    fr fr Stirling's approximation for large values
    check x <= 0.0 {
        damn NAN
    }
    check x < 0.5 {
        damn PI / (sin_meal(PI * x) * gamma_approximation(1.0 - x))
    }
    
    x = x - 1.0
    sus result meal = sqrt_meal(2.0 * PI / x) * pow_meal_meal(x / E, x)
    damn result
}

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

slay lcm(a normie, b normie) normie {
    check a == 0 || b == 0 {
        damn 0
    }
    sus g normie = gcd(a, b)
    damn abs_normie(a * b) / g
}

slay fibonacci(n normie) normie {
    check n < 0 {
        damn 0
    }
    check n <= 1 {
        damn n
    }
    check n > 46 {
        damn 0 fr fr Overflow protection
    }
    
    sus a normie = 0
    sus b normie = 1
    
    sus i normie = 2
    bestie i <= n {
        sus temp normie = a + b
        a = b
        b = temp
        i = i + 1
    }
    
    damn b
}

slay is_prime(n normie) lit {
    check n <= 1 {
        damn cringe
    }
    check n <= 3 {
        damn based
    }
    check n % 2 == 0 || n % 3 == 0 {
        damn cringe
    }
    
    sus i normie = 5
    bestie i * i <= n {
        check n % i == 0 || n % (i + 2) == 0 {
            damn cringe
        }
        i = i + 6
    }
    
    damn based
}

fr fr ===== UTILITY AND HELPER FUNCTIONS =====

slay is_nan(x meal) lit {
    damn x != x
}

slay is_infinite(x meal) lit {
    damn x == INFINITY || x == NEG_INFINITY
}

slay is_finite(x meal) lit {
    damn !is_nan(x) && !is_infinite(x)
}

slay is_integer_float(x meal) lit {
    check is_nan(x) || is_infinite(x) {
        damn cringe
    }
    damn x == trunc_meal(x)
}

slay is_even(x normie) lit {
    damn (x % 2) == 0
}

slay is_odd(x normie) lit {
    damn (x % 2) == 1
}

slay is_approximately_equal(a meal, b meal, tolerance meal) lit {
    check is_nan(a) || is_nan(b) {
        damn cringe
    }
    damn abs_meal(a - b) <= tolerance
}

slay clamp_meal(value meal, min_val meal, max_val meal) meal {
    check value < min_val {
        damn min_val
    }
    check value > max_val {
        damn max_val
    }
    damn value
}

slay clamp_normie(value normie, min_val normie, max_val normie) normie {
    check value < min_val {
        damn min_val
    }
    check value > max_val {
        damn max_val
    }
    damn value
}

slay lerp_meal(a meal, b meal, t meal) meal {
    check t <= 0.0 {
        damn a
    }
    check t >= 1.0 {
        damn b
    }
    damn a + t * (b - a)
}

slay smoothstep(edge0 meal, edge1 meal, x meal) meal {
    sus t meal = clamp_meal((x - edge0) / (edge1 - edge0), 0.0, 1.0)
    damn t * t * (3.0 - 2.0 * t)
}

fr fr ===== STATISTICAL FUNCTIONS =====

slay sum_array(values []meal) meal {
    check len_array(values) == 0 {
        damn 0.0
    }
    
    sus total meal = 0.0
    sus i normie = 0
    bestie i < len_array(values) {
        total = total + values[i]
        i = i + 1
    }
    damn total
}

slay mean_array(values []meal) meal {
    sus count normie = len_array(values)
    check count == 0 {
        damn NAN
    }
    damn sum_array(values) / count
}

slay variance_array(values []meal) meal {
    sus count normie = len_array(values)
    check count <= 1 {
        damn 0.0
    }
    
    sus mean_val meal = mean_array(values)
    sus sum_squares meal = 0.0
    sus i normie = 0
    
    bestie i < count {
        sus diff meal = values[i] - mean_val
        sum_squares = sum_squares + diff * diff
        i = i + 1
    }
    
    damn sum_squares / (count - 1)
}

slay std_deviation_array(values []meal) meal {
    damn sqrt_meal(variance_array(values))
}

fr fr ===== DISTANCE AND GEOMETRY =====

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

slay dot_product_2d(x1 meal, y1 meal, x2 meal, y2 meal) meal {
    damn x1 * x2 + y1 * y2
}

slay cross_product_2d(x1 meal, y1 meal, x2 meal, y2 meal) meal {
    damn x1 * y2 - y1 * x2
}

slay magnitude_2d(x meal, y meal) meal {
    damn sqrt_meal(x * x + y * y)
}

slay normalize_2d(x meal, y meal) (meal, meal) {
    sus mag meal = magnitude_2d(x, y)
    check mag < EPSILON {
        damn (0.0, 0.0)
    }
    damn (x / mag, y / mag)
}

fr fr ===== RANDOM NUMBER GENERATION =====

sus random_seed normie = 1

slay set_random_seed(seed normie) cringe {
    random_seed = seed
    damn cringe
}

slay random_int() normie {
    random_seed = (random_seed * 1103515245 + 12345) % 2147483647
    damn random_seed
}

slay random_meal() meal {
    damn random_int() / 2147483646.0
}

slay random_range(min_val normie, max_val normie) normie {
    check min_val >= max_val {
        damn min_val
    }
    sus range normie = max_val - min_val + 1
    damn min_val + (random_int() % range)
}

slay random_meal_range(min_val meal, max_val meal) meal {
    check min_val >= max_val {
        damn min_val
    }
    sus range meal = max_val - min_val
    damn min_val + random_meal() * range
}

slay random_gaussian() meal {
    fr fr Box-Muller transform
    sus u1 meal = random_meal()
    sus u2 meal = random_meal()
    
    check u1 <= EPSILON {
        u1 = EPSILON
    }
    
    sus z0 meal = sqrt_meal(-2.0 * ln_meal(u1)) * cos_meal(TAU * u2)
    damn z0
}

fr fr ===== SERIES AND SEQUENCES =====

slay arithmetic_sum(first normie, last normie, count normie) normie {
    check count <= 0 {
        damn 0
    }
    damn count * (first + last) / 2
}

slay geometric_sum(first meal, ratio meal, count normie) meal {
    check count <= 0 {
        damn 0.0
    }
    check abs_meal(ratio - 1.0) < EPSILON {
        damn first * count
    }
    damn first * (1.0 - pow_meal_meal(ratio, count)) / (1.0 - ratio)
}

slay harmonic_sum(n normie) meal {
    check n <= 0 {
        damn 0.0
    }
    
    sus sum meal = 0.0
    sus i normie = 1
    bestie i <= n {
        sum = sum + 1.0 / i
        i = i + 1
    }
    damn sum
}

fr fr ===== ANGLE CONVERSION =====

slay degrees_to_radians(degrees meal) meal {
    damn degrees * DEGREES_TO_RADIANS
}

slay radians_to_degrees(radians meal) meal {
    damn radians * RADIANS_TO_DEGREES
}

slay sin_deg(degrees meal) meal {
    damn sin_meal(degrees_to_radians(degrees))
}

slay cos_deg(degrees meal) meal {
    damn cos_meal(degrees_to_radians(degrees))
}

slay tan_deg(degrees meal) meal {
    damn tan_meal(degrees_to_radians(degrees))
}

fr fr ===== HELPER FUNCTIONS =====

slay len_array(arr []meal) normie {
    fr fr This would be implemented by the runtime
    damn runtime_array_length(arr)
}

slay runtime_array_length(arr []meal) normie {
    fr fr Implemented in Zig runtime
    damn core.array_length(arr)
}
