// Optimization Demo - Examples to showcase LLVM optimization effectiveness
// This file contains various code patterns that benefit from different optimization levels

// Simple arithmetic that can be constant-folded
fn constant_folding_demo() normie {
    let a = 5 + 3;
    let b = a * 2;
    let c = b / 4;
    return c + 1;
}

// Dead code elimination example
fn dead_code_demo(x normie) normie {
    let unused = 42;  // Dead code
    let y = x + 1;
    let z = y * 2;
    let dead_result = unused * 100;  // Dead code
    return z;
}

// Function that can benefit from inlining
fn small_helper(a normie, b normie) normie {
    return a + b * 2;
}

fn inlining_demo(x normie) normie {
    let result1 = small_helper(x, 5);
    let result2 = small_helper(result1, 3);
    return result2;
}

// Loop that can benefit from optimization
fn loop_optimization_demo(n normie) normie {
    let sum = 0;
    let i = 0;
    
    while i < n {
        sum = sum + i;
        i = i + 1;
    }
    
    return sum;
}

// Loop with invariant code motion opportunities
fn loop_invariant_demo(arr []normie, multiplier normie) []normie {
    let result []normie;
    let i = 0;
    
    while i < len(arr) {
        // This calculation could be moved outside the loop
        let constant_factor = multiplier * 2 + 5;
        result = append(result, arr[i] * constant_factor);
        i = i + 1;
    }
    
    return result;
}

// Common subexpression elimination example
fn cse_demo(a normie, b normie, c normie) normie {
    let x = a + b;
    let y = x * c;
    let z = a + b;  // Common subexpression
    return y + z;
}

// Redundant memory operations
fn memory_optimization_demo(x normie) normie {
    let temp = x;
    temp = temp + 1;
    temp = temp * 2;
    temp = temp - 1;
    return temp;
}

// Branch optimization example
fn branch_optimization_demo(condition bool, x normie) normie {
    if condition {
        if condition {  // Redundant check
            return x + 1;
        } else {
            return x;
        }
    } else {
        return x - 1;
    }
}

// Tail call optimization candidate
fn tail_recursive_factorial(n normie, acc normie) normie {
    if n <= 1 {
        return acc;
    }
    return tail_recursive_factorial(n - 1, acc * n);
}

fn tail_call_demo(n normie) normie {
    return tail_recursive_factorial(n, 1);
}

// Vectorization opportunity (simple array operations)
fn vectorization_demo(arr []normie) []normie {
    let result []normie;
    let i = 0;
    
    while i < len(arr) {
        // Simple arithmetic that could be vectorized
        result = append(result, arr[i] * 2 + 1);
        i = i + 1;
    }
    
    return result;
}

// Complex optimization example combining multiple opportunities
fn complex_optimization_demo(data []normie, threshold normie) normie {
    let sum = 0;
    let count = 0;
    let i = 0;
    
    // Loop with multiple optimization opportunities
    while i < len(data) {
        let value = data[i];
        
        // Constant calculation (could be moved out)
        let adjusted_threshold = threshold + 5;
        
        if value > adjusted_threshold {
            // Dead code if condition is never true
            let unused_calc = value * 100;
            
            sum = sum + value;
            count = count + 1;
            
            // Common subexpression
            let processed_value = value + adjusted_threshold;
            sum = sum + processed_value;
        }
        
        i = i + 1;
    }
    
    if count > 0 {
        return sum / count;
    } else {
        return 0;
    }
}

// Main function demonstrating different optimization scenarios
fn main() normie {
    vibez.spill("Running optimization demo...");
    
    // Test constant folding
    let folded = constant_folding_demo();
    vibez.spill("Constant folding result: " + folded.as.string);
    
    // Test dead code elimination
    let dead_result = dead_code_demo(10);
    vibez.spill("Dead code demo result: " + dead_result.as.string);
    
    // Test inlining
    let inlined = inlining_demo(5);
    vibez.spill("Inlining demo result: " + inlined.as.string);
    
    // Test loop optimization
    let loop_result = loop_optimization_demo(100);
    vibez.spill("Loop optimization result: " + loop_result.as.string);
    
    // Test CSE
    let cse_result = cse_demo(3, 4, 5);
    vibez.spill("CSE demo result: " + cse_result.as.string);
    
    // Test memory optimization
    let mem_result = memory_optimization_demo(10);
    vibez.spill("Memory optimization result: " + mem_result.as.string);
    
    // Test branch optimization
    let branch_result = branch_optimization_demo(true, 20);
    vibez.spill("Branch optimization result: " + branch_result.as.string);
    
    // Test tail call optimization
    let factorial_result = tail_call_demo(5);
    vibez.spill("Tail call demo result: " + factorial_result.as.string);
    
    // Test with array data
    let test_array = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let vectorized = vectorization_demo(test_array);
    vibez.spill("Vectorization demo completed");
    
    // Test loop invariant motion
    let invariant_result = loop_invariant_demo(test_array, 3);
    vibez.spill("Loop invariant demo completed");
    
    // Test complex optimization
    let complex_result = complex_optimization_demo(test_array, 5);
    vibez.spill("Complex optimization result: " + complex_result.as.string);
    
    vibez.spill("Optimization demo completed!");
    
    return 0;
}
