slay fibonacci(n normie) normie {
    finna n <= 1 {
        damn n
    }
    damn fibonacci(n - 1) + fibonacci(n - 2)
}

slay factorial(n normie) normie {
    finna n <= 1 {
        damn 1
    }
    damn n * factorial(n - 1)
}

slay is_prime(n normie) lit {
    finna n < 2 {
        damn cap
    }
    bestie i := 2; i * i <= n; i++ {
        finna n % i == 0 {
            damn cap
        }
    }
    damn based
}

vibez.spill("Complex CURSED program test:")
vibez.spill("Fibonacci(7):", fibonacci(7))
vibez.spill("Factorial(5):", factorial(5))
vibez.spill("Is 17 prime?", is_prime(17))
vibez.spill("Is 15 prime?", is_prime(15))

sus numbers [5]normie = {1, 2, 3, 4, 5}
vibez.spill("Array test:")
bestie i := 0; i < 5; i++ {
    vibez.spill("numbers[", i, "] =", numbers[i])
}
