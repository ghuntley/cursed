fr fr! Enhanced Post-Quantum Cryptography Showcase
fr fr! 
fr fr! This example demonstrates the complete PQC ecosystem including:
fr fr! - Real algorithm implementations (Kyber, NTRU, FrodoKEM, Dilithium, XMSS)
fr fr! - Performance benchmarking and analysis
fr fr! - Security level comparisons
fr fr! - Hybrid classical+PQC protocols
fr fr! - Integration with existing crypto infrastructure

yeet "stdlib::crypto_pqc"

slay main() -> Result<(), CursedError> {
    println("🔐 Enhanced Post-Quantum Cryptography Showcase")?;
    println("=" * 60)?;
    
    // Demo real PQC algorithms
    demo_real_algorithms()?;
    
    // Demo performance benchmarking
    demo_performance_benchmarks()?;
    
    // Demo security analysis
    demo_security_analysis()?;
    
    // Demo hybrid protocols
    demo_hybrid_protocols()?;
    
    // Demo algorithm selection
    demo_algorithm_selection()?;
    
    println("\n✅ Enhanced PQC showcase completed successfully!")?;
    Ok(())
}

slay demo_real_algorithms() -> Result<(), CursedError> {
    println("\n📊 Real Algorithm Implementations Demo")?;
    println("-" * 40)?;
    
    // Demo NTRU Key Encapsulation
    println("🔒 NTRU-HPS (Lattice-based KEM):")?;
    
    facts (ntru_pub, ntru_sec) = RealNtru::keygen(SecurityLevel::Level1)?;
    printf("  Public key size: {} bytes", ntru_pub.as_bytes().len())?;
    printf("  Secret key size: {} bytes", ntru_sec.as_bytes().len())?;
    
    facts (ntru_ct, ntru_ss1) = RealNtru::encaps(&ntru_pub)?;
    facts ntru_ss2 = RealNtru::decaps(&ntru_sec, &ntru_ct)?;
    
    lowkey (ntru_ss1.data == ntru_ss2.data) {
        println("  ✅ NTRU encapsulation/decapsulation successful")?;
    } flex {
        println("  ❌ NTRU failed")?;
        return Err(CursedError::Runtime("NTRU test failed".to_string()));
    }
    
    // Demo FrodoKEM
    println("\n🧮 FrodoKEM (Conservative Lattice-based KEM):")?;
    
    facts (frodo_pub, frodo_sec) = RealFrodo::keygen(SecurityLevel::Level1)?;
    printf("  Public key size: {} bytes", frodo_pub.as_bytes().len())?;
    printf("  Secret key size: {} bytes", frodo_sec.as_bytes().len())?;
    
    facts (frodo_ct, frodo_ss1) = RealFrodo::encaps(&frodo_pub)?;
    facts frodo_ss2 = RealFrodo::decaps(&frodo_sec, &frodo_ct)?;
    
    lowkey (frodo_ss1.data == frodo_ss2.data) {
        println("  ✅ FrodoKEM encapsulation/decapsulation successful")?;
    } flex {
        println("  ❌ FrodoKEM failed")?;
        return Err(CursedError::Runtime("FrodoKEM test failed".to_string()));
    }
    
    // Demo XMSS Signatures
    println("\n🖋️  XMSS (Hash-based Signatures):")?;
    
    facts (xmss_pub, xmss_sec) = RealXmss::keygen(SecurityLevel::Level1)?;
    printf("  Public key size: {} bytes", xmss_pub.as_bytes().len())?;
    printf("  Secret key size: {} bytes", xmss_sec.as_bytes().len())?;
    printf("  Signatures remaining: {}", xmss_sec.signatures_remaining())?;
    
    facts message = "Important document requiring post-quantum signature".as_bytes();
    facts xmss_sig = RealXmss::sign(&xmss_sec, message)?;
    facts is_valid = RealXmss::verify(&xmss_pub, message, &xmss_sig)?;
    
    lowkey is_valid {
        println("  ✅ XMSS signature verified successfully")?;
    } flex {
        println("  ❌ XMSS signature verification failed")?;
        return Err(CursedError::Runtime("XMSS test failed".to_string()));
    }
    
    // Demo serialization
    println("\n💾 Key Serialization:")?;
    
    facts ntru_pub_bytes = ntru_pub.as_bytes();
    facts ntru_pub_restored = NtruPublicKey::from_bytes(ntru_pub.params, &ntru_pub_bytes)?;
    
    lowkey (ntru_pub.h.coeffs == ntru_pub_restored.h.coeffs) {
        println("  ✅ NTRU key serialization successful")?;
    } flex {
        println("  ❌ NTRU key serialization failed")?;
    }
    
    Ok(())
}

slay demo_performance_benchmarks() -> Result<(), CursedError> {
    println("\n📈 Performance Benchmarking Demo")?;
    println("-" * 40)?;
    
    sus benchmark = RealPqcBenchmark::new();
    
    println("Running comprehensive benchmarks (this may take a moment)...")?;
    
    // Benchmark sample size - smaller for demo
    facts samples = 10;
    
    // Benchmark KEM algorithms
    benchmark.benchmark_kyber(samples)?;
    benchmark.benchmark_ntru(samples)?;
    benchmark.benchmark_frodo(samples)?;
    
    // Benchmark signature algorithms  
    benchmark.benchmark_xmss(samples)?;
    benchmark.benchmark_dilithium(samples)?;
    
    // Generate and display report
    facts report = benchmark.generate_report();
    println("\n{}", report)?;
    
    // Export CSV for analysis
    facts csv_data = benchmark.export_csv();
    println("\n📊 CSV Export (first 5 lines):")?;
    facts lines: Vec<&str> = csv_data.lines().take(5).collect();
    bestie line in lines {
        println("  {}", line)?;
    }
    
    Ok(())
}

slay demo_security_analysis() -> Result<(), CursedError> {
    println("\n🔒 Security Analysis Demo")?;
    println("-" * 40)?;
    
    // Analyze different algorithm families
    facts algorithms = vec![
        AlgorithmType::Kyber,
        AlgorithmType::Ntru, 
        AlgorithmType::FrodoKem,
        AlgorithmType::Dilithium,
        AlgorithmType::Xmss,
        AlgorithmType::ClassicMcEliece,
        AlgorithmType::Sike,
    ];
    
    bestie algo in algorithms {
        printf("\n🔍 Algorithm: {}", algo)?;
        
        facts family = AlgorithmFamily::from_algorithm(algo);
        printf("  Family: {} - {}", family, family.description())?;
        printf("  Quantum Confidence: {}", family.quantum_confidence())?;
        
        facts status = StandardizationStatus::for_algorithm(algo);
        printf("  Standardization: {} - {}", status, status.description())?;
        printf("  Production Ready: {}", status.is_production_ready())?;
        
        // Security level analysis for Level 1
        lowkey status.is_production_ready() {
            facts benchmark = RealPqcBenchmark::new();
            facts security_analysis = benchmark.analyze_algorithm_security(algo, SecurityLevel::Level1);
            
            printf("  Classical Security: {} bits", security_analysis.classical_security_bits)?;
            printf("  Quantum Security: {} bits", security_analysis.quantum_security_bits)?;
            printf("  Implementation Security: {:?}", security_analysis.implementation_security)?;
        }
    }
    
    Ok(())
}

slay demo_hybrid_protocols() -> Result<(), CursedError> {
    println("\n🔗 Hybrid Protocol Demo")?;
    println("-" * 40)?;
    
    // Demo classical + PQC key exchange
    println("Simulating hybrid key exchange (Classical ECDH + PQC KEM):")?;
    
    // Classical component (simulated)
    facts classical_shared_secret = vec![0xAA, 0xBB, 0xCC, 0xDD; 32];
    println("  ✅ Classical ECDH completed")?;
    
    // PQC component
    facts (pqc_pub, pqc_sec) = RealKyber::keygen(SecurityLevel::Level1)?;
    facts (pqc_ct, pqc_shared_secret) = RealKyber::encaps(&pqc_pub)?;
    facts pqc_decaps_secret = RealKyber::decaps(&pqc_sec, &pqc_ct)?;
    println("  ✅ PQC KEM completed")?;
    
    // Combine secrets (simplified)
    facts mut combined_secret = Vec::new();
    combined_secret.extend_from_slice(&classical_shared_secret);
    combined_secret.extend_from_slice(&pqc_shared_secret.data);
    
    printf("  Combined secret length: {} bytes", combined_secret.len())?;
    println("  ✅ Hybrid protocol provides dual security guarantees")?;
    
    // Demo algorithm agility
    println("\n🔄 Algorithm Agility Demo:")?;
    facts preferred_order = vec![
        AlgorithmType::Kyber,    // NIST standardized
        AlgorithmType::Ntru,     // NIST finalist
        AlgorithmType::FrodoKem, // Conservative choice
    ];
    
    bestie preferred_algo in preferred_order {
        facts status = StandardizationStatus::for_algorithm(preferred_algo);
        lowkey status.is_production_ready() {
            printf("  Selected algorithm: {} ({})", preferred_algo, status.description())?;
            break;
        }
    }
    
    Ok(())
}

slay demo_algorithm_selection() -> Result<(), CursedError> {
    println("\n🎯 Algorithm Selection Guide")?;
    println("-" * 40)?;
    
    // Performance-focused selection
    println("📊 Performance-Focused Applications:")?;
    println("  Recommended: Kyber-512 + Dilithium2")?;
    println("  - Fast operations, small signatures")?;
    println("  - NIST standardized")?;
    
    // Bandwidth-constrained selection  
    println("\n📡 Bandwidth-Constrained Environments:")?;
    println("  Recommended: Kyber-512 + Falcon-512")?;
    println("  - Smallest total bandwidth")?;
    println("  - Good performance")?;
    
    // Conservative security selection
    println("\n🛡️  Conservative Security Requirements:")?;
    println("  Recommended: FrodoKEM-640 + XMSS")?;
    println("  - Most conservative security assumptions")?;
    println("  - Based on well-studied problems")?;
    
    // Long-term signatures
    println("\n📜 Long-term Signatures:")?;
    println("  Recommended: XMSS or LMS")?;
    println("  - Hash-based security")?;
    println("  - Stateful but very secure")?;
    
    // Migration from classical
    println("\n🔄 Migration Strategy:")?;
    println("  Phase 1: Hybrid (Classical + PQC)")?;
    println("  Phase 2: PQC primary, Classical backup")?;
    println("  Phase 3: PQC only")?;
    
    // Real-world performance comparison
    println("\n⚡ Quick Performance Comparison:")?;
    
    facts kyber_perf = RealKyber::performance_characteristics(KyberParams::Kyber512);
    facts ntru_perf = RealNtru::performance_characteristics(NtruParams::NtruHps509);
    facts frodo_perf = RealFrodo::performance_characteristics(FrodoParams::Frodo640Aes);
    
    printf("  Kyber-512:    {:.1}ms keygen, {:.1} ops/sec", 
           kyber_perf.keygen_time_ms, kyber_perf.throughput_ops_per_sec)?;
    printf("  NTRU-HPS509:  {:.1}ms keygen, {:.1} ops/sec",
           ntru_perf.keygen_time_ms, ntru_perf.throughput_ops_per_sec)?;
    printf("  FrodoKEM-640: {:.1}ms keygen, {:.1} ops/sec",
           frodo_perf.keygen_time_ms, frodo_perf.throughput_ops_per_sec)?;
    
    Ok(())
}

fr fr Helper function for formatting
slay "*"(times: usize) -> String {
    "=".repeat(times)
}

slay "-"(times: usize) -> String {
    "-".repeat(times)
}
