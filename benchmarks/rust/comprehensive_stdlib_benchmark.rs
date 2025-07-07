// Comprehensive Rust Stdlib Performance Benchmark (for comparison)

use std::time::Instant;
use std::collections::HashMap;
use std::sync::mpsc;
use std::thread;
use sha2::{Sha256, Digest};
use base64;

const BENCHMARK_ITERATIONS: usize = 1000;

fn benchmark_math_operations() {
    println!("Starting benchmark: Math Operations");
    let start = Instant::now();
    
    let mut total = 0.0f64;
    for i in 0..BENCHMARK_ITERATIONS {
        let x = i as f64;
        total += x.sqrt();
        total += x.sin();
        total += x.cos();
        total += (x + 1.0).ln();
        total += (x / 1000.0).exp();
    }
    
    let elapsed = start.elapsed();
    println!("Math operations total: {}", total);
    println!("Benchmark Math Operations completed in {}ms", elapsed.as_millis());
}

fn benchmark_string_operations() {
    println!("Starting benchmark: String Operations");
    let start = Instant::now();
    
    let test_string = "Hello World! This is a test string for benchmarking.";
    let mut result = String::new();
    
    for i in 0..BENCHMARK_ITERATIONS {
        let upper = test_string.to_uppercase();
        let lower = upper.to_lowercase();
        let concat = format!("{} - iteration {}", lower, i);
        let substring = &concat[0..20.min(concat.len())];
        result.push_str(substring);
    }
    
    let elapsed = start.elapsed();
    println!("String operations result length: {}", result.len());
    println!("Benchmark String Operations completed in {}ms", elapsed.as_millis());
}

fn benchmark_collections_operations() {
    println!("Starting benchmark: Collections Operations");
    let start = Instant::now();
    
    let mut map = HashMap::new();
    let mut vec = Vec::new();
    
    // HashMap operations
    for i in 0..BENCHMARK_ITERATIONS {
        let key = format!("key_{}", i);
        let value = i * 2;
        map.insert(key.clone(), value);
        
        if i % 2 == 0 {
            let _retrieved = map.get(&key);
        }
    }
    
    // Vector operations
    for i in 0..BENCHMARK_ITERATIONS {
        vec.push(i);
        
        if i % 3 == 0 && !vec.is_empty() {
            let _popped = vec.pop();
        }
    }
    
    let elapsed = start.elapsed();
    println!("HashMap size: {}", map.len());
    println!("Vector size: {}", vec.len());
    println!("Benchmark Collections Operations completed in {}ms", elapsed.as_millis());
}

fn benchmark_crypto_operations() {
    println!("Starting benchmark: Crypto Operations");
    let start = Instant::now();
    
    let test_data = "This is test data for cryptographic operations benchmarking.";
    let iterations = BENCHMARK_ITERATIONS / 10; // Crypto operations are expensive
    
    for i in 0..iterations {
        // SHA256 hashing
        let mut hasher = Sha256::new();
        hasher.update(format!("{}{}", test_data, i).as_bytes());
        let _hash = hasher.finalize();
        
        // Base64 encoding/decoding
        let encoded = base64::encode(test_data);
        let _decoded = base64::decode(&encoded).unwrap();
        
        // Simple XOR encryption/decryption (placeholder for AES)
        let key = b"test_key_1234567890123456";
        let mut encrypted = Vec::new();
        for (i, byte) in test_data.bytes().enumerate() {
            encrypted.push(byte ^ key[i % key.len()]);
        }
        
        let mut decrypted = Vec::new();
        for (i, byte) in encrypted.iter().enumerate() {
            decrypted.push(byte ^ key[i % key.len()]);
        }
    }
    
    let elapsed = start.elapsed();
    println!("Crypto operations completed: {}", iterations);
    println!("Benchmark Crypto Operations completed in {}ms", elapsed.as_millis());
}

fn benchmark_memory_operations() {
    println!("Starting benchmark: Memory Operations");
    let start = Instant::now();
    
    let allocations = BENCHMARK_ITERATIONS / 5; // Memory operations are expensive
    let mut pointers = Vec::new();
    
    // Allocation benchmark
    for i in 0..allocations {
        let size = 1024 + (i % 1024); // Variable size allocations
        let vec = vec![0u8; size];
        pointers.push(vec);
    }
    
    // Deallocation benchmark (automatic with Vec::drop)
    drop(pointers);
    
    let elapsed = start.elapsed();
    println!("Memory operations completed: {} allocations", allocations);
    println!("Benchmark Memory Operations completed in {}ms", elapsed.as_millis());
}

fn benchmark_async_operations() {
    println!("Starting benchmark: Async Operations");
    let start = Instant::now();
    
    let (tx, rx) = mpsc::channel();
    
    // Spawn producer thread
    let producer_handle = thread::spawn(move || {
        for i in 0..BENCHMARK_ITERATIONS {
            tx.send(i).unwrap();
        }
    });
    
    // Consume messages
    let mut received = 0;
    for _ in 0..BENCHMARK_ITERATIONS {
        let msg = rx.recv().unwrap();
        received += msg;
    }
    
    producer_handle.join().unwrap();
    
    let elapsed = start.elapsed();
    println!("Async operations - messages received: {}", received);
    println!("Benchmark Async Operations completed in {}ms", elapsed.as_millis());
}

fn run_comprehensive_benchmarks() {
    println!("=== Rust Stdlib Comprehensive Performance Benchmark ===");
    println!("Iterations per benchmark: {}", BENCHMARK_ITERATIONS);
    println!();
    
    let total_start = Instant::now();
    
    benchmark_math_operations();
    benchmark_string_operations();
    benchmark_collections_operations();
    benchmark_crypto_operations();
    benchmark_memory_operations();
    benchmark_async_operations();
    
    let total_elapsed = total_start.elapsed();
    println!();
    println!("=== Benchmark Suite Complete ===");
    println!("Total time: {}ms", total_elapsed.as_millis());
    println!("Average time per benchmark: {}ms", total_elapsed.as_millis() / 6);
    
    // Memory usage summary (simplified)
    println!("Final memory stats:");
    println!("  Note: Rust uses automatic memory management");
    println!("  No explicit GC statistics available");
}

fn main() {
    run_comprehensive_benchmarks();
}
