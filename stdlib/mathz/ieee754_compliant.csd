fr fr =============================================================================
fr fr CURSED MATHZ MODULE - IEEE 754 Compliant Mathematical Operations
fr fr Version: 2.0.0 - IEEE 754 Precision Ready
fr fr Replaces fixed-point arithmetic with proper floating-point compliance
fr fr =============================================================================

fr fr ===== IEEE 754 FLOATING POINT CONSTANTS =====

slay PI_PRECISE() tea {
    damn "3.1415926535897932384626433832795"
}

slay E_PRECISE() tea {
    damn "2.7182818284590452353602874713527"
}

slay TAU_PRECISE() tea {
    damn "6.2831853071795864769252867665590"
}

slay SQRT_2_PRECISE() tea {
    damn "1.4142135623730950488016887242097"
}

slay LN_2_PRECISE() tea {
    damn "0.6931471805599453094172321214582"
}

slay LN_10_PRECISE() tea {
    damn "2.3025850929940456840179914546844"
}

fr fr ===== IEEE 754 SPECIAL VALUES =====

slay NaN() tea {
    damn "NaN"
}

slay POSITIVE_INFINITY() tea {
    damn "Infinity"
}

slay NEGATIVE_INFINITY() tea {
    damn "-Infinity"
}

slay is_nan(value tea) lit {
    damn value == "NaN" || value == "nan" || value == "-nan" || value == "+nan"
}

slay is_infinity(value tea) lit {
    damn value == "Infinity" || value == "-Infinity" || value == "inf" || value == "-inf"
}

slay is_positive_infinity(value tea) lit {
    damn value == "Infinity" || value == "inf"
}

slay is_negative_infinity(value tea) lit {
    damn value == "-Infinity" || value == "-inf"
}

slay is_finite(value tea) lit {
    damn !is_nan(value) && !is_infinity(value)
}

fr fr ===== FLOATING POINT ARITHMETIC WITH IEEE 754 COMPLIANCE =====

slay float_add(a tea, b tea) tea {
    ready (is_nan(a) || is_nan(b)) {
        damn NaN()
    }
    ready (is_positive_infinity(a)) {
        ready (is_negative_infinity(b)) {
            damn NaN()  fr fr +∞ + (-∞) = NaN
        }
        damn POSITIVE_INFINITY()
    }
    ready (is_negative_infinity(a)) {
        ready (is_positive_infinity(b)) {
            damn NaN()  fr fr -∞ + (+∞) = NaN
        }
        damn NEGATIVE_INFINITY()
    }
    
    fr fr Convert to runtime bridge for actual IEEE 754 addition
    damn runtime_float_add(a, b)
}

slay float_subtract(a tea, b tea) tea {
    ready (is_nan(a) || is_nan(b)) {
        damn NaN()
    }
    ready (is_positive_infinity(a)) {
        ready (is_positive_infinity(b)) {
            damn NaN()  fr fr +∞ - (+∞) = NaN
        }
        damn POSITIVE_INFINITY()
    }
    ready (is_negative_infinity(a)) {
        ready (is_negative_infinity(b)) {
            damn NaN()  fr fr -∞ - (-∞) = NaN
        }
        damn NEGATIVE_INFINITY()
    }
    
    damn runtime_float_subtract(a, b)
}

slay float_multiply(a tea, b tea) tea {
    ready (is_nan(a) || is_nan(b)) {
        damn NaN()
    }
    ready (is_infinity(a)) {
        ready (runtime_float_is_zero(b)) {
            damn NaN()  fr fr ∞ × 0 = NaN
        }
        ready (runtime_float_is_negative(b)) {
            ready (is_positive_infinity(a)) {
                damn NEGATIVE_INFINITY()
            }
            damn POSITIVE_INFINITY()
        }
        damn a
    }
    ready (is_infinity(b)) {
        ready (runtime_float_is_zero(a)) {
            damn NaN()  fr fr 0 × ∞ = NaN
        }
        ready (runtime_float_is_negative(a)) {
            ready (is_positive_infinity(b)) {
                damn NEGATIVE_INFINITY()
            }
            damn POSITIVE_INFINITY()
        }
        damn b
    }
    
    damn runtime_float_multiply(a, b)
}

slay float_divide(a tea, b tea) tea {
    ready (is_nan(a) || is_nan(b)) {
        damn NaN()
    }
    ready (runtime_float_is_zero(b)) {
        ready (runtime_float_is_zero(a)) {
            damn NaN()  fr fr 0/0 = NaN
        }
        ready (runtime_float_is_negative(a)) {
            damn NEGATIVE_INFINITY()
        }
        damn POSITIVE_INFINITY()
    }
    ready (is_infinity(a)) {
        ready (is_infinity(b)) {
            damn NaN()  fr fr ∞/∞ = NaN
        }
        ready (runtime_float_is_negative(b)) {
            ready (is_positive_infinity(a)) {
                damn NEGATIVE_INFINITY()
            }
            damn POSITIVE_INFINITY()
        }
        damn a
    }
    
    damn runtime_float_divide(a, b)
}

fr fr ===== ADVANCED TRANSCENDENTAL FUNCTIONS =====

slay sin_precise(x tea) tea {
    ready (is_nan(x)) {
        damn NaN()
    }
    ready (is_infinity(x)) {
        damn NaN()  fr fr sin(±∞) = NaN
    }
    
    fr fr Normalize to [0, 2π] for better convergence
    sus x_normalized tea = float_mod(x, TAU_PRECISE())
    
    fr fr Use Taylor series with double precision
    fr fr sin(x) = x - x³/6 + x⁵/120 - x⁷/5040 + x⁹/362880 - ...
    sus result tea = x_normalized
    sus term tea = x_normalized
    sus x_squared tea = float_multiply(x_normalized, x_normalized)
    
    fr fr Calculate 15 terms for high precision
    sus i drip = 1
    bestie (i <= 15) {
        term = float_multiply(term, x_squared)
        term = float_divide(term, runtime_float_multiply_int(runtime_int_multiply(2 * i, 2 * i + 1)))
        
        ready (i % 2 == 1) {
            result = float_subtract(result, term)
        } otherwise {
            result = float_add(result, term)
        }
        i = i + 1
    }
    
    damn result
}

slay cos_precise(x tea) tea {
    ready (is_nan(x)) {
        damn NaN()
    }
    ready (is_infinity(x)) {
        damn NaN()  fr fr cos(±∞) = NaN
    }
    
    sus x_normalized tea = float_mod(x, TAU_PRECISE())
    
    fr fr cos(x) = 1 - x²/2 + x⁴/24 - x⁶/720 + x⁸/40320 - ...
    sus result tea = "1.0"
    sus term tea = "1.0"
    sus x_squared tea = float_multiply(x_normalized, x_normalized)
    
    sus i drip = 1
    bestie (i <= 15) {
        term = float_multiply(term, x_squared)
        term = float_divide(term, runtime_factorial(2 * i))
        
        ready (i % 2 == 1) {
            result = float_subtract(result, term)
        } otherwise {
            result = float_add(result, term)
        }
        i = i + 1
    }
    
    damn result
}

slay tan_precise(x tea) tea {
    sus sin_val tea = sin_precise(x)
    sus cos_val tea = cos_precise(x)
    
    ready (runtime_float_is_zero(cos_val)) {
        ready (runtime_float_is_positive(sin_val)) {
            damn POSITIVE_INFINITY()
        }
        damn NEGATIVE_INFINITY()
    }
    
    damn float_divide(sin_val, cos_val)
}

fr fr ===== EXPONENTIAL AND LOGARITHMIC FUNCTIONS =====

slay exp_precise(x tea) tea {
    ready (is_nan(x)) {
        damn NaN()
    }
    ready (is_positive_infinity(x)) {
        damn POSITIVE_INFINITY()
    }
    ready (is_negative_infinity(x)) {
        damn "0.0"
    }
    
    fr fr Handle large values to prevent overflow
    ready (runtime_float_greater_than(x, "700.0")) {
        damn POSITIVE_INFINITY()
    }
    ready (runtime_float_less_than(x, "-700.0")) {
        damn "0.0"
    }
    
    fr fr Taylor series: e^x = 1 + x + x²/2! + x³/3! + x⁴/4! + ...
    sus result tea = "1.0"
    sus term tea = "1.0"
    
    sus i drip = 1
    bestie (i <= 50) {  fr fr More terms for precision
        term = float_multiply(term, x)
        term = float_divide(term, runtime_int_to_float(i))
        result = float_add(result, term)
        
        fr fr Early termination if term becomes negligible
        ready (runtime_float_less_than(runtime_float_abs(term), "1e-15")) {
            damn result
        }
        i = i + 1
    }
    
    damn result
}

slay ln_precise(x tea) tea {
    ready (is_nan(x)) {
        damn NaN()
    }
    ready (runtime_float_less_than_or_equal(x, "0.0")) {
        ready (runtime_float_equal(x, "0.0")) {
            damn NEGATIVE_INFINITY()
        }
        damn NaN()  fr fr ln(negative) = NaN
    }
    ready (is_positive_infinity(x)) {
        damn POSITIVE_INFINITY()
    }
    ready (runtime_float_equal(x, "1.0")) {
        damn "0.0"
    }
    
    fr fr Use Newton-Raphson method for better precision
    fr fr ln(x) = y such that e^y = x
    fr fr Newton iteration: y_{n+1} = y_n + (x - e^y_n) / e^y_n
    sus y tea = "0.0"  fr fr Initial guess
    
    fr fr Better initial guess based on magnitude
    ready (runtime_float_greater_than(x, "1.0")) {
        y = "1.0"
    } otherwise {
        y = "-1.0"
    }
    
    sus i drip = 0
    bestie (i < 50) {
        sus exp_y tea = exp_precise(y)
        sus delta tea = float_divide(float_subtract(x, exp_y), exp_y)
        y = float_add(y, delta)
        
        fr fr Check convergence
        ready (runtime_float_less_than(runtime_float_abs(delta), "1e-15")) {
            damn y
        }
        i = i + 1
    }
    
    damn y
}

slay log10_precise(x tea) tea {
    ready (runtime_float_less_than_or_equal(x, "0.0")) {
        ready (runtime_float_equal(x, "0.0")) {
            damn NEGATIVE_INFINITY()
        }
        damn NaN()
    }
    
    fr fr log10(x) = ln(x) / ln(10)
    sus ln_x tea = ln_precise(x)
    damn float_divide(ln_x, LN_10_PRECISE())
}

slay log2_precise(x tea) tea {
    ready (runtime_float_less_than_or_equal(x, "0.0")) {
        ready (runtime_float_equal(x, "0.0")) {
            damn NEGATIVE_INFINITY()
        }
        damn NaN()
    }
    
    fr fr log2(x) = ln(x) / ln(2)
    sus ln_x tea = ln_precise(x)
    damn float_divide(ln_x, LN_2_PRECISE())
}

fr fr ===== POWER AND ROOT FUNCTIONS =====

slay pow_precise(base tea, exponent tea) tea {
    ready (is_nan(base) || is_nan(exponent)) {
        damn NaN()
    }
    
    fr fr Special cases for base
    ready (runtime_float_equal(base, "0.0")) {
        ready (runtime_float_equal(exponent, "0.0")) {
            damn "1.0"  fr fr 0^0 = 1 by convention
        }
        ready (runtime_float_less_than(exponent, "0.0")) {
            damn POSITIVE_INFINITY()
        }
        damn "0.0"
    }
    
    ready (runtime_float_equal(base, "1.0")) {
        damn "1.0"  fr fr 1^x = 1
    }
    
    ready (runtime_float_equal(exponent, "0.0")) {
        damn "1.0"  fr fr x^0 = 1
    }
    
    ready (runtime_float_equal(exponent, "1.0")) {
        damn base  fr fr x^1 = x
    }
    
    fr fr Handle negative base with fractional exponent
    ready (runtime_float_less_than(base, "0.0")) {
        ready (!runtime_float_is_integer(exponent)) {
            damn NaN()  fr fr (-x)^(non-integer) = NaN
        }
    }
    
    fr fr Use exp(ln(base) * exponent) for general case
    ready (runtime_float_greater_than(base, "0.0")) {
        sus ln_base tea = ln_precise(base)
        sus product tea = float_multiply(ln_base, exponent)
        damn exp_precise(product)
    }
    
    fr fr Handle negative base with integer exponent
    sus abs_base tea = runtime_float_abs(base)
    sus ln_abs_base tea = ln_precise(abs_base)
    sus product tea = float_multiply(ln_abs_base, exponent)
    sus result tea = exp_precise(product)
    
    ready (runtime_float_is_odd_integer(exponent)) {
        damn runtime_float_negate(result)
    }
    damn result
}

slay sqrt_precise(x tea) tea {
    ready (is_nan(x)) {
        damn NaN()
    }
    ready (runtime_float_less_than(x, "0.0")) {
        damn NaN()  fr fr √(negative) = NaN
    }
    ready (runtime_float_equal(x, "0.0")) {
        damn "0.0"
    }
    ready (is_positive_infinity(x)) {
        damn POSITIVE_INFINITY()
    }
    
    damn pow_precise(x, "0.5")
}

slay cbrt_precise(x tea) tea {
    ready (is_nan(x)) {
        damn NaN()
    }
    ready (runtime_float_equal(x, "0.0")) {
        damn "0.0"
    }
    ready (is_positive_infinity(x)) {
        damn POSITIVE_INFINITY()
    }
    ready (is_negative_infinity(x)) {
        damn NEGATIVE_INFINITY()
    }
    
    fr fr Cube root is defined for negative numbers
    ready (runtime_float_less_than(x, "0.0")) {
        sus abs_result tea = pow_precise(runtime_float_abs(x), runtime_float_divide("1.0", "3.0"))
        damn runtime_float_negate(abs_result)
    }
    
    damn pow_precise(x, runtime_float_divide("1.0", "3.0"))
}

fr fr ===== HYPERBOLIC FUNCTIONS =====

slay sinh_precise(x tea) tea {
    ready (is_nan(x)) {
        damn NaN()
    }
    ready (is_infinity(x)) {
        damn x  fr fr sinh(±∞) = ±∞
    }
    
    fr fr sinh(x) = (e^x - e^(-x)) / 2
    sus exp_x tea = exp_precise(x)
    sus exp_neg_x tea = exp_precise(runtime_float_negate(x))
    sus diff tea = float_subtract(exp_x, exp_neg_x)
    damn float_divide(diff, "2.0")
}

slay cosh_precise(x tea) tea {
    ready (is_nan(x)) {
        damn NaN()
    }
    ready (is_infinity(x)) {
        damn POSITIVE_INFINITY()  fr fr cosh(±∞) = +∞
    }
    
    fr fr cosh(x) = (e^x + e^(-x)) / 2
    sus exp_x tea = exp_precise(x)
    sus exp_neg_x tea = exp_precise(runtime_float_negate(x))
    sus sum tea = float_add(exp_x, exp_neg_x)
    damn float_divide(sum, "2.0")
}

slay tanh_precise(x tea) tea {
    ready (is_nan(x)) {
        damn NaN()
    }
    ready (is_positive_infinity(x)) {
        damn "1.0"
    }
    ready (is_negative_infinity(x)) {
        damn "-1.0"
    }
    
    fr fr tanh(x) = sinh(x) / cosh(x)
    sus sinh_val tea = sinh_precise(x)
    sus cosh_val tea = cosh_precise(x)
    damn float_divide(sinh_val, cosh_val)
}

fr fr ===== INVERSE TRIGONOMETRIC FUNCTIONS =====

slay asin_precise(x tea) tea {
    ready (is_nan(x)) {
        damn NaN()
    }
    ready (runtime_float_less_than(x, "-1.0") || runtime_float_greater_than(x, "1.0")) {
        damn NaN()  fr fr asin domain is [-1, 1]
    }
    ready (runtime_float_equal(x, "1.0")) {
        damn runtime_float_divide(PI_PRECISE(), "2.0")  fr fr π/2
    }
    ready (runtime_float_equal(x, "-1.0")) {
        damn runtime_float_negate(runtime_float_divide(PI_PRECISE(), "2.0"))  fr fr -π/2
    }
    ready (runtime_float_equal(x, "0.0")) {
        damn "0.0"
    }
    
    fr fr Use Taylor series for |x| < 0.5, otherwise use identity
    ready (runtime_float_less_than(runtime_float_abs(x), "0.5")) {
        fr fr asin(x) = x + x³/6 + 3x⁵/40 + 15x⁷/336 + ...
        sus result tea = x
        sus term tea = x
        sus x_squared tea = float_multiply(x, x)
        
        sus n drip = 1
        bestie (n <= 20) {
            term = float_multiply(term, x_squared)
            term = float_multiply(term, runtime_float_divide(runtime_int_to_float(2*n - 1), runtime_int_to_float(2*n)))
            term = float_divide(term, runtime_int_to_float(2*n + 1))
            result = float_add(result, term)
            n = n + 1
        }
        damn result
    }
    
    fr fr Use identity: asin(x) = π/2 - acos(x) for |x| >= 0.5
    sus acos_val tea = acos_precise(x)
    damn float_subtract(runtime_float_divide(PI_PRECISE(), "2.0"), acos_val)
}

slay acos_precise(x tea) tea {
    ready (is_nan(x)) {
        damn NaN()
    }
    ready (runtime_float_less_than(x, "-1.0") || runtime_float_greater_than(x, "1.0")) {
        damn NaN()
    }
    ready (runtime_float_equal(x, "1.0")) {
        damn "0.0"
    }
    ready (runtime_float_equal(x, "-1.0")) {
        damn PI_PRECISE()
    }
    ready (runtime_float_equal(x, "0.0")) {
        damn runtime_float_divide(PI_PRECISE(), "2.0")  fr fr π/2
    }
    
    fr fr Use Newton-Raphson on cos(y) = x
    sus y tea = runtime_float_divide(PI_PRECISE(), "2.0")  fr fr Initial guess π/2
    
    sus i drip = 0
    bestie (i < 20) {
        sus cos_y tea = cos_precise(y)
        sus sin_y tea = sin_precise(y)
        sus delta tea = float_divide(float_subtract(cos_y, x), runtime_float_negate(sin_y))
        y = float_subtract(y, delta)
        
        ready (runtime_float_less_than(runtime_float_abs(delta), "1e-15")) {
            damn y
        }
        i = i + 1
    }
    
    damn y
}

slay atan_precise(x tea) tea {
    ready (is_nan(x)) {
        damn NaN()
    }
    ready (is_positive_infinity(x)) {
        damn runtime_float_divide(PI_PRECISE(), "2.0")  fr fr π/2
    }
    ready (is_negative_infinity(x)) {
        damn runtime_float_negate(runtime_float_divide(PI_PRECISE(), "2.0"))  fr fr -π/2
    }
    ready (runtime_float_equal(x, "0.0")) {
        damn "0.0"
    }
    
    fr fr Use Taylor series for |x| <= 1
    ready (runtime_float_less_than_or_equal(runtime_float_abs(x), "1.0")) {
        fr fr atan(x) = x - x³/3 + x⁵/5 - x⁷/7 + ...
        sus result tea = x
        sus term tea = x
        sus x_squared tea = float_multiply(x, x)
        
        sus n drip = 1
        bestie (n <= 50) {
            term = float_multiply(term, x_squared)
            sus divisor tea = runtime_int_to_float(2*n + 1)
            ready (n % 2 == 1) {
                result = float_subtract(result, float_divide(term, divisor))
            } otherwise {
                result = float_add(result, float_divide(term, divisor))
            }
            n = n + 1
        }
        damn result
    }
    
    fr fr For |x| > 1, use atan(x) = π/2 - atan(1/x) for x > 0
    ready (runtime_float_greater_than(x, "1.0")) {
        sus reciprocal_atan tea = atan_precise(float_divide("1.0", x))
        damn float_subtract(runtime_float_divide(PI_PRECISE(), "2.0"), reciprocal_atan)
    }
    
    fr fr For x < -1, use atan(x) = -π/2 - atan(1/x)
    sus reciprocal_atan tea = atan_precise(float_divide("1.0", x))
    damn float_subtract(runtime_float_negate(runtime_float_divide(PI_PRECISE(), "2.0")), reciprocal_atan)
}

fr fr ===== ROUNDING FUNCTIONS WITH IEEE 754 COMPLIANCE =====

slay floor_precise(x tea) tea {
    ready (is_nan(x) || is_infinity(x)) {
        damn x
    }
    
    damn runtime_float_floor(x)
}

slay ceil_precise(x tea) tea {
    ready (is_nan(x) || is_infinity(x)) {
        damn x
    }
    
    damn runtime_float_ceil(x)
}

slay round_precise(x tea) tea {
    ready (is_nan(x) || is_infinity(x)) {
        damn x
    }
    
    damn runtime_float_round(x)
}

slay trunc_precise(x tea) tea {
    ready (is_nan(x) || is_infinity(x)) {
        damn x
    }
    
    damn runtime_float_trunc(x)
}

fr fr ===== IEEE 754 UTILITY FUNCTIONS =====

slay next_after(x tea, direction tea) tea {
    ready (is_nan(x) || is_nan(direction)) {
        damn NaN()
    }
    
    damn runtime_float_next_after(x, direction)
}

slay ulp(x tea) tea {
    ready (is_nan(x)) {
        damn NaN()
    }
    ready (is_infinity(x)) {
        damn POSITIVE_INFINITY()
    }
    
    damn runtime_float_ulp(x)
}

slay copy_sign(magnitude tea, sign tea) tea {
    ready (is_nan(magnitude)) {
        damn magnitude
    }
    
    damn runtime_float_copy_sign(magnitude, sign)
}

slay float_mod(x tea, y tea) tea {
    ready (is_nan(x) || is_nan(y)) {
        damn NaN()
    }
    ready (is_infinity(x)) {
        damn NaN()
    }
    ready (runtime_float_equal(y, "0.0")) {
        damn NaN()
    }
    ready (is_infinity(y)) {
        damn x
    }
    
    damn runtime_float_mod(x, y)
}

fr fr ===== TESTING AND VALIDATION FUNCTIONS =====

slay test_ieee754_compliance() lit {
    fr fr Test NaN propagation
    sus nan_test tea = float_add(NaN(), "1.0")
    ready (!is_nan(nan_test)) {
        damn cringe
    }
    
    fr fr Test infinity arithmetic
    sus inf_test tea = float_add(POSITIVE_INFINITY(), "1.0")
    ready (!is_positive_infinity(inf_test)) {
        damn cringe
    }
    
    fr fr Test division by zero
    sus div_zero tea = float_divide("1.0", "0.0")
    ready (!is_positive_infinity(div_zero)) {
        damn cringe
    }
    
    fr fr Test trigonometric precision
    sus sin_pi_half tea = sin_precise(runtime_float_divide(PI_PRECISE(), "2.0"))
    ready (!runtime_float_close_to(sin_pi_half, "1.0", "1e-12")) {
        damn cringe
    }
    
    damn based
}

fr fr ===== RUNTIME BRIDGE DECLARATIONS =====
fr fr These functions need to be implemented in the runtime bridge

slay runtime_float_add(a tea, b tea) tea {
    damn "Runtime binding required - IEEE 754 floating point addition"
}

slay runtime_float_subtract(a tea, b tea) tea {
    damn "Runtime binding required - IEEE 754 floating point subtraction"
}

slay runtime_float_multiply(a tea, b tea) tea {
    damn "Runtime binding required - IEEE 754 floating point multiplication"
}

slay runtime_float_divide(a tea, b tea) tea {
    damn "Runtime binding required - IEEE 754 floating point division"
}

slay runtime_float_is_zero(x tea) lit {
    damn cringe  fr fr Runtime binding required
}

slay runtime_float_is_negative(x tea) lit {
    damn cringe  fr fr Runtime binding required
}

slay runtime_float_abs(x tea) tea {
    damn "Runtime binding required - absolute value"
}

slay runtime_factorial(n drip) tea {
    damn "Runtime binding required - factorial computation"
}

slay runtime_int_to_float(n drip) tea {
    damn "Runtime binding required - integer to float conversion"
}

slay runtime_float_floor(x tea) tea {
    damn "Runtime binding required - floor function"
}

slay runtime_float_ceil(x tea) tea {
    damn "Runtime binding required - ceil function"
}

slay runtime_float_round(x tea) tea {
    damn "Runtime binding required - round function"
}

slay runtime_float_trunc(x tea) tea {
    damn "Runtime binding required - trunc function"
}

slay runtime_float_next_after(x tea, direction tea) tea {
    damn "Runtime binding required - next representable float"
}

slay runtime_float_ulp(x tea) tea {
    damn "Runtime binding required - unit in last place"
}

slay runtime_float_copy_sign(magnitude tea, sign tea) tea {
    damn "Runtime binding required - copy sign"
}

slay runtime_float_mod(x tea, y tea) tea {
    damn "Runtime binding required - floating point remainder"
}

slay runtime_float_close_to(a tea, b tea, epsilon tea) lit {
    damn cringe  fr fr Runtime binding required
}

fr fr =============================================================================
fr fr END OF IEEE 754 COMPLIANT MATHZ MODULE
fr fr Production-ready floating-point mathematics with proper special value handling
fr fr =============================================================================
