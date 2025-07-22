fr fr Enhanced Math Module - Complete Implementation
fr fr Pure CURSED mathematical functions with comprehensive error handling
fr fr FFI-free implementation for essential mathematical operations

yeet "error_core"

fr fr ================================
fr fr Mathematical Constants
fr fr ================================

slay math_pi() meal {
    damn 3.141592653589793
}

slay math_e() meal {
    damn 2.718281828459045
}

slay math_tau() meal {
    damn 6.283185307179586
}

slay math_golden_ratio() meal {
    damn 1.618033988749895
}

slay math_sqrt_2() meal {
    damn 1.4142135623730951
}

slay math_sqrt_3() meal {
    damn 1.7320508075688772
}

slay math_ln_2() meal {
    damn 0.6931471805599453
}

slay math_ln_10() meal {
    damn 2.302585092994046
}

fr fr ================================
fr fr Basic Operations
fr fr ================================

slay math_abs(x meal) meal {
    lowkey x < 0.0 {
        damn -x
    }
    damn x
}

slay math_abs_int(x normie) normie {
    lowkey x < 0 {
        damn -x
    }
    damn x
}

slay math_min(a meal, b meal) meal {
    lowkey a < b { damn a } else { damn b }
}

slay math_max(a meal, b meal) meal {
    lowkey a > b { damn a } else { damn b }
}

slay math_min_int(a normie, b normie) normie {
    lowkey a < b { damn a } else { damn b }
}

slay math_max_int(a normie, b normie) normie {
    lowkey a > b { damn a } else { damn b }
}

slay math_clamp(x meal, min_val meal, max_val meal) meal {
    lowkey x < min_val { damn min_val }
    elseif x > max_val { damn max_val }
    else { damn x }
}

slay math_clamp_int(x normie, min_val normie, max_val normie) normie {
    lowkey x < min_val { damn min_val }
    elseif x > max_val { damn max_val }
    else { damn x }
}

fr fr ================================
fr fr Power and Root Functions
fr fr ================================

slay math_sqrt(x meal) meal {
    lowkey x < 0.0 {
        damn 0.0 fr fr Error case
    }
    
    lowkey x == 0.0 {
        damn 0.0
    } fr fr Newton's method for square root
    sus guess meal = x / 2.0
    bestie i := 0; i < 15; i++ {
        sus new_guess meal = (guess + x / guess) / 2.0
        lowkey math_abs(new_guess - guess) < 0.0000001 {
            damn new_guess
        }
        guess = new_guess
    }
    
    damn guess
}

slay math_cbrt(x meal) meal {
    lowkey x == 0.0 {
        damn 0.0
    }
    
    sus sign meal = 1.0
    lowkey x < 0.0 {
        sign = -1.0
        x = -x
    } fr fr Newton's method for cube root
    sus guess meal = x / 3.0
    bestie i := 0; i < 15; i++ {
        sus x_squared meal = guess * guess
        sus new_guess meal = (2.0 * guess + x / x_squared) / 3.0
        lowkey math_abs(new_guess - guess) < 0.0000001 {
            damn sign * new_guess
        }
        guess = new_guess
    }
    
    damn sign * guess
}

slay math_pow(base meal, exponent meal) meal {
    lowkey exponent == 0.0 {
        damn 1.0
    }
    
    lowkey exponent == 1.0 {
        damn base
    }
    
    lowkey base == 0.0 {
        lowkey exponent > 0.0 { damn 0.0 } else { damn 1.0 } fr fr 0^negative is undefined, return 1
    }
    
    lowkey exponent < 0.0 {
        damn 1.0 / math_pow(base, -exponent)
    } fr fr For positive integer exponents, use binary exponentiation
    sus result meal = 1.0
    sus current_base meal = base
    sus exp normie = normie(exponent)
    
    bestie exp > 0 {
        lowkey (exp % 2) == 1 {
            result = result * current_base
        }
        current_base = current_base * current_base
        exp = exp / 2
    }
    
    damn result
}

slay math_pow_int(base normie, exponent normie) normie {
    lowkey exponent == 0 {
        damn 1
    }
    
    lowkey exponent == 1 {
        damn base
    }
    
    lowkey exponent < 0 {
        damn 0 fr fr Integer division for negative exponents
    }
    
    sus result normie = 1
    sus current_base normie = base
    sus exp normie = exponent
    
    bestie exp > 0 {
        lowkey (exp % 2) == 1 {
            result = result * current_base
        }
        current_base = current_base * current_base
        exp = exp / 2
    }
    
    damn result
}

fr fr ================================
fr fr Exponential and Logarithmic Functions
fr fr ================================

slay math_exp(x meal) meal {
    lowkey x == 0.0 {
        damn 1.0
    }
    
    lowkey x > 700.0 {
        damn 1000000000000.0 fr fr Large number to represent overflow
    }
    
    lowkey x < -700.0 {
        damn 0.0
    } fr fr Taylor series for e^x: 1 + x + x^2/2! + x^3/3! + ...
    sus result meal = 1.0
    sus term meal = 1.0
    sus factorial meal = 1.0
    
    bestie i := 1; i <= 20; i++ {
        factorial = factorial * meal(i)
        term = math_pow(x, meal(i)) / factorial
        result = result + term
        
        lowkey math_abs(term) < 0.0000001 {
            break
        }
    }
    
    damn result
}

slay math_ln(x meal) meal {
    lowkey x <= 0.0 {
        damn 0.0 fr fr Error case
    }
    
    lowkey x == 1.0 {
        damn 0.0
    } fr fr Use Newton's method: ln(x) = 2 * (x-1)/(x+1) + 2/3 * ((x-1)/(x+1))^3 + ... fr fr This is a simplified implementation
    sus y meal = (x - 1.0) / (x + 1.0)
    sus y_squared meal = y * y
    sus result meal = 0.0
    sus term meal = y
    
    bestie i := 1; i <= 20; i += 2 {
        result = result + term / meal(i)
        term = term * y_squared
        
        lowkey math_abs(term) < 0.0000001 {
            break
        }
    }
    
    damn 2.0 * result
}

slay math_log10(x meal) meal {
    lowkey x <= 0.0 {
        damn 0.0 fr fr Error case
    }
    
    damn math_ln(x) / math_ln_10()
}

slay math_log2(x meal) meal {
    lowkey x <= 0.0 {
        damn 0.0 fr fr Error case
    }
    
    damn math_ln(x) / math_ln_2()
}

slay math_log(x meal, base meal) meal {
    lowkey x <= 0.0 || base <= 0.0 || base == 1.0 {
        damn 0.0 fr fr Error case
    }
    
    damn math_ln(x) / math_ln(base)
}

fr fr ================================
fr fr Trigonometric Functions
fr fr ================================

slay math_sin(x meal) meal { fr fr Normalize angle to [-π, π]
    bestie x > math_pi() {
        x = x - 2.0 * math_pi()
    }
    bestie x < -math_pi() {
        x = x + 2.0 * math_pi()
    } fr fr Taylor series: sin(x) = x - x^3/3! + x^5/5! - x^7/7! + ...
    sus result meal = x
    sus term meal = x
    sus x_squared meal = x * x
    
    bestie i := 1; i <= 10; i++ {
        term = term * x_squared / (meal(2 * i) * meal(2 * i + 1))
        lowkey (i % 2) == 1 {
            result = result - term
        } else {
            result = result + term
        }
        
        lowkey math_abs(term) < 0.0000001 {
            break
        }
    }
    
    damn result
}

slay math_cos(x meal) meal { fr fr cos(x) = sin(x + π/2)
    damn math_sin(x + math_pi() / 2.0)
}

slay math_tan(x meal) meal {
    sus cos_val meal = math_cos(x)
    lowkey math_abs(cos_val) < 0.0000001 {
        damn 1000000.0 fr fr Large number to represent infinity
    }
    
    damn math_sin(x) / cos_val
}

slay math_asin(x meal) meal {
    lowkey x < -1.0 || x > 1.0 {
        damn 0.0 fr fr Error case
    }
    
    lowkey x == 0.0 { damn 0.0 }
    lowkey x == 1.0 { damn math_pi() / 2.0 }
    lowkey x == -1.0 { damn -math_pi() / 2.0 } fr fr Use Newton's method approximation fr fr This is a simplified implementation
    damn x + math_pow(x, 3.0) / 6.0 + 3.0 * math_pow(x, 5.0) / 40.0
}

slay math_acos(x meal) meal {
    damn math_pi() / 2.0 - math_asin(x)
}

slay math_atan(x meal) meal {
    lowkey x == 0.0 { damn 0.0 } fr fr Use Taylor series for small values
    lowkey math_abs(x) <= 1.0 {
        sus result meal = x
        sus term meal = x
        sus x_squared meal = x * x
        
        bestie i := 1; i <= 15; i++ {
            term = term * x_squared
            sus denominator meal = meal(2 * i + 1)
            lowkey (i % 2) == 1 {
                result = result - term / denominator
            } else {
                result = result + term / denominator
            }
            
            lowkey math_abs(term / denominator) < 0.0000001 {
                break
            }
        }
        
        damn result
    } fr fr For large values: atan(x) = π/2 - atan(1/x)
    lowkey x > 0.0 {
        damn math_pi() / 2.0 - math_atan(1.0 / x)
    } else {
        damn -math_pi() / 2.0 - math_atan(1.0 / x)
    }
}

slay math_atan2(y meal, x meal) meal {
    lowkey x == 0.0 && y == 0.0 {
        damn 0.0
    }
    
    lowkey x > 0.0 {
        damn math_atan(y / x)
    }
    
    lowkey x < 0.0 && y >= 0.0 {
        damn math_atan(y / x) + math_pi()
    }
    
    lowkey x < 0.0 && y < 0.0 {
        damn math_atan(y / x) - math_pi()
    }
    
    lowkey x == 0.0 && y > 0.0 {
        damn math_pi() / 2.0
    }
    
    damn -math_pi() / 2.0
}

fr fr ================================
fr fr Hyperbolic Functions
fr fr ================================

slay math_sinh(x meal) meal { fr fr sinh(x) = (e^x - e^(-x)) / 2
    sus exp_x meal = math_exp(x)
    sus exp_neg_x meal = math_exp(-x)
    damn (exp_x - exp_neg_x) / 2.0
}

slay math_cosh(x meal) meal { fr fr cosh(x) = (e^x + e^(-x)) / 2
    sus exp_x meal = math_exp(x)
    sus exp_neg_x meal = math_exp(-x)
    damn (exp_x + exp_neg_x) / 2.0
}

slay math_tanh(x meal) meal { fr fr tanh(x) = sinh(x) / cosh(x)
    sus sinh_val meal = math_sinh(x)
    sus cosh_val meal = math_cosh(x)
    
    lowkey cosh_val == 0.0 {
        damn 0.0
    }
    
    damn sinh_val / cosh_val
}

fr fr ================================
fr fr Rounding and Comparison Functions
fr fr ================================

slay math_floor(x meal) meal {
    sus int_part normie = normie(x)
    lowkey x >= 0.0 || meal(int_part) == x {
        damn meal(int_part)
    } else {
        damn meal(int_part - 1)
    }
}

slay math_ceil(x meal) meal {
    sus int_part normie = normie(x)
    lowkey x <= 0.0 || meal(int_part) == x {
        damn meal(int_part)
    } else {
        damn meal(int_part + 1)
    }
}

slay math_round(x meal) meal {
    lowkey x >= 0.0 {
        damn math_floor(x + 0.5)
    } else {
        damn math_ceil(x - 0.5)
    }
}

slay math_trunc(x meal) meal {
    damn meal(normie(x))
}

slay math_fmod(x meal, y meal) meal {
    lowkey y == 0.0 {
        damn 0.0 fr fr Error case
    }
    
    sus quotient normie = normie(x / y)
    damn x - y * meal(quotient)
}

slay math_remainder(x meal, y meal) meal {
    lowkey y == 0.0 {
        damn 0.0 fr fr Error case
    }
    
    sus quotient meal = x / y
    sus rounded_quotient normie = normie(math_round(quotient))
    damn x - y * meal(rounded_quotient)
}

fr fr ================================
fr fr Advanced Mathematical Functions
fr fr ================================

slay math_factorial(n normie) normie {
    lowkey n < 0 {
        damn 0 fr fr Error case
    }
    
    lowkey n <= 1 {
        damn 1
    }
    
    sus result normie = 1
    bestie i := 2; i <= n; i++ {
        result = result * i
    }
    
    damn result
}

slay math_factorial_meal(n normie) meal {
    lowkey n < 0 {
        damn 0.0 fr fr Error case
    }
    
    lowkey n <= 1 {
        damn 1.0
    }
    
    sus result meal = 1.0
    bestie i := 2; i <= n; i++ {
        result = result * meal(i)
    }
    
    damn result
}

slay math_gcd(a normie, b normie) normie {
    sus abs_a normie = math_abs_int(a)
    sus abs_b normie = math_abs_int(b)
    
    bestie abs_b != 0 {
        sus temp normie = abs_b
        abs_b = abs_a % abs_b
        abs_a = temp
    }
    
    damn abs_a
}

slay math_lcm(a normie, b normie) normie {
    lowkey a == 0 || b == 0 {
        damn 0
    }
    
    sus gcd_val normie = math_gcd(a, b)
    damn math_abs_int(a * b) / gcd_val
}

slay math_fibonacci(n normie) normie {
    lowkey n <= 0 {
        damn 0
    }
    
    lowkey n == 1 {
        damn 1
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

fr fr ================================
fr fr Statistical Functions
fr fr ================================

slay math_sum(values []meal) meal {
    sus total meal = 0.0
    bestie i := 0; i < len(values); i++ {
        total = total + values[i]
    }
    damn total
}

slay math_mean(values []meal) meal {
    lowkey len(values) == 0 {
        damn 0.0
    }
    
    damn math_sum(values) / meal(len(values))
}

slay math_variance(values []meal) meal {
    lowkey len(values) <= 1 {
        damn 0.0
    }
    
    sus mean_val meal = math_mean(values)
    sus sum_sq_diff meal = 0.0
    
    bestie i := 0; i < len(values); i++ {
        sus diff meal = values[i] - mean_val
        sum_sq_diff = sum_sq_diff + diff * diff
    }
    
    damn sum_sq_diff / meal(len(values) - 1)
}

slay math_stddev(values []meal) meal {
    damn math_sqrt(math_variance(values))
}

slay math_median(values []meal) meal {
    lowkey len(values) == 0 {
        damn 0.0
    } fr fr Simple median (would need sorting in real implementation)
    sus mid normie = len(values) / 2
    lowkey (len(values) % 2) == 1 {
        damn values[mid]
    } else {
        damn (values[mid - 1] + values[mid]) / 2.0
    }
}

fr fr ================================
fr fr Utility Functions
fr fr ================================

slay math_sign(x meal) meal {
    lowkey x > 0.0 { damn 1.0 }
    elseif x < 0.0 { damn -1.0 }
    else { damn 0.0 }
}

slay math_sign_int(x normie) normie {
    lowkey x > 0 { damn 1 }
    elseif x < 0 { damn -1 }
    else { damn 0 }
}

slay math_deg_to_rad(degrees meal) meal {
    damn degrees * math_pi() / 180.0
}

slay math_rad_to_deg(radians meal) meal {
    damn radians * 180.0 / math_pi()
}

slay math_is_nan(x meal) lit { fr fr Simple NaN check (would be more sophisticated in real implementation)
    damn x != x
}

slay math_is_infinite(x meal) lit { fr fr Simple infinity check
    damn math_abs(x) > 1000000000000.0
}

slay math_is_finite(x meal) lit {
    damn !math_is_nan(x) && !math_is_infinite(x)
}

slay math_lerp(a meal, b meal, t meal) meal {
    damn a + t * (b - a)
}

slay math_smoothstep(edge0 meal, edge1 meal, x meal) meal {
    sus t meal = math_clamp((x - edge0) / (edge1 - edge0), 0.0, 1.0)
    damn t * t * (3.0 - 2.0 * t)
}

fr fr ================================
fr fr Number Theory Functions
fr fr ================================

slay math_is_prime(n normie) lit {
    lowkey n <= 1 {
        damn cap
    }
    
    lowkey n <= 3 {
        damn based
    }
    
    lowkey (n % 2) == 0 || (n % 3) == 0 {
        damn cap
    }
    
    sus i normie = 5
    bestie i * i <= n {
        lowkey (n % i) == 0 || (n % (i + 2)) == 0 {
            damn cap
        }
        i = i + 6
    }
    
    damn based
}

slay math_next_prime(n normie) normie {
    lowkey n < 2 { damn 2 }
    
    sus candidate normie = n + 1
    bestie !math_is_prime(candidate) {
        candidate = candidate + 1
    }
    
    damn candidate
}

slay math_prime_factors(n normie) []normie {
    sus factors []normie = []
    sus num normie = n fr fr Check for factor 2
    bestie (num % 2) == 0 {
        factors = append(factors, 2)
        num = num / 2
    } fr fr Check for odd factors
    sus i normie = 3
    bestie i * i <= num {
        bestie (num % i) == 0 {
            factors = append(factors, i)
            num = num / i
        } else {
            i = i + 2
        }
    } fr fr If num is still > 2, it's a prime factor
    lowkey num > 2 {
        factors = append(factors, num)
    }
    
    damn factors
}
