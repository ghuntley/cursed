// Test different optimization levels
slay fibonacci(n normie) normie {
    sus a normie = 0
    sus b normie = 1
    sus i normie = 0
    bestie i = 0; i < n; i++ {
        sus temp normie = a
        a = b
        b = temp + b
    }
    damn a
}

slay matrix_multiply(size normie) normie {
    sus result normie = 0
    sus i normie = 0
    sus j normie = 0
    sus k normie = 0
    
    bestie i = 0; i < size; i++ {
        bestie j = 0; j < size; j++ {
            bestie k = 0; k < size; k++ {
                result = result + (i * j * k)
            }
        }
    }
    
    damn result
}

slay main() {
    sus fib_result normie = fibonacci(10)
    sus matrix_result normie = matrix_multiply(5)
    
    vibez.spill("Fibonacci(10): ", fib_result)
    vibez.spill("Matrix result: ", matrix_result)
}
