// Mathematical utility functions
export fibonacci, factorial, prime_check, random_in_range

func fibonacci(n: int) -> int {
    if n <= 1 {
        return n
    }
    return fibonacci(n - 1) + fibonacci(n - 2)
}

func factorial(n: int) -> int {
    if n <= 1 {
        return 1
    }
    return n * factorial(n - 1)
}

func prime_check(n: int) -> bool {
    if n < 2 {
        return false
    }
    for i in range(2, sqrt(n) + 1) {
        if n % i == 0 {
            return false
        }
    }
    return true
}

func random_in_range(min: int, max: int) -> int {
    return min + (rand() % (max - min + 1))
}

func sqrt(n: int) -> int {
    // Simple integer square root
    if n == 0 {
        return 0
    }
    let x = n
    while true {
        let root = (x + n / x) / 2
        if root >= x {
            return x
        }
        x = root
    }
}

func rand() -> int {
    // Simple pseudo-random number generator
    static mut seed: int = 12345
    seed = (seed * 1103515245 + 12345) % (1 << 31)
    return seed
}
