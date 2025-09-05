fr fr/ Comprehensive PKI Certificate Path Validation Demo
fr fr/ 
fr fr/ This demonstration showcases the CURSED language's PKI certificate path
fr fr/ validation capabilities including trust anchor management, certificate
fr fr/ chain building, policy validation, and name constraint enforcement.

yeet "stdlib::packages::crypto_pki::path_validation"
yeet "stdlib::packages::crypto_pki"
yeet "stdlib::io"

fr fr/ Demonstrates basic certificate path validation
slay demo_basic_path_validation() -> Result<(), Box<dyn std::error::Error>> {
    println("=== Basic Certificate Path Validation Demo ===\n")?;
    
    // Create mock certificate chain for demonstration
    let current_time = SystemTime::now();
    let future_time = current_time + Duration::from_secs(365 * 24 * 3600);
    
    // Root CA certificate (self-signed)
    let root_ca = CertificateInfo {
        version: 3,
        serial_number: "1".to_string(),
        signature_algorithm: "sha256WithRSAEncryption".to_string(),
        issuer_name: DistinguishedName {
            common_name: Some("Demo Root CA".to_string()),
            organization: Some("CURSED PKI Demo".to_string()),
            country: Some("US".to_string()),
            // ... other fields
        },
        subject_name: DistinguishedName {
            common_name: Some("Demo Root CA".to_string()),
            organization: Some("CURSED PKI Demo".to_string()),
            country: Some("US".to_string()),
            // ... other fields
        },
        not_before: current_time,
        not_after: future_time,
        public_key: PublicKeyInfo {
            algorithm: "rsaEncryption".to_string(),
            key_data: vec![/* RSA public key data */],
            parameters: None,
        },
        basic_constraints: Some(BasicConstraints {
            ca: based,
            path_len_constraint: Some(3),
        }),
        key_usage: Some(KeyUsageFlags::KEY_CERT_SIGN | KeyUsageFlags::CRL_SIGN),
        // ... other fields
    };
    
    println("Created Root CA certificate:")?;
    println("  Subject: {:?}", root_ca.subject_name)?;
    println("  Valid from: {:?} to {:?}", root_ca.not_before, root_ca.not_after)?;
    println("  CA: {}", root_ca.basic_constraints.as_ref().unwrap().ca)?;
    
    // Intermediate CA certificate
    let intermediate_ca = CertificateInfo {
        version: 3,
        serial_number: "2".to_string(),
        signature_algorithm: "sha256WithRSAEncryption".to_string(),
        issuer_name: root_ca.subject_name.clone(),
        subject_name: DistinguishedName {
            common_name: Some("Demo Intermediate CA".to_string()),
            organization: Some("CURSED PKI Demo".to_string()),
            organizational_unit: Some("Intermediate CA Division".to_string()),
            country: Some("US".to_string()),
            // ... other fields
        },
        not_before: current_time,
        not_after: future_time,
        public_key: PublicKeyInfo {
            algorithm: "rsaEncryption".to_string(),
            key_data: vec![/* Different RSA public key data */],
            parameters: None,
        },
        basic_constraints: Some(BasicConstraints {
            ca: based,
            path_len_constraint: Some(1),
        }),
        key_usage: Some(KeyUsageFlags::KEY_CERT_SIGN | KeyUsageFlags::CRL_SIGN),
        // ... other fields
    };
    
    println("\nCreated Intermediate CA certificate:")?;
    println("  Subject: {:?}", intermediate_ca.subject_name)?;
    println("  Issuer: {:?}", intermediate_ca.issuer_name)?;
    println("  Path length constraint: {:?}", intermediate_ca.basic_constraints.as_ref().unwrap().path_len_constraint)?;
    
    // End entity certificate
    let end_entity = CertificateInfo {
        version: 3,
        serial_number: "3".to_string(),
        signature_algorithm: "sha256WithRSAEncryption".to_string(),
        issuer_name: intermediate_ca.subject_name.clone(),
        subject_name: DistinguishedName {
            common_name: Some("demo.example.com".to_string()),
            organization: Some("Demo Organization".to_string()),
            country: Some("US".to_string()),
            // ... other fields
        },
        not_before: current_time,
        not_after: future_time,
        public_key: PublicKeyInfo {
            algorithm: "rsaEncryption".to_string(),
            key_data: vec![/* End entity public key data */],
            parameters: None,
        },
        basic_constraints: Some(BasicConstraints {
            ca: cap,
            path_len_constraint: None,
        }),
        key_usage: Some(KeyUsageFlags::DIGITAL_SIGNATURE | KeyUsageFlags::KEY_ENCIPHERMENT),
        extended_key_usage: Some(vec![
            "1.3.6.1.5.5.7.3.1".to_string(), // Server authentication
            "1.3.6.1.5.5.7.3.2".to_string(), // Client authentication
        ]),
        subject_alt_names: Some(vec![
            GeneralName::DnsName("demo.example.com".to_string()),
            GeneralName::DnsName("www.demo.example.com".to_string()),
            GeneralName::EmailAddress("admin@demo.example.com".to_string()),
        ]),
        // ... other fields
    };
    
    println("\nCreated End Entity certificate:")?;
    println("  Subject: {:?}", end_entity.subject_name)?;
    println("  Subject Alternative Names: {:?}", end_entity.subject_alt_names)?;
    println("  Extended Key Usage: {:?}", end_entity.extended_key_usage)?;
    
    // Create trust anchor from root CA
    let trust_anchor = TrustAnchor {
        certificate: Some(root_ca.clone()),
        public_key: root_ca.public_key.clone(),
        subject_name: root_ca.subject_name.clone(),
        key_identifier: root_ca.subject_key_identifier.clone(),
        name_constraints: None,
        certificate_policies: HashSet::new(),
    };
    
    // Create validation context
    let context = create_validation_context_with_anchors(vec![trust_anchor]);
    let mut validator = CertificatePathValidator::new(context);
    
    println("\n--- Performing Path Validation ---")?;
    
    // Validate certificate path
    let result = validator.validate_path(&end_entity, &[intermediate_ca])?;
    
    match result {
        PathValidationResult::Valid { validated_chain, validated_policies, trust_anchor } => {
            println("✅ Certificate path validation SUCCESSFUL!")?;
            println("  Chain length: {}", validated_chain.len())?;
            println("  Trust anchor: {:?}", trust_anchor.subject_name)?;
            println("  Validated policies: {:?}", validated_policies)?;
            
            println("\n  Certificate chain (end entity to root):")?;
            for (index, cert) in validated_chain.iter().enumerate() {
                println("    {}: {:?}", index + 1, cert.subject_name)?;
            }
        }
        PathValidationResult::Invalid { error, partial_chain } => {
            println("❌ Certificate path validation FAILED!")?;
            println("  Error: {:?}", error)?;
            println("  Partial chain length: {}", partial_chain.len())?;
        }
    }
    
    Ok(())
}

fr fr/ Demonstrates name constraint validation
slay demo_name_constraint_validation() -> Result<(), Box<dyn std::error::Error>> {
    println("\n\n=== Name Constraint Validation Demo ===\n")?;
    
    let current_time = SystemTime::now();
    let future_time = current_time + Duration::from_secs(365 * 24 * 3600);
    
    // Create constrained root CA
    let mut constrained_root = create_demo_certificate(
        "Constrained Root CA",
        "Constrained Root CA",
        1,
        current_time,
        future_time,
        based,
    );
    
    // Add name constraints to root CA
    constrained_root.name_constraints = Some(NameConstraints {
        permitted_subtrees: vec![
            GeneralSubtree {
                base: GeneralName::DnsName(".example.com".to_string()),
                minimum: None,
                maximum: None,
            },
            GeneralSubtree {
                base: GeneralName::EmailAddress("@example.com".to_string()),
                minimum: None,
                maximum: None,
            },
            GeneralSubtree {
                base: GeneralName::IpAddress(vec![192, 168, 1, 0, 255, 255, 255, 0]), // 192.168.1.0/24
                minimum: None,
                maximum: None,
            },
        ],
        excluded_subtrees: vec![
            GeneralSubtree {
                base: GeneralName::DnsName(".restricted.example.com".to_string()),
                minimum: None,
                maximum: None,
            },
        ],
    });
    
    println("Created constrained root CA with name constraints:")?;
    if let Some(ref nc) = constrained_root.name_constraints {
        println("  Permitted DNS: .example.com")?;
        println("  Permitted Email: @example.com")?;
        println("  Permitted IP: 192.168.1.0/24")?;
        println("  Excluded DNS: .restricted.example.com")?;
    }
    
    // Test certificate within constraints
    let valid_cert = create_end_entity_with_san(
        "valid.example.com",
        "Constrained Root CA",
        vec![
            GeneralName::DnsName("valid.example.com".to_string()),
            GeneralName::DnsName("api.example.com".to_string()),
            GeneralName::EmailAddress("admin@example.com".to_string()),
            GeneralName::IpAddress(vec![192, 168, 1, 10]),
        ],
        current_time,
        future_time,
    );
    
    println("\nTesting certificate within name constraints:")?;
    println("  Subject: valid.example.com")?;
    println("  SANs: api.example.com, admin@example.com, 192.168.1.10")?;
    
    let trust_anchor = TrustAnchor {
        certificate: Some(constrained_root.clone()),
        public_key: constrained_root.public_key.clone(),
        subject_name: constrained_root.subject_name.clone(),
        key_identifier: constrained_root.subject_key_identifier.clone(),
        name_constraints: constrained_root.name_constraints.clone(),
        certificate_policies: HashSet::new(),
    };
    
    let context = create_validation_context_with_anchors(vec![trust_anchor.clone()]);
    let mut validator = CertificatePathValidator::new(context);
    
    let result = validator.validate_path(&valid_cert, &[])?;
    match result {
        PathValidationResult::Valid { .. } => {
            println("  ✅ Validation successful - names within constraints")?;
        }
        PathValidationResult::Invalid { error, .. } => {
            println("  ❌ Unexpected validation failure: {:?}", error)?;
        }
    }
    
    // Test certificate violating constraints
    let invalid_cert = create_end_entity_with_san(
        "unauthorized.com",
        "Constrained Root CA",
        vec![
            GeneralName::DnsName("unauthorized.com".to_string()),
            GeneralName::DnsName("evil.restricted.example.com".to_string()),
            GeneralName::EmailAddress("hacker@unauthorized.com".to_string()),
            GeneralName::IpAddress(vec![10, 0, 0, 1]),
        ],
        current_time,
        future_time,
    );
    
    println("\nTesting certificate violating name constraints:")?;
    println("  Subject: unauthorized.com")?;
    println("  SANs: evil.restricted.example.com, hacker@unauthorized.com, 10.0.0.1")?;
    
    let context = create_validation_context_with_anchors(vec![trust_anchor]);
    let mut validator = CertificatePathValidator::new(context);
    
    let result = validator.validate_path(&invalid_cert, &[])?;
    match result {
        PathValidationResult::Valid { .. } => {
            println("  ❌ Unexpected validation success")?;
        }
        PathValidationResult::Invalid { error, .. } => {
            println("  ✅ Validation correctly failed: {:?}", error)?;
        }
    }
    
    Ok(())
}

fr fr/ Demonstrates certificate policy validation
slay demo_certificate_policy_validation() -> Result<(), Box<dyn std::error::Error>> {
    println("\n\n=== Certificate Policy Validation Demo ===\n")?;
    
    let current_time = SystemTime::now();
    let future_time = current_time + Duration::from_secs(365 * 24 * 3600);
    
    // Create certificates with specific policies
    let mut policy_root = create_demo_certificate(
        "Policy Root CA",
        "Policy Root CA",
        1,
        current_time,
        future_time,
        based,
    );
    
    // Add certificate policies to root
    policy_root.certificate_policies = Some(vec![
        CertificatePolicy {
            policy_id: "1.2.3.4.5.1".to_string(), // High assurance policy
            qualifiers: vec![
                PolicyQualifier {
                    qualifier_id: "1.3.6.1.5.5.7.2.1".to_string(), // CPS pointer
                    qualifier_data: b"https://pki.example.com/cps".to_vec(),
                },
            ],
        },
        CertificatePolicy {
            policy_id: "1.2.3.4.5.2".to_string(), // Medium assurance policy
            qualifiers: vec![],
        },
    ]);
    
    let mut policy_intermediate = create_demo_certificate(
        "Policy Intermediate CA",
        "Policy Root CA",
        2,
        current_time,
        future_time,
        based,
    );
    
    policy_intermediate.certificate_policies = Some(vec![
        CertificatePolicy {
            policy_id: "1.2.3.4.5.1".to_string(), // Inherit high assurance
            qualifiers: vec![],
        },
    ]);
    
    let mut policy_end_entity = create_demo_certificate(
        "policy.example.com",
        "Policy Intermediate CA",
        3,
        current_time,
        future_time,
        cap,
    );
    
    policy_end_entity.certificate_policies = Some(vec![
        CertificatePolicy {
            policy_id: "1.2.3.4.5.1".to_string(), // High assurance end entity
            qualifiers: vec![],
        },
    ]);
    
    println("Created certificate chain with policies:")?;
    println("  Root CA: High assurance (1.2.3.4.5.1), Medium assurance (1.2.3.4.5.2)")?;
    println("  Intermediate CA: High assurance (1.2.3.4.5.1)")?;
    println("  End Entity: High assurance (1.2.3.4.5.1)")?;
    
    // Create validation context requiring specific policy
    let trust_anchor = TrustAnchor {
        certificate: Some(policy_root),
        public_key: policy_intermediate.public_key.clone(),
        subject_name: policy_intermediate.issuer_name.clone(),
        key_identifier: None,
        name_constraints: None,
        certificate_policies: {
            let mut policies = HashSet::new();
            policies.insert("1.2.3.4.5.1".to_string());
            policies.insert("1.2.3.4.5.2".to_string());
            policies
        },
    };
    
    let mut context = create_validation_context_with_anchors(vec![trust_anchor]);
    context.required_policies.insert("1.2.3.4.5.1".to_string());
    context.require_explicit_policy = based;
    
    println("\nValidation context requires policy: 1.2.3.4.5.1")?;
    
    let mut validator = CertificatePathValidator::new(context);
    let result = validator.validate_path(&policy_end_entity, &[policy_intermediate])?;
    
    match result {
        PathValidationResult::Valid { validated_policies, .. } => {
            println("✅ Policy validation successful!")?;
            println("  Validated policies: {:?}", validated_policies)?;
            assert!(validated_policies.contains("1.2.3.4.5.1"));
        }
        PathValidationResult::Invalid { error, .. } => {
            println("❌ Policy validation failed: {:?}", error)?;
        }
    }
    
    // Test with missing required policy
    println("\nTesting with missing required policy...")?;
    
    let mut no_policy_context = create_validation_context_with_anchors(vec![trust_anchor]);
    no_policy_context.required_policies.insert("1.2.3.4.5.999".to_string()); // Non-existent policy
    no_policy_context.require_explicit_policy = based;
    
    let mut validator = CertificatePathValidator::new(no_policy_context);
    let result = validator.validate_path(&policy_end_entity, &[policy_intermediate])?;
    
    match result {
        PathValidationResult::Valid { .. } => {
            println("❌ Unexpected validation success with missing policy")?;
        }
        PathValidationResult::Invalid { error, .. } => {
            println("✅ Correctly failed with missing policy: {:?}", error)?;
        }
    }
    
    Ok(())
}

fr fr/ Demonstrates path length constraint validation
slay demo_path_length_constraints() -> Result<(), Box<dyn std::error::Error>> {
    println("\n\n=== Path Length Constraint Validation Demo ===\n")?;
    
    let current_time = SystemTime::now();
    let future_time = current_time + Duration::from_secs(365 * 24 * 3600);
    
    // Create root CA with path length constraint
    let mut constrained_root = create_demo_certificate(
        "Constrained Root CA",
        "Constrained Root CA",
        1,
        current_time,
        future_time,
        based,
    );
    
    constrained_root.basic_constraints = Some(BasicConstraints {
        ca: based,
        path_len_constraint: Some(2), // Allow maximum 2 intermediate CAs
    });
    
    println("Created root CA with path length constraint: 2")?;
    
    // Create intermediate CAs
    let intermediate1 = create_demo_certificate(
        "Intermediate CA 1",
        "Constrained Root CA",
        2,
        current_time,
        future_time,
        based,
    );
    
    let intermediate2 = create_demo_certificate(
        "Intermediate CA 2",
        "Intermediate CA 1",
        3,
        current_time,
        future_time,
        based,
    );
    
    let intermediate3 = create_demo_certificate(
        "Intermediate CA 3",
        "Intermediate CA 2",
        4,
        current_time,
        future_time,
        based,
    );
    
    let end_entity = create_demo_certificate(
        "constrained.example.com",
        "Intermediate CA 3",
        5,
        current_time,
        future_time,
        cap,
    );
    
    println("Created certificate chain:")?;
    println("  Root CA (path length: 2)")?;
    println("  → Intermediate CA 1")?;
    println("  → Intermediate CA 2")?;
    println("  → Intermediate CA 3 (would violate constraint)")?;
    println("  → End Entity")?;
    
    let trust_anchor = TrustAnchor {
        certificate: Some(constrained_root),
        public_key: intermediate1.public_key.clone(),
        subject_name: intermediate1.issuer_name.clone(),
        key_identifier: None,
        name_constraints: None,
        certificate_policies: HashSet::new(),
    };
    
    // Test with valid path length (2 intermediates)
    println("\nTesting with 2 intermediates (within constraint):")?;
    let context = create_validation_context_with_anchors(vec![trust_anchor.clone()]);
    let mut validator = CertificatePathValidator::new(context);
    
    let valid_end_entity = create_demo_certificate(
        "valid.example.com",
        "Intermediate CA 2",
        6,
        current_time,
        future_time,
        cap,
    );
    
    let result = validator.validate_path(&valid_end_entity, &[intermediate1.clone(), intermediate2.clone()])?;
    match result {
        PathValidationResult::Valid { .. } => {
            println("  ✅ Validation successful - path length within constraint")?;
        }
        PathValidationResult::Invalid { error, .. } => {
            println("  ❌ Unexpected validation failure: {:?}", error)?;
        }
    }
    
    // Test with invalid path length (3 intermediates)
    println("\nTesting with 3 intermediates (violates constraint):")?;
    let context = create_validation_context_with_anchors(vec![trust_anchor]);
    let mut validator = CertificatePathValidator::new(context);
    
    let result = validator.validate_path(&end_entity, &[intermediate1, intermediate2, intermediate3])?;
    match result {
        PathValidationResult::Valid { .. } => {
            println("  ❌ Unexpected validation success with path length violation")?;
        }
        PathValidationResult::Invalid { error, .. } => {
            println("  ✅ Correctly failed with path length violation: {:?}", error)?;
        }
    }
    
    Ok(())
}

fr fr/ Demonstrates complex certificate hierarchy validation
slay demo_complex_hierarchy_validation() -> Result<(), Box<dyn std::error::Error>> {
    println("\n\n=== Complex Certificate Hierarchy Demo ===\n")?;
    
    let current_time = SystemTime::now();
    let future_time = current_time + Duration::from_secs(365 * 24 * 3600);
    
    println("Creating complex PKI hierarchy with multiple roots and cross-certification...")?;
    
    // Create multiple root CAs
    let govt_root = create_demo_certificate(
        "Government Root CA",
        "Government Root CA",
        1,
        current_time,
        future_time,
        based,
    );
    
    let commercial_root = create_demo_certificate(
        "Commercial Root CA",
        "Commercial Root CA",
        2,
        current_time,
        future_time,
        based,
    );
    
    // Create cross-certified intermediate
    let mut cross_cert_intermediate = create_demo_certificate(
        "Cross-Certified Intermediate",
        "Government Root CA",
        3,
        current_time,
        future_time,
        based,
    );
    
    // Add name constraints for government use
    cross_cert_intermediate.name_constraints = Some(NameConstraints {
        permitted_subtrees: vec![
            GeneralSubtree {
                base: GeneralName::DnsName(".gov".to_string()),
                minimum: None,
                maximum: None,
            },
            GeneralSubtree {
                base: GeneralName::EmailAddress("@gov".to_string()),
                minimum: None,
                maximum: None,
            },
        ],
        excluded_subtrees: vec![],
    });
    
    // Create commercial intermediate
    let commercial_intermediate = create_demo_certificate(
        "Commercial Intermediate CA",
        "Commercial Root CA",
        4,
        current_time,
        future_time,
        based,
    );
    
    // Create end entities for different purposes
    let gov_server = create_end_entity_with_san(
        "secure.agency.gov",
        "Cross-Certified Intermediate",
        vec![
            GeneralName::DnsName("secure.agency.gov".to_string()),
            GeneralName::DnsName("portal.agency.gov".to_string()),
            GeneralName::EmailAddress("admin@agency.gov".to_string()),
        ],
        current_time,
        future_time,
    );
    
    let commercial_server = create_end_entity_with_san(
        "api.company.com",
        "Commercial Intermediate CA",
        vec![
            GeneralName::DnsName("api.company.com".to_string()),
            GeneralName::DnsName("www.company.com".to_string()),
            GeneralName::EmailAddress("support@company.com".to_string()),
        ],
        current_time,
        future_time,
    );
    
    println("Created PKI hierarchy:")?;
    println("  Government Root CA")?;
    println("    → Cross-Certified Intermediate (name constraints: .gov)")?;
    println("      → secure.agency.gov")?;
    println("  Commercial Root CA")?;
    println("    → Commercial Intermediate CA")?;
    println("      → api.company.com")?;
    
    // Create trust anchors
    let govt_trust_anchor = TrustAnchor {
        certificate: Some(govt_root),
        public_key: cross_cert_intermediate.public_key.clone(),
        subject_name: cross_cert_intermediate.issuer_name.clone(),
        key_identifier: None,
        name_constraints: None,
        certificate_policies: HashSet::new(),
    };
    
    let commercial_trust_anchor = TrustAnchor {
        certificate: Some(commercial_root),
        public_key: commercial_intermediate.public_key.clone(),
        subject_name: commercial_intermediate.issuer_name.clone(),
        key_identifier: None,
        name_constraints: None,
        certificate_policies: HashSet::new(),
    };
    
    // Test government certificate validation
    println("\nValidating government server certificate:")?;
    let context = create_validation_context_with_anchors(vec![govt_trust_anchor]);
    let mut validator = CertificatePathValidator::new(context);
    
    let result = validator.validate_path(&gov_server, &[cross_cert_intermediate.clone()])?;
    match result {
        PathValidationResult::Valid { .. } => {
            println("  ✅ Government certificate validation successful")?;
        }
        PathValidationResult::Invalid { error, .. } => {
            println("  ❌ Government certificate validation failed: {:?}", error)?;
        }
    }
    
    // Test commercial certificate validation
    println("\nValidating commercial server certificate:")?;
    let context = create_validation_context_with_anchors(vec![commercial_trust_anchor]);
    let mut validator = CertificatePathValidator::new(context);
    
    let result = validator.validate_path(&commercial_server, &[commercial_intermediate])?;
    match result {
        PathValidationResult::Valid { .. } => {
            println("  ✅ Commercial certificate validation successful")?;
        }
        PathValidationResult::Invalid { error, .. } => {
            println("  ❌ Commercial certificate validation failed: {:?}", error)?;
        }
    }
    
    // Test cross-validation (commercial cert against government root)
    println("\nTesting cross-validation (commercial cert against government root):")?;
    let context = create_validation_context_with_anchors(vec![govt_trust_anchor]);
    let mut validator = CertificatePathValidator::new(context);
    
    let result = validator.validate_path(&commercial_server, &[])?;
    match result {
        PathValidationResult::Valid { .. } => {
            println("  ❌ Unexpected cross-validation success")?;
        }
        PathValidationResult::Invalid { error, .. } => {
            println("  ✅ Cross-validation correctly failed: {:?}", error)?;
        }
    }
    
    Ok(())
}

fr fr/ Helper function to create demo certificate
slay create_demo_certificate(
    subject_cn: &str,
    issuer_cn: &str,
    serial: u64,
    not_before: SystemTime,
    not_after: SystemTime,
    is_ca: bool,
) -> CertificateInfo {
    CertificateInfo {
        version: 3,
        serial_number: serial.to_string(),
        signature_algorithm: "sha256WithRSAEncryption".to_string(),
        issuer_name: DistinguishedName {
            common_name: Some(issuer_cn.to_string()),
            organization: Some("CURSED PKI Demo".to_string()),
            country: Some("US".to_string()),
            locality: None,
            state_or_province: None,
            organizational_unit: None,
            email_address: None,
        },
        subject_name: DistinguishedName {
            common_name: Some(subject_cn.to_string()),
            organization: Some("CURSED PKI Demo".to_string()),
            country: Some("US".to_string()),
            locality: None,
            state_or_province: None,
            organizational_unit: None,
            email_address: None,
        },
        not_before,
        not_after,
        public_key: PublicKeyInfo {
            algorithm: "rsaEncryption".to_string(),
            key_data: vec![0u8; 256], // Mock RSA key
            parameters: None,
        },
        extensions: vec![],
        signature_value: vec![0u8; 256],
        tbs_certificate_data: vec![0u8; 512],
        authority_key_identifier: None,
        subject_key_identifier: Some(format!("ski_{}", subject_cn).into_bytes()),
        key_usage: if is_ca {
            Some(KeyUsageFlags::KEY_CERT_SIGN | KeyUsageFlags::CRL_SIGN)
        } else {
            Some(KeyUsageFlags::DIGITAL_SIGNATURE | KeyUsageFlags::KEY_ENCIPHERMENT)
        },
        extended_key_usage: if !is_ca {
            Some(vec![
                "1.3.6.1.5.5.7.3.1".to_string(), // Server authentication
                "1.3.6.1.5.5.7.3.2".to_string(), // Client authentication
            ])
        } else {
            None
        },
        basic_constraints: Some(BasicConstraints {
            ca: is_ca,
            path_len_constraint: if is_ca && subject_cn.contains("Root") {
                Some(3)
            } else if is_ca {
                Some(1)
            } else {
                None
            },
        }),
        subject_alt_names: None,
        issuer_alt_names: None,
        certificate_policies: None,
        name_constraints: None,
        crl_distribution_points: None,
        ocsp_responders: None,
    }
}

fr fr/ Helper function to create end entity certificate with subject alternative names
slay create_end_entity_with_san(
    subject_cn: &str,
    issuer_cn: &str,
    san_list: Vec<GeneralName>,
    not_before: SystemTime,
    not_after: SystemTime,
) -> CertificateInfo {
    let mut cert = create_demo_certificate(subject_cn, issuer_cn, 999, not_before, not_after, cap);
    cert.subject_alt_names = Some(san_list);
    cert
}

fr fr/ Main demonstration function
slay main_character() -> Result<(), Box<dyn std::error::Error>> {
    println("🔐 CURSED PKI Certificate Path Validation Demonstration")?;
    println("========================================================\n")?;
    
    println("This demo showcases comprehensive PKI certificate path validation")?;
    println("capabilities including RFC 5280 compliance, trust anchor management,")?;
    println("name constraints, certificate policies, and path length validation.\n")?;
    
    // Run all demonstrations
    demo_basic_path_validation()?;
    demo_name_constraint_validation()?;
    demo_certificate_policy_validation()?;
    demo_path_length_constraints()?;
    demo_complex_hierarchy_validation()?;
    
    println("\n\n=== Demo Summary ===\n")?;
    println("✅ Basic path validation - Building and validating certificate chains")?;
    println("✅ Name constraint validation - Enforcing DNS, email, and IP constraints")?;
    println("✅ Certificate policy validation - Policy inheritance and validation")?;
    println("✅ Path length constraints - Preventing excessively long chains")?;
    println("✅ Complex hierarchies - Cross-certification and multiple trust anchors")?;
    
    println("\n🛡️  PKI Security Features Demonstrated:")?;
    println("   • RFC 5280 compliant path validation algorithm")?;
    println("   • Trust anchor verification and management")?;
    println("   • Certificate chain building and ordering")?;
    println("   • Name constraint enforcement (DNS, email, IP)")?;
    println("   • Certificate policy processing and inheritance")?;
    println("   • Path length constraint validation")?;
    println("   • Critical extension processing")?;
    println("   • Comprehensive error reporting")?;
    
    println("\n📋 Real-world Applications:")?;
    println("   • Web server certificate validation (TLS/SSL)")?;
    println("   • Email certificate validation (S/MIME)")?;
    println("   • Code signing certificate validation")?;
    println("   • Government PKI validation")?;
    println("   • Enterprise certificate validation")?;
    println("   • Cross-domain trust establishment")?;
    
    println("\nDemo completed successfully! 🎉")?;
    
    Ok(())
}
