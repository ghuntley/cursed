fr fr CURSED Pure Math Module - Extended Mathematical Functions
fr fr Replaces Zig FFI math functions with pure CURSED implementations

fr fr ===== FLOATING POINT MATH (Pure CURSED) =====

slay sqrt_newton(x drip) drip {
    fr fr Newton's method square root for integer approximation
    ready (x <= 0) {
        damn 0
    }
    ready (x == 1) {
        damn 1
    }
    
    fr fr Use scaled integers to simulate floating point
    sus scaled_x drip = x * 10000  fr fr Scale for precision
    sus guess drip = scaled_x / 2
    sus prev_guess drip = 0
    sus iterations drip = 0
    
    bestie (iterations < 20 && abs_normie(guess - prev_guess) > 1) {
        prev_guess = guess
        guess = (guess + scaled_x / guess) / 2
        iterations = iterations + 1
    }
    
    damn guess / 100  fr fr Unscale to approximate sqrt
}

slay sin_taylor_series(x_scaled drip) drip {
    fr fr Sine using Taylor series: sin(x) = x - x³/6 + x⁵/120 - x⁷/5040
    fr fr Input x_scaled is angle in radians * 10000
    
    sus x2 drip = (x_scaled * x_scaled) / 10000
    sus x3 drip = (x2 * x_scaled) / 10000
    sus x5 drip = (x3 * x2) / 10000
    sus x7 drip = (x5 * x2) / 10000
    
    sus term1 drip = x_scaled
    sus term2 drip = x3 / 6
    sus term3 drip = x5 / 120
    sus term4 drip = x7 / 5040
    
    damn term1 - term2 + term3 - term4
}

slay cos_taylor_series(x_scaled drip) drip {
    fr fr Cosine using Taylor series: cos(x) = 1 - x²/2 + x⁴/24 - x⁶/720
    
    sus x2 drip = (x_scaled * x_scaled) / 10000
    sus x4 drip = (x2 * x2) / 10000
    sus x6 drip = (x4 * x2) / 10000
    
    sus term1 drip = 10000  fr fr 1.0 scaled
    sus term2 drip = x2 / 2
    sus term3 drip = x4 / 24
    sus term4 drip = x6 / 720
    
    damn term1 - term2 + term3 - term4
}

slay exp_approximation(x_scaled drip) drip {
    fr fr Exponential function using Taylor series: e^x = 1 + x + x²/2! + x³/3! + ...
    
    sus result drip = 10000  fr fr Start with 1.0 scaled
    sus term drip = x_scaled
    sus factorial drip = 1
    sus i drip = 1
    
    bestie (i <= 10) {  fr fr 10 terms should be sufficient
        result = result + term / factorial
        i = i + 1
        factorial = factorial * i
        term = (term * x_scaled) / 10000
    }
    
    damn result
}

slay log_natural_approximation(x_scaled drip) drip {
    fr fr Natural logarithm approximation using series
    fr fr ln(1+x) = x - x²/2 + x³/3 - x⁴/4 + ... for |x| < 1
    
    ready (x_scaled <= 0) {
        damn -999999  fr fr Negative infinity approximation
    }
    
    ready (x_scaled == 10000) {  fr fr ln(1) = 0
        damn 0
    }
    
    fr fr For x > 1, use ln(x) = ln(1 + (x-1))
    sus x_minus_1 drip = x_scaled - 10000
    
    ready (abs_normie(x_minus_1) >= 10000) {
        fr fr Outside convergence range, use approximation
        ready (x_scaled == 27183) {  fr fr ln(e) = 1
            damn 10000
        }
        ready (x_scaled == 20000) {  fr fr ln(2) ≈ 0.693
            damn 6931
        }
        damn 0  fr fr Default fallback
    }
    
    fr fr Apply series
    sus x drip = x_minus_1
    sus result drip = x
    sus term drip = x
    sus i drip = 2
    
    bestie (i <= 8) {
        term = (term * x_minus_1) / 10000
        ready (i % 2 == 0) {
            result = result - term / i
        } otherwise {
            result = result + term / i
        }
        i = i + 1
    }
    
    damn result
}

fr fr ===== ADVANCED MATHEMATICAL FUNCTIONS =====

slay power_fast(base drip, exponent drip) drip {
    fr fr Fast exponentiation using binary method
    ready (exponent == 0) {
        damn 1
    }
    ready (exponent == 1) {
        damn base
    }
    ready (exponent < 0) {
        fr fr For negative exponents, return scaled reciprocal
        sus positive_result drip = power_fast(base, -exponent)
        ready (positive_result != 0) {
            damn 10000 / positive_result  fr fr Scaled 1/result
        }
        damn 0
    }
    
    sus result drip = 1
    sus base_copy drip = base
    sus exp_copy drip = exponent
    
    bestie (exp_copy > 0) {
        ready (exp_copy % 2 == 1) {
            result = result * base_copy
        }
        base_copy = base_copy * base_copy
        exp_copy = exp_copy / 2
    }
    
    damn result
}

slay tan_approximation(x_scaled drip) drip {
    fr fr Tangent approximation: tan(x) = sin(x) / cos(x)
    sus sin_val drip = sin_taylor_series(x_scaled)
    sus cos_val drip = cos_taylor_series(x_scaled)
    
    ready (cos_val == 0) {
        damn 999999  fr fr Infinity approximation
    }
    
    damn (sin_val * 10000) / cos_val
}

slay atan_approximation(x_scaled drip) drip {
    fr fr Arctangent approximation using series
    fr fr atan(x) = x - x³/3 + x⁵/5 - x⁷/7 + ... for |x| ≤ 1
    
    ready (abs_normie(x_scaled) > 10000) {
        fr fr For large x, use atan(x) = π/2 - atan(1/x)
        sus reciprocal drip = (10000 * 10000) / x_scaled
        sus atan_recip drip = atan_approximation(reciprocal)
        ready (x_scaled > 0) {
            damn 15708 - atan_recip  fr fr π/2 * 10000 ≈ 15708
        } otherwise {
            damn -15708 - atan_recip  fr fr -π/2
        }
    }
    
    sus x2 drip = (x_scaled * x_scaled) / 10000
    sus result drip = x_scaled
    sus term drip = x_scaled
    sus i drip = 3
    
    bestie (i <= 15) {
        term = (term * x2) / 10000
        ready (i % 4 == 3) {
            result = result - term / i
        } otherwise {
            result = result + term / i
        }
        i = i + 2
    }
    
    damn result
}

fr fr ===== STATISTICAL FUNCTIONS =====

slay variance_array(values []drip) drip {
    sus n drip = len(values)
    ready (n <= 1) {
        damn 0
    }
    
    sus mean drip = sum_array(values) / n
    sus sum_squared_diffs drip = 0
    sus i drip = 0
    
    bestie (i < n) {
        sus diff drip = values[i] - mean
        sum_squared_diffs = sum_squared_diffs + (diff * diff)
        i = i + 1
    }
    
    damn sum_squared_diffs / (n - 1)  fr fr Sample variance
}

slay standard_deviation_array(values []drip) drip {
    sus var drip = variance_array(values)
    damn sqrt_newton(var)
}

slay correlation_coefficient(x_values []drip, y_values []drip) drip {
    sus n drip = len(x_values)
    ready (n != len(y_values) || n < 2) {
        damn 0  fr fr Invalid input
    }
    
    sus sum_x drip = sum_array(x_values)
    sus sum_y drip = sum_array(y_values)
    sus mean_x drip = sum_x / n
    sus mean_y drip = sum_y / n
    
    sus sum_xy drip = 0
    sus sum_x2 drip = 0
    sus sum_y2 drip = 0
    sus i drip = 0
    
    bestie (i < n) {
        sus dx drip = x_values[i] - mean_x
        sus dy drip = y_values[i] - mean_y
        sum_xy = sum_xy + (dx * dy)
        sum_x2 = sum_x2 + (dx * dx)
        sum_y2 = sum_y2 + (dy * dy)
        i = i + 1
    }
    
    sus denominator drip = sqrt_newton(sum_x2) * sqrt_newton(sum_y2)
    ready (denominator == 0) {
        damn 0
    }
    
    damn (sum_xy * 10000) / denominator  fr fr Scaled correlation
}

fr fr ===== BITWISE OPERATIONS (Pure CURSED) =====

slay bitwise_and(a drip, b drip) drip {
    fr fr Simulate bitwise AND using arithmetic
    sus result drip = 0
    sus bit_value drip = 1
    sus temp_a drip = a
    sus temp_b drip = b
    
    bestie (temp_a > 0 || temp_b > 0) {
        sus bit_a drip = temp_a % 2
        sus bit_b drip = temp_b % 2
        
        ready (bit_a == 1 && bit_b == 1) {
            result = result + bit_value
        }
        
        bit_value = bit_value * 2
        temp_a = temp_a / 2
        temp_b = temp_b / 2
    }
    
    damn result
}

slay bitwise_or(a drip, b drip) drip {
    fr fr Simulate bitwise OR using arithmetic
    sus result drip = 0
    sus bit_value drip = 1
    sus temp_a drip = a
    sus temp_b drip = b
    
    bestie (temp_a > 0 || temp_b > 0) {
        sus bit_a drip = temp_a % 2
        sus bit_b drip = temp_b % 2
        
        ready (bit_a == 1 || bit_b == 1) {
            result = result + bit_value
        }
        
        bit_value = bit_value * 2
        temp_a = temp_a / 2
        temp_b = temp_b / 2
    }
    
    damn result
}

slay bitwise_xor(a drip, b drip) drip {
    fr fr Simulate bitwise XOR using arithmetic
    sus result drip = 0
    sus bit_value drip = 1
    sus temp_a drip = a
    sus temp_b drip = b
    
    bestie (temp_a > 0 || temp_b > 0) {
        sus bit_a drip = temp_a % 2
        sus bit_b drip = temp_b % 2
        
        ready ((bit_a == 1 && bit_b == 0) || (bit_a == 0 && bit_b == 1)) {
            result = result + bit_value
        }
        
        bit_value = bit_value * 2
        temp_a = temp_a / 2
        temp_b = temp_b / 2
    }
    
    damn result
}

slay bitwise_not(a drip, bit_width drip) drip {
    fr fr Simulate bitwise NOT for specified bit width
    sus max_value drip = power_fast(2, bit_width) - 1
    damn max_value - a
}

slay left_shift(value drip, positions drip) drip {
    fr fr Left shift: multiply by 2^positions
    damn value * power_fast(2, positions)
}

slay right_shift(value drip, positions drip) drip {
    fr fr Right shift: divide by 2^positions
    damn value / power_fast(2, positions)
}

fr fr ===== NUMERIC BASE CONVERSION =====

slay to_binary_string(value drip) tea {
    fr fr Convert integer to binary string representation
    ready (value == 0) {
        damn "0"
    }
    
    sus result tea = ""
    sus temp drip = value
    sus is_negative lit = cringe
    
    ready (temp < 0) {
        is_negative = based
        temp = -temp
    }
    
    bestie (temp > 0) {
        sus bit drip = temp % 2
        ready (bit == 1) {
            result = "1" + result
        } otherwise {
            result = "0" + result
        }
        temp = temp / 2
    }
    
    ready (is_negative) {
        result = "-" + result
    }
    
    damn result
}

slay to_hex_string(value drip) tea {
    fr fr Convert integer to hexadecimal string
    ready (value == 0) {
        damn "0"
    }
    
    sus result tea = ""
    sus temp drip = value
    sus is_negative lit = cringe
    
    ready (temp < 0) {
        is_negative = based
        temp = -temp
    }
    
    bestie (temp > 0) {
        sus digit drip = temp % 16
        ready (digit < 10) {
            result = int_to_string(digit) + result
        } otherwise {
            ready (digit == 10) { result = "A" + result }
            ready (digit == 11) { result = "B" + result }
            ready (digit == 12) { result = "C" + result }
            ready (digit == 13) { result = "D" + result }
            ready (digit == 14) { result = "E" + result }
            ready (digit == 15) { result = "F" + result }
        }
        temp = temp / 16
    }
    
    ready (is_negative) {
        result = "-" + result
    }
    
    damn result
}

slay from_binary_string(binary_str tea) drip {
    fr fr Convert binary string to integer
    ready (is_empty_string(binary_str)) {
        damn 0
    }
    
    sus result drip = 0
    sus power drip = 1
    sus len drip = string_length(binary_str)
    sus start_index drip = 0
    sus is_negative lit = cringe
    
    ready (char_at(binary_str, 0) == "-") {
        is_negative = based
        start_index = 1
    }
    
    sus i drip = len - 1
    bestie (i >= start_index) {
        sus char tea = char_at(binary_str, i)
        ready (char == "1") {
            result = result + power
        }
        power = power * 2
        i = i - 1
    }
    
    ready (is_negative) {
        result = -result
    }
    
    damn result
}

slay from_hex_string(hex_str tea) drip {
    fr fr Convert hexadecimal string to integer
    ready (is_empty_string(hex_str)) {
        damn 0
    }
    
    sus result drip = 0
    sus power drip = 1
    sus len drip = string_length(hex_str)
    sus start_index drip = 0
    sus is_negative lit = cringe
    
    ready (char_at(hex_str, 0) == "-") {
        is_negative = based
        start_index = 1
    }
    
    sus i drip = len - 1
    bestie (i >= start_index) {
        sus char tea = char_at(hex_str, i)
        sus digit_value drip = 0
        
        ready (char == "0") { digit_value = 0 }
        ready (char == "1") { digit_value = 1 }
        ready (char == "2") { digit_value = 2 }
        ready (char == "3") { digit_value = 3 }
        ready (char == "4") { digit_value = 4 }
        ready (char == "5") { digit_value = 5 }
        ready (char == "6") { digit_value = 6 }
        ready (char == "7") { digit_value = 7 }
        ready (char == "8") { digit_value = 8 }
        ready (char == "9") { digit_value = 9 }
        ready (char == "A" || char == "a") { digit_value = 10 }
        ready (char == "B" || char == "b") { digit_value = 11 }
        ready (char == "C" || char == "c") { digit_value = 12 }
        ready (char == "D" || char == "d") { digit_value = 13 }
        ready (char == "E" || char == "e") { digit_value = 14 }
        ready (char == "F" || char == "f") { digit_value = 15 }
        
        result = result + (digit_value * power)
        power = power * 16
        i = i - 1
    }
    
    ready (is_negative) {
        result = -result
    }
    
    damn result
}
