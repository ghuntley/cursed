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
