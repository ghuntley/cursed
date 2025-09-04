vibe mathz

fr fr mathz module using thicc (i64) to match variable storage

slay abs_normie(x thicc) thicc {
    ready x < 0 {
        damn -x
    } otherwise {
        damn x
    }
}

slay abs_meal(x meal) meal {
    damn x
}

slay max(a thicc, b thicc) thicc {
    ready a > b { damn a } otherwise { damn b }
}

slay min(a thicc, b thicc) thicc {
    ready a < b { damn a } otherwise { damn b }
}

slay add_two(a thicc, b thicc) thicc {
    fr fr Simple 64-bit integer addition
    damn a + b
}

fr fr ===== basic arithmetic ======================================================

slay add(a thicc, b thicc) thicc {
    damn a + b
}

slay subtract(a thicc, b thicc) thicc {
    damn a - b
}

slay multiply(a thicc, b thicc) thicc {
    damn a * b
}

fr fr Integer division – caller is responsible for zero-check if they care.
slay divide(a thicc, b thicc) thicc {
    damn a / b
}

fr fr ===== remainder helpers =====================================================

slay mod(a thicc, b thicc) thicc {
    damn a % b
}

fr fr alias with full word (some tests use modulo())
slay modulo(a thicc, b thicc) thicc {
    damn a % b
}

fr fr ===== exponentiation ========================================================

fr fr fast exponentiation for integers (O(log exp))
slay power(base thicc, exp thicc) thicc {
    ready exp < 0 {
        fr fr Integer power with negative exponent would be fractional → just return 0
        damn 0
    }
    sus res thicc = 1
    sus b thicc   = base
    sus e thicc   = exp
    bestie e > 0 {
        ready (e % 2) == 1 {               
            res = res * b
        }
        b = b * b
        e = e / 2
    }
    damn res
}

fr fr alias – many people instinctively call pow()
slay pow(base thicc, exp thicc) thicc {
    damn power(base, exp)
}

fr fr ===== square root (Newton–Raphson) =========================================

slay sqrt(x meal) meal {
    ready x < 0.0 {
        damn 0.0
    }
    ready x == 0.0 {
        damn 0.0
    }

    sus z    meal = x
    sus prev meal = 0.0
    bestie abs_meal(z - prev) > 1e-9 {
        prev = z
        z    = 0.5 * (z + x / z)
    }
    damn z
}
