fr fr CURSED Enhanced Mathematics Module - Comprehensive Math Operations
fr fr Pure CURSED implementation with advanced mathematical functions

fr fr ===== CONSTANTS =====

sus PI drip = 3141592653589793
sus E drip = 2718281828459045  
sus SQRT2 drip = 1414213562373095
sus LOG10E drip = 434294481903252
sus LN2 drip = 693147180559945

fr fr ===== BASIC ARITHMETIC (Extended) =====

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

slay pow_int(base drip, exponent drip) drip {
    ready (exponent == 0) {
        damn 1
    }
    ready (exponent == 1) {
        damn base
    }
    ready (exponent < 0) {
        damn 0  fr fr Simplified: no floating point division
    }
    
    sus result drip = 1
    sus i drip = 0
    bestie (i < exponent) {
        result = result * base
        i = i + 1
    }
    damn result
}

slay sqrt_int(n drip) drip {
    ready (n <= 0) {
        damn 0
    }
    ready (n == 1) {
        damn 1
    }
    
    sus guess drip = n / 2
    sus prev drip = 0
    
    bestie (abs_normie(guess - prev) > 1) {
        prev = guess
        guess = (guess + n / guess) / 2
    }
    damn guess
}

fr fr ===== TRIGONOMETRIC FUNCTIONS (Taylor Series Approximations) =====

slay sin_radians(x drip) drip {
    fr fr Normalize x to [-2π, 2π] range (simplified)
    bestie (x > 6283) {  fr fr 2π ≈ 6.283
        x = x - 6283
    }
    bestie (x < -6283) {
        x = x + 6283
    }
    
    fr fr Taylor series: sin(x) = x - x³/3! + x⁵/5! - x⁷/7! + ...
    sus result drip = x
    sus term drip = x
    sus i drip = 1
    
    bestie (i < 10) {  fr fr 10 terms for reasonable precision
        term = -term * x * x / ((2 * i) * (2 * i + 1))
        result = result + term
        i = i + 1
    }
    damn result / 1000000  fr fr Scale down from fixed point
}

slay cos_radians(x drip) drip {
    fr fr cos(x) = sin(x + π/2)
    damn sin_radians(x + 1570796)  fr fr π/2 ≈ 1.5708
}

slay tan_radians(x drip) drip {
    sus cos_val drip = cos_radians(x)
    ready (cos_val == 0) {
        damn 999999999  fr fr "Infinity" approximation
    }
    damn sin_radians(x) / cos_val
}

fr fr ===== LOGARITHMIC AND EXPONENTIAL FUNCTIONS =====

slay exp_approx(x drip) drip {
    fr fr Taylor series: e^x = 1 + x + x²/2! + x³/3! + ...
    sus result drip = 1000000  fr fr Start with 1.0 in fixed point
    sus term drip = 1000000
    sus i drip = 1
    
    bestie (i < 15) {
        term = term * x / i
        result = result + term
        i = i + 1
    }
    damn result / 1000000
}

slay ln_approx(x drip) drip {
    ready (x <= 0) {
        damn -999999999  fr fr Negative infinity approximation
    }
    ready (x == 1) {
        damn 0
    }
    
    fr fr Use ln(x) = 2 * [(x-1)/(x+1) + (x-1)³/3(x+1)³ + ...]
    sus y drip = (x - 1000000) / (x + 1000000)  fr fr (x-1)/(x+1) in fixed point
    sus result drip = 2 * y
    sus term drip = y
    sus i drip = 1
    
    bestie (i < 10) {
        term = term * y * y
        result = result + (2 * term) / (2 * i + 1)
        i = i + 1
    }
    damn result
}

slay log10_approx(x drip) drip {
    sus ln_val drip = ln_approx(x)
    damn ln_val / LN2  fr fr log10(x) = ln(x) / ln(10)
}

fr fr ===== RANDOM NUMBER GENERATION (Linear Congruential Generator) =====

sus rand_seed drip = 1

slay seed_random(seed drip) lit {
    rand_seed = seed
    damn based
}

slay rand() drip {
    fr fr LCG: next = (a * seed + c) mod m
    rand_seed = (1103515245 * rand_seed + 12345) % 2147483647
    damn rand_seed
}

slay rand_int(min_val drip, max_val drip) drip {
    sus range drip = max_val - min_val + 1
    sus random_val drip = rand() % range
    damn min_val + random_val
}

slay rand_bool() lit {
    sus val drip = rand() % 2
    ready (val == 0) {
        damn cringe
    }
    damn based
}

fr fr ===== STATISTICAL FUNCTIONS =====

slay mean_two(a drip, b drip) drip {
    damn (a + b) / 2
}

slay mean_three(a drip, b drip, c drip) drip {
    damn (a + b + c) / 3
}

slay mean_four(a drip, b drip, c drip, d drip) drip {
    damn (a + b + c + d) / 4
}

slay variance_two(a drip, b drip) drip {
    sus mean_val drip = mean_two(a, b)
    sus diff_a drip = a - mean_val
    sus diff_b drip = b - mean_val
    damn (diff_a * diff_a + diff_b * diff_b) / 2
}

slay std_dev_two(a drip, b drip) drip {
    sus var drip = variance_two(a, b)
    damn sqrt_int(var)
}

fr fr ===== ADVANCED MATHEMATICAL FUNCTIONS =====

slay gcd(a drip, b drip) drip {
    ready (b == 0) {
        damn a
    }
    damn gcd(b, a % b)
}

slay lcm(a drip, b drip) drip {
    sus gcd_val drip = gcd(a, b)
    damn (a * b) / gcd_val
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

slay is_prime(n drip) lit {
    ready (n <= 1) {
        damn cringe
    }
    ready (n <= 3) {
        damn based
    }
    ready (n % 2 == 0) {
        damn cringe
    }
    
    sus i drip = 3
    sus sqrt_n drip = sqrt_int(n)
    bestie (i <= sqrt_n) {
        ready (n % i == 0) {
            damn cringe
        }
        i = i + 2
    }
    damn based
}

slay next_prime(n drip) drip {
    sus candidate drip = n + 1
    bestie (is_prime(candidate) == cringe) {
        candidate = candidate + 1
    }
    damn candidate
}

fr fr ===== UTILITY FUNCTIONS =====

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

slay is_even(n drip) lit {
    damn (n % 2) == 0
}

slay is_odd(n drip) lit {
    damn (n % 2) == 1
}

slay degrees_to_radians(degrees drip) drip {
    damn (degrees * PI) / 180000000  fr fr π/180 in fixed point
}

slay radians_to_degrees(radians drip) drip {
    damn (radians * 180000000) / PI  fr fr 180/π in fixed point
}
