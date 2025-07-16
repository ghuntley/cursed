yeet "testz"

# Mathematical utilities library for CURSED

# Basic arithmetic operations
slay add(a normie, b normie) normie {
    damn a + b
}

slay multiply(a normie, b normie) normie {
    damn a * b
}

slay power(base normie, exponent normie) normie {
    lowkey exponent == 0 {
        damn 1
    }
    
    sus result normie = base
    bestie i := 1; i < exponent; i++ {
        result = result * base
    }
    damn result
}

# Advanced mathematical functions
slay factorial(n normie) normie {
    lowkey n <= 1 {
        damn 1
    }
    damn n * factorial(n - 1)
}

slay fibonacci(n normie) normie {
    lowkey n <= 1 {
        damn n
    }
    damn fibonacci(n - 1) + fibonacci(n - 2)
}

# Prime number utilities
slay is_prime(n normie) lit {
    lowkey n < 2 {
        damn cap
    }
    
    bestie i := 2; i * i <= n; i++ {
        lowkey n % i == 0 {
            damn cap
        }
    }
    damn based
}

# Greatest common divisor
slay gcd(a normie, b normie) normie {
    lowkey b == 0 {
        damn a
    }
    damn gcd(b, a % b)
}

# Least common multiple
slay lcm(a normie, b normie) normie {
    damn (a * b) / gcd(a, b)
}
