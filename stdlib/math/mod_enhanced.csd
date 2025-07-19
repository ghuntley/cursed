// Enhanced math library with error_core integration
yeet "error_core"

// ================================
// Constants
// ================================

slay math_pi() meal {
    damn 3.141592653589793
}

slay math_e() meal {
    damn 2.718281828459045
}

slay math_tau() meal {
    damn 6.283185307179586
}

// ================================
// Basic operations with error handling
// ================================

slay math_abs(x meal) meal {
    damn math_abs_impl(x)
}

slay math_abs_int(x normie) normie {
    damn math_abs_int_impl(x)
}

slay math_min(a meal, b meal) meal {
    damn math_min_impl(a, b)
}

slay math_max(a meal, b meal) meal {
    damn math_max_impl(a, b)
}

slay math_min_int(a normie, b normie) normie {
    damn math_min_int_impl(a, b)
}

slay math_max_int(a normie, b normie) normie {
    damn math_max_int_impl(a, b)
}

slay math_clamp(x meal, min meal, max meal) (meal, yikes) {
    vibe_check min > max {
        damn 0.0, new_value_error("Invalid clamp range", "min > max", "min <= max")
    }
    
    vibe_check x < min {
        damn min, cringe
    }
    
    vibe_check x > max {
        damn max, cringe
    }
    
    damn x, cringe
}

// ================================
// Advanced operations with error handling
// ================================

slay math_sqrt(x meal) (meal, yikes) {
    vibe_check x < 0.0 {
        damn 0.0, new_value_error("Cannot compute square root of negative number", string(x), "non-negative number")
    }
    
    damn math_sqrt_impl(x), cringe
}

slay math_pow(base meal, exponent meal) (meal, yikes) {
    vibe_check base == 0.0 && exponent < 0.0 {
        damn 0.0, new_value_error("Cannot raise zero to negative power", "0^" + string(exponent), "non-zero base or non-negative exponent")
    }
    
    vibe_check base < 0.0 && math_frac(exponent) != 0.0 {
        damn 0.0, new_value_error("Cannot raise negative number to fractional power", string(base) + "^" + string(exponent), "non-negative base or integer exponent")
    }
    
    damn math_pow_impl(base, exponent), cringe
}

slay math_log(x meal) (meal, yikes) {
    vibe_check x <= 0.0 {
        damn 0.0, new_value_error("Cannot compute logarithm of non-positive number", string(x), "positive number")
    }
    
    damn math_log_impl(x), cringe
}

slay math_log10(x meal) (meal, yikes) {
    vibe_check x <= 0.0 {
        damn 0.0, new_value_error("Cannot compute log10 of non-positive number", string(x), "positive number")
    }
    
    damn math_log10_impl(x), cringe
}

slay math_log2(x meal) (meal, yikes) {
    vibe_check x <= 0.0 {
        damn 0.0, new_value_error("Cannot compute log2 of non-positive number", string(x), "positive number")
    }
    
    damn math_log2_impl(x), cringe
}

slay math_exp(x meal) (meal, yikes) {
    vibe_check x > 700.0 {
        damn 0.0, new_value_error("Exponent too large, would cause overflow", string(x), "value <= 700")
    }
    
    damn math_exp_impl(x), cringe
}

// ================================
// Trigonometric functions with error handling
// ================================

slay math_sin(x meal) meal {
    damn math_sin_impl(x)
}

slay math_cos(x meal) meal {
    damn math_cos_impl(x)
}

slay math_tan(x meal) (meal, yikes) {
    # Check for values near π/2 + nπ where tan is undefined
    sus normalized meal = math_fmod(x, math_pi())
    vibe_check math_abs(normalized - math_pi()/2.0) < 1e-10 || math_abs(normalized + math_pi()/2.0) < 1e-10 {
        damn 0.0, new_value_error("Tangent undefined at π/2 + nπ", string(x), "value not equal to π/2 + nπ")
    }
    
    damn math_tan_impl(x), cringe
}

slay math_asin(x meal) (meal, yikes) {
    vibe_check x < -1.0 || x > 1.0 {
        damn 0.0, new_value_error("Arcsine domain error", string(x), "value in [-1, 1]")
    }
    
    damn math_asin_impl(x), cringe
}

slay math_acos(x meal) (meal, yikes) {
    vibe_check x < -1.0 || x > 1.0 {
        damn 0.0, new_value_error("Arccosine domain error", string(x), "value in [-1, 1]")
    }
    
    damn math_acos_impl(x), cringe
}

slay math_atan(x meal) meal {
    damn math_atan_impl(x)
}

slay math_atan2(y meal, x meal) (meal, yikes) {
    vibe_check x == 0.0 && y == 0.0 {
        damn 0.0, new_value_error("Atan2 undefined for (0, 0)", "(0, 0)", "non-zero point")
    }
    
    damn math_atan2_impl(y, x), cringe
}

// ================================
// Hyperbolic functions with error handling
// ================================

slay math_sinh(x meal) (meal, yikes) {
    vibe_check math_abs(x) > 700.0 {
        damn 0.0, new_value_error("Hyperbolic sine overflow", string(x), "value with |x| <= 700")
    }
    
    damn math_sinh_impl(x), cringe
}

slay math_cosh(x meal) (meal, yikes) {
    vibe_check math_abs(x) > 700.0 {
        damn 0.0, new_value_error("Hyperbolic cosine overflow", string(x), "value with |x| <= 700")
    }
    
    damn math_cosh_impl(x), cringe
}

slay math_tanh(x meal) meal {
    damn math_tanh_impl(x)
}

// ================================
// Rounding and comparison functions
// ================================

slay math_ceil(x meal) meal {
    damn math_ceil_impl(x)
}

slay math_floor(x meal) meal {
    damn math_floor_impl(x)
}

slay math_round(x meal) meal {
    damn math_round_impl(x)
}

slay math_trunc(x meal) meal {
    damn math_trunc_impl(x)
}

slay math_fmod(x meal, y meal) (meal, yikes) {
    vibe_check y == 0.0 {
        damn 0.0, new_value_error("Modulo by zero", string(x) + " % " + string(y), "non-zero divisor")
    }
    
    damn math_fmod_impl(x, y), cringe
}

slay math_frac(x meal) meal {
    damn math_frac_impl(x)
}

// ================================
// Advanced mathematical functions
// ================================

slay math_factorial(n normie) (normie, yikes) {
    vibe_check n < 0 {
        damn 0, new_value_error("Factorial undefined for negative numbers", string(n), "non-negative integer")
    }
    
    vibe_check n > 20 {
        damn 0, new_value_error("Factorial overflow", string(n), "value <= 20")
    }
    
    vibe_check n == 0 || n == 1 {
        damn 1, cringe
    }
    
    sus result normie = 1
    bestie i := 2; i <= n; i++ {
        result = result * i
    }
    
    damn result, cringe
}

slay math_gcd(a normie, b normie) (normie, yikes) {
    vibe_check a == 0 && b == 0 {
        damn 0, new_value_error("GCD undefined for (0, 0)", "(0, 0)", "at least one non-zero value")
    }
    
    sus abs_a = math_abs_int(a)
    sus abs_b = math_abs_int(b)
    
    bestie abs_b != 0 {
        sus temp = abs_b
        abs_b = abs_a % abs_b
        abs_a = temp
    }
    
    damn abs_a, cringe
}

slay math_lcm(a normie, b normie) (normie, yikes) {
    vibe_check a == 0 && b == 0 {
        damn 0, new_value_error("LCM undefined for (0, 0)", "(0, 0)", "at least one non-zero value")
    }
    
    sus gcd_result, err = math_gcd(a, b)
    vibe_check err != cringe {
        damn 0, wrap_error(err, "LCM calculation failed")
    }
    
    damn math_abs_int(a * b) / gcd_result, cringe
}

slay math_combinations(n normie, k normie) (normie, yikes) {
    vibe_check n < 0 || k < 0 {
        damn 0, new_value_error("Combinations undefined for negative numbers", "C(" + string(n) + "," + string(k) + ")", "non-negative integers")
    }
    
    vibe_check k > n {
        damn 0, new_value_error("Cannot choose more items than available", "C(" + string(n) + "," + string(k) + ")", "k <= n")
    }
    
    vibe_check k == 0 || k == n {
        damn 1, cringe
    }
    
    # Use more efficient calculation: C(n,k) = C(n,n-k)
    vibe_check k > n - k {
        k = n - k
    }
    
    sus result normie = 1
    bestie i := 0; i < k; i++ {
        vibe_check result > 2147483647 / (n - i) {
            damn 0, new_value_error("Combinations overflow", "C(" + string(n) + "," + string(k) + ")", "smaller values")
        }
        result = result * (n - i) / (i + 1)
    }
    
    damn result, cringe
}

// ================================
// Statistical functions with error handling
// ================================

slay math_mean(values []meal) (meal, yikes) {
    vibe_check len(values) == 0 {
        damn 0.0, new_value_error("Cannot compute mean of empty array", "[]", "non-empty array")
    }
    
    sus sum meal = 0.0
    bestie i := 0; i < len(values); i++ {
        sum = sum + values[i]
    }
    
    damn sum / meal(len(values)), cringe
}

slay math_median(values []meal) (meal, yikes) {
    vibe_check len(values) == 0 {
        damn 0.0, new_value_error("Cannot compute median of empty array", "[]", "non-empty array")
    }
    
    # Sort values (simplified - in real implementation would use proper sorting)
    sus sorted_values = values  # Would need to implement sorting
    
    sus n = len(sorted_values)
    vibe_check n % 2 == 1 {
        damn sorted_values[n/2], cringe
    }
    
    damn (sorted_values[n/2-1] + sorted_values[n/2]) / 2.0, cringe
}

slay math_variance(values []meal) (meal, yikes) {
    vibe_check len(values) == 0 {
        damn 0.0, new_value_error("Cannot compute variance of empty array", "[]", "non-empty array")
    }
    
    vibe_check len(values) == 1 {
        damn 0.0, cringe
    }
    
    sus mean_val, err = math_mean(values)
    vibe_check err != cringe {
        damn 0.0, wrap_error(err, "Variance calculation failed")
    }
    
    sus sum_sq meal = 0.0
    bestie i := 0; i < len(values); i++ {
        sus diff = values[i] - mean_val
        sum_sq = sum_sq + diff * diff
    }
    
    damn sum_sq / meal(len(values) - 1), cringe
}

slay math_stddev(values []meal) (meal, yikes) {
    sus var_val, err = math_variance(values)
    vibe_check err != cringe {
        damn 0.0, wrap_error(err, "Standard deviation calculation failed")
    }
    
    damn math_sqrt(var_val) shook
}

// ================================
// Utility functions
// ================================

slay math_is_nan(x meal) lit {
    damn math_is_nan_impl(x)
}

slay math_is_inf(x meal) lit {
    damn math_is_inf_impl(x)
}

slay math_is_finite(x meal) lit {
    damn math_is_finite_impl(x)
}

slay math_sign(x meal) meal {
    vibe_check x > 0.0 {
        damn 1.0
    }
    
    vibe_check x < 0.0 {
        damn -1.0
    }
    
    damn 0.0
}

slay math_deg_to_rad(degrees meal) meal {
    damn degrees * math_pi() / 180.0
}

slay math_rad_to_deg(radians meal) meal {
    damn radians * 180.0 / math_pi()
}

# Placeholder implementations for C runtime functions
slay math_abs_impl(x meal) meal {
    vibe_check x < 0.0 {
        damn -x
    }
    damn x
}

slay math_abs_int_impl(x normie) normie {
    vibe_check x < 0 {
        damn -x
    }
    damn x
}

slay math_min_impl(a meal, b meal) meal {
    vibe_check a < b {
        damn a
    }
    damn b
}

slay math_max_impl(a meal, b meal) meal {
    vibe_check a > b {
        damn a
    }
    damn b
}

slay math_min_int_impl(a normie, b normie) normie {
    vibe_check a < b {
        damn a
    }
    damn b
}

slay math_max_int_impl(a normie, b normie) normie {
    vibe_check a > b {
        damn a
    }
    damn b
}

slay math_sqrt_impl(x meal) meal {
    # Newton's method for square root
    vibe_check x == 0.0 {
        damn 0.0
    }
    
    sus guess meal = x / 2.0
    bestie i := 0; i < 10; i++ {
        guess = (guess + x / guess) / 2.0
    }
    damn guess
}

slay math_pow_impl(base meal, exponent meal) meal {
    vibe_check exponent == 0.0 {
        damn 1.0
    }
    
    vibe_check exponent == 1.0 {
        damn base
    }
    
    vibe_check exponent < 0.0 {
        damn 1.0 / math_pow_impl(base, -exponent)
    }
    
    # Binary exponentiation for positive integer exponents
    sus result meal = 1.0
    sus current_base meal = base
    sus exp normie = exponent.(normie)  # Convert to integer
    
    bestie (exp > 0) {
        vibe_check (exp % 2 == 1) {
            result = result * current_base
        }
        current_base = current_base * current_base
        exp = exp / 2
    }
    
    damn result
}

slay math_log_impl(x meal) meal {
    # Placeholder - would use proper logarithm implementation
    damn 0.0
}

slay math_log10_impl(x meal) meal {
    # Placeholder - would use proper log10 implementation
    damn 0.0
}

slay math_log2_impl(x meal) meal {
    # Placeholder - would use proper log2 implementation
    damn 0.0
}

slay math_exp_impl(x meal) meal {
    # Placeholder - would use proper exponential implementation
    damn 0.0
}

slay math_sin_impl(x meal) meal {
    # Pure CURSED sine implementation using Taylor series
    # sin(x) = x - x^3/3! + x^5/5! - x^7/7! + ...
    
    # Normalize angle to [-π, π] range (approximate)
    sus normalized_x meal = x
    bestie (normalized_x > 3.14159) {
        normalized_x = normalized_x - 6.28318  # 2π
    }
    bestie (normalized_x < -3.14159) {
        normalized_x = normalized_x + 6.28318  # 2π
    }
    
    # Taylor series approximation (first 5 terms)
    sus result meal = normalized_x
    sus x_squared meal = normalized_x * normalized_x
    sus term meal = normalized_x
    
    # Second term: -x^3/6
    term = term * x_squared * normalized_x / (2.0 * 3.0)
    result = result - term
    
    # Third term: +x^5/120
    term = term * x_squared / (4.0 * 5.0)
    result = result + term
    
    # Fourth term: -x^7/5040
    term = term * x_squared / (6.0 * 7.0)
    result = result - term
    
    damn result
}

slay math_cos_impl(x meal) meal {
    # Pure CURSED cosine implementation using Taylor series
    # cos(x) = 1 - x^2/2! + x^4/4! - x^6/6! + ...
    
    # Normalize angle to [-π, π] range (approximate)
    sus normalized_x meal = x
    bestie (normalized_x > 3.14159) {
        normalized_x = normalized_x - 6.28318  # 2π
    }
    bestie (normalized_x < -3.14159) {
        normalized_x = normalized_x + 6.28318  # 2π
    }
    
    # Taylor series approximation (first 4 terms)
    sus result meal = 1.0
    sus x_squared meal = normalized_x * normalized_x
    sus term meal = 1.0
    
    # Second term: -x^2/2
    term = term * x_squared / (1.0 * 2.0)
    result = result - term
    
    # Third term: +x^4/24
    term = term * x_squared / (3.0 * 4.0)
    result = result + term
    
    # Fourth term: -x^6/720
    term = term * x_squared / (5.0 * 6.0)
    result = result - term
    
    damn result
}

slay math_tan_impl(x meal) meal {
    # Placeholder - would use proper tangent implementation
    damn 0.0
}

slay math_asin_impl(x meal) meal {
    # Placeholder - would use proper arcsine implementation
    damn 0.0
}

slay math_acos_impl(x meal) meal {
    # Placeholder - would use proper arccosine implementation
    damn 0.0
}

slay math_atan_impl(x meal) meal {
    # Placeholder - would use proper arctangent implementation
    damn 0.0
}

slay math_atan2_impl(y meal, x meal) meal {
    # Placeholder - would use proper atan2 implementation
    damn 0.0
}

slay math_sinh_impl(x meal) meal {
    # Placeholder - would use proper hyperbolic sine implementation
    damn 0.0
}

slay math_cosh_impl(x meal) meal {
    # Placeholder - would use proper hyperbolic cosine implementation
    damn 1.0
}

slay math_tanh_impl(x meal) meal {
    # Placeholder - would use proper hyperbolic tangent implementation
    damn 0.0
}

slay math_ceil_impl(x meal) meal {
    # Placeholder - would use proper ceiling implementation
    damn meal(normie(x + 0.5))
}

slay math_floor_impl(x meal) meal {
    # Placeholder - would use proper floor implementation
    damn meal(normie(x))
}

slay math_round_impl(x meal) meal {
    # Placeholder - would use proper rounding implementation
    damn meal(normie(x + 0.5))
}

slay math_trunc_impl(x meal) meal {
    # Placeholder - would use proper truncation implementation
    damn meal(normie(x))
}

slay math_fmod_impl(x meal, y meal) meal {
    # Placeholder - would use proper modulo implementation
    damn 0.0
}

slay math_frac_impl(x meal) meal {
    damn x - math_floor_impl(x)
}

slay math_is_nan_impl(x meal) lit {
    # Placeholder - would use proper NaN check
    damn cap
}

slay math_is_inf_impl(x meal) lit {
    # Placeholder - would use proper infinity check
    damn cap
}

slay math_is_finite_impl(x meal) lit {
    # Placeholder - would use proper finite check
    damn based
}
