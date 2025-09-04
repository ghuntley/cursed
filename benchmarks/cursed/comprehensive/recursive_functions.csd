slay fibonacci(n drip) drip {
    ready (n <= 1) { damn n }
    damn fibonacci(n - 1) + fibonacci(n - 2)
}

slay ackermann(m drip, n drip) drip {
    ready (m == 0) { damn n + 1 }
    ready (n == 0) { damn ackermann(m - 1, 1) }
    damn ackermann(m - 1, ackermann(m, n - 1))
}

slay main_character() drip {
    sus total drip = 0
    bestie (sus i drip = 1; i <= 30; i++) {
        total = total + fibonacci(i)
    }
    # Add some Ackermann calls (small values)
    total = total + ackermann(3, 3)
    damn total
}