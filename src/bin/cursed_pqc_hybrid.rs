use crate::error::Error;
//! CURSED Post-Quantum Cryptography Hybrid Tool
//! 
//! A command-line tool for managing hybrid cryptographic operations
//! that combine classical and post-quantum algorithms.

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
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate hybrid key pairs
    Keygen {
        /// Classical algorithm to use
        #[arg(short, long)]
        classical: ClassicalAlgorithmCli,
        
        /// Post-quantum algorithm to use
        #[arg(short, long)]
        pqc: AlgorithmTypeCli,
        
        /// Security level
        #[arg(short, long)]
        security_level: SecurityLevelCli,
        
        /// Output file for public key
        #[arg(long)]
        public_key_out: Option<PathBuf>,
        
        /// Output file for secret key
        #[arg(long)]
        secret_key_out: Option<PathBuf>,
        
        /// Enable performance caching
        #[arg(long)]
        enable_caching: bool,
        
        /// Enable security logging
        #[arg(long)]
        enable_logging: bool,
    },
    
    /// Perform hybrid encapsulation
    Encaps {
        /// Public key file
        #[arg(short, long)]
        public_key: PathBuf,
        
        /// Output file for ciphertext
        #[arg(short, long)]
        ciphertext_out: PathBuf,
        
        /// Output file for shared secret
        #[arg(short, long)]
        shared_secret_out: PathBuf,
        
        /// Classical algorithm used
        #[arg(long)]
        classical: ClassicalAlgorithmCli,
        
        /// Post-quantum algorithm used
        #[arg(long)]
        pqc: AlgorithmTypeCli,
        
        /// Security level used
        #[arg(long)]
        security_level: SecurityLevelCli,
    },
    
    /// Perform hybrid decapsulation
    Decaps {
        /// Secret key file
        #[arg(short, long)]
        secret_key: PathBuf,
        
        /// Ciphertext file
        #[arg(short, long)]
        ciphertext: PathBuf,
        
        /// Output file for shared secret
        #[arg(short, long)]
        shared_secret_out: PathBuf,
        
        /// Classical algorithm used
        #[arg(long)]
        classical: ClassicalAlgorithmCli,
        
        /// Post-quantum algorithm used
        #[arg(long)]
        pqc: AlgorithmTypeCli,
        
        /// Security level used
        #[arg(long)]
        security_level: SecurityLevelCli,
    },
    
    /// Run benchmarks
    Benchmark {
        /// Classical algorithm to benchmark
        #[arg(short, long)]
        classical: Option<ClassicalAlgorithmCli>,
        
        /// Post-quantum algorithm to benchmark
        #[arg(short, long)]
        pqc: Option<AlgorithmTypeCli>,
        
        /// Security level to benchmark
        #[arg(short, long)]
        security_level: Option<SecurityLevelCli>,
        
        /// Number of iterations
        #[arg(short, long, default_value = "10")]
        iterations: usize,
        
        /// Output results to JSON file
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
    
    /// Show compatibility matrix
    Compatibility {
        /// Show only excellent combinations
        #[arg(long)]
        excellent_only: bool,
        
        /// Filter by security level
        #[arg(long)]
        security_level: Option<SecurityLevelCli>,
        
        /// Output format
        #[arg(long, default_value = "table")]
        format: OutputFormat,
    },
    
    /// Show migration strategy
    Migration {
        /// Show specific phase (0-4)
        #[arg(short, long)]
        phase: Option<usize>,
        
        /// Output format
        #[arg(long, default_value = "table")]
        format: OutputFormat,
    },
    
    /// Validate hybrid key pair
    Validate {
        /// Public key file
        #[arg(short, long)]
        public_key: PathBuf,
        
        /// Secret key file
        #[arg(short, long)]
        secret_key: PathBuf,
        
        /// Classical algorithm used
        #[arg(long)]
        classical: ClassicalAlgorithmCli,
        
        /// Post-quantum algorithm used
        #[arg(long)]
        pqc: AlgorithmTypeCli,
        
        /// Security level used
        #[arg(long)]
        security_level: SecurityLevelCli,
    },
}

#[derive(ValueEnum, Clone, Debug)]
enum ClassicalAlgorithmCli {
    EcdhP256,
    EcdhP384,
    EcdhP521,
    X25519,
    Rsa2048,
    Rsa3072,
    Rsa4096,
}

impl From<ClassicalAlgorithmCli> for ClassicalAlgorithm {
    fn from(cli: ClassicalAlgorithmCli) -> Self {
        match cli {
            ClassicalAlgorithmCli::EcdhP256 => ClassicalAlgorithm::EcdhP256,
            ClassicalAlgorithmCli::EcdhP384 => ClassicalAlgorithm::EcdhP384,
            ClassicalAlgorithmCli::EcdhP521 => ClassicalAlgorithm::EcdhP521,
            ClassicalAlgorithmCli::X25519 => ClassicalAlgorithm::X25519,
            ClassicalAlgorithmCli::Rsa2048 => ClassicalAlgorithm::Rsa2048,
            ClassicalAlgorithmCli::Rsa3072 => ClassicalAlgorithm::Rsa3072,
            ClassicalAlgorithmCli::Rsa4096 => ClassicalAlgorithm::Rsa4096,
        }
    }
}

#[derive(ValueEnum, Clone, Debug)]
enum AlgorithmTypeCli {
    Kyber,
    Dilithium,
    Ntru,
    FrodoKem,
    Sphincs,
    Lms,
    Xmss,
    Rainbow,
    GeMSS,
    ClassicMcEliece,
    Bike,
    Hqc,
    Sike,
}

impl From<AlgorithmTypeCli> for AlgorithmType {
    fn from(cli: AlgorithmTypeCli) -> Self {
        match cli {
            AlgorithmTypeCli::Kyber => AlgorithmType::Kyber,
            AlgorithmTypeCli::Dilithium => AlgorithmType::Dilithium,
            AlgorithmTypeCli::Ntru => AlgorithmType::Ntru,
            AlgorithmTypeCli::FrodoKem => AlgorithmType::FrodoKem,
            AlgorithmTypeCli::Sphincs => AlgorithmType::Sphincs,
            AlgorithmTypeCli::Lms => AlgorithmType::Lms,
            AlgorithmTypeCli::Xmss => AlgorithmType::Xmss,
            AlgorithmTypeCli::Rainbow => AlgorithmType::Rainbow,
            AlgorithmTypeCli::GeMSS => AlgorithmType::GeMSS,
            AlgorithmTypeCli::ClassicMcEliece => AlgorithmType::ClassicMcEliece,
            AlgorithmTypeCli::Bike => AlgorithmType::Bike,
            AlgorithmTypeCli::Hqc => AlgorithmType::Hqc,
            AlgorithmTypeCli::Sike => AlgorithmType::Sike,
        }
    }
}

#[derive(ValueEnum, Clone, Debug)]
enum SecurityLevelCli {
    Level1,
    Level3,
    Level5,
}

impl From<SecurityLevelCli> for SecurityLevel {
    fn from(cli: SecurityLevelCli) -> Self {
        match cli {
            SecurityLevelCli::Level1 => SecurityLevel::Level1,
            SecurityLevelCli::Level3 => SecurityLevel::Level3,
            SecurityLevelCli::Level5 => SecurityLevel::Level5,
        }
    }
}

#[derive(ValueEnum, Clone, Debug)]
enum OutputFormat {
    Table,
    Json,
    Yaml,
}

#[derive(Serialize, Deserialize)]
struct BenchmarkResults {
    algorithm_combination: String,
    security_level: String,
    iterations: usize,
    keygen_avg_ms: f64,
    keygen_min_ms: f64,
    keygen_max_ms: f64,
    encaps_avg_ms: f64,
    encaps_min_ms: f64,
    encaps_max_ms: f64,
    decaps_avg_ms: f64,
    decaps_min_ms: f64,
    decaps_max_ms: f64,
    total_avg_ms: f64,
    throughput_ops_per_sec: f64,
}

fn main() -> Result<(), Error> {
    env_logger::init();
    
    let cli = Cli::parse();
    
    match cli.command {
        Commands::Keygen { 
            classical, pqc, security_level, 
            public_key_out, secret_key_out,
            enable_caching, enable_logging 
        } => {
            cmd_keygen(classical, pqc, security_level, public_key_out, secret_key_out, enable_caching, enable_logging)?;
        },
        Commands::Encaps { 
            public_key, ciphertext_out, shared_secret_out,
            classical, pqc, security_level 
        } => {
            cmd_encaps(public_key, ciphertext_out, shared_secret_out, classical, pqc, security_level)?;
        },
        Commands::Decaps { 
            secret_key, ciphertext, shared_secret_out,
            classical, pqc, security_level 
        } => {
            cmd_decaps(secret_key, ciphertext, shared_secret_out, classical, pqc, security_level)?;
        },
        Commands::Benchmark { classical, pqc, security_level, iterations, output } => {
            cmd_benchmark(classical, pqc, security_level, iterations, output)?;
        },
        Commands::Compatibility { excellent_only, security_level, format } => {
            cmd_compatibility(excellent_only, security_level, format)?;
        },
        Commands::Migration { phase, format } => {
            cmd_migration(phase, format)?;
        },
        Commands::Validate { public_key, secret_key, classical, pqc, security_level } => {
            cmd_validate(public_key, secret_key, classical, pqc, security_level)?;
        },
    }
    
    Ok(())
}

fn cmd_keygen(
    classical: ClassicalAlgorithmCli,
    pqc: AlgorithmTypeCli,
    security_level: SecurityLevelCli,
    public_key_out: Option<PathBuf>,
    secret_key_out: Option<PathBuf>,
    enable_caching: bool,
    enable_logging: bool,
) -> Result<(), Error> {
    println!("🔑 Generating hybrid key pair...");
    println!("   Classical: {:?}", classical);
    println!("   Post-Quantum: {:?}", pqc);
    println!("   Security Level: {:?}", security_level);
    
    let config = HybridConfig {
        enable_performance_caching: enable_caching,
        enable_security_logging: enable_logging,
        max_cached_operations: 1000,
        key_derivation_iterations: 100_000,
        secure_memory_zeroing: true,
        timing_attack_resistance: true,
    };
    
    let hybrid_kem = HybridKem::new_with_config(
        classical.into(),
        pqc.into(),
        security_level.into(),
        config,
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
            &key_pair.classical_public,
            &key_pair.pqc_public,
            &key_pair.algorithm_info
        ))?;
        fs::write(&path, public_key_data)?;
        println!("   Public key saved to: {}", path.display());
    }
    
    if let Some(path) = secret_key_out {
        let secret_key_data = bincode::serialize(&(
            &key_pair.classical_secret,
            &key_pair.pqc_secret,
            &key_pair.algorithm_info
        ))?;
        fs::write(&path, secret_key_data)?;
        println!("   Secret key saved to: {}", path.display());
    }
    
    Ok(())
}

fn cmd_encaps(
    public_key: PathBuf,
    ciphertext_out: PathBuf,
    shared_secret_out: PathBuf,
    classical: ClassicalAlgorithmCli,
    pqc: AlgorithmTypeCli,
    security_level: SecurityLevelCli,
) -> Result<(), Error> {
    println!("🔒 Performing hybrid encapsulation...");
    
    // Load public key
    let public_key_data = fs::read(public_key)?;
    let (classical_public, pqc_public, algorithm_info): (Vec<u8>, Vec<u8>, HybridAlgorithmInfo) = 
        bincode::deserialize(&public_key_data)?;
    
    let key_pair = HybridKeyPair {
        classical_public,
        classical_secret: vec![], // Not needed for encaps
        pqc_public,
        pqc_secret: vec![], // Not needed for encaps
        algorithm_info,
    };
    
    let hybrid_kem = HybridKem::new(
        classical.into(),
        pqc.into(),
        security_level.into(),
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
}

fn cmd_decaps(
    secret_key: PathBuf,
    ciphertext: PathBuf,
    shared_secret_out: PathBuf,
    classical: ClassicalAlgorithmCli,
    pqc: AlgorithmTypeCli,
    security_level: SecurityLevelCli,
) -> Result<(), Error> {
    println!("🔓 Performing hybrid decapsulation...");
    
    // Load secret key and ciphertext
    let secret_key_data = fs::read(secret_key)?;
    let (classical_secret, pqc_secret, algorithm_info): (Vec<u8>, Vec<u8>, HybridAlgorithmInfo) = 
        bincode::deserialize(&secret_key_data)?;
    
    let ciphertext_data = fs::read(ciphertext)?;
    
    let key_pair = HybridKeyPair {
        classical_public: vec![], // Not needed for decaps
        classical_secret,
        pqc_public: vec![], // Not needed for decaps
        pqc_secret,
        algorithm_info,
    };
    
    let hybrid_kem = HybridKem::new(
        classical.into(),
        pqc.into(),
        security_level.into(),
    );
    
    let start = Instant::now();
    let shared_secret = hybrid_kem.decaps(&key_pair, &ciphertext_data)?;
    let duration = start.elapsed();
    
    println!("✅ Decapsulation completed in {:?}", duration);
    println!("   Shared secret size: {} bytes", shared_secret.len());
    
    // Save output
    fs::write(shared_secret_out, shared_secret)?;
    
    Ok(())
}

fn cmd_benchmark(
    classical: Option<ClassicalAlgorithmCli>,
    pqc: Option<AlgorithmTypeCli>,
    security_level: Option<SecurityLevelCli>,
    iterations: usize,
    output: Option<PathBuf>,
) -> Result<(), Error> {
    println!("📊 Running hybrid cryptography benchmarks...");
    
    let classical_algorithms = classical.map(|c| vec![c]).unwrap_or_else(|| vec![
        ClassicalAlgorithmCli::X25519,
        ClassicalAlgorithmCli::EcdhP256,
        ClassicalAlgorithmCli::EcdhP384,
    ]);
    
    let pqc_algorithms = pqc.map(|p| vec![p]).unwrap_or_else(|| vec![
        AlgorithmTypeCli::Kyber,
    ]);
    
    let security_levels = security_level.map(|s| vec![s]).unwrap_or_else(|| vec![
        SecurityLevelCli::Level1,
        SecurityLevelCli::Level3,
        SecurityLevelCli::Level5,
    ]);
    
    let mut all_results = Vec::new();
    
    for classical_alg in &classical_algorithms {
        for pqc_alg in &pqc_algorithms {
            for sec_level in &security_levels {
                println!("\nBenchmarking {:?} + {:?} at {:?}...", classical_alg, pqc_alg, sec_level);
                
                let result = benchmark_combination(
                    classical_alg.clone().into(),
                    pqc_alg.clone().into(),
                    sec_level.clone().into(),
                    iterations,
                )?;
                
                println!("  Key generation: {:.2}ms avg", result.keygen_avg_ms);
                println!("  Encapsulation:  {:.2}ms avg", result.encaps_avg_ms);
                println!("  Decapsulation:  {:.2}ms avg", result.decaps_avg_ms);
                println!("  Throughput:     {:.1} ops/sec", result.throughput_ops_per_sec);
                
                all_results.push(result);
            }
        }
    }
    
    // Save results if output path provided
    if let Some(path) = output {
        let json_output = serde_json::to_string_pretty(&all_results)?;
        fs::write(path, json_output)?;
        println!("\n📁 Results saved to file");
    }
    
    Ok(())
}

fn benchmark_combination(
    classical: ClassicalAlgorithm,
    pqc: AlgorithmType,
    security_level: SecurityLevel,
    iterations: usize,
) -> Result<(), Error> {
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
    }
    
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
        algorithm_combination: format!("{:?}+{:?}", classical, pqc),
        security_level: format!("{:?}", security_level),
        iterations,
        keygen_avg_ms: keygen_avg,
        keygen_min_ms: keygen_min,
        keygen_max_ms: keygen_max,
        encaps_avg_ms: encaps_avg,
        encaps_min_ms: encaps_min,
        encaps_max_ms: encaps_max,
        decaps_avg_ms: decaps_avg,
        decaps_min_ms: decaps_min,
        decaps_max_ms: decaps_max,
        total_avg_ms: total_avg,
        throughput_ops_per_sec: throughput,
    })
}

fn cmd_compatibility(
    excellent_only: bool,
    security_level: Option<SecurityLevelCli>,
    format: OutputFormat,
) -> Result<(), Error> {
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
            ClassicalAlgorithm::X25519,
            ClassicalAlgorithm::EcdhP256,
            ClassicalAlgorithm::EcdhP384,
            ClassicalAlgorithm::EcdhP521,
            ClassicalAlgorithm::Rsa2048,
            ClassicalAlgorithm::Rsa3072,
            ClassicalAlgorithm::Rsa4096,
        ] {
            for pqc in [
                AlgorithmType::Kyber,
                AlgorithmType::Dilithium,
                AlgorithmType::Ntru,
                AlgorithmType::FrodoKem,
                AlgorithmType::Sphincs,
            ] {
                let rating = matrix.get_rating(classical, pqc);
                if rating != CompatibilityRating::Incompatible {
                    all_combos.push((classical, pqc));
                }
            }
        }
        all_combos
    };
    
    match format {
        OutputFormat::Table => {
            println!("\n📋 Algorithm Compatibility Matrix:");
            println!("{:-<60}", "");
            println!("{:<20} {:<15} {:<20}", "Classical", "Post-Quantum", "Rating");
            println!("{:-<60}", "");
            
            for (classical, pqc) in combinations {
                let rating = matrix.get_rating(classical, pqc);
                println!("{:<20} {:<15} {:<20}", 
                    format!("{:?}", classical),
                    format!("{:?}", pqc),
                    format!("{:?}", rating)
                );
            }
            println!("{:-<60}", "");
        },
        OutputFormat::Json => {
            let results: Vec<_> = combinations.iter().map(|(classical, pqc)| {
                serde_json::json!({
                    "classical": format!("{:?}", classical),
                    "post_quantum": format!("{:?}", pqc),
                    "rating": format!("{:?}", matrix.get_rating(*classical, *pqc))
                })
            }).collect();
            println!("{}", serde_json::to_string_pretty(&results)?);
        },
        OutputFormat::Yaml => {
            let results: Vec<_> = combinations.iter().map(|(classical, pqc)| {
                format!("- classical: {:?}\n  post_quantum: {:?}\n  rating: {:?}",
                    classical, pqc, matrix.get_rating(*classical, *pqc))
            }).collect();
            println!("{}", results.join("\n"));
        },
    }
    
    Ok(())
}

fn cmd_migration(phase: Option<usize>, format: OutputFormat) -> Result<(), Error> {
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
                    println!("  Classical: {:.0}% | PQC: {:.0}%", 
                        phase_info.classical_weight * 100.0,
                        phase_info.pqc_weight * 100.0);
                    println!("  Security: {:?}", phase_info.minimum_security_level);
                    println!("  Algorithms: {} combinations", phase_info.recommended_algorithms.len());
                }
            }
        },
        OutputFormat::Json => {
            let json_strategy = serde_json::to_string_pretty(&strategy)?;
            println!("{}", json_strategy);
        },
        OutputFormat::Yaml => {
            // Simple YAML output for phases
            for (i, phase_info) in strategy.phases.iter().enumerate() {
                println!("phase_{}:", i);
                println!("  name: \"{}\"", phase_info.name);
                println!("  classical_weight: {}", phase_info.classical_weight);
                println!("  pqc_weight: {}", phase_info.pqc_weight);
                println!("  security_level: {:?}", phase_info.minimum_security_level);
            }
        },
    }
    
    Ok(())
}

fn cmd_validate(
    public_key: PathBuf,
    secret_key: PathBuf,
    classical: ClassicalAlgorithmCli,
    pqc: AlgorithmTypeCli,
    security_level: SecurityLevelCli,
) -> Result<(), Error> {
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
    }
    
    let key_pair = HybridKeyPair {
        classical_public,
        classical_secret,
        pqc_public,
        pqc_secret,
        algorithm_info: pub_algorithm_info,
    };
    
    let hybrid_kem = HybridKem::new(
        classical.into(),
        pqc.into(),
        security_level.into(),
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
    }
    
    Ok(())
}
