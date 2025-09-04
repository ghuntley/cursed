vibe mathz

fr fr ===== BASIC MATHEMATICAL OPERATIONS =====

slay abs_normie(x normie) normie {
    ready (x < 0) {
        damn -x
    }
    damn x
}

slay add_two(a normie, b normie) normie {
    damn a + b
}

slay max(a normie, b normie) normie {
    ready (a > b) {
        damn a
    }
    damn b
}

slay min(a normie, b normie) normie {
    ready (a < b) {
        damn a
    }
    damn b
}

slay pow(base normie, exponent normie) normie {
    ready (exponent == 0) {
        damn 1
    }
    ready (exponent < 0) {
        damn 0  fr fr Integer division, negative exponent gives 0
    }
    ready (base == 0) {
        damn 0
    }
    ready (base == 1) {
        damn 1
    }
    
    sus result normie = 1
    sus exp normie = exponent
    sus base_power normie = base
    
    bestie (exp > 0) {
        ready (exp % 2 == 1) {
            result = result * base_power
        }
        base_power = base_power * base_power
        exp = exp / 2
    }
    damn result
}

slay sqrt(x normie) normie {
    ready (x <= 0) {
        damn 0
    }
    ready (x == 1) {
        damn 1
    }
    
    fr fr Perfect squares lookup
    ready (x == 4) { damn 2 }
    ready (x == 9) { damn 3 }
    ready (x == 16) { damn 4 }
    ready (x == 25) { damn 5 }
    ready (x == 36) { damn 6 }
    ready (x == 49) { damn 7 }
    ready (x == 64) { damn 8 }
    ready (x == 81) { damn 9 }
    ready (x == 100) { damn 10 }
    
    fr fr Newton's method for general case
    sus guess normie = x / 2
    sus iterations normie = 10
    sus i normie = 0
    
    bestie (i < iterations) {
        sus new_guess normie = (guess + x / guess) / 2
        ready (abs_normie(new_guess - guess) <= 1) {
            damn new_guess
        }
        guess = new_guess
        i = i + 1
    }
    
    damn guess
}

slay mod(a normie, b normie) normie {
    ready (b == 0) {
        damn 0
    }
    damn a % b
}

fr fr Additional common functions for compatibility
slay add(a normie, b normie) normie {
    damn a + b
}

slay subtract(a normie, b normie) normie {
    damn a - b
}

slay multiply(a normie, b normie) normie {
    damn a * b
}

slay divide(a normie, b normie) normie {
    ready (b == 0) {
        damn 0
    }
    damn a / b
}
