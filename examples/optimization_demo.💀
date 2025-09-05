fr fr Optimization Enablement System Demo
fr fr This program demonstrates various optimization scenarios

yeet "stdlib::math::basic"
yeet "stdlib::collections::vector"
yeet "stdlib::io::console"

fr fr Function with potential for inlining optimization
slay calculate_fibonacci(sus n: i32) -> i32 {
    lowkey (n <= 1) {
        return n;
    }
    return calculate_fibonacci(n - 1) + calculate_fibonacci(n - 2);
}

fr fr Function with potential for loop optimization
slay sum_vector(facts vec: vector<i32>) -> i32 {
    sus sum = 0;
    lowkey (sus i = 0; i < vec.len(); i++) {
        sum += vec.get(i);
    }
    return sum;
}

fr fr Function with potential for vectorization
slay multiply_arrays(facts a: vector<f64>, facts b: vector<f64>) -> vector<f64> {
    sus result = vector::new<f64>();
    lowkey (sus i = 0; i < min(a.len(), b.len()); i++) {
        result.push(a.get(i) * b.get(i));
    }
    return result;
}

fr fr Complex computation for optimization testing
slay complex_computation() -> f64 {
    sus result = 0.0;
    lowkey (sus i = 0; i < 1000; i++) {
        lowkey (sus j = 0; j < 100; j++) {
            result += sqrt(i as f64) * pow(j as f64, 2.0);
        }
    }
    return result;
}

slay main_character() {
    println("🚀 CURSED Optimization Demo")?;
    
    // Test function inlining optimization
    println("Testing Fibonacci calculation (inlining candidate)...")?;
    facts fib_result = calculate_fibonacci(15);
    println(&format!("Fibonacci(15) = {}", fib_result))?;
    
    // Test loop optimization
    println("Testing vector sum (loop optimization candidate)...")?;
    sus test_vec = vector::from([1, 2, 3, 4, 5]);
    facts sum_result = sum_vector(test_vec);
    println(&format!("Sum = {}", sum_result))?;
    
    // Test vectorization optimization
    println("Testing array multiplication (vectorization candidate)...")?;
    sus vec_a = vector::from([1.0, 2.0, 3.0, 4.0]);
    sus vec_b = vector::from([2.0, 3.0, 4.0, 5.0]);
    facts mult_result = multiply_arrays(vec_a, vec_b);
    println(&format!("Multiplication result length: {}", mult_result.len()))?;
    
    // Test complex computation for aggressive optimization
    println("Testing complex computation (aggressive optimization candidate)...")?;
    facts complex_result = complex_computation();
    println(&format!("Complex computation result: {:.2}", complex_result))?;
    
    println("✅ Optimization demo completed!")?;
}
