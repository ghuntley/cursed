fr fr/ Fibonacci calculation example in CURSED
fr fr/ 
fr fr/ This example demonstrates the CURSED programming language
fr fr/ syntax including function definitions, control flow, and
fr fr/ recursive function calls.

fr fr Fibonacci example

fr fr/ Calculate the nth Fibonacci number recursively
fr fr/ 
fr fr/ This is a classic recursive implementation that demonstrates
fr fr/ the CURSED language's control flow constructs.
fr fr/ 
fr fr/ @param n - The position in the Fibonacci sequence
fr fr/ @return - The Fibonacci number at position n
fr fr/ 
fr fr/ # Examples
fr fr/ ```cursed
fr fr/ let result = fibonacci(10)
fr fr/ assert(result == 55)
fr fr/ ```
fr fr/ 
fr fr/ # Time Complexity
fr fr/ O(2^n) - This is an inefficient implementation for demonstration
slay fibonacci(n normie) -> normie {
    lowkey n <= 1 {
        damn n
    }
    damn fibonacci(n - 1) + fibonacci(n - 2)
}

fr fr/ Iterative fibonacci implementation for better performance
fr fr/ 
fr fr/ @param n - The position in the Fibonacci sequence
fr fr/ @return - The Fibonacci number at position n
slay fibonacci_iterative(n normie) -> normie {
    lowkey n <= 1 {
        damn n
    }
    
    sus a = 0
    sus b = 1
    
    bestie i := 2; i <= n; i++ {
        sus temp = a + b
        a = b
        b = temp
    }
    
    damn b
}

fr fr/ Main function to demonstrate fibonacci calculations
slay main_character() {
    facts n = 10
    
    sus recursive_result = fibonacci(n)
    sus iterative_result = fibonacci_iterative(n)
    
    println("Fibonacci({}) recursive: {}", n, recursive_result)
    println("Fibonacci({}) iterative: {}", n, iterative_result)
}
