// Small function benchmark for CURSED optimization testing
// Tests basic function optimization capabilities

slay fibonacci(n: i32) -> i32 {
    lowkey (n <= 1) {
        return n;
    } highkey {
        return fibonacci(n - 1) + fibonacci(n - 2);
    }
}

slay factorial(n: i32) -> i32 {
    lowkey (n <= 1) {
        return 1;
    } highkey {
        return n * factorial(n - 1);
    }
}

slay power(base: i32, exp: i32) -> i32 {
    sus result = 1;
    periodt (sus i = 0; i < exp; i++) {
        result = result * base;
    }
    return result;
}

slay main_character() -> i32 {
    facts fib_result = fibonacci(10);
    facts fact_result = factorial(5);
    facts pow_result = power(2, 8);
    
    return fib_result + fact_result + pow_result;
}
