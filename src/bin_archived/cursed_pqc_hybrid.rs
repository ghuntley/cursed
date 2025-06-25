use crate::error::CursedError;
// CURSED Post-Quantum Cryptography Hybrid Tool
// 
// A command-line tool for managing hybrid cryptographic operations
// that combine classical and post-quantum algorithms.

use std::fs;
use std::path::PathBuf;
use std::time::Instant;
use clap::{Parser, Subcommand, ValueEnum};
use serde::{Serialize, Deserialize};

use cursed::stdlib::crypto_pqc::*;
use cursed::stdlib::crypto_pqc::hybrid::*;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(name = "cursed-pqc-hybrid")]
struct Cli {
    #[command(subcommand)]
#[derive(Subcommand)]
enum Commands {
    /// Generate hybrid key pairs
    Keygen {
        /// Classical algorithm to use
        #[arg(short, long)]
        
        /// Post-quantum algorithm to use
        #[arg(short, long)]
        
        /// Security level
        #[arg(short, long)]
        
        /// Output file for public key
        #[arg(long)]
        
        /// Output file for secret key
        #[arg(long)]
        
        /// Enable performance caching
        #[arg(long)]
        
        /// Enable security logging
        #[arg(long)]
    
    /// Perform hybrid encapsulation
    Encaps {
        /// Public key file
        #[arg(short, long)]
        
        /// Output file for ciphertext
        #[arg(short, long)]
        
        /// Output file for shared secret
        #[arg(short, long)]
        
        /// Classical algorithm used
        #[arg(long)]
        
        /// Post-quantum algorithm used
        #[arg(long)]
        
        /// Security level used
        #[arg(long)]
    
    /// Perform hybrid decapsulation
    Decaps {
        /// Secret key file
        #[arg(short, long)]
        
        /// Ciphertext file
        #[arg(short, long)]
        
        /// Output file for shared secret
        #[arg(short, long)]
        
        /// Classical algorithm used
        #[arg(long)]
        
        /// Post-quantum algorithm used
        #[arg(long)]
        
        /// Security level used
        #[arg(long)]
    
    /// Run benchmarks
    Benchmark {
        /// Classical algorithm to benchmark
        #[arg(short, long)]
        
        /// Post-quantum algorithm to benchmark
        #[arg(short, long)]
        
        /// Security level to benchmark
        #[arg(short, long)]
        
        /// Number of iterations
        #[arg(short, long, default_value = "10")]
        
        /// Output results to JSON file
        #[arg(short, long)]
    
    /// Show compatibility matrix
    Compatibility {
        /// Show only excellent combinations
        #[arg(long)]
        
        /// Filter by security level
        #[arg(long)]
        
        /// Output format
        #[arg(long, default_value = "table")]
    
    /// Show migration strategy
    Migration {
        /// Show specific phase (0-4)
        #[arg(short, long)]
        
        /// Output format
        #[arg(long, default_value = "table")]
    
    /// Validate hybrid key pair
    Validate {
        /// Public key file
        #[arg(short, long)]
        
        /// Secret key file
        #[arg(short, long)]
        
        /// Classical algorithm used
        #[arg(long)]
        
        /// Post-quantum algorithm used
        #[arg(long)]
        
        /// Security level used
        #[arg(long)]
#[derive(ValueEnum, Clone, Debug)]
enum ClassicalAlgorithmCli {
impl From<ClassicalAlgorithmCli> for ClassicalAlgorithm {
    fn from(cli: ClassicalAlgorithmCli) -> Self {
        match cli {
        }
    }
#[derive(ValueEnum, Clone, Debug)]
enum AlgorithmTypeCli {
impl From<AlgorithmTypeCli> for AlgorithmType {
    fn from(cli: AlgorithmTypeCli) -> Self {
        match cli {
        }
    }
#[derive(ValueEnum, Clone, Debug)]
enum SecurityLevelCli {
impl From<SecurityLevelCli> for SecurityLevel {
    fn from(cli: SecurityLevelCli) -> Self {
        match cli {
        }
    }
#[derive(ValueEnum, Clone, Debug)]
enum OutputFormat {
#[derive(Serialize, Deserialize)]
struct BenchmarkResults {
fn main() -> crate::error::Result<()> {
    env_logger::init();
    
    let cli = Cli::parse();
    
    match cli.command {
        Commands::Keygen { 
            enable_caching, enable_logging 
        } => {
            cmd_keygen(classical, pqc, security_level, public_key_out, secret_key_out, enable_caching, enable_logging)?;
        Commands::Encaps { 
            classical, pqc, security_level 
        } => {
            cmd_encaps(public_key, ciphertext_out, shared_secret_out, classical, pqc, security_level)?;
        Commands::Decaps { 
            classical, pqc, security_level 
        } => {
            cmd_decaps(secret_key, ciphertext, shared_secret_out, classical, pqc, security_level)?;
        Commands::Benchmark { classical, pqc, security_level, iterations, output } => {
            cmd_benchmark(classical, pqc, security_level, iterations, output)?;
        Commands::Compatibility { excellent_only, security_level, format } => {
            cmd_compatibility(excellent_only, security_level, format)?;
        Commands::Migration { phase, format } => {
            cmd_migration(phase, format)?;
        Commands::Validate { public_key, secret_key, classical, pqc, security_level } => {
            cmd_validate(public_key, secret_key, classical, pqc, security_level)?;
    Ok(())
fn cmd_keygen(
) -> crate::error::Result<()> {
    println!("🔑 Generating hybrid key pair...");
    println!("   Classical: {:?}", classical);
    println!("   Post-Quantum: {:?}", pqc);
    println!("   Security Level: {:?}", security_level);
    
    let config = HybridConfig {
    
    let hybrid_kem = HybridKem::new_with_config(
    );
    
    let start = Instant::now();
    let key_pair = hybrid_kem.keygen()?;
    let duration = start.elapsed();
    
    println!("✅ Key generation completed in {:?}", duration);
    println!("   Classical public key size: {} bytes", key_pair.classical_public.len());
    println!("   Classical secret key size: {} bytes", key_pair.classical_secret.len());
    println!("   PQC public key size: {} bytes", key_pair.pqc_public.len());
    println!("   PQC secret key size: {} bytes", key_pair.pqc_secret.len());
    
    // Save keys if output paths provided
    if let Some(path) = public_key_out {
        let public_key_data = bincode::serialize(&(
            &key_pair.algorithm_info
        ))?;
        fs::write(&path, public_key_data)?;
        println!("   Public key saved to: {}", path.display());
    if let Some(path) = secret_key_out {
        let secret_key_data = bincode::serialize(&(
            &key_pair.algorithm_info
        ))?;
        fs::write(&path, secret_key_data)?;
        println!("   Secret key saved to: {}", path.display());
    Ok(())
fn cmd_encaps(
) -> crate::error::Result<()> {
    println!("🔒 Performing hybrid encapsulation...");
    
    // Load public key
    let public_key_data = fs::read(public_key)?;
    let (classical_public, pqc_public, algorithm_info): (Vec<u8>, Vec<u8>, HybridAlgorithmInfo) = 
        bincode::deserialize(&public_key_data)?;
    
    let key_pair = HybridKeyPair {
        classical_secret: vec![], // Not needed for encaps
        pqc_secret: vec![], // Not needed for encaps
    
    let hybrid_kem = HybridKem::new(
    );
    
    let start = Instant::now();
    let (ciphertext, shared_secret) = hybrid_kem.encaps(&key_pair)?;
    let duration = start.elapsed();
    
    println!("✅ Encapsulation completed in {:?}", duration);
    println!("   Ciphertext size: {} bytes", ciphertext.len());
    println!("   Shared secret size: {} bytes", shared_secret.len());
    
    // Save outputs
    fs::write(ciphertext_out, ciphertext)?;
    fs::write(shared_secret_out, shared_secret)?;
    
    Ok(())
fn cmd_decaps(
) -> crate::error::Result<()> {
    println!("🔓 Performing hybrid decapsulation...");
    
    // Load secret key and ciphertext
    let secret_key_data = fs::read(secret_key)?;
    let (classical_secret, pqc_secret, algorithm_info): (Vec<u8>, Vec<u8>, HybridAlgorithmInfo) = 
        bincode::deserialize(&secret_key_data)?;
    
    let ciphertext_data = fs::read(ciphertext)?;
    
    let key_pair = HybridKeyPair {
        classical_public: vec![], // Not needed for decaps
        pqc_public: vec![], // Not needed for decaps
    
    let hybrid_kem = HybridKem::new(
    );
    
    let start = Instant::now();
    let shared_secret = hybrid_kem.decaps(&key_pair, &ciphertext_data)?;
    let duration = start.elapsed();
    
    println!("✅ Decapsulation completed in {:?}", duration);
    println!("   Shared secret size: {} bytes", shared_secret.len());
    
    // Save output
    fs::write(shared_secret_out, shared_secret)?;
    
    Ok(())
fn cmd_benchmark(
) -> crate::error::Result<()> {
    println!("📊 Running hybrid cryptography benchmarks...");
    
    let classical_algorithms = classical.map(|c| vec![c]).unwrap_or_else(|| vec![
    ]);
    
    let pqc_algorithms = pqc.map(|p| vec![p]).unwrap_or_else(|| vec![
    ]);
    
    let security_levels = security_level.map(|s| vec![s]).unwrap_or_else(|| vec![
    ]);
    
    let mut all_results = Vec::new();
    
    for classical_alg in &classical_algorithms {
        for pqc_alg in &pqc_algorithms {
            for sec_level in &security_levels {
                println!("\nBenchmarking {:?} + {:?} at {:?}...", classical_alg, pqc_alg, sec_level);
                
                let result = benchmark_combination(
                )?;
                
                println!("  Key generation: {:.2}ms avg", result.keygen_avg_ms);
                println!("  Encapsulation:  {:.2}ms avg", result.encaps_avg_ms);
                println!("  Decapsulation:  {:.2}ms avg", result.decaps_avg_ms);
                println!("  Throughput:     {:.1} ops/sec", result.throughput_ops_per_sec);
                
                all_results.push(result);
            }
        }
    // Save results if output path provided
    if let Some(path) = output {
        let json_output = serde_json::to_string_pretty(&all_results)?;
        fs::write(path, json_output)?;
        println!("\n📁 Results saved to file");
    Ok(())
fn benchmark_combination(
) -> crate::error::Result<()> {
    let hybrid_kem = HybridKem::new(classical, pqc, security_level);
    
    let mut keygen_times = Vec::new();
    let mut encaps_times = Vec::new();
    let mut decaps_times = Vec::new();
    
    for _ in 0..iterations {
        // Benchmark key generation
        let start = Instant::now();
        let key_pair = hybrid_kem.keygen()?;
        keygen_times.push(start.elapsed().as_millis() as f64);
        
        // Benchmark encapsulation
        let start = Instant::now();
        let (ciphertext, shared_secret1) = hybrid_kem.encaps(&key_pair)?;
        encaps_times.push(start.elapsed().as_millis() as f64);
        
        // Benchmark decapsulation
        let start = Instant::now();
        let shared_secret2 = hybrid_kem.decaps(&key_pair, &ciphertext)?;
        decaps_times.push(start.elapsed().as_millis() as f64);
        
        // Verify correctness
        assert_eq!(shared_secret1, shared_secret2);
    let keygen_avg = keygen_times.iter().sum::<f64>() / iterations as f64;
    let keygen_min = keygen_times.iter().cloned().fold(f64::INFINITY, f64::min);
    let keygen_max = keygen_times.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    
    let encaps_avg = encaps_times.iter().sum::<f64>() / iterations as f64;
    let encaps_min = encaps_times.iter().cloned().fold(f64::INFINITY, f64::min);
    let encaps_max = encaps_times.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    
    let decaps_avg = decaps_times.iter().sum::<f64>() / iterations as f64;
    let decaps_min = decaps_times.iter().cloned().fold(f64::INFINITY, f64::min);
    let decaps_max = decaps_times.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    
    let total_avg = keygen_avg + encaps_avg + decaps_avg;
    let throughput = 1000.0 / total_avg; // ops per second
    
    Ok(BenchmarkResults {
    })
fn cmd_compatibility(
) -> crate::error::Result<()> {
    println!("🔍 Analyzing algorithm compatibility...");
    
    let matrix = HybridCompatibilityMatrix::new();
    
    let combinations = if excellent_only {
        matrix.get_excellent_combinations()
    } else if let Some(level) = security_level {
        matrix.get_recommended_for_security_level(level.into())
    } else {
        // Get all combinations with their ratings
        let mut all_combos = Vec::new();
        for classical in [
        ] {
            for pqc in [
            ] {
                let rating = matrix.get_rating(classical, pqc);
                if rating != CompatibilityRating::Incompatible {
                    all_combos.push((classical, pqc));
                }
            }
        }
        all_combos
    
    match format {
        OutputFormat::Table => {
            println!("\n📋 Algorithm Compatibility Matrix:");
            println!("{:-<60}", "");
            println!("{:<20} {:<15} {:<20}", "Classical", "Post-Quantum", "Rating");
            println!("{:-<60}", "");
            
            for (classical, pqc) in combinations {
                let rating = matrix.get_rating(classical, pqc);
                    format!("{:?}", rating)
                );
            }
            println!("{:-<60}", "");
        OutputFormat::Json => {
            let results: Vec<_> = combinations.iter().map(|(classical, pqc)| {
                serde_json::json!({
                    "rating": format!("{:?}", matrix.get_rating(*classical, *pqc))
                })
            }).collect();
            println!("{}", serde_json::to_string_pretty(&results)?);
        OutputFormat::Yaml => {
            let results: Vec<_> = combinations.iter().map(|(classical, pqc)| {
                    classical, pqc, matrix.get_rating(*classical, *pqc))
            }).collect();
            println!("{}", results.join("\n"));
    Ok(())
fn cmd_migration(phase: Option<usize>, format: OutputFormat) -> crate::error::Result<()> {
    println!("🚀 Post-Quantum Migration Strategy:");
    
    let strategy = HybridMigrationStrategy::standard();
    
    match format {
        OutputFormat::Table => {
            if let Some(phase_num) = phase {
                if let Some(phase_info) = strategy.phases.get(phase_num) {
                    println!("\nPhase {}: {}", phase_num, phase_info.name);
                    println!("  Classical weight: {:.1}%", phase_info.classical_weight * 100.0);
                    println!("  PQC weight: {:.1}%", phase_info.pqc_weight * 100.0);
                    println!("  Minimum security: {:?}", phase_info.minimum_security_level);
                    println!("  Recommended algorithms:");
                    for (classical, pqc) in &phase_info.recommended_algorithms {
                        println!("    {:?} + {:?}", classical, pqc);
                    }
                } else {
                    eprintln!("❌ Invalid phase number: {}", phase_num);
                }
            } else {
                println!("\n📊 All Migration Phases:");
                for (i, phase_info) in strategy.phases.iter().enumerate() {
                    println!("\nPhase {}: {}", i, phase_info.name);
                        phase_info.pqc_weight * 100.0);
                    println!("  Security: {:?}", phase_info.minimum_security_level);
                    println!("  Algorithms: {} combinations", phase_info.recommended_algorithms.len());
                }
            }
        OutputFormat::Json => {
            let json_strategy = serde_json::to_string_pretty(&strategy)?;
            println!("{}", json_strategy);
        OutputFormat::Yaml => {
            // Simple YAML output for phases
            for (i, phase_info) in strategy.phases.iter().enumerate() {
                println!("phase_{}:", i);
                println!("  name: \"{}\"", phase_info.name);
                println!("  classical_weight: {}", phase_info.classical_weight);
                println!("  pqc_weight: {}", phase_info.pqc_weight);
                println!("  security_level: {:?}", phase_info.minimum_security_level);
            }
    Ok(())
fn cmd_validate(
) -> crate::error::Result<()> {
    println!("🔍 Validating hybrid key pair...");
    
    // Load keys
    let public_key_data = fs::read(public_key)?;
    let secret_key_data = fs::read(secret_key)?;
    
    let (classical_public, pqc_public, pub_algorithm_info): (Vec<u8>, Vec<u8>, HybridAlgorithmInfo) = 
        bincode::deserialize(&public_key_data)?;
    let (classical_secret, pqc_secret, sec_algorithm_info): (Vec<u8>, Vec<u8>, HybridAlgorithmInfo) = 
        bincode::deserialize(&secret_key_data)?;
    
    // Verify algorithm info matches
    if pub_algorithm_info.classical != sec_algorithm_info.classical ||
       pub_algorithm_info.pqc != sec_algorithm_info.pqc ||
       pub_algorithm_info.security_level != sec_algorithm_info.security_level {
        return Err("Algorithm info mismatch between public and secret keys".into());
    let key_pair = HybridKeyPair {
    
    let hybrid_kem = HybridKem::new(
    );
    
    // Test the key pair by doing encaps/decaps
    println!("  Testing encapsulation/decapsulation...");
    let start = Instant::now();
    let (ciphertext, shared_secret1) = hybrid_kem.encaps(&key_pair)?;
    let shared_secret2 = hybrid_kem.decaps(&key_pair, &ciphertext)?;
    let duration = start.elapsed();
    
    if shared_secret1 == shared_secret2 {
        println!("✅ Key pair validation successful!");
        println!("   Test completed in: {:?}", duration);
        println!("   Shared secret size: {} bytes", shared_secret1.len());
        println!("   Ciphertext size: {} bytes", ciphertext.len());
    } else {
        return Err("Key pair validation failed: shared secrets don't match".into());
    Ok(())
}
