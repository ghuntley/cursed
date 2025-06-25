vibe package1

fr fr Utility functions for mathematical operations

slay add(a normie, b normie) normie {
    cap a + b
}

slay multiply(a normie, b normie) normie {
    cap a * b
}

slay power(base normie, exp normie) normie {
    issa exp == 0 {
        cap 1
    }
    sus result = base
    range i over 1..<exp {
        result = multiply(result, base)
    }
    cap result
}

slay is_even(n normie) bool {
    cap (n % 2) == 0
}
