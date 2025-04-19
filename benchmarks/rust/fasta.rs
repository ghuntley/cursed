// FASTA benchmark - generate and write random DNA sequences

use std::time::Instant;

// Constants for the random number generator
const IM: i32 = 139968;
const IA: i32 = 3877;
const IC: i32 = 29573;
const SEED: i32 = 42;

// Define DNA sequences
const ALU: &str = "GGCCGGGCGCGGTGGCTCACGCCTGTAATCCCAGCACTTTGGGAGGCCGAGGCGGGCGGATCACCTGAGGTCAGGAGTTCGAGACCAGCCTGGCCAACATGGTGAAACCCCGTCTCTACTAAAAATACAAAAATTAGCCGGGCGTGGTGGCGCGCGCCTGTAATCCCAGCTACTCGGGAGGCTGAGGCAGGAGAATCGCTTGAACCCGGGAGGCGGAGGTTGCAGTGAGCCGAGATCGCGCCACTGCACTCCAGCCTGGGCGACAGAGCGAGACTCCGTCTCAAAAA";

static IUB_PROB: [f64; 15] = [
    0.27, 0.12, 0.12, 0.27, 0.02,
    0.02, 0.02, 0.02, 0.02, 0.02,
    0.02, 0.02, 0.02, 0.02, 0.02,
];

static IUB_CHAR: [char; 15] = [
    'a', 'c', 'g', 't', 'B',
    'D', 'H', 'K', 'M', 'N',
    'R', 'S', 'V', 'W', 'Y',
];

static HOMO_SAPIENS_PROB: [f64; 4] = [
    0.3029549426680, 0.1979883004921,
    0.1975473066391, 0.3015094502008,
];

static HOMO_SAPIENS_CHAR: [char; 4] = [
    'a', 'c', 'g', 't',
];

// Generate a random number
fn gen_random(seed: &mut i32) -> f64 {
    let value = (*seed * IA + IC) % IM;
    *seed = value;
    value as f64 / IM as f64
}

// Generate a random FASTA sequence
fn gen_random_fasta(n: usize, seed: &mut i32, probs: &[f64], chars: &[char]) -> String {
    let length = probs.len();
    let mut buffer = String::with_capacity(n);
    
    for _ in 0..n {
        let mut r = gen_random(seed);
        let mut c = ' ';
        
        for j in 0..length {
            if r < probs[j] {
                c = chars[j];
                break;
            }
            r -= probs[j];
        }
        
        buffer.push(c);
    }
    
    buffer
}

// Repeat a sequence until it reaches the required length
fn repeat_fasta(n: usize, seq: &str) -> String {
    let seq_len = seq.len();
    let seq_bytes = seq.as_bytes();
    let mut buffer = String::with_capacity(n);
    
    for i in 0..n {
        buffer.push(seq_bytes[i % seq_len] as char);
    }
    
    buffer
}

fn main() {
    let n = 1_000_000; // Default sequence length
    let mut seed = SEED;
    let start_time = Instant::now();
    
    // Write FASTA header and sequence for Homo sapiens Alu
    println!(">ONE Homo sapiens alu");
    let alu_seq = repeat_fasta(n, ALU);
    println!("{}", alu_seq);
    
    // Write FASTA header and random sequence for IUB ambiguity codes
    println!(">TWO IUB ambiguity codes");
    let iub_seq = gen_random_fasta(n, &mut seed, &IUB_PROB, &IUB_CHAR);
    println!("{}", iub_seq);
    
    // Write FASTA header and random sequence for Homo sapiens frequency
    println!(">THREE Homo sapiens frequency");
    let sapiens_seq = gen_random_fasta(n, &mut seed, &HOMO_SAPIENS_PROB, &HOMO_SAPIENS_CHAR);
    println!("{}", sapiens_seq);
    
    let elapsed = start_time.elapsed();
    println!("Time taken: {} ms", elapsed.as_millis());
    
    // Calculate approximate memory usage
    let memory_usage = alu_seq.len() + iub_seq.len() + sapiens_seq.len();
    println!("Memory used: {} KB", memory_usage / 1024);
}