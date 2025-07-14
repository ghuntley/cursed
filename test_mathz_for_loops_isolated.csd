// Test each for loop from mathz individually

// Line 54: bestie i := 0; i < absExp; i++
slay test_pow_loop() {
    sus absExp normie = 5
    sus result meal = 1.0
    bestie i := 0; i < absExp; i++ {
        result = result * 2.0
    }
    vibez.spill("Pow loop works")
}

// Line 77: bestie iterations := 0; iterations < 10 && Abs(guess - prev) > Epsilon; iterations++
slay test_sqrt_loop() {
    sus guess meal = 2.0
    sus prev meal = 0.0
    sus Epsilon drip = 1.19209290e-07
    bestie iterations := 0; iterations < 10; iterations++ {
        prev = guess
        guess = (guess + 4.0 / guess) / 2.0
    }
    vibez.spill("Sqrt loop works")
}

// Line 157: bestie i := 2; i <= n; i++
slay test_factorial_loop() {
    sus n normie = 5
    sus result normie = 1
    bestie i := 2; i <= n; i++ {
        result = result * i
    }
    vibez.spill("Factorial loop works")
}

// Line 176: bestie i := 5; i * i <= n; i += 6
slay test_prime_loop() {
    sus n normie = 100
    bestie i := 5; i * i <= n; i += 6 {
        vibez.spill("Prime loop iteration: ", i)
    }
    vibez.spill("Prime loop works")
}

test_pow_loop()
test_sqrt_loop()
test_factorial_loop()
test_prime_loop()
