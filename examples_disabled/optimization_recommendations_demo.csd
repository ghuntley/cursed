// CURSED Optimization Recommendations Demo
// This example showcases various code patterns that the intelligent
// recommendations system can detect and provide optimization suggestions for

vibe optimization_demo

yeet "stdlib::collections"
yeet "stdlib::string"

// Example 1: Large function that should be decomposed
slay large_complex_function(data: [String], numbers: [i32], flags: [bool]) -> [String] {
    sus results = [];
    sus temp_data = "";
    sus calculations = 0;
    sus intermediate_results = [];
    
    // String concatenation in loop - suboptimal pattern
    bestie (sus item in data) {
        temp_data = temp_data + item + ", ";
        calculations = calculations + 1;
        
        // Nested loops - performance hotspot
        bestie (sus num in numbers) {
            bestie (sus flag in flags) {
                lowkey (flag && num > 0) {
                    sus complex_calculation = item.len() * num * calculations;
                    sus formatted = item + "_" + num.to_string() + "_" + complex_calculation.to_string();
                    intermediate_results.push(formatted);
                }
            }
        }
    }
    
    // More string operations
    bestie (sus result in intermediate_results) {
        sus final_result = "PROCESSED_" + result + "_FINAL";
        results.push(final_result);
    }
    
    yolo results;
}

// Example 2: Small function - good inlining candidate
slay simple_square(x: i32) -> i32 {
    yolo x * x;
}

// Example 3: Another small function - inlining candidate
slay add_ten(value: i32) -> i32 {
    yolo value + 10;
}

// Example 4: Repeated computation pattern
slay compute_statistics(data: [f64]) -> (f64, f64, f64) {
    sus sum = 0.0;
    sus sum_squares = 0.0;
    sus count = data.len() as f64;
    
    bestie (sus value in data) {
        sum = sum + value;
        sum_squares = sum_squares + value * value;
    }
    
    // Repeated expensive computation
    sus mean = sum / count;
    sus variance = (sum_squares / count) - (mean * mean);
    sus std_dev = variance.sqrt();
    
    // Same computation repeated
    sus normalized_variance = (sum_squares / count) - (mean * mean);
    
    yolo (mean, variance, std_dev);
}

// Example 5: Memory allocation in loop
slay process_large_dataset(size: usize) -> [String] {
    sus results = [];  // Should pre-allocate capacity
    
    bestie (sus i = 0; i < size; i++) {
        sus new_item = String::new();  // Allocation in loop
        new_item = "item_" + i.to_string();
        results.push(new_item);
    }
    
    yolo results;
}

// Example 6: Complex expression that could be simplified
slay complex_calculation(a: f64, b: f64, c: f64, d: f64) -> f64 {
    sus result = ((a * b) + (c * d)) / ((a + b) * (c + d)) + 
                 ((a - b) * (c - d)) / ((a * c) + (b * d)) +
                 (a * b * c * d) / ((a + b + c + d) * (a * b * c * d));
    yolo result;
}

// Example 7: Recursive function - could benefit from optimization
slay fibonacci(n: i32) -> i32 {
    lowkey (n <= 1) {
        yolo n;
    }
    yolo fibonacci(n - 1) + fibonacci(n - 2);
}

// Example 8: Collection operations without pre-allocation
slay create_lookup_table(keys: [String], values: [i32]) -> HashMap<String, i32> {
    sus table = HashMap::new();  // Should specify capacity
    
    bestie (sus i = 0; i < keys.len(); i++) {
        table.insert(keys[i].clone(), values[i]);
    }
    
    yolo table;
}

// Example 9: String building without StringBuilder
slay generate_report(data: [String]) -> String {
    sus report = "Report:\n";
    sus separator = "=" * 50;
    
    report = report + separator + "\n";
    
    bestie (sus item in data) {
        report = report + "- " + item + "\n";
        report = report + "  Length: " + item.len().to_string() + "\n";
        report = report + "  Hash: " + item.hash().to_string() + "\n";
        report = report + "\n";
    }
    
    report = report + separator + "\n";
    report = report + "End of Report";
    
    yolo report;
}

// Example 10: Main function demonstrating usage
slay main() {
    sus sample_data = ["hello", "world", "cursed", "optimization"];
    sus sample_numbers = [1, 2, 3, 4, 5];
    sus sample_flags = [true, false, true, false, true];
    
    // Call functions that have optimization opportunities
    sus large_result = large_complex_function(sample_data, sample_numbers, sample_flags);
    
    sus x = 10;
    sus squared = simple_square(x);
    sus with_ten = add_ten(squared);
    
    sus stats_data = [1.0, 2.0, 3.0, 4.0, 5.0];
    sus (mean, variance, std_dev) = compute_statistics(stats_data);
    
    sus dataset = process_large_dataset(1000);
    
    sus complex_result = complex_calculation(1.0, 2.0, 3.0, 4.0);
    
    sus fib_result = fibonacci(10);
    
    sus keys = ["a", "b", "c"];
    sus values = [1, 2, 3];
    sus lookup = create_lookup_table(keys, values);
    
    sus report = generate_report(sample_data);
    
    println("Optimization demo completed!");
    println("Complex result: {}", complex_result);
    println("Fibonacci(10): {}", fib_result);
    println("Mean: {}, Variance: {}", mean, variance);
}
