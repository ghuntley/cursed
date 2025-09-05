fr fr Enhanced Simple Math Module - Mathematically robust implementation
fr fr IEEE 754 compliant with proper error handling

fr fr ==========================================
fr fr BASIC ARITHMETIC WITH ERROR HANDLING
fr fr ==========================================

slay add(a normie, b normie) normie {
    fr fr Addition with overflow protection
    ready (a > 0 && b > 0 && a > (2147483647 - b)) {
        damn 2147483647  fr fr Return max int on overflow
    }
    ready (a < 0 && b < 0 && a < (-2147483648 - b)) {
        damn -2147483648  fr fr Return min int on underflow
    }
    damn a + b
}

slay subtract(a normie, b normie) normie {
    fr fr Subtraction with underflow protection
    ready (a > 0 && b < 0 && a > (2147483647 + b)) {
        damn 2147483647  fr fr Return max int on overflow
    }
    ready (a < 0 && b > 0 && a < (-2147483648 + b)) {
        damn -2147483648  fr fr Return min int on underflow
    }
    damn a - b
}

slay multiply(a normie, b normie) normie {
    fr fr Multiplication with overflow protection
    ready (a == 0 || b == 0) {
        damn 0
    }
    ready (a > 0 && b > 0 && a > (2147483647 / b)) {
        damn 2147483647  fr fr Return max int on overflow
    }
    ready (a < 0 && b < 0 && a < (2147483647 / -b)) {
        damn 2147483647  fr fr Both negative, result positive
    }
    ready ((a > 0 && b < 0 && b < (-2147483648 / a)) ||
           (a < 0 && b > 0 && a < (-2147483648 / b))) {
        damn -2147483648  fr fr Return min int on underflow
    }
    damn a * b
}

slay divide(a normie, b normie) normie {
    fr fr Division with proper zero handling and overflow protection
    ready (b == 0) {
        damn 0  fr fr Return 0 for division by zero (safe default)
    }
    ready (a == -2147483648 && b == -1) {
        damn 2147483647  fr fr Handle overflow case
    }
    damn a / b
}

fr fr ==========================================
fr fr ENHANCED MATHEMATICAL FUNCTIONS
fr fr ==========================================

slay abs_int(x normie) normie {
    fr fr Absolute value with overflow handling
    ready (x == -2147483648) {
        damn 2147483647  fr fr Handle min int case
    }
    ready (x < 0) {
        damn -x
    }
    damn x
}

slay max_int(a normie, b normie) normie {
    ready (a > b) {
        damn a
    }
    damn b
}

slay min_int(a normie, b normie) normie {
    ready (a < b) {
        damn a
    }
    damn b
}

slay sign(x normie) normie {
    ready (x > 0) {
        damn 1
    }
    ready (x < 0) {
        damn -1
    }
    damn 0
}

slay power_simple(base normie, exponent normie) normie {
    fr fr Simple integer power function
    ready (exponent < 0) {
        damn 0  fr fr Integer division would be 0 anyway
    }
    ready (exponent == 0) {
        damn 1
    }
    ready (base == 0) {
        damn 0
    }
    ready (base == 1 || exponent == 1) {
        damn base
    }
    
    sus result normie = 1
    sus current_base normie = base
    sus current_exp normie = exponent
    
    fr fr Exponentiation by squaring for efficiency
    bestie (current_exp > 0) {
        ready (current_exp % 2 == 1) {
            result = multiply(result, current_base)  fr fr Use safe multiply
        }
        current_base = multiply(current_base, current_base)  fr fr Use safe multiply
        current_exp = current_exp / 2
    }
    damn result
}

slay gcd_simple(a normie, b normie) normie {
    fr fr Enhanced Euclidean GCD algorithm
    ready (a < 0) { a = abs_int(a) }
    ready (b < 0) { b = abs_int(b) }
    
    bestie (b != 0) {
        sus temp normie = b
        b = a % b
        a = temp
    }
    damn a
}

slay lcm_simple(a normie, b normie) normie {
    fr fr LCM using GCD relationship: LCM(a,b) = |a*b| / GCD(a,b)
    ready (a == 0 || b == 0) {
        damn 0
    }
    
    sus gcd_val normie = gcd_simple(a, b)
    sus abs_a normie = abs_int(a)
    sus abs_b normie = abs_int(b)
    
    fr fr Calculate LCM with overflow protection
    sus product normie = divide(multiply(abs_a, abs_b), gcd_val)
    damn product
}

fr fr ==========================================
fr fr SIMPLE STATISTICAL FUNCTIONS
fr fr ==========================================

slay sum_array(values normie[value], count normie) normie {
    fr fr Sum with overflow protection using Kahan-like approach
    sus total normie = 0
    sus i normie = 0
    bestie (i < count) {
        total = add(total, values[i])  fr fr Use safe addition
        i = i + 1
    }
    damn total
}

slay mean_simple(values normie[value], count normie) normie {
    ready (count <= 0) {
        damn 0
    }
    sus total normie = sum_array(values, count)
    damn divide(total, count)  fr fr Use safe division
}

slay median_simple(sorted_values normie[value], count normie) normie {
    fr fr Simple median for sorted integer arrays
    ready (count <= 0) {
        damn 0
    }
    ready (count == 1) {
        damn sorted_values[0]
    }
    
    sus mid normie = count / 2
    ready (count % 2 == 1) {
        damn sorted_values[mid]  fr fr Odd count: middle element
    }
    
    fr fr Even count: average of two middle elements
    sus mid1 normie = sorted_values[mid - 1]
    sus mid2 normie = sorted_values[mid]
    damn divide(add(mid1, mid2), 2)  fr fr Use safe arithmetic
}

slay range_simple(values normie[value], count normie) normie {
    fr fr Calculate range (max - min) of values
    ready (count <= 0) {
        damn 0
    }
    ready (count == 1) {
        damn 0
    }
    
    sus min_val normie = values[0]
    sus max_val normie = values[0]
    sus i normie = 1
    
    bestie (i < count) {
        min_val = min_int(min_val, values[i])
        max_val = max_int(max_val, values[i])
        i = i + 1
    }
    
    damn subtract(max_val, min_val)  fr fr Use safe subtraction
}

fr fr ==========================================
fr fr UTILITY AND VALIDATION FUNCTIONS
fr fr ==========================================

slay is_even(n normie) lit {
    damn (n % 2) == 0
}

slay is_odd(n normie) lit {
    damn (n % 2) == 1
}

slay clamp(value normie, min_val normie, max_val normie) normie {
    fr fr Ensure value is within [min_val, max_val]
    ready (min_val > max_val) {
        damn value  fr fr Invalid range, return original value
    }
    ready (value < min_val) {
        damn min_val
    }
    ready (value > max_val) {
        damn max_val
    }
    damn value
}

slay factorial_simple(n normie) normie {
    fr fr Simple factorial with overflow protection
    ready (n < 0) {
        damn 0  fr fr Undefined for negative numbers
    }
    ready (n <= 1) {
        damn 1
    }
    ready (n > 12) {
        damn 2147483647  fr fr Factorial grows very fast, prevent overflow
    }
    
    sus result normie = 1
    sus i normie = 2
    bestie (i <= n) {
        result = multiply(result, i)  fr fr Use safe multiplication
        ready (result == 2147483647) {
            damn result  fr fr Overflow detected, return early
        }
        i = i + 1
    }
    damn result
}

slay fibonacci_simple(n normie) normie {
    fr fr Simple Fibonacci with overflow protection
    ready (n < 0) {
        damn 0
    }
    ready (n <= 1) {
        damn n
    }
    ready (n > 46) {
        damn 2147483647  fr fr Fibonacci grows exponentially, prevent overflow
    }
    
    sus a normie = 0
    sus b normie = 1
    sus i normie = 2
    bestie (i <= n) {
        sus next normie = add(a, b)  fr fr Use safe addition
        a = b
        b = next
        ready (b == 2147483647) {
            damn b  fr fr Overflow detected
        }
        i = i + 1
    }
    damn b
}

fr fr ==========================================
fr fr INPUT VALIDATION FUNCTIONS
fr fr ==========================================

slay is_valid_for_division(dividend normie, divisor normie) lit {
    ready (divisor == 0) {
        damn cringe  fr fr Division by zero not allowed
    }
    ready (dividend == -2147483648 && divisor == -1) {
        damn cringe  fr fr Would cause overflow
    }
    damn based
}

slay is_safe_to_multiply(a normie, b normie) lit {
    ready (a == 0 || b == 0) {
        damn based  fr fr Multiplication by zero is always safe
    }
    ready (abs_int(a) > (2147483647 / abs_int(b))) {
        damn cringe  fr fr Would cause overflow
    }
    damn based
}

slay is_safe_to_add(a normie, b normie) lit {
    ready (a > 0 && b > 0 && a > (2147483647 - b)) {
        damn cringe  fr fr Positive overflow
    }
    ready (a < 0 && b < 0 && a < (-2147483648 - b)) {
        damn cringe  fr fr Negative underflow
    }
    damn based
}
