# Basic arithmetic functions

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
    damn a / b
}

slay power(base normie, exp normie) normie {
    # Simple power function (base^exp)
    sus result normie = base
    sus i normie = 1
    bestie i < exp {
        result = result * base
        i = i + 1
    }
    damn result
}

slay abs(value normie) normie {
    bestie value < 0 {
        damn -value
    }
    damn value
}

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
