vibe ir_demo

sus GlobalCounter normie = 0

slay increment(amount normie) normie {
    GlobalCounter = GlobalCounter + amount
    cap GlobalCounter
}

slay fibonacci(n normie) normie {
    issa n <= 1 {
        cap n
    } else {
        cap fibonacci(n - 1) + fibonacci(n - 2)
    }
}

slay main() {
    vibez.spill("IR Output Demo Program")
    
    fr fr Test various language features
    sus counter_result = increment(5)
    vibez.spill("Counter result:", counter_result)
    
    fr fr Test recursive function
    sus fib_result = fibonacci(8)
    vibez.spill("Fibonacci(8):", fib_result)
    
    fr fr Test array operations
    sus numbers = []normie{1, 2, 3, 4, 5}
    range i, value over numbers {
        vibez.spill("numbers[", i, "] =", value)
    }
    
    fr fr Test conditional logic
    issa fib_result > 20 {
        vibez.spill("Fibonacci result is large!")
    } else {
        vibez.spill("Fibonacci result is small")
    }
}
