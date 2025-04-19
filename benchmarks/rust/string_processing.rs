// String processing benchmark

use std::time::Instant;
use rand::{Rng, thread_rng};

fn process_strings(count: usize, size: usize) -> String {
    let mut result = String::new();
    
    for _ in 0..count {
        let str = create_random_string(size);
        let processed = process_string(&str);
        result.push_str(&processed);
    }
    
    result
}

fn create_random_string(size: usize) -> String {
    let chars = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
    let chars: Vec<char> = chars.chars().collect();
    let mut rng = thread_rng();
    
    (0..size)
        .map(|_| {
            let idx = rng.gen_range(0..chars.len());
            chars[idx]
        })
        .collect()
}

fn process_string(input: &str) -> String {
    let mut result = input.to_string();
    
    // Replace all vowels with their uppercase version
    result = result.replace("a", "A");
    result = result.replace("e", "E");
    result = result.replace("i", "I");
    result = result.replace("o", "O");
    result = result.replace("u", "U");
    
    // Replace all digits with their doubled value
    for i in 0..10 {
        let digit = i.to_string();
        let doubled = (i * 2).to_string();
        result = result.replace(&digit, &doubled);
    }
    
    // Capitalize the first letter
    if !result.is_empty() {
        let first = result.chars().next().unwrap().to_uppercase().to_string();
        let rest: String = result.chars().skip(1).collect();
        result = format!("{}{}", first, rest);
    }
    
    // Reverse the string
    let reversed: String = result.chars().rev().collect();
    
    // Take the first half of the reversed string
    let half_len = reversed.len() / 2;
    let result = &reversed[0..half_len];
    
    result.to_string()
}

fn main() {
    let start_time = Instant::now();
    
    // Process strings of different sizes
    let small = process_strings(10_000, 10);   // 10,000 strings of length 10
    let medium = process_strings(1_000, 100);  // 1,000 strings of length 100
    let large = process_strings(100, 1_000);   // 100 strings of length 1,000
    
    let result_length = small.len() + medium.len() + large.len();
    println!("Processed string length: {}", result_length);
    
    let elapsed = start_time.elapsed();
    println!("Time taken: {} ms", elapsed.as_millis());
    
    // Get approximate memory usage
    let memory_usage = (small.len() + medium.len() + large.len()) / 1024;
    println!("Memory used: {} KB", memory_usage);
}