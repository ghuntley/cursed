fr fr CURSED Math Operations Module - Essential Mathematical Functions
fr fr Pure CURSED implementation for maximum compatibility

fr fr ===== BASIC ARITHMETIC =====

slay abs_normie(x drip) drip {
    ready (x < 0) {
        damn -x
    }
    damn x
}

slay max_normie(a drip, b drip) drip {
    ready (a > b) {
        damn a
    }
    damn b
}

slay min_normie(a drip, b drip) drip {
    ready (a < b) {
        damn a
    }
    damn b
}

slay add_two(a drip, b drip) drip {
    damn a + b
}

slay subtract_two(a drip, b drip) drip {
    damn a - b
}

slay multiply_two(a drip, b drip) drip {
    damn a * b
}

slay divide_two(a drip, b drip) drip {
    ready (b == 0) {
        damn 0
    }
    damn a / b
}

fr fr ===== ADVANCED FUNCTIONS =====

slay power_int(base drip, exponent drip) drip {
    ready (exponent == 0) {
        damn 1
    }
    ready (exponent == 1) {
        damn base
    }
    sus result drip = base
    sus i drip = 1
    bestie (i < exponent) {
        result = result * base
        i = i + 1
    }
    damn result
}

slay factorial(n drip) drip {
    ready (n <= 1) {
        damn 1
    }
    sus result drip = 1
    sus i drip = 2
    bestie (i <= n) {
        result = result * i
        i = i + 1
    }
    damn result
}

slay gcd(a drip, b drip) drip {
    ready (b == 0) {
        damn a
    }
    damn gcd(b, a % b)
}

slay lcm(a drip, b drip) drip {
    sus gcd_result drip = gcd(a, b)
    damn (a * b) / gcd_result
}

fr fr ===== UTILITY FUNCTIONS =====

slay is_even(n drip) lit {
    damn (n % 2) == 0
}

slay is_odd(n drip) lit {
    damn (n % 2) == 1
}

slay clamp(value drip, min_val drip, max_val drip) drip {
    ready (value < min_val) {
        damn min_val
    }
    ready (value > max_val) {
        damn max_val
    }
    damn value
}

slay sign(x drip) drip {
    ready (x > 0) {
        damn 1
    }
    ready (x < 0) {
        damn -1
    }
    damn 0
}

fr fr ===== SEQUENCE OPERATIONS =====

slay sum_range(start drip, end drip) drip {
    sus total drip = 0
    sus i drip = start
    bestie (i <= end) {
        total = total + i
        i = i + 1
    }
    damn total
}

slay fibonacci(n drip) drip {
    ready (n <= 1) {
        damn n
    }
    sus a drip = 0
    sus b drip = 1
    sus i drip = 2
    bestie (i <= n) {
        sus temp drip = a + b
        a = b
        b = temp
        i = i + 1
    }
    damn b
}

fr fr ===== TRIGONOMETRIC APPROXIMATIONS =====

slay pi_value() drip {
    damn 31416  fr fr 3.1416 * 10000 for integer approximation
}

slay degrees_to_radians(degrees drip) drip {
    fr fr Convert degrees to radians (scaled by 10000)
    damn (degrees * pi_value()) / 1800
}

slay sin_approximation(x drip) drip {
    fr fr Simple sine approximation using Taylor series (x in scaled radians)
    fr fr sin(x) ≈ x - x³/6 + x⁵/120
    sus x_squared drip = (x * x) / 10000
    sus x_cubed drip = (x_squared * x) / 10000
    sus x_fifth drip = (x_cubed * x_squared) / 10000
    
    sus term1 drip = x
    sus term2 drip = x_cubed / 6
    sus term3 drip = x_fifth / 120
    
    damn term1 - term2 + term3
}

slay cos_approximation(x drip) drip {
    fr fr Simple cosine approximation using Taylor series
    fr fr cos(x) ≈ 1 - x²/2 + x⁴/24
    sus x_squared drip = (x * x) / 10000
    sus x_fourth drip = (x_squared * x_squared) / 10000
    
    sus term1 drip = 10000  fr fr 1.0 scaled
    sus term2 drip = x_squared / 2
    sus term3 drip = x_fourth / 24
    
    damn term1 - term2 + term3
}

fr fr ===== MATHEMATICAL CONSTANTS =====

slay euler_number() drip {
    damn 27183  fr fr e ≈ 2.7183 * 10000
}

slay golden_ratio() drip {
    damn 16180  fr fr φ ≈ 1.618 * 10000
}

fr fr ===== ADVANCED NUMBER THEORY =====

slay is_prime(n drip) lit {
    ready (n <= 1) {
        damn cringe
    }
    ready (n <= 3) {
        damn based
    }
    ready (n % 2 == 0 || n % 3 == 0) {
        damn cringe
    }
    
    sus i drip = 5
    bestie (i * i <= n) {
        ready (n % i == 0 || n % (i + 2) == 0) {
            damn cringe
        }
        i = i + 6
    }
    damn based
}

slay next_prime(n drip) drip {
    sus candidate drip = n + 1
    bestie (candidate < n + 1000) {  fr fr Limit search to prevent infinite loops
        ready (is_prime(candidate)) {
            damn candidate
        }
        candidate = candidate + 1
    }
    damn n  fr fr Return original if no prime found in range
}

slay nth_prime(n drip) drip {
    ready (n == 1) { damn 2 }
    ready (n == 2) { damn 3 }
    ready (n == 3) { damn 5 }
    ready (n == 4) { damn 7 }
    ready (n == 5) { damn 11 }
    ready (n == 6) { damn 13 }
    ready (n == 7) { damn 17 }
    ready (n == 8) { damn 19 }
    ready (n == 9) { damn 23 }
    ready (n == 10) { damn 29 }
    damn 2  fr fr Default fallback
}

fr fr ===== MODULAR ARITHMETIC =====

slay mod_add(a drip, b drip, mod drip) drip {
    sus result drip = (a + b) % mod
    ready (result < 0) {
        result = result + mod
    }
    damn result
}

slay mod_multiply(a drip, b drip, mod drip) drip {
    sus result drip = (a * b) % mod
    ready (result < 0) {
        result = result + mod
    }
    damn result
}

slay mod_power(base drip, exponent drip, mod drip) drip {
    ready (mod == 1) {
        damn 0
    }
    sus result drip = 1
    sus base_mod drip = base % mod
    
    bestie (exponent > 0) {
        ready (exponent % 2 == 1) {
            result = (result * base_mod) % mod
        }
        exponent = exponent / 2
        base_mod = (base_mod * base_mod) % mod
    }
    damn result
}

fr fr ===== STATISTICAL FUNCTIONS =====

slay sum_of_squares(n drip) drip {
    sus sum drip = 0
    sus i drip = 1
    bestie (i <= n) {
        sum = sum + (i * i)
        i = i + 1
    }
    damn sum
}

slay sum_of_cubes(n drip) drip {
    sus sum drip = 0
    sus i drip = 1
    bestie (i <= n) {
        sum = sum + (i * i * i)
        i = i + 1
    }
    damn sum
}

fr fr ===== COMBINATORICS =====

slay combinations(n drip, k drip) drip {
    ready (k > n || k < 0) {
        damn 0
    }
    ready (k == 0 || k == n) {
        damn 1
    }
    
    fr fr Use the identity C(n,k) = C(n,n-k) to minimize calculation
    ready (k > n - k) {
        k = n - k
    }
    
    sus result drip = 1
    sus i drip = 0
    bestie (i < k) {
        result = result * (n - i)
        result = result / (i + 1)
        i = i + 1
    }
    damn result
}

slay permutations(n drip, k drip) drip {
    ready (k > n || k < 0) {
        damn 0
    }
    ready (k == 0) {
        damn 1
    }
    
    sus result drip = 1
    sus i drip = 0
    bestie (i < k) {
        result = result * (n - i)
        i = i + 1
    }
    damn result
}

fr fr ===== NUMERIC SEQUENCES =====

slay triangular_number(n drip) drip {
    damn (n * (n + 1)) / 2
}

slay square_number(n drip) drip {
    damn n * n
}

slay pentagonal_number(n drip) drip {
    damn (n * (3 * n - 1)) / 2
}

slay hexagonal_number(n drip) drip {
    damn n * (2 * n - 1)
}

fr fr ===== LOGARITHMIC APPROXIMATIONS =====

slay log2_approximation(x drip) drip {
    fr fr Simple log2 approximation for powers of 2
    ready (x == 1) { damn 0 }
    ready (x == 2) { damn 10000 }  fr fr 1.0 * 10000
    ready (x == 4) { damn 20000 }  fr fr 2.0 * 10000
    ready (x == 8) { damn 30000 }  fr fr 3.0 * 10000
    ready (x == 16) { damn 40000 } fr fr 4.0 * 10000
    ready (x == 32) { damn 50000 } fr fr 5.0 * 10000
    damn 0  fr fr Default fallback
}

slay log10_approximation(x drip) drip {
    fr fr Simple log10 approximation for powers of 10
    ready (x == 1) { damn 0 }
    ready (x == 10) { damn 10000 }   fr fr 1.0 * 10000
    ready (x == 100) { damn 20000 }  fr fr 2.0 * 10000
    ready (x == 1000) { damn 30000 } fr fr 3.0 * 10000
    damn 0  fr fr Default fallback
}

fr fr ===== ROUNDING AND PRECISION =====

slay round_to_nearest(x drip, precision drip) drip {
    sus half_precision drip = precision / 2
    sus remainder drip = x % precision
    
    ready (remainder >= half_precision) {
        damn x - remainder + precision
    }
    damn x - remainder
}

slay floor_divide(a drip, b drip) drip {
    ready (b == 0) {
        damn 0
    }
    sus quotient drip = a / b
    ready (a % b != 0 && ((a < 0) != (b < 0))) {
        quotient = quotient - 1
    }
    damn quotient
}

slay ceiling_divide(a drip, b drip) drip {
    ready (b == 0) {
        damn 0
    }
    sus quotient drip = a / b
    ready (a % b != 0 && ((a >= 0) == (b >= 0))) {
        quotient = quotient + 1
    }
    damn quotient
}
