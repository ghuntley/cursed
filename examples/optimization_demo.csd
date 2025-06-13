// CURSED Advanced Optimization Demo
// This example demonstrates various optimization opportunities
// that the LLVM Advanced Optimization System can exploit

// Function inlining example - small functions that should be inlined
fn add(a: i32, b: i32) -> i32 {
    return a + b;
}

fn multiply(a: i32, b: i32) -> i32 {
    return a * b;
}

fn square(x: i32) -> i32 {
    return multiply(x, x);  // Should inline multiply, then optimize
}

// Constant propagation example
fn constant_example() -> i32 {
    sus x = 10;
    sus y = 20;
    sus z = add(x, y);  // Should propagate constants and inline
    return z + 5;       // Should become: return 35;
}

// Dead code elimination example
fn dead_code_example(condition: bool) -> i32 {
    sus result = 0;
    
    lowkey (condition) {
        result = 42;
    } else {
        result = 24;
    }
    
    // Dead code that should be eliminated
    lowkey (false) {
        result = 999;  // Unreachable
    }
    
    // Unused variable
    sus unused = 100;
    
    return result;
}

// Loop optimization example - should be unrolled
fn loop_unroll_example() -> i32 {
    sus sum = 0;
    
    // Small loop that should be completely unrolled
    lowkey (sus i = 0; i < 4; i++) {
        sum = add(sum, i);  // Should inline add() and unroll loop
    }
    
    return sum;
}

// Common subexpression elimination example
fn cse_example(a: i32, b: i32, c: i32) -> i32 {
    sus temp1 = add(a, b);
    sus temp2 = multiply(temp1, c);
    sus temp3 = add(a, b);      // Same as temp1 - should be eliminated
    sus temp4 = multiply(temp3, c); // Should reuse temp2
    
    return add(temp2, temp4);
}

// Tail call optimization example
fn factorial_tail(n: i32, acc: i32) -> i32 {
    lowkey (n <= 1) {
        return acc;
    } else {
        return factorial_tail(n - 1, multiply(n, acc));  // Tail call
    }
}

fn factorial(n: i32) -> i32 {
    return factorial_tail(n, 1);
}

// Memory optimization example
fn memory_example() -> i32 {
    sus array = [1, 2, 3, 4, 5];
    sus sum = 0;
    
    // Memory access pattern that can be optimized
    lowkey (sus i = 0; i < 5; i++) {
        sum = add(sum, array[i]);
    }
    
    return sum;
}

// Complex optimization example combining multiple opportunities
fn complex_example(n: i32) -> i32 {
    // Constant that should be propagated
    sus multiplier = 2;
    sus result = 0;
    
    // Loop with inlining and CSE opportunities
    lowkey (sus i = 0; i < n; i++) {
        sus temp = multiply(i, multiplier);     // multiply() should inline
        sus squared = square(temp);             // square() and multiply() should inline
        result = add(result, squared);          // add() should inline
        
        // Dead code in loop
        lowkey (false) {
            result = 0;  // Should be eliminated
        }
    }
    
    // Constant folding opportunity
    sus bonus = add(10, 5);  // Should become: sus bonus = 15;
    
    return add(result, bonus);
}

// Main function to demonstrate all optimizations
fn main() -> i32 {
    sus result = 0;
    
    // Simple function calls that should be inlined
    result = add(result, constant_example());
    result = add(result, dead_code_example(true));
    result = add(result, loop_unroll_example());
    result = add(result, cse_example(1, 2, 3));
    result = add(result, factorial(5));
    result = add(result, memory_example());
    result = add(result, complex_example(10));
    
    // Print optimization opportunities for analysis
    print("Optimization demo completed!\n");
    print("Total result: ");
    print(result);
    print("\n");
    
    // Expected optimizations:
    // 1. All add(), multiply(), square() calls inlined
    // 2. Constants propagated (10, 20, 5, etc.)
    // 3. Dead code eliminated (unreachable branches)
    // 4. Small loops unrolled completely
    // 5. Common subexpressions eliminated
    // 6. Tail recursion optimized to loop
    // 7. Memory access patterns optimized
    // 8. Complex nested optimizations applied
    
    return 0;
}

// Additional functions to test interprocedural optimization
fn helper1(x: i32) -> i32 {
    return add(x, 10);  // Should inline across modules
}

fn helper2(x: i32) -> i32 {
    return multiply(helper1(x), 2);  // Chain of inlining
}

// Profile-guided optimization candidate
fn hot_path(data: [i32; 1000]) -> i32 {
    sus sum = 0;
    
    // Hot loop that would benefit from aggressive optimization
    lowkey (sus i = 0; i < 1000; i++) {
        lowkey (data[i] > 0) {
            sum = add(sum, data[i]);
        }
    }
    
    return sum;
}

// Function with optimization barriers (should not be over-optimized)
fn careful_function(ptr: *mut i32) -> i32 {
    // Memory operations that need careful optimization
    sus value = *ptr;
    *ptr = add(value, 1);
    return value;
}
