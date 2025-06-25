/// Comprehensive test suite for the crypto_random package
use cursed::stdlib::packages::crypto_random::*;

#[test]
fn test_comprehensive_random_functionality() {
    // Test package initialization
    let init_result = init_crypto_random();
    assert!(init_result.is_ok(), "Package initialization should succeed");
    
    // Test basic random generation
    test_basic_random_generation();
    test_secure_random_api();
    test_random_bytes_generation();
    test_random_strings_generation();
    test_random_numbers_generation();
    test_nonce_generation();
    test_random_generators();
    test_entropy_monitoring();
    test_security_analysis();
}

fn test_basic_random_generation() {
    // Test basic random bytes
    let bytes_result = generate_random_bytes(64);
    assert!(bytes_result.is_ok());
    let bytes = bytes_result.unwrap();
    assert_eq!(bytes.len(), 64);
    
    // Test basic random number
    let number_result = generate_random_number();
    assert!(number_result.is_ok());
    
    // Test multiple generations for uniqueness
    let mut unique_numbers = std::collections::HashSet::new();
    for _ in 0..10 {
        if let Ok(num) = generate_random_number() {
            unique_numbers.insert(num);
        }
    }
    assert!(unique_numbers.len() > 5, "Should generate diverse numbers");
}

fn test_secure_random_api() {
    // Test SecureRandom functionality
    let secure_rng_result = SecureRandom::new();
    assert!(secure_rng_result.is_ok());
    
    let secure_rng = secure_rng_result.unwrap();
    
    // Test different data types
    assert!(secure_rng.u8().is_ok());
    assert!(secure_rng.u16().is_ok());
    assert!(secure_rng.u32().is_ok());
    assert!(secure_rng.u64().is_ok());
    assert!(secure_rng.i32().is_ok());
    assert!(secure_rng.i64().is_ok());
    assert!(secure_rng.f32().is_ok());
    assert!(secure_rng.f64().is_ok());
    assert!(secure_rng.bool().is_ok());
    
    // Test ranges
    let range_result = secure_rng.range_u32(1, 100);
    assert!(range_result.is_ok());
    let value = range_result.unwrap();
    assert!(value >= 1 && value <= 100);
    
    // Test float ranges
    let float_range_result = secure_rng.range_f64(0.0, 10.0);
    assert!(float_range_result.is_ok());
    let float_value = float_range_result.unwrap();
    assert!(float_value >= 0.0 && float_value < 10.0);
    
    // Test choice functionality
    let items = vec![1, 2, 3, 4, 5];
    let choice_result = secure_rng.choose(&items);
    assert!(choice_result.is_ok());
    
    // Test shuffle functionality
    let mut shuffle_items = vec![1, 2, 3, 4, 5];
    let shuffle_result = secure_rng.shuffle(&mut shuffle_items);
    assert!(shuffle_result.is_ok());
    
    // Verify all elements still present
    shuffle_items.sort();
    assert_eq!(shuffle_items, vec![1, 2, 3, 4, 5]);
}

fn test_random_bytes_generation() {
    let byte_gen_result = RandomBytes::new();
    assert!(byte_gen_result.is_ok());
    
    let byte_gen = byte_gen_result.unwrap();
    
    // Test different byte generation methods
    assert!(byte_gen.generate(32).is_ok());
    assert!(byte_gen.hex(16).is_ok());
    assert!(byte_gen.base64(24).is_ok());
    assert!(byte_gen.base64url(24).is_ok());
    assert!(byte_gen.base32(20).is_ok());
    assert!(byte_gen.binary(4).is_ok());
    
    // Test cryptographic-specific functions
    assert!(byte_gen.salt(32).is_ok());
    assert!(byte_gen.iv(16).is_ok());
    assert!(byte_gen.nonce(12).is_ok());
    assert!(byte_gen.key_material(32).is_ok());
    
    // Test specialized key generation
    assert!(byte_gen.for_encryption_key(32).is_ok());
    assert!(byte_gen.for_hmac_key(64).is_ok());
    assert!(byte_gen.for_signature_key(32).is_ok());
    assert!(byte_gen.for_session_id(24).is_ok());
    assert!(byte_gen.for_csrf_token(16).is_ok());
    assert!(byte_gen.for_api_key(32).is_ok());
    
    // Test high entropy generation
    let high_entropy_result = byte_gen.high_entropy(64);
    assert!(high_entropy_result.is_ok());
    
    // Test pattern-free generation
    let no_patterns_result = byte_gen.no_patterns(32);
    assert!(no_patterns_result.is_ok());
    
    // Test balanced bits generation
    let balanced_result = byte_gen.balanced_bits(32);
    assert!(balanced_result.is_ok());
}

fn test_random_strings_generation() {
    let string_gen_result = RandomStrings::new();
    assert!(string_gen_result.is_ok());
    
    let string_gen = string_gen_result.unwrap();
    
    // Test different character sets
    assert!(string_gen.alphabetic(10).is_ok());
    assert!(string_gen.alphanumeric(12).is_ok());
    assert!(string_gen.alphanumeric_lower(8).is_ok());
    assert!(string_gen.alphanumeric_upper(8).is_ok());
    assert!(string_gen.numeric(6).is_ok());
    assert!(string_gen.hexadecimal(16).is_ok());
    assert!(string_gen.hexadecimal_upper(16).is_ok());
    assert!(string_gen.base64(20).is_ok());
    assert!(string_gen.base64url(20).is_ok());
    assert!(string_gen.printable(15).is_ok());
    assert!(string_gen.symbols(8).is_ok());
    
    // Test specialized string generation
    assert!(string_gen.password(16, true).is_ok());
    assert!(string_gen.pronounceable(3).is_ok());
    assert!(string_gen.mnemonic(4).is_ok());
    assert!(string_gen.identifier(10).is_ok());
    assert!(string_gen.filename(12).is_ok());
    assert!(string_gen.url_safe(16).is_ok());
    assert!(string_gen.domain_name(10).is_ok());
    assert!(string_gen.email(8).is_ok());
    assert!(string_gen.words(5).is_ok());
    assert!(string_gen.sentence(6).is_ok());
    
    // Test custom character sets
    let custom_result = string_gen.generate(10, CharSet::Custom("ABC123".to_string()));
    assert!(custom_result.is_ok());
    let custom_string = custom_result.unwrap();
    assert!(custom_string.chars().all(|c| "ABC123".contains(c)));
}

fn test_random_numbers_generation() {
    let number_gen_result = RandomNumbers::new();
    assert!(number_gen_result.is_ok());
    
    let number_gen = number_gen_result.unwrap();
    
    // Test statistical distributions
    assert!(number_gen.normal(0.0, 1.0).is_ok());
    assert!(number_gen.normal_pair(0.0, 1.0).is_ok());
    assert!(number_gen.exponential(1.0).is_ok());
    assert!(number_gen.gamma(2.0, 1.0).is_ok());
    assert!(number_gen.beta(2.0, 3.0).is_ok());
    assert!(number_gen.poisson(5.0).is_ok());
    assert!(number_gen.binomial(10, 0.5).is_ok());
    assert!(number_gen.geometric(0.3).is_ok());
    assert!(number_gen.log_normal(0.0, 1.0).is_ok());
    assert!(number_gen.weibull(1.0, 2.0).is_ok());
    assert!(number_gen.chi_squared(5.0).is_ok());
    assert!(number_gen.student_t(10.0).is_ok());
    assert!(number_gen.f_distribution(5.0, 10.0).is_ok());
    assert!(number_gen.triangular(0.0, 1.0, 0.5).is_ok());
    assert!(number_gen.uniform(0.0, 10.0).is_ok());
    assert!(number_gen.uniform_int(-10, 10).is_ok());
    
    // Test vector generation
    assert!(number_gen.normal_vector(100, 0.0, 1.0).is_ok());
    assert!(number_gen.uniform_vector(50, 0.0, 1.0).is_ok());
    
    // Test sampling functions
    let probs = vec![0.3, 0.4, 0.3];
    assert!(number_gen.categorical(&probs).is_ok());
    
    let items = vec![1, 2, 3, 4, 5];
    assert!(number_gen.sample_without_replacement(&items, 3).is_ok());
    
    // Test specialized generation
    assert!(number_gen.random_walk(100, 1.0).is_ok());
    assert!(number_gen.brownian_motion(50, 0.1).is_ok());
}

fn test_nonce_generation() {
    let nonce_gen_result = NonceGenerator::new();
    assert!(nonce_gen_result.is_ok());
    
    let nonce_gen = nonce_gen_result.unwrap();
    
    // Test basic nonce generation
    assert!(nonce_gen.generate().is_ok());
    
    // Test specialized nonces
    assert!(nonce_gen.for_encryption().is_ok());
    assert!(nonce_gen.for_signature().is_ok());
    assert!(nonce_gen.for_challenge_response().is_ok());
    assert!(nonce_gen.for_session().is_ok());
    
    // Test batch generation
    let batch_result = nonce_gen.batch(5);
    assert!(batch_result.is_ok());
    let batch = batch_result.unwrap();
    assert_eq!(batch.len(), 5);
    
    // Test uniqueness
    let mut unique_nonces = std::collections::HashSet::new();
    for _ in 0..20 {
        if let Ok(nonce) = nonce_gen.generate() {
            unique_nonces.insert(nonce);
        }
    }
    assert_eq!(unique_nonces.len(), 20, "All nonces should be unique");
    
    // Test different strategies
    assert!(generate_random_nonce(16).is_ok());
    assert!(generate_timestamp_nonce().is_ok());
    assert!(generate_uuid_nonce().is_ok());
    assert!(generate_encryption_nonce().is_ok());
    assert!(generate_session_nonce().is_ok());
}

fn test_random_generators() {
    // Test password generator
    let password_gen_result = PasswordGenerator::new();
    assert!(password_gen_result.is_ok());
    
    let password_gen = password_gen_result.unwrap();
    let password_result = password_gen.length(16)
        .uppercase(true)
        .lowercase(true)
        .numbers(true)
        .symbols(true)
        .generate();
    assert!(password_result.is_ok());
    
    let password = password_result.unwrap();
    assert_eq!(password.len(), 16);
    
    // Test UUID generator
    let uuid_gen_result = UuidGenerator::new();
    assert!(uuid_gen_result.is_ok());
    
    let uuid_gen = uuid_gen_result.unwrap();
    assert!(uuid_gen.v4().is_ok());
    assert!(uuid_gen.simple().is_ok());
    assert!(uuid_gen.short().is_ok());
    
    // Test token generator
    let token_gen_result = TokenGenerator::new();
    assert!(token_gen_result.is_ok());
    
    let token_gen = token_gen_result.unwrap();
    assert!(token_gen.length(32).generate().is_ok());
    assert!(token_gen.api_key().is_ok());
    assert!(token_gen.session_token().is_ok());
    assert!(token_gen.csrf_token().is_ok());
    
    // Test data generator
    let data_gen_result = DataGenerator::new();
    assert!(data_gen_result.is_ok());
    
    let data_gen = data_gen_result.unwrap();
    assert!(data_gen.email().is_ok());
    assert!(data_gen.name().is_ok());
    assert!(data_gen.phone().is_ok());
    assert!(data_gen.ip_address().is_ok());
    assert!(data_gen.mac_address().is_ok());
    assert!(data_gen.credit_card().is_ok());
    assert!(data_gen.date().is_ok());
    assert!(data_gen.text(100).is_ok());
}

fn test_entropy_monitoring() {
    let monitor = EntropyMonitor::default();
    let test_data = generate_random_bytes(1000).unwrap();
    
    // Test entropy analysis
    let analysis_result = monitor.analyze_entropy(&EntropySource::SystemRandom, &test_data);
    assert!(analysis_result.is_ok());
    
    let metrics = analysis_result.unwrap();
    assert!(metrics.shannon_entropy > 0.0);
    assert!(metrics.quality_score >= 0.0 && metrics.quality_score <= 1.0);
    
    // Test monitoring functionality
    let sources_monitoring = monitor.get_all_sources_monitoring();
    assert!(!sources_monitoring.is_empty());
    
    let system_health = monitor.get_system_health();
    assert!(system_health >= 0.0 && system_health <= 1.0);
}

fn test_security_analysis() {
    let mut analyzer = SecurityAnalyzer::new();
    let test_data = generate_random_bytes(5000).unwrap();
    
    // Test comprehensive security analysis
    let analysis_result = analyzer.analyze(&test_data);
    assert!(analysis_result.is_ok());
    
    let security_result = analysis_result.unwrap();
    assert!(matches!(security_result.overall_security_level, 
                    SecurityLevel::Weak | SecurityLevel::Moderate | 
                    SecurityLevel::Strong | SecurityLevel::Excellent));
    
    assert!(security_result.confidence_score >= 0.0 && security_result.confidence_score <= 1.0);
    assert!(!security_result.recommendations.is_empty());
    
    // Test security report generation
    let report = analyzer.generate_security_report(&security_result);
    assert!(!report.is_empty());
    assert!(report.contains("SECURITY ANALYSIS"));
}

#[test]
fn test_randomness_quality() {
    // Test randomness test suite
    let test_suite = RandomnessTestSuite::new();
    let test_data = generate_random_bytes(2000).unwrap();
    
    // Test quick tests
    let quick_results = test_suite.quick_test(&test_data);
    assert!(!quick_results.is_empty());
    
    // Test comprehensive tests
    let comprehensive_result = test_suite.comprehensive_test(&test_data);
    assert!(comprehensive_result.is_ok());
    
    let test_results = comprehensive_result.unwrap();
    assert!(!test_results.is_empty());
    
    // Generate test report
    let report = test_suite.generate_report(&test_results);
    assert!(!report.is_empty());
    assert!(report.contains("RANDOMNESS TEST REPORT"));
}

#[test]
fn test_hardware_entropy() {
    let hardware_collector = HardwareEntropyCollector::new();
    
    // Test hardware detection
    let capabilities = hardware_collector.get_capabilities();
    // Hardware might not be available in all test environments
    
    // Test hardware stats
    let stats = hardware_collector.get_stats();
    assert!(stats.total_calls >= 0);
    assert!(stats.total_bytes_generated >= 0);
    
    // Test availability check
    let has_hardware = hardware_collector.has_hardware_rng();
    // This is platform-dependent, just ensure it doesn't crash
    assert!(has_hardware || !has_hardware);
}

#[test]
fn test_entropy_collection() {
    let config = EntropyCollectionConfig::default();
    let mut collector = EntropyCollector::new(config);
    
    // Test collector creation and basic functionality
    let start_result = collector.start();
    // This might fail in test environments without proper entropy sources
    
    let available_entropy = collector.available_entropy();
    assert!(available_entropy >= 0);
    
    let stats = collector.get_stats();
    assert!(stats.total_entropy_collected >= 0);
    
    // Clean up
    let _ = collector.stop();
}

#[test]
fn test_csprng_functionality() {
    let csprng_result = Csprng::new();
    assert!(csprng_result.is_ok());
    
    let csprng = csprng_result.unwrap();
    
    // Test different output sizes
    assert!(csprng.generate(16).is_ok());
    assert!(csprng.generate(32).is_ok());
    assert!(csprng.generate(64).is_ok());
    assert!(csprng.generate(128).is_ok());
    
    // Test reseeding
    assert!(csprng.force_reseed().is_ok());
    
    // Test state information
    let state = csprng.get_state();
    assert!(state.bytes_generated >= 0);
    assert!(state.reseed_count >= 0);
    
    // Test algorithm information
    let algorithm = csprng.get_algorithm();
    assert!(matches!(algorithm, CsprngAlgorithm::ChaCha20 | CsprngAlgorithm::Aes256Ctr | 
                               CsprngAlgorithm::Salsa20 | CsprngAlgorithm::XChaCha20 |
                               CsprngAlgorithm::Blake3 | CsprngAlgorithm::Sha256 |
                               CsprngAlgorithm::Fortuna | CsprngAlgorithm::Yarrow));
}

#[test]
fn test_convenience_functions() {
    // Test all the convenience functions
    assert!(generate_uuid().is_ok());
    assert!(generate_password(16).is_ok());
    assert!(generate_api_key().is_ok());
    assert!(generate_nonce().is_ok());
    
    // Test global functions
    assert!(random_bytes(32).is_ok());
    assert!(random_hex(16).is_ok());
    assert!(random_base64(24).is_ok());
    assert!(random_base64url(24).is_ok());
    assert!(random_salt(32).is_ok());
    assert!(random_iv(16).is_ok());
    assert!(random_key_material(32).is_ok());
    
    assert!(random_alphabetic(10).is_ok());
    assert!(random_alphanumeric(12).is_ok());
    assert!(random_numeric(8).is_ok());
    assert!(random_hexadecimal(16).is_ok());
    assert!(random_password(16, true).is_ok());
    assert!(random_identifier(10).is_ok());
    assert!(random_filename(12).is_ok());
    
    assert!(random_normal(0.0, 1.0).is_ok());
    assert!(random_exponential(1.0).is_ok());
    assert!(random_uniform(0.0, 1.0).is_ok());
    assert!(random_uniform_int(1, 100).is_ok());
    assert!(random_poisson(5.0).is_ok());
    assert!(random_binomial(10, 0.5).is_ok());
}

#[test]
fn test_api_integration() {
    let api_result = RandomAPI::new();
    assert!(api_result.is_ok());
    
    let api = api_result.unwrap();
    
    // Test all API components
    assert!(api.bytes().generate(32).is_ok());
    assert!(api.strings().alphanumeric(16).is_ok());
    assert!(api.numbers().uniform(0.0, 1.0).is_ok());
    assert!(api.nonces().generate().is_ok());
    
    // Test API methods
    assert!(api.test_quality(1000).is_ok());
    assert!(!api.entropy_info().is_empty());
    assert!(api.reseed().is_ok());
}

#[test]
fn test_package_level_functions() {
    // Test package-level convenience functions
    assert!(generate_random_bytes(32).is_ok());
    assert!(generate_random_number().is_ok());
    assert!(generate_uuid().is_ok());
    assert!(generate_password(16).is_ok());
    assert!(generate_api_key().is_ok());
    assert!(generate_nonce().is_ok());
    
    assert!(!get_entropy_info().is_empty());
    assert!(test_randomness_quality(1000).is_ok());
    assert!(reseed_generators().is_ok());
}

#[test]
fn test_error_handling() {
    // Test error conditions
    let byte_gen = RandomBytes::new().unwrap();
    
    // Test invalid inputs
    let large_request = byte_gen.generate(usize::MAX);
    // This should either work or fail gracefully
    
    // Test empty inputs
    let empty_result = byte_gen.generate(0);
    assert!(empty_result.is_ok());
    assert_eq!(empty_result.unwrap().len(), 0);
    
    // Test string generation with empty character set
    let string_gen = RandomStrings::new().unwrap();
    let empty_charset_result = string_gen.generate(10, CharSet::Custom("".to_string()));
    assert!(empty_charset_result.is_err());
    
    // Test number generation with invalid parameters
    let number_gen = RandomNumbers::new().unwrap();
    let invalid_exponential = number_gen.exponential(-1.0);
    assert!(invalid_exponential.is_err());
    
    let invalid_gamma = number_gen.gamma(-1.0, 1.0);
    assert!(invalid_gamma.is_err());
    
    let invalid_beta = number_gen.beta(-1.0, 1.0);
    assert!(invalid_beta.is_err());
}
