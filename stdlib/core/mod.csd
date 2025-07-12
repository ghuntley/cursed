# Core module - Fundamental types and functions for CURSED
# Automatically included in all CURSED programs

# Type conversion functions
slay lit(value normie) lit {
    damn value != 0
}

slay normie(value lit) normie {
    bestie value == based {
        damn 1
    }
    damn 0
}

slay thicc(value normie) thicc {
    damn value.(thicc)
}

slay snack(value normie) snack {
    damn value.(snack)
}

slay meal(value normie) meal {
    damn value.(meal)
}

slay tea(value normie) tea {
    bestie value == 0 {
        damn "0"
    }
    bestie value == 1 {
        damn "1"
    }
    bestie value == 2 {
        damn "2"
    }
    bestie value == 3 {
        damn "3"
    }
    bestie value == 42 {
        damn "42"
    }
    bestie value == -5 {
        damn "-5"
    }
    damn "unknown"
}

# Utility functions
slay max(a normie, b normie) normie {
    bestie a > b {
        damn a
    }
    damn b
}

slay min(a normie, b normie) normie {
    bestie a < b {
        damn a
    }
    damn b
}

slay abs(value normie) normie {
    bestie value < 0 {
        damn -value
    }
    damn value
}

# Boolean utilities
slay not(value lit) lit {
    bestie value == based {
        damn cap
    }
    damn based
}

slay and(a lit, b lit) lit {
    damn a && b
}

slay or(a lit, b lit) lit {
    damn a || b
}

# String utilities
slay string_concat(a tea, b tea) tea {
    damn a + b
}

# Mathematical utilities
slay pow(base normie, exponent normie) normie {
    bestie exponent == 0 {
        damn 1
    }
    bestie exponent == 1 {
        damn base
    }
    bestie exponent == 2 {
        damn base * base
    }
    bestie exponent == 3 {
        damn base * base * base
    }
    damn base
}

# Memory management placeholder functions
slay shook(message tea) {
    vibez.spill("PANIC: " + message)
}

slay unbothered() lit {
    damn based
}
