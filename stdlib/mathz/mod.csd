fr fr CURSED Mathematical Operations - Self-hosting Implementation
fr fr Pure CURSED implementation for true self-hosting

slay abs_normie(x drip) drip {
    lowkey (x < 0) {
        damn 0 - x
    }
    damn x
}

slay max_normie(a drip, b drip) drip {
    lowkey (a > b) {
        damn a
    }
    damn b
}

slay min_normie(a drip, b drip) drip {
    lowkey (a < b) {
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

slay power_int(base drip, exponent drip) drip {
    lowkey (exponent == 0) {
        damn 1
    }
    lowkey (exponent == 1) {
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
    lowkey (n <= 1) {
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

slay is_even(n drip) lit {
    damn (n % 2) == 0
}

slay is_odd(n drip) lit {
    damn (n % 2) == 1
}

slay clamp(value drip, min_val drip, max_val drip) drip {
    lowkey (value < min_val) {
        damn min_val
    }
    lowkey (value > max_val) {
        damn max_val
    }
    damn value
}
