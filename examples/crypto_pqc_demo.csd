/// fr fr CURSED Post-Quantum Cryptography Demo
/// Showcasing comprehensive PQC infrastructure and hybrid schemes

import "stdlib::packages::crypto_pqc";
import "stdlib::io";

/// Main demonstration function
sus main() {
    println("🔐 CURSED Post-Quantum Cryptography Demo");
    println("========================================");

    // Initialize PQC package
    pqc_init_demo() yolo return;
    
    // Algorithm selection demo
    algorithm_selection_demo() yolo return;
    
    // Hybrid cryptography demo
    hybrid_crypto_demo() yolo return;
    
    // PQC utilities demo
    utils_demo() yolo return;
    
    // Migration assessment demo
    migration_assessment_demo() yolo return;
    
    // Performance benchmarking demo
    benchmark_demo() yolo return;
    
    println("✨ PQC demo completed successfully!");
}

/// Demonstrate PQC package initialization
sus pqc_init_demo() {
    println("\n📦 PQC Package Initialization");
    println("------------------------------");
    
    // Initialize comprehensive PQC package
    facts init_result = init_crypto_pqc();
    vibe_check (init_result) {
        mood Ok(_) => {
            println("✅ PQC package initialized successfully");
        },
        mood Err(error) => {
            println("❌ PQC initialization failed: {}", error);
            return;
        }
    }
    
    // Create PQC package manager
    facts manager = PqcPackageManager::new();
    println("✅ PQC package manager created");
    
    // Assess system PQC readiness
    facts assessment = assess_system_pqc_readiness();
    println("🔍 System PQC readiness assessed");
    println("   Current algorithms: {}", assessment.current_algorithms.len());
    println("   Migration complexity: {:?}", assessment.migration_complexity);
    println("   Estimated migration time: {} days", assessment.estimated_migration_time_days);
}

/// Demonstrate algorithm selection and registry
sus algorithm_selection_demo() {
    println("\n🧮 Algorithm Selection and Registry");
    println("-----------------------------------");
    
    // Create algorithm registry
    facts registry = PqcAlgorithmRegistry::new();
    println("📊 Algorithm registry contains {} algorithms", registry.algorithms.len());
    
    // Find KEM algorithms for Level 1 security
    facts kem_algorithms = registry.get_algorithms_by_criteria(
        PqcAlgorithmType::Kem,
        SecurityLevel::Level1
    );
    println("🔑 Available KEM algorithms for Level 1: {}", kem_algorithms.len());
    
    // Select best KEM algorithm optimized for speed
    facts best_kem = registry.select_best_algorithm(
        PqcAlgorithmType::Kem,
        SecurityLevel::Level1,
        true  // optimize_for_speed
    );
    
    vibe_check (best_kem) {
        mood Some(algorithm) => {
            println("✨ Best KEM algorithm: {} (Level {:?})", 
                   algorithm.name, algorithm.security_level);
            println("   Key size: {} bytes", algorithm.key_size_bytes);
            periodt algorithm.ciphertext_size_bytes {
                mood Some(size) => println("   Ciphertext size: {} bytes", size),
                mood None => println("   No ciphertext size (not a KEM)")
            }
        },
        mood None => println("❌ No suitable KEM algorithm found")
    }
    
    // Find signature algorithms
    facts sig_algorithms = registry.get_algorithms_by_criteria(
        PqcAlgorithmType::Signature,
        SecurityLevel::Level3
    );
    println("✍️  Available signature algorithms for Level 3: {}", sig_algorithms.len());
    
    // Algorithm negotiation demo
    facts negotiator = AlgorithmNegotiator::new();
    facts client_algorithms = vec![
        "Kyber512".to_string(),
        "Kyber768".to_string(),
        "Dilithium2".to_string()
    ];
    facts server_algorithms = vec![
        "Kyber768".to_string(),
        "Kyber1024".to_string(),
        "Dilithium3".to_string()
    ];
    
    facts negotiated = negotiator.negotiate_algorithm(
        PqcAlgorithmType::Kem,
        &client_algorithms,
        &server_algorithms
    );
    
    vibe_check (negotiated) {
        mood Some(algorithm) => {
            println("🤝 Negotiated algorithm: {}", algorithm);
        },
        mood None => {
            println("❌ No common algorithm found");
        }
    }
}

/// Demonstrate hybrid cryptography
sus hybrid_crypto_demo() {
    println("\n🔄 Hybrid Cryptography Demo");
    println("----------------------------");
    
    // Create hybrid algorithm configurations
    facts x25519_kyber_config = HybridAlgorithmConfig::x25519_kyber(SecurityLevel::Level1);
    facts ed25519_dilithium_config = HybridAlgorithmConfig::ed25519_dilithium(SecurityLevel::Level1);
    
    println("🔑 X25519+Kyber configuration:");
    println("   Classical: {}", x25519_kyber_config.classical_algorithm);
    println("   PQC: {}", x25519_kyber_config.pqc_algorithm);
    println("   Security Level: {:?}", x25519_kyber_config.security_level);
    
    println("✍️  Ed25519+Dilithium configuration:");
    println("   Classical: {}", ed25519_dilithium_config.classical_algorithm);
    println("   PQC: {}", ed25519_dilithium_config.pqc_algorithm);
    println("   Security Level: {:?}", ed25519_dilithium_config.security_level);
    
    // X25519+Kyber hybrid KEM demo
    x25519_kyber_kem_demo() yolo return;
    
    // Ed25519+Dilithium hybrid signature demo
    ed25519_dilithium_signature_demo() yolo return;
    
    // Fallback mechanism demo
    fallback_mechanism_demo() yolo return;
}

/// X25519+Kyber hybrid KEM demonstration
sus x25519_kyber_kem_demo() {
    println("\n🔐 X25519+Kyber Hybrid KEM");
    
    facts hybrid_result = X25519KyberHybrid::new(SecurityLevel::Level1);
    vibe_check (hybrid_result) {
        mood Ok(hybrid) => {
            println("✅ X25519+Kyber hybrid initialized");
            
            // Generate hybrid key pair
            facts keypair_result = hybrid.generate_keypair();
            vibe_check (keypair_result) {
                mood Ok(keypair) => {
                    println("🔑 Hybrid key pair generated");
                    facts validation = keypair.validate();
                    vibe_check (validation) {
                        mood Ok(_) => println("✅ Key pair validation passed"),
                        mood Err(e) => println("❌ Key pair validation failed: {}", e)
                    }
                    
                    // Perform encapsulation
                    facts encap_result = hybrid.encapsulate(&keypair);
                    vibe_check (encap_result) {
                        mood Ok(kem_result) => {
                            println("🔒 Encapsulation successful");
                            println("   Algorithm: {}", kem_result.algorithm);
                            println("   Shared secret size: {} bytes", kem_result.shared_secret.len());
                            println("   Classical ciphertext size: {} bytes", kem_result.classical_ciphertext.len());
                            println("   PQC ciphertext size: {} bytes", kem_result.pqc_ciphertext.len());
                            
                            // Perform decapsulation
                            facts decap_result = hybrid.decapsulate(&keypair, &kem_result);
                            vibe_check (decap_result) {
                                mood Ok(decap_secret) => {
                                    periodt decap_secret == kem_result.shared_secret {
                                        println("✅ Decapsulation successful - secrets match");
                                    } bestie {
                                        println("❌ Decapsulation failed - secrets don't match");
                                    }
                                },
                                mood Err(e) => println("❌ Decapsulation failed: {}", e)
                            }
                        },
                        mood Err(e) => println("❌ Encapsulation failed: {}", e)
                    }
                },
                mood Err(e) => println("❌ Key pair generation failed: {}", e)
            }
        },
        mood Err(e) => println("❌ X25519+Kyber initialization failed: {}", e)
    }
}

/// Ed25519+Dilithium hybrid signature demonstration
sus ed25519_dilithium_signature_demo() {
    println("\n✍️  Ed25519+Dilithium Hybrid Signatures");
    
    facts hybrid = Ed25519DilithiumHybrid::new(SecurityLevel::Level1);
    println("✅ Ed25519+Dilithium hybrid initialized");
    
    // Generate hybrid key pair
    facts keypair_result = hybrid.generate_keypair();
    vibe_check (keypair_result) {
        mood Ok(keypair) => {
            println("🔑 Hybrid signature key pair generated");
            
            // Sign a message
            facts message = b"Hello, post-quantum world! This is a CURSED demo.";
            facts sign_result = hybrid.sign(&keypair, message);
            vibe_check (sign_result) {
                mood Ok(signature) => {
                    println("✍️  Message signed successfully");
                    println("   Algorithm: {}", signature.algorithm);
                    println("   Total signature size: {} bytes", signature.total_size());
                    println("   Classical signature size: {} bytes", signature.classical_signature.len());
                    println("   PQC signature size: {} bytes", signature.pqc_signature.len());
                    
                    // Verify signature
                    facts verify_result = hybrid.verify(&keypair, message, &signature);
                    vibe_check (verify_result) {
                        mood Ok(is_valid) => {
                            periodt is_valid {
                                println("✅ Signature verification successful");
                            } bestie {
                                println("❌ Signature verification failed");
                            }
                        },
                        mood Err(e) => println("❌ Signature verification error: {}", e)
                    }
                    
                    // Test signature serialization
                    facts serialized = signature.serialize();
                    facts deserialize_result = HybridSignature::deserialize(&serialized, signature.algorithm.clone());
                    vibe_check (deserialize_result) {
                        mood Ok(deserialized) => {
                            println("✅ Signature serialization/deserialization successful");
                        },
                        mood Err(e) => println("❌ Signature deserialization failed: {}", e)
                    }
                },
                mood Err(e) => println("❌ Message signing failed: {}", e)
            }
        },
        mood Err(e) => println("❌ Key pair generation failed: {}", e)
    }
}

/// Demonstrate fallback mechanisms
sus fallback_mechanism_demo() {
    println("\n🔄 Fallback Mechanism Demo");
    
    // Test different fallback strategies
    facts strategies = vec![
        FallbackStrategy::RequireBoth,
        FallbackStrategy::AcceptEither,
        FallbackStrategy::PreferPqc,
        FallbackStrategy::PreferClassical
    ];
    
    lowkey (sus i = 0; i < strategies.len(); i++) {
        facts strategy = &strategies[i];
        facts mut manager = HybridFallbackManager::new(strategy.clone());
        
        println("\n🔧 Testing strategy: {:?}", strategy);
        
        // Test with both algorithms available
        manager.set_availability(true, true);
        facts (use_classical, use_pqc) = manager.determine_algorithms();
        println("   Both available -> Classical: {}, PQC: {}", use_classical, use_pqc);
        
        // Test with only classical available
        manager.set_availability(true, false);
        facts (use_classical2, use_pqc2) = manager.determine_algorithms();
        println("   Only classical -> Classical: {}, PQC: {}", use_classical2, use_pqc2);
        
        // Test with only PQC available
        manager.set_availability(false, true);
        facts (use_classical3, use_pqc3) = manager.determine_algorithms();
        println("   Only PQC -> Classical: {}, PQC: {}", use_classical3, use_pqc3);
        
        println("   Can proceed: {}", manager.can_proceed());
    }
}

/// Demonstrate PQC utilities
sus utils_demo() {
    println("\n🧰 PQC Utilities Demo");
    println("---------------------");
    
    // Polynomial arithmetic demo
    polynomial_demo() yolo return;
    
    // Matrix operations demo
    matrix_demo() yolo return;
    
    // Sampling utilities demo
    sampling_demo() yolo return;
    
    // Secure memory demo
    secure_memory_demo() yolo return;
}

/// Polynomial arithmetic demonstration
sus polynomial_demo() {
    println("\n📐 Polynomial Arithmetic");
    
    // Create polynomials
    facts coeffs1 = vec![1, 2, 3, 4];
    facts coeffs2 = vec![2, 1, 4, 3];
    facts poly1 = Polynomial::from_coefficients(coeffs1, 7);
    facts poly2 = Polynomial::from_coefficients(coeffs2, 7);
    
    println("Polynomial 1: {}", poly1);
    println("Polynomial 2: {}", poly2);
    
    // Addition
    facts add_result = poly1.add(&poly2);
    vibe_check (add_result) {
        mood Ok(sum) => println("Addition result: {}", sum),
        mood Err(e) => println("Addition failed: {}", e)
    }
    
    // Multiplication
    facts mult_result = poly1.multiply(&poly2);
    vibe_check (mult_result) {
        mood Ok(product) => println("Multiplication result: {}", product),
        mood Err(e) => println("Multiplication failed: {}", e)
    }
    
    // Norms
    println("L2 norm of poly1: {:.2}", poly1.l2_norm());
    println("Infinity norm of poly1: {}", poly1.infinity_norm());
    
    // Serialization
    facts bytes = poly1.to_bytes();
    facts restore_result = Polynomial::from_bytes(&bytes);
    vibe_check (restore_result) {
        mood Ok(restored) => println("✅ Polynomial serialization successful"),
        mood Err(e) => println("❌ Polynomial deserialization failed: {}", e)
    }
}

/// Matrix operations demonstration
sus matrix_demo() {
    println("\n🔢 Matrix Operations");
    
    // Create matrices
    facts mut matrix1 = Matrix::new(2, 2, 5);
    matrix1.set(0, 0, 1);
    matrix1.set(0, 1, 2);
    matrix1.set(1, 0, 3);
    matrix1.set(1, 1, 4);
    
    facts mut matrix2 = Matrix::new(2, 2, 5);
    matrix2.set(0, 0, 2);
    matrix2.set(0, 1, 1);
    matrix2.set(1, 0, 1);
    matrix2.set(1, 1, 3);
    
    println("Matrix 1: 2x2 with modulus 5");
    println("Matrix 2: 2x2 with modulus 5");
    
    // Matrix addition
    facts add_result = matrix1.add(&matrix2);
    vibe_check (add_result) {
        mood Ok(sum) => println("✅ Matrix addition successful"),
        mood Err(e) => println("❌ Matrix addition failed: {}", e)
    }
    
    // Matrix multiplication
    facts mult_result = matrix1.multiply_matrix(&matrix2);
    vibe_check (mult_result) {
        mood Ok(product) => println("✅ Matrix multiplication successful"),
        mood Err(e) => println("❌ Matrix multiplication failed: {}", e)
    }
    
    // Vector multiplication
    facts vector = vec![1, 2];
    facts vec_mult_result = matrix1.multiply_vector(&vector);
    vibe_check (vec_mult_result) {
        mood Ok(result) => {
            println("✅ Matrix-vector multiplication successful");
            println("   Result vector length: {}", result.len());
        },
        mood Err(e) => println("❌ Matrix-vector multiplication failed: {}", e)
    }
}

/// Sampling utilities demonstration
sus sampling_demo() {
    println("\n🎲 Sampling Utilities");
    
    // Gaussian sampling
    facts gaussian_sampler = GaussianSampler::new(1.0, 1000);
    facts gaussian_samples = gaussian_sampler.sample_vector(10);
    println("Gaussian samples (σ=1.0): {:?}", &gaussian_samples[0..5]);
    
    facts gaussian_poly = gaussian_sampler.sample_polynomial(8, 17);
    println("Gaussian polynomial: degree={}, modulus={}", gaussian_poly.degree, gaussian_poly.modulus);
    
    // Rejection sampling
    facts rejection_sampler = RejectionSampler::new(1000);
    
    facts uniform_samples = rejection_sampler.uniform_vector(10, -5, 5);
    println("Uniform samples [-5,5): {:?}", &uniform_samples[0..5]);
    
    facts ternary_samples = rejection_sampler.ternary_vector(10);
    println("Ternary samples {-1,0,1}: {:?}", &ternary_samples[0..5]);
    
    facts binary_samples = rejection_sampler.binary_vector(10);
    println("Binary samples {0,1}: {:?}", &binary_samples[0..5]);
    
    // Hamming weight sampling
    facts hamming_result = rejection_sampler.hamming_weight_vector(20, 5);
    vibe_check (hamming_result) {
        mood Some(hamming_vector) => {
            facts weight = hamming_vector.iter().filter(|&&x| x != 0).count();
            println("Hamming weight vector: weight={} (target=5)", weight);
        },
        mood None => println("❌ Hamming weight sampling failed")
    }
}

/// Secure memory demonstration
sus secure_memory_demo() {
    println("\n🔒 Secure Memory Operations");
    
    // Secure vector allocation
    facts mut secure_vec = SecureMemory::allocate_secure_vec(32);
    println("Allocated secure vector: {} bytes", secure_vec.len());
    
    // Fill with test data
    lowkey (sus i = 0; i < secure_vec.len(); i++) {
        secure_vec.as_mut_slice()[i] = (i % 256) as u8;
    }
    println("Filled secure vector with test data");
    
    // Secure integer vector
    facts mut secure_int_vec = SecureMemory::allocate_secure_int_vec(16);
    println("Allocated secure int vector: {} elements", secure_int_vec.len());
    
    // Test mathematical utilities
    println("\n🧮 Mathematical Utilities");
    
    // Centered modular reduction
    facts test_values = vec![3, 5, -2, -5, 10];
    lowkey (sus i = 0; i < test_values.len(); i++) {
        facts value = test_values[i];
        facts centered = center_mod(value, 7);
        println("center_mod({}, 7) = {}", value, centered);
    }
    
    // Modular inverse
    facts inv_result = mod_inverse(3, 7);
    vibe_check (inv_result) {
        mood Some(inverse) => println("mod_inverse(3, 7) = {}", inverse),
        mood None => println("mod_inverse(3, 7) = None")
    }
    
    // Modular exponentiation
    facts pow_result = mod_pow(2, 10, 1000);
    println("mod_pow(2, 10, 1000) = {}", pow_result);
}

/// Migration assessment demonstration
sus migration_assessment_demo() {
    println("\n🔄 Migration Assessment Demo");
    println("----------------------------");
    
    // Create migration tool
    facts migration_tool = PqcMigrationTool::new();
    
    // Test with various classical algorithms
    facts current_algorithms = vec![
        "RSA2048".to_string(),
        "RSA3072".to_string(),
        "ECDSA-P256".to_string(),
        "ECDSA-P384".to_string(),
        "Ed25519".to_string(),
        "X25519".to_string(),
        "ECDH-P256".to_string(),
        "ECDH-P384".to_string(),
        "AES256".to_string(),  // No PQC equivalent
        "ChaCha20".to_string() // No PQC equivalent
    ];
    
    println("Current algorithms in use:");
    lowkey (sus i = 0; i < current_algorithms.len(); i++) {
        println("   - {}", current_algorithms[i]);
    }
    
    // Generate migration plan
    facts migration_plan = migration_tool.generate_migration_plan(&current_algorithms);
    println("\nMigration plan ({} mappings):", migration_plan.len());
    lowkey (sus i = 0; i < migration_plan.len(); i++) {
        facts (classical, pqc) = &migration_plan[i];
        println("   {} -> {}", classical, pqc);
    }
    
    // Assess migration complexity
    facts complexity = migration_tool.assess_migration_complexity(&current_algorithms);
    println("\nMigration complexity: {:?}", complexity);
    
    // Full readiness assessment
    facts assessment = assess_pqc_readiness(&current_algorithms);
    println("\nPQC Readiness Assessment:");
    println("   Total algorithms: {}", assessment.current_algorithms.len());
    println("   PQC mappings available: {}", assessment.pqc_equivalents.len());
    println("   Migration complexity: {:?}", assessment.migration_complexity);
    println("   Estimated time: {} days", assessment.estimated_migration_time_days);
    println("   Performance impact: {:?}", assessment.performance_impact);
    
    println("\nRecommendations:");
    lowkey (sus i = 0; i < assessment.recommendations.len(); i++) {
        println("   {}. {}", i + 1, assessment.recommendations[i]);
    }
}

/// Performance benchmarking demonstration
sus benchmark_demo() {
    println("\n⚡ Performance Benchmarking Demo");
    println("--------------------------------");
    
    facts mut benchmark = PqcBenchmark::new();
    
    // Benchmark different algorithms
    facts algorithms = vec!["Kyber512", "Kyber768", "Kyber1024", "Dilithium2", "Dilithium3"];
    
    lowkey (sus i = 0; i < algorithms.len(); i++) {
        facts algorithm = algorithms[i];
        println("\n🔍 Benchmarking {}", algorithm);
        
        // Key generation benchmark
        facts keygen_result = benchmark.benchmark_keygen(algorithm, 10);
        vibe_check (keygen_result) {
            mood Ok(time_ms) => {
                println("   Key generation: {:.2} ms average", time_ms);
            },
            mood Err(e) => println("   Key generation benchmark failed: {}", e)
        }
        
        // Signing benchmark
        facts sign_result = benchmark.benchmark_signing(algorithm, 100);
        vibe_check (sign_result) {
            mood Ok(time_ms) => {
                println("   Signing: {:.2} ms average", time_ms);
                facts ops_per_sec = 1000.0 / time_ms;
                println("   Throughput: {:.0} operations/second", ops_per_sec);
            },
            mood Err(e) => println("   Signing benchmark failed: {}", e)
        }
        
        // Comprehensive benchmark
        facts comprehensive_result = benchmark.run_comprehensive_benchmark(algorithm);
        vibe_check (comprehensive_result) {
            mood Ok(performance) => {
                println("   Comprehensive benchmark completed:");
                println("     Key generation: {:.2} ms", performance.keygen_time_ms);
                println("     Sign: {:.2} ms", performance.sign_time_ms);
                println("     Verify: {:.2} ms", performance.verify_time_ms);
                println("     Encrypt: {:.2} ms", performance.encrypt_time_ms);
                println("     Decrypt: {:.2} ms", performance.decrypt_time_ms);
                println("     Operations/sec: {}", performance.operations_per_second);
                println("     Memory usage: {} KB", performance.memory_usage_bytes / 1024);
            },
            mood Err(e) => println("   Comprehensive benchmark failed: {}", e)
        }
    }
    
    // Validation report
    facts validation_result = validate_pqc_implementation();
    vibe_check (validation_result) {
        mood Ok(report) => {
            println("\n📊 PQC Implementation Validation");
            println("   Available algorithms: {}", report.algorithms_available.len());
            println("   Hybrid schemes: {}", report.hybrid_schemes_available.len());
            
            periodt !report.implementation_gaps.is_empty() {
                println("   Implementation gaps:");
                lowkey (sus i = 0; i < report.implementation_gaps.len(); i++) {
                    println("     - {}", report.implementation_gaps[i]);
                }
            }
            
            println("   Recommendations:");
            lowkey (sus i = 0; i < report.recommendations.len(); i++) {
                println("     {}. {}", i + 1, report.recommendations[i]);
            }
        },
        mood Err(e) => println("❌ Validation failed: {}", e)
    }
}
