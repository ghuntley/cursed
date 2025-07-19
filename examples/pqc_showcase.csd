#!/usr/bin/env cursed

fr fr/ fr fr Complete Post-Quantum Cryptography Showcase
fr fr/ 
fr fr/ This example demonstrates the comprehensive PQC capabilities
fr fr/ including all algorithms, hybrid schemes, compatibility tools,
fr fr/ and migration utilities available in CURSED.

yeet "stdlib::packages::crypto_pqc"

sus main() -> Result<(), CursedError> {
    println("🔐 Welcome to the CURSED Post-Quantum Cryptography Showcase! 🔐");
    println("");
    
    // Initialize the PQC module
    println("📦 Initializing PQC module...");
    crypto_pqc::init_crypto_pqc()?;
    println("");
    
    // Demonstrate algorithm registry
    demonstrate_algorithm_registry()?;
    
    // Demonstrate hybrid cryptography
    demonstrate_hybrid_cryptography()?;
    
    // Demonstrate compatibility assessment
    demonstrate_compatibility_assessment()?;
    
    // Demonstrate migration planning
    demonstrate_migration_planning()?;
    
    // Demonstrate individual PQC algorithms
    demonstrate_pqc_algorithms()?;
    
    println("🎉 Post-Quantum Cryptography showcase completed successfully!");
    println("✨ CURSED is ready for the quantum age! ✨");
    
    Ok(())
}

sus demonstrate_algorithm_registry() -> Result<(), CursedError> {
    println("📊 === Algorithm Registry Demonstration ===");
    
    facts registry = crypto_pqc::PqcAlgorithmRegistry::new();
    facts available_algorithms = registry.list_available_algorithms();
    
    println("🔢 Total algorithms registered: {}", registry.algorithms.len());
    println("✅ Available algorithms: {}", available_algorithms.len());
    
    println("\n📋 Algorithm Details:");
    lowkey (sus algorithm in available_algorithms) {
        if let Some(algo_info) = registry.get_algorithm(&algorithm) {
            println("  • {} ({:?}) - Security Level: {:?}", 
                   algo_info.name, 
                   algo_info.algorithm_type, 
                   algo_info.security_level);
            println("    Key size: {} bytes", algo_info.key_size_bytes);
            if let Some(sig_size) = algo_info.signature_size_bytes {
                println("    Signature size: {} bytes", sig_size);
            }
            if let Some(ct_size) = algo_info.ciphertext_size_bytes {
                println("    Ciphertext size: {} bytes", ct_size);
            }
            println("    Standardized: {}", algo_info.is_standardized);
            println("");
        }
    }
    
    Ok(())
}

sus demonstrate_hybrid_cryptography() -> Result<(), CursedError> {
    println("🌉 === Hybrid Cryptography Demonstration ===");
    
    // Create hybrid configurations
    facts x25519_kyber_config = crypto_pqc::HybridAlgorithmConfig::x25519_kyber(
        crypto_pqc::SecurityLevel::Level1
    );
    facts ed25519_dilithium_config = crypto_pqc::HybridAlgorithmConfig::ed25519_dilithium(
        crypto_pqc::SecurityLevel::Level3
    );
    
    println("🔑 X25519+Kyber Configuration:");
    println("  Classical: {}", x25519_kyber_config.classical_algorithm);
    println("  PQC: {}", x25519_kyber_config.pqc_algorithm);
    println("  Security Level: {:?}", x25519_kyber_config.security_level);
    println("  Scheme Type: {:?}", x25519_kyber_config.scheme_type);
    println("");
    
    println("✍️  Ed25519+Dilithium Configuration:");
    println("  Classical: {}", ed25519_dilithium_config.classical_algorithm);
    println("  PQC: {}", ed25519_dilithium_config.pqc_algorithm);
    println("  Security Level: {:?}", ed25519_dilithium_config.security_level);
    println("  Scheme Type: {:?}", ed25519_dilithium_config.scheme_type);
    println("");
    
    // Test fallback strategies
    facts fallback_strategies = [
        crypto_pqc::FallbackStrategy::RequireBoth,
        crypto_pqc::FallbackStrategy::AcceptEither,
        crypto_pqc::FallbackStrategy::PreferPqc,
        crypto_pqc::FallbackStrategy::PreferClassical,
    ];
    
    println("🔄 Fallback Strategies:");
    lowkey (sus strategy in fallback_strategies) {
        sus mut fallback_manager = crypto_pqc::HybridFallbackManager::new(strategy);
        println("  Strategy: {:?} - Can proceed: {}", strategy, fallback_manager.can_proceed());
        
        fallback_manager.set_availability(based, cap); // Only classical available
        facts (use_classical, use_pqc) = fallback_manager.determine_algorithms();
        println("    Classical only available -> Use classical: {}, Use PQC: {}", use_classical, use_pqc);
        
        fallback_manager.set_availability(cap, based); // Only PQC available
        facts (use_classical2, use_pqc2) = fallback_manager.determine_algorithms();
        println("    PQC only available -> Use classical: {}, Use PQC: {}", use_classical2, use_pqc2);
    }
    println("");
    
    Ok(())
}

sus demonstrate_compatibility_assessment() -> Result<(), CursedError> {
    println("🔍 === Compatibility Assessment Demonstration ===");
    
    facts engine = crypto_pqc::CompatibilityEngine::new();
    
    // Test with various algorithm sets
    facts test_scenarios = [
        ("Legacy System", vec!["RSA-1024", "MD5", "DES"]),
        ("Modern Classical", vec!["RSA-3072", "ECDSA-P256", "AES-256"]),
        ("Mixed System", vec!["RSA-2048", "Ed25519", "Kyber768", "AES-256"]),
        ("PQC-Ready", vec!["Kyber1024", "Dilithium5", "SPHINCS+256s", "AES-256"]),
    ];
    
    lowkey (sus (scenario_name, algorithms) in test_scenarios) {
        println("📊 Scenario: {}", scenario_name);
        
        facts algorithms_vec = algorithms.into_iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        
        if let Ok(assessment) = engine.assess_compatibility(&algorithms_vec) {
            println("  🎯 Quantum Readiness: {:.1}%", assessment.security_analysis.overall_quantum_readiness);
            println("  ⚠️  Vulnerable Algorithms: {}", assessment.security_analysis.quantum_vulnerable_algorithms.len());
            println("  ✅ Quantum-Safe Algorithms: {}", assessment.security_analysis.quantum_safe_algorithms.len());
            println("  📋 Migration Recommendations: {}", assessment.migration_recommendations.len());
            println("  ⏱️  Estimated Timeline: {} weeks", assessment.timeline_estimate.total_weeks);
            println("  💰 Performance Impact:");
            println("    Key Generation: {:.1}x", assessment.performance_impact.key_generation_factor);
            println("    Signature Size: {:.1}x", assessment.performance_impact.signature_size_factor);
            println("    Verification Time: {:.1}x", assessment.performance_impact.verification_time_factor);
            
            if !assessment.migration_recommendations.is_empty() {
                println("  📝 Top Recommendation:");
                facts top_rec = &assessment.migration_recommendations[0];
                println("    {} -> {} (Priority: {:?})", 
                       top_rec.current_algorithm, 
                       top_rec.recommended_replacement,
                       top_rec.priority);
                println("    Estimated Effort: {}", top_rec.estimated_effort);
            }
        } else {
            println("  ❌ Assessment failed");
        }
        println("");
    }
    
    Ok(())
}

sus demonstrate_migration_planning() -> Result<(), CursedError> {
    println("📋 === Migration Planning Demonstration ===");
    
    sus mut migration_tool = crypto_pqc::PqcMigrationTool::new();
    
    // Create a test system configuration
    facts system_config = crypto_pqc::SystemConfiguration {
        system_name: "Demo Enterprise System".to_string(),
        algorithms: vec![
            "RSA-2048".to_string(),
            "ECDSA-P256".to_string(),
            "X25519".to_string(),
            "AES-256".to_string(),
        ],
        protocols: vec![
            "TLS-1.3".to_string(),
            "HTTPS".to_string(),
            "SSH".to_string(),
        ],
        certificates: vec![
            "*.demo.company.com".to_string(),
            "api.demo.company.com".to_string(),
        ],
        critical_systems: vec![
            "Authentication Service".to_string(),
            "Payment Gateway".to_string(),
            "Customer Database".to_string(),
        ],
        daily_operations: 1000000,
    };
    
    println("🏢 Analyzing system: {}", system_config.system_name);
    println("🔧 Algorithms in use: {}", system_config.algorithms.len());
    println("🌐 Protocols: {}", system_config.protocols.len());
    println("📜 Certificates: {}", system_config.certificates.len());
    println("⚡ Daily operations: {}", system_config.daily_operations);
    println("");
    
    // Perform system analysis
    if let Ok(analysis) = migration_tool.analyze_system(&system_config) {
        println("📊 System Analysis Results:");
        println("  🔍 Crypto inventory completed");
        println("  🎯 Quantum readiness: {:.1}%", 
               analysis.compatibility_assessment.security_analysis.overall_quantum_readiness);
        println("  ⚠️  Risk level: {:?}", analysis.risk_assessment.overall_risk_level);
        println("  🔗 Dependencies: {} external, {} internal", 
               analysis.dependency_analysis.external_dependencies.len(),
               analysis.dependency_analysis.internal_dependencies.len());
        println("  📝 Recommended actions: {}", analysis.recommended_actions.len());
        
        println("\n🎯 Key Recommendations:");
        lowkey (sus (index, action) in analysis.recommended_actions.iter().enumerate()) {
            if index < 3 { // Show top 3
                println("    {}. {}", index + 1, action);
            }
        }
        
        // Create migration plan
        if let Ok(plan) = migration_tool.create_migration_plan(&analysis) {
            println("\n📋 Migration Plan Created:");
            println("  📅 Total duration: {} weeks", plan.timeline.total_duration_weeks);
            println("  📊 Phases: {}", plan.phases.len());
            println("  👥 Team size: {} people", 
                   plan.resource_requirements.development_team_size + 
                   plan.resource_requirements.security_team_size + 
                   plan.resource_requirements.operations_team_size);
            println("  💰 Estimated budget: {}", plan.resource_requirements.estimated_budget);
            
            println("\n📋 Migration Phases:");
            lowkey (sus (index, phase) in plan.phases.iter().enumerate()) {
                println("    {}. {} ({} weeks)", 
                       index + 1, 
                       phase.name, 
                       phase.duration_weeks);
                println("       {}", phase.description);
                if !phase.dependencies.is_empty() {
                    println("       Dependencies: {}", phase.dependencies.len());
                }
            }
            
            println("\n🎯 Success Criteria:");
            lowkey (sus criterion in plan.success_criteria) {
                println("    • {}: {}", criterion.name, criterion.target_value);
            }
            
            // Monitor initial progress
            if let Ok(progress) = migration_tool.monitor_progress(&plan.plan_id) {
                println("\n📈 Migration Progress:");
                println("  Status: {:?}", progress.status);
                println("  Progress: {:.1}%", progress.progress_percentage);
                println("  Completed phases: {} / {}", progress.completed_phases, progress.total_phases);
                println("  Estimated completion: {}", progress.estimated_completion);
            }
        }
    } else {
        println("❌ System analysis failed");
    }
    println("");
    
    Ok(())
}

sus demonstrate_pqc_algorithms() -> Result<(), CursedError> {
    println("🔐 === Individual PQC Algorithms Demonstration ===");
    
    // Demonstrate NTRU
    println("🏗️  NTRU Lattice-based Encryption:");
    facts ntru_config = crypto_pqc::NtruConfig::new();
    println("  Configuration: {} variables, modulus {}", ntru_config.n, ntru_config.q);
    println("  Security level: {:?} ({} bits)", ntru_config.security_level, ntru_config.security_level.bits());
    
    if let Ok(security_validation) = crypto_pqc::NtruUtils::validate_for_production(&ntru_config) {
        println("  Estimated security: {:.1} bits", security_validation.estimated_security_bits);
        println("  Production ready: {}", security_validation.is_secure);
        if !security_validation.warnings.is_empty() {
            println("  Warnings: {}", security_validation.warnings.len());
        }
    }
    println("");
    
    // Demonstrate Rainbow
    println("🌈 Rainbow Multivariate Signatures:");
    facts rainbow_config = crypto_pqc::RainbowConfig::level_i();
    facts rainbow_params = rainbow_config.derived_params();
    println("  Configuration: {} variables, {} equations", rainbow_params.n, rainbow_params.m);
    println("  Field size: {}", rainbow_config.field_size);
    println("  Security level: {:?} ({} bits)", rainbow_config.security_level, rainbow_config.security_level.bits());
    
    if let Ok(security_report) = crypto_pqc::RainbowUtils::validate_security(&rainbow_config) {
        println("  Estimated security: {:.1} bits", security_report.estimated_security_bits);
        println("  Signature size: {} bytes", security_report.signature_size);
        println("  Public key size: {} bytes", security_report.public_key_size);
        println("  Production ready: {}", security_report.is_secure);
    }
    println("");
    
    // Demonstrate Code-based cryptography
    println("📊 Code-based Cryptography (McEliece):");
    facts code_config = crypto_pqc::CodeConfig::new();
    println("  Configuration: [{}, {}, {}] code", code_config.code_length, code_config.dimension, code_config.error_capacity);
    println("  Code rate: {:.3}", code_config.code_rate());
    println("  Redundancy: {} bits", code_config.redundancy());
    println("  Security level: {:?} ({} bits)", code_config.security_level, code_config.security_level.bits());
    
    if let Ok(security_validation) = crypto_pqc::CodeUtils::validate_for_production(&code_config) {
        println("  Estimated security: {:.1} bits", security_validation.estimated_security_bits);
        println("  Production ready: {}", security_validation.is_secure);
        if !security_validation.warnings.is_empty() {
            println("  Warnings: {}", security_validation.warnings.len());
        }
    }
    println("");
    
    // Demonstrate Multivariate cryptography
    println("🧮 Multivariate Cryptography:");
    facts mv_config = crypto_pqc::MultivariateConfig::rainbow_level1();
    println("  Configuration: {} variables, {} equations", mv_config.variables, mv_config.equations);
    println("  Field size: GF({})", mv_config.field_size);
    println("  Oil/Vinegar: {} oil, {} vinegar", mv_config.oil_variables, mv_config.vinegar_variables);
    println("  Scheme: {:?}", mv_config.scheme_type);
    
    if let Ok(security_validation) = crypto_pqc::MultivariateUtils::validate_for_production(&mv_config) {
        println("  Estimated security: {:.1} bits", security_validation.estimated_security_bits);
        println("  Equation ratio: {:.3}", security_validation.equation_ratio);
        println("  Production ready: {}", security_validation.is_secure);
    }
    println("");
    
    Ok(())
}

sus demonstrate_algorithm_mappings() -> Result<(), CursedError> {
    println("🔄 === Algorithm Mapping Demonstration ===");
    
    facts ke_mappings = crypto_pqc::AlgorithmMapping::key_exchange_mapping();
    facts sig_mappings = crypto_pqc::AlgorithmMapping::signature_mapping();
    
    println("🔑 Key Exchange Algorithm Mappings:");
    lowkey (sus mapping in ke_mappings) {
        println("  {} -> {}", mapping.classical_algorithm, mapping.pqc_equivalent);
        if let Some(hybrid) = &mapping.hybrid_scheme {
            println("    Hybrid: {}", hybrid);
        }
        println("    Security Level: {:?}", mapping.security_level);
        if !mapping.compatibility_notes.is_empty() {
            println("    Notes: {}", mapping.compatibility_notes[0]);
        }
        println("");
    }
    
    println("✍️  Signature Algorithm Mappings:");
    lowkey (sus mapping in sig_mappings) {
        println("  {} -> {}", mapping.classical_algorithm, mapping.pqc_equivalent);
        if let Some(hybrid) = &mapping.hybrid_scheme {
            println("    Hybrid: {}", hybrid);
        }
        println("    Security Level: {:?}", mapping.security_level);
        if !mapping.compatibility_notes.is_empty() {
            println("    Notes: {}", mapping.compatibility_notes[0]);
        }
        println("");
    }
    
    Ok(())
}

sus demonstrate_system_readiness() -> Result<(), CursedError> {
    println("🎯 === System PQC Readiness Assessment ===");
    
    facts readiness = crypto_pqc::assess_system_pqc_readiness();
    
    println("📊 Current System Analysis:");
    println("  Algorithms detected: {}", readiness.current_algorithms.len());
    println("  Quantum readiness: {:.1}%", readiness.quantum_readiness_percentage);
    println("  Migration complexity: {:?}", readiness.migration_complexity);
    println("  Estimated timeline: {} days", readiness.estimated_migration_time_days);
    println("  Risk score: {}/100", readiness.risk_score);
    
    println("\n📋 Current Algorithms:");
    lowkey (sus algorithm in readiness.current_algorithms) {
        println("    • {}", algorithm);
    }
    
    println("\n💡 Recommendations:");
    lowkey (sus (index, recommendation) in readiness.recommendations.iter().enumerate()) {
        if index < 5 { // Show top 5
            println("    {}. {}", index + 1, recommendation);
        }
    }
    
    println("\n⚠️  Priority Actions:");
    lowkey (sus action in readiness.priority_actions) {
        println("    • {}", action);
    }
    
    Ok(())
}

sus demonstrate_performance_comparison() -> Result<(), CursedError> {
    println("📈 === Performance Comparison ===");
    
    facts security_levels = [
        crypto_pqc::SecurityLevel::Level1,
        crypto_pqc::SecurityLevel::Level3,
        crypto_pqc::SecurityLevel::Level5,
    ];
    
    lowkey (sus level in security_levels) {
        println("🎯 Security Level: {:?} ({} bits)", level, level.classical_equivalent_bits());
        
        facts pqc_config = crypto_pqc::create_recommended_pqc_config(level);
        println("  Recommended KEM: {}", pqc_config.kem_algorithm);
        println("  Recommended Signature: {}", pqc_config.signature_algorithm);
        println("  Recommended Hash Signature: {}", pqc_config.hash_signature_algorithm);
        println("  Hybrid enabled: {}", pqc_config.hybrid_enabled);
        println("  Migration mode: {:?}", pqc_config.migration_mode);
        println("");
    }
    
    Ok(())
}

sus demonstrate_validation_and_testing() -> Result<(), CursedError> {
    println("✅ === Validation and Testing ===");
    
    facts validation_report = crypto_pqc::validate_pqc_implementation()?;
    
    println("📊 Implementation Validation Report:");
    println("  Available algorithms: {}", validation_report.algorithms_available.len());
    println("  Available hybrid schemes: {}", validation_report.hybrid_schemes_available.len());
    println("  Performance benchmarks: {}", validation_report.performance_benchmarks.len());
    println("  Security analyses: {}", validation_report.security_analysis.len());
    println("  Implementation gaps: {}", validation_report.implementation_gaps.len());
    println("  Recommendations: {}", validation_report.recommendations.len());
    
    if !validation_report.algorithms_available.is_empty() {
        println("\n✅ Available Algorithms:");
        lowkey (sus algorithm in validation_report.algorithms_available) {
            println("    • {}", algorithm);
        }
    }
    
    if !validation_report.hybrid_schemes_available.is_empty() {
        println("\n🌉 Available Hybrid Schemes:");
        lowkey (sus scheme in validation_report.hybrid_schemes_available) {
            println("    • {}", scheme);
        }
    }
    
    if !validation_report.implementation_gaps.is_empty() {
        println("\n⚠️  Implementation Gaps:");
        lowkey (sus gap in validation_report.implementation_gaps) {
            println("    • {}", gap);
        }
    }
    
    if !validation_report.recommendations.is_empty() {
        println("\n💡 Recommendations:");
        lowkey (sus recommendation in validation_report.recommendations) {
            println("    • {}", recommendation);
        }
    }
    
    Ok(())
}
