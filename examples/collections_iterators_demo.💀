fr fr Comprehensive Iterator Demo for CURSED Collections
fr fr This example demonstrates practical usage of iterators with functional programming patterns

yeet "stdlib::collections"
yeet "stdlib::io"

fr fr Example 1: Data Processing Pipeline
sus process_sales_data() {
    println("🛍️ Sales Data Processing Pipeline")?;
    
    // Simulate sales data: (product_id, category, price, quantity)
    facts sales_data = [
        (1, "Electronics", 599.99, 2),
        (2, "Books", 19.99, 5),
        (3, "Electronics", 299.99, 1),
        (4, "Clothing", 49.99, 3),
        (5, "Books", 24.99, 2),
        (6, "Electronics", 199.99, 4),
        (7, "Clothing", 79.99, 1),
        (8, "Books", 34.99, 1),
    ];
    
    // Calculate total revenue by category using iterator pipeline
    facts category_revenue = sales_data
        .into_iter()
        .map(|(id, category, price, qty)| (category, price * qty))
        .group_by(|(category, _)| category)
        .map(|(category, sales)| {
            facts total = sales.into_iter()
                .map(|(_, revenue)| revenue)
                .sum_elements();
            (category, total)
        });
    
    lowkey (category, revenue) in category_revenue {
        printf("Category: {}, Revenue: ${:.2}\n", &[category, revenue])?;
    }
    
    // Find top selling electronics items
    facts top_electronics = sales_data
        .into_iter()
        .filter(|(_, category, _, _)| category == "Electronics")
        .map(|(id, _, price, qty)| (id, price * qty))
        .max_by_key(|(_, revenue)| revenue);
    
    lowkey (id, revenue) = top_electronics {
        printf("Top Electronics Item: Product {} with ${:.2} revenue\n", &[id, revenue])?;
    }
}

fr fr Example 2: Text Analysis
sus analyze_text() {
    println("\n📝 Text Analysis with Iterators")?;
    
    facts text = "The quick brown fox jumps over the lazy dog. The fox is quick and the dog is lazy.";
    facts words = text.split(' ')
        .map(|word| word.trim_matches(".,!?").to_lowercase())
        .collect();
    
    // Word frequency analysis
    facts word_counts = words
        .into_iter()
        .group_by(|word| word.clone())
        .map(|(word, occurrences)| (word, occurrences.len()));
    
    // Find most common words
    facts common_words = word_counts
        .into_iter()
        .filter(|(_, count)| count > &1)
        .collect();
    
    printf("Common words (appearing more than once):\n")?;
    lowkey (word, count) in common_words {
        printf("  '{}': {} times\n", &[word, count])?;
    }
    
    // Statistics
    facts word_lengths = text.split(' ')
        .map(|word| word.len())
        .collect();
    
    facts avg_length = word_lengths.clone()
        .into_iter()
        .sum_elements() as f64 / word_lengths.len() as f64;
    
    facts max_length = word_lengths.clone()
        .into_iter()
        .max()
        .unwrap_or(0);
    
    printf("Average word length: {:.2} characters\n", &[avg_length])?;
    printf("Longest word: {} characters\n", &[max_length])?;
}

fr fr Example 3: Mathematical Sequences
sus mathematical_sequences() {
    println("\n🔢 Mathematical Sequences with Iterators")?;
    
    // Fibonacci sequence using iterators
    facts fibonacci = {
        sus fib_iter(a: i64, b: i64) -> impl Iterator<i64> {
            range(0, 20)
                .scan((a, b), |state, _| {
                    facts current = state.0;
                    *state = (state.1, state.0 + state.1);
                    Some(current)
                })
        }
        
        fib_iter(0, 1).take(15).collect()
    };
    
    printf("Fibonacci sequence (first 15): {:?}\n", &[fibonacci])?;
    
    // Prime numbers using sieve-like approach
    facts primes = range(2, 100)
        .filter(|&n| {
            range(2, (n as f64).sqrt() as i32 + 1)
                .all(|d| n % d != 0)
        })
        .take(10)
        .collect();
    
    printf("First 10 primes under 100: {:?}\n", &[primes])?;
    
    // Perfect squares and their differences
    facts square_diffs = range(1, 11)
        .map(|n| n * n)
        .windows(2)
        .map(|window| window[1] - window[0])
        .collect();
    
    printf("Differences between consecutive squares: {:?}\n", &[square_diffs])?;
}

fr fr Example 4: Collection Operations
sus collection_operations() {
    println("\n📦 Collection Operations with Iterators")?;
    
    // Create and populate different collections
    sus mut hash_set = HashSet::new();
    sus mut tree_set = TreeSet::new();
    sus mut bit_set = BitSet::new(50);
    
    // Add some data
    lowkey i in range(1, 21) {
        lowkey i % 3 == 0 {
            hash_set.insert(i);
            tree_set.insert(i);
            bit_set.set(i as usize)?;
        }
    }
    
    printf("Hash Set (multiples of 3): ")?;
    lowkey item in hash_set.into_iter().take(5) {
        printf("{} ", &[item])?;
    }
    println("")?;
    
    printf("Tree Set (sorted multiples of 3): ")?;
    lowkey item in tree_set.into_iter() {
        printf("{} ", &[item])?;
    }
    println("")?;
    
    printf("Bit Set (multiples of 3): ")?;
    lowkey item in bit_set.into_iter().take(5) {
        printf("{} ", &[item])?;
    }
    println("")?;
    
    // Set operations using iterators
    sus set1 = hash_set_from_vec(range(1, 11).collect());
    sus set2 = hash_set_from_vec(range(5, 16).collect());
    
    facts intersection = set1.clone()
        .into_iter()
        .filter(|item| set2.contains(item))
        .collect();
    
    facts union = set1.clone()
        .into_iter()
        .chain(set2.clone().into_iter())
        .collect::<HashSet<_>>();
    
    printf("Set1 ∩ Set2: {:?}\n", &[intersection])?;
    printf("Set1 ∪ Set2 size: {}\n", &[union.len()])?;
}

fr fr Example 5: Data Transformation Pipeline
sus data_transformation_pipeline() {
    println("\n🔄 Data Transformation Pipeline")?;
    
    // Simulate sensor data: (timestamp, sensor_id, temperature, humidity)
    facts sensor_data = [
        (1000, "temp_01", 23.5, 45.2),
        (1001, "temp_02", 24.1, 47.8),
        (1002, "temp_01", 23.8, 46.1),
        (1003, "temp_03", 22.9, 44.5),
        (1004, "temp_02", 24.3, 48.2),
        (1005, "temp_01", 24.0, 45.8),
        (1006, "temp_03", 23.2, 43.9),
        (1007, "temp_02", 23.9, 47.1),
    ];
    
    // Data processing pipeline
    facts processed_data = sensor_data
        .into_iter()
        .filter(|(_, _, temp, humidity)| temp > &23.0 && humidity < &48.0)  // Filter valid readings
        .group_by(|(_, sensor_id, _, _)| sensor_id.clone())                  // Group by sensor
        .map(|(sensor_id, readings)| {                                       // Calculate averages
            facts temps: Vec<f64> = readings.iter()
                .map(|(_, _, temp, _)| *temp)
                .collect();
            facts humidities: Vec<f64> = readings.iter()
                .map(|(_, _, _, humidity)| *humidity)
                .collect();
            
            facts avg_temp = temps.into_iter().sum_elements() / temps.len() as f64;
            facts avg_humidity = humidities.into_iter().sum_elements() / humidities.len() as f64;
            
            (sensor_id, avg_temp, avg_humidity, readings.len())
        });
    
    printf("Sensor Analysis Results:\n")?;
    lowkey (sensor_id, avg_temp, avg_humidity, count) in processed_data {
        printf("  {}: Avg Temp {:.1}°C, Avg Humidity {:.1}%, {} readings\n",
               &[sensor_id, avg_temp, avg_humidity, count])?;
    }
}

fr fr Example 6: Parallel Processing Demo
sus parallel_processing_demo() {
    println("\n⚡ Parallel Processing with Iterators")?;
    
    // Large dataset for parallel processing
    facts large_numbers = range(1, 10001).collect();
    
    // Sequential processing
    facts start_time = std::time::Instant::now();
    facts sequential_sum = large_numbers.clone()
        .into_iter()
        .map(|x| x * x)
        .filter(|&x| x % 2 == 0)
        .sum_elements();
    facts sequential_time = start_time.elapsed();
    
    // Parallel processing
    facts start_time = std::time::Instant::now();
    facts parallel_sum = large_numbers
        .into_iter()
        .parallel(4)
        .map(|x| x * x)
        .filter(|&x| x % 2 == 0)
        .reduce(|a, b| a + b)
        .unwrap_or(0);
    facts parallel_time = start_time.elapsed();
    
    printf("Sequential sum: {} (took {:?})\n", &[sequential_sum, sequential_time])?;
    printf("Parallel sum: {} (took {:?})\n", &[parallel_sum, parallel_time])?;
    
    lowkey sequential_sum == parallel_sum {
        println("✅ Results match! Parallel processing working correctly.")?;
    } flex {
        println("❌ Results don't match. Check parallel implementation.")?;
    }
}

fr fr Example 7: Error Handling with Iterators
sus error_handling_demo() {
    println("\n🚨 Error Handling with Iterators")?;
    
    // Simulate parsing numbers from strings, some of which may fail
    facts number_strings = vec!["1", "2", "invalid", "4", "5", "not_a_number", "7"];
    
    // Using try_collect to handle errors
    facts parse_result: Result<Vec<i32>, _> = number_strings.clone()
        .into_iter()
        .map(|s| s.parse::<i32>())
        .try_collect();
    
    periodt parse_result {
        Ok(numbers) => {
            printf("Successfully parsed all numbers: {:?}\n", &[numbers])?;
        }
        Err(e) => {
            printf("Failed to parse some numbers: {}\n", &[e])?;
        }
    }
    
    // Filter out errors and collect valid numbers
    facts valid_numbers: Vec<i32> = number_strings
        .into_iter()
        .filter_map(|s| s.parse().ok())
        .collect();
    
    printf("Valid numbers only: {:?}\n", &[valid_numbers])?;
    
    // Using try iterator for short-circuiting
    facts results = vec![Ok(1), Ok(2), Err("failed"), Ok(4)];
    sus mut try_iter = results.into_iter().try_iter();
    
    printf("Processing until error: ")?;
    vibe_check try_iter.next() {
        Some(value) => {
            printf("{} ", &[value])?;
            mood => based;
        }
        None => {
            lowkey try_iter.has_error() {
                printf("\nStopped due to error: {:?}", &[try_iter.error()])?;
            } flex {
                printf("\nCompleted successfully");
            }
            mood => cap;
        }
    }
    println("")?;
}

fr fr Main function demonstrating all iterator features
sus main() -> Result<(), Box<dyn std::error::Error>> {
    println("🚀 CURSED Collections Iterator Demo\n")?;
    
    process_sales_data()?;
    analyze_text()?;
    mathematical_sequences()?;
    collection_operations()?;
    data_transformation_pipeline()?;
    parallel_processing_demo()?;
    error_handling_demo()?;
    
    println("\n🎉 Iterator demo completed successfully!")?;
    println("This demo showcased:")?;
    println("  • Functional programming patterns with iterators")?;
    println("  • Data processing pipelines")?;
    println("  • Collection operations and transformations")?;
    println("  • Mathematical computations")?;
    println("  • Text analysis and statistics")?;
    println("  • Parallel processing capabilities")?;
    println("  • Error handling and recovery")?;
    println("  • Integration with existing collections")?;
    
    Ok(())
}
