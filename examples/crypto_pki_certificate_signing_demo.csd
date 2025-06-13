// Certificate Signing Demo for CURSED Crypto PKI Package
// 
// This example demonstrates comprehensive certificate signing functionality including:
// - Creating and configuring a Certificate Authority (CA)
// - Generating Certificate Signing Requests (CSRs)
// - Signing certificates with various algorithms
// - Batch certificate signing
// - Certificate renewal
// - Signature verification

import "stdlib::crypto_pki";
import "stdlib::io";

sus main() {
    periodt {
        // Initialize PKI crypto package
        crypto_pki::init_crypto_pki()?;
        
        // Demo 1: Basic certificate signing
        demo_basic_certificate_signing()?;
        
        // Demo 2: Certificate signing with templates
        demo_certificate_templates()?;
        
        // Demo 3: Batch certificate signing
        demo_batch_certificate_signing()?;
        
        // Demo 4: Certificate renewal
        demo_certificate_renewal()?;
        
        // Demo 5: Certificate signature verification
        demo_signature_verification()?;
        
        // Demo 6: Certificate signing policies
        demo_signing_policies()?;
        
        println("✅ All certificate signing demos completed successfully!")?;
    }
}

// Demo 1: Basic Certificate Signing
sus demo_basic_certificate_signing() {
    println("\n🔐 Demo 1: Basic Certificate Signing")?;
    
    // Create a CA configuration
    sus ca_config = crypto_pki::CaConfig {
        distinguished_name: crypto_pki::DistinguishedName::from_common_name("Demo Root CA"),
        signature_algorithm: crypto_pki::SignatureAlgorithm::RsaWithSha256,
        ca_key_usage: crypto_pki::KeyUsage {
            key_cert_sign: true,
            crl_sign: true,
            digital_signature: false,
            key_encipherment: false,
            data_encipherment: false,
            key_agreement: false,
            non_repudiation: false,
            encipher_only: false,
            decipher_only: false,
        },
        basic_constraints: crypto_pki::BasicConstraints {
            is_ca: true,
            path_length_constraint: None,
        },
        validity_days: 3650, // 10 years
    };
    
    // Create the CA
    sus ca_name = crypto_pki::create_certificate_authority("demo_ca".to_string(), ca_config)?;
    println("   ✅ Created CA: {}", ca_name)?;
    
    // Create a certificate signer
    crypto_pki::create_certificate_signer("demo_ca", "demo_signer".to_string())?;
    println("   ✅ Created certificate signer")?;
    
    // Create a CSR for a server certificate
    sus server_dn = crypto_pki::DistinguishedName {
        common_name: Some("example.com".to_string()),
        organization: Some("Example Corp".to_string()),
        organizational_unit: Some("IT Department".to_string()),
        country: Some("US".to_string()),
        state_or_province: Some("California".to_string()),
        locality: Some("San Francisco".to_string()),
        email_address: Some("admin@example.com".to_string()),
        additional_attributes: HashMap::new(),
    };
    
    sus csr = crypto_pki::CertificateSigningRequest {
        csr: crypto_pki::types::CertificateSigningRequest {
            version: 0,
            subject: server_dn,
            subject_public_key_info: crypto_pki::SubjectPublicKeyInfo {
                algorithm: crypto_pki::PublicKeyAlgorithm::Rsa { key_size: 2048 },
                public_key: vec![0x30, 0x82, 0x01, 0x0A], // Mock RSA public key
                parameters: None,
            },
            attributes: vec![],
            signature_algorithm: crypto_pki::SignatureAlgorithm::RsaWithSha256,
            signature: vec![],
            raw_data: vec![],
        },
        validity_period: Some(Duration::from_secs(365 * 24 * 60 * 60)), // 1 year
        additional_extensions: vec![],
        template: None,
        purpose: crypto_pki::CertificatePurpose::ServerAuth,
    };
    
    // Sign the certificate
    sus certificate = crypto_pki::sign_certificate_from_csr("demo_signer", csr)?;
    println("   ✅ Signed server certificate")?;
    println("   📋 Serial Number: {}", certificate.serial_number.to_hex_string())?;
    println("   🏷️  Subject: {}", certificate.subject.to_string())?;
    println("   ⏰ Valid from: {:?}", certificate.validity.not_before)?;
    println("   ⏰ Valid until: {:?}", certificate.validity.not_after)?;
}

// Demo 2: Certificate Templates
sus demo_certificate_templates() {
    println("\n📋 Demo 2: Certificate Templates")?;
    
    // Create a server authentication template
    sus server_template = crypto_pki::CertificateTemplate {
        name: "TLS Server Template".to_string(),
        key_usage: crypto_pki::KeyUsage {
            digital_signature: true,
            key_encipherment: true,
            data_encipherment: false,
            key_agreement: false,
            key_cert_sign: false,
            crl_sign: false,
            non_repudiation: false,
            encipher_only: false,
            decipher_only: false,
        },
        extended_key_usage: crypto_pki::ExtendedKeyUsage {
            server_auth: true,
            client_auth: false,
            code_signing: false,
            email_protection: false,
            time_stamping: false,
            ocsp_signing: false,
            custom_purposes: vec![],
        },
        validity_period: Duration::from_secs(2 * 365 * 24 * 60 * 60), // 2 years
        required_extensions: vec![],
        extension_overrides: HashMap::new(),
    };
    
    // Create a client authentication template
    sus client_template = crypto_pki::CertificateTemplate {
        name: "TLS Client Template".to_string(),
        key_usage: crypto_pki::KeyUsage {
            digital_signature: true,
            key_encipherment: false,
            data_encipherment: false,
            key_agreement: false,
            key_cert_sign: false,
            crl_sign: false,
            non_repudiation: false,
            encipher_only: false,
            decipher_only: false,
        },
        extended_key_usage: crypto_pki::ExtendedKeyUsage {
            server_auth: false,
            client_auth: true,
            code_signing: false,
            email_protection: false,
            time_stamping: false,
            ocsp_signing: false,
            custom_purposes: vec![],
        },
        validity_period: Duration::from_secs(365 * 24 * 60 * 60), // 1 year
        required_extensions: vec![],
        extension_overrides: HashMap::new(),
    };
    
    println("   ✅ Created server authentication template")?;
    println("   ✅ Created client authentication template")?;
    
    // Example: Use template for certificate signing
    println("   📝 Templates provide consistent certificate configuration")?;
    println("   🎯 Server template: TLS Server Authentication")?;
    println("   🎯 Client template: TLS Client Authentication")?;
}

// Demo 3: Batch Certificate Signing
sus demo_batch_certificate_signing() {
    println("\n📦 Demo 3: Batch Certificate Signing")?;
    
    // Create multiple CSRs for batch signing
    sus batch_csrs = vec![];
    
    lowkey (sus i = 0; i < 5; i++) {
        sus domain = format!("server{}.example.com", i);
        sus dn = crypto_pki::DistinguishedName::from_common_name(domain);
        
        sus csr_request = crypto_pki::CertificateSigningRequest {
            csr: crypto_pki::types::CertificateSigningRequest {
                version: 0,
                subject: dn,
                subject_public_key_info: crypto_pki::SubjectPublicKeyInfo {
                    algorithm: crypto_pki::PublicKeyAlgorithm::Rsa { key_size: 2048 },
                    public_key: vec![0x30, 0x82, 0x01, 0x0A], // Mock public key
                    parameters: None,
                },
                attributes: vec![],
                signature_algorithm: crypto_pki::SignatureAlgorithm::RsaWithSha256,
                signature: vec![],
                raw_data: vec![],
            },
            validity_period: Some(Duration::from_secs(365 * 24 * 60 * 60)),
            additional_extensions: vec![],
            template: None,
            purpose: crypto_pki::CertificatePurpose::ServerAuth,
        };
        
        batch_csrs.push(csr_request);
    }
    
    // Configure batch signing options
    sus batch_options = crypto_pki::BatchSigningOptions {
        continue_on_failure: true,
        max_concurrent: 3,
        signing_timeout: Duration::from_secs(30),
        detailed_report: true,
    };
    
    // Create batch signing request
    sus batch_request = crypto_pki::BatchSigningRequest {
        csrs: batch_csrs,
        batch_template: None,
        options: batch_options,
    };
    
    // Process batch signing
    sus batch_result = crypto_pki::sign_certificate_batch("demo_signer", batch_request)?;
    
    println("   ✅ Batch signing completed")?;
    println("   📊 Total requests: {}", batch_result.statistics.total_requests)?;
    println("   ✅ Successful signings: {}", batch_result.statistics.successful_signings)?;
    println("   ❌ Failed signings: {}", batch_result.statistics.failed_signings)?;
    println("   ⏱️  Average processing time: {:?}", batch_result.statistics.average_processing_time)?;
    
    bestie batch_result.report {
        sus report = report;
        println("   📋 Detailed Report Available ({} characters)", report.len())?;
    }
}

// Demo 4: Certificate Renewal
sus demo_certificate_renewal() {
    println("\n🔄 Demo 4: Certificate Renewal")?;
    
    // Create original certificate
    sus original_dn = crypto_pki::DistinguishedName::from_common_name("renewable.example.com");
    
    sus original_csr = crypto_pki::CertificateSigningRequest {
        csr: crypto_pki::types::CertificateSigningRequest {
            version: 0,
            subject: original_dn.clone(),
            subject_public_key_info: crypto_pki::SubjectPublicKeyInfo {
                algorithm: crypto_pki::PublicKeyAlgorithm::Rsa { key_size: 2048 },
                public_key: vec![0x30, 0x82, 0x01, 0x0A], // Mock public key
                parameters: None,
            },
            attributes: vec![],
            signature_algorithm: crypto_pki::SignatureAlgorithm::RsaWithSha256,
            signature: vec![],
            raw_data: vec![],
        },
        validity_period: Some(Duration::from_secs(30 * 24 * 60 * 60)), // 30 days
        additional_extensions: vec![],
        template: None,
        purpose: crypto_pki::CertificatePurpose::ServerAuth,
    };
    
    sus original_cert = crypto_pki::sign_certificate_from_csr("demo_signer", original_csr)?;
    println("   ✅ Created original certificate")?;
    println("   📅 Original validity: 30 days")?;
    
    // Create renewal request
    sus renewal_request = crypto_pki::CertificateRenewalRequest {
        original_certificate: original_cert,
        new_public_key: Some(crypto_pki::SubjectPublicKeyInfo {
            algorithm: crypto_pki::PublicKeyAlgorithm::Rsa { key_size: 4096 },
            public_key: vec![0x30, 0x82, 0x02, 0x0A], // New RSA 4096 key
            parameters: None,
        }),
        new_validity_period: Some(Duration::from_secs(2 * 365 * 24 * 60 * 60)), // 2 years
        extension_updates: HashMap::new(),
        keep_serial_number: false,
    };
    
    println("   ✅ Created renewal request with new 4096-bit RSA key")?;
    println("   📅 New validity: 2 years")?;
    println("   🔄 Certificate renewal demonstrates key rotation and lifecycle management")?;
}

// Demo 5: Certificate Signature Verification
sus demo_signature_verification() {
    println("\n🔍 Demo 5: Certificate Signature Verification")?;
    
    // Create a certificate for verification
    sus verify_dn = crypto_pki::DistinguishedName::from_common_name("verify.example.com");
    
    sus verify_csr = crypto_pki::CertificateSigningRequest {
        csr: crypto_pki::types::CertificateSigningRequest {
            version: 0,
            subject: verify_dn,
            subject_public_key_info: crypto_pki::SubjectPublicKeyInfo {
                algorithm: crypto_pki::PublicKeyAlgorithm::EllipticCurve { 
                    curve: crypto_pki::EllipticCurve::P256 
                },
                public_key: vec![0x30, 0x59, 0x30, 0x13], // Mock ECDSA P-256 key
                parameters: None,
            },
            attributes: vec![],
            signature_algorithm: crypto_pki::SignatureAlgorithm::EcdsaWithSha256,
            signature: vec![],
            raw_data: vec![],
        },
        validity_period: Some(Duration::from_secs(365 * 24 * 60 * 60)),
        additional_extensions: vec![],
        template: None,
        purpose: crypto_pki::CertificatePurpose::ServerAuth,
    };
    
    sus certificate = crypto_pki::sign_certificate_from_csr("demo_signer", verify_csr)?;
    println("   ✅ Created certificate with ECDSA P-256 signature")?;
    
    // Note: In a real implementation, we would verify the signature
    // For this demo, we describe the verification process
    println("   🔍 Certificate signature verification process:")?;
    println("     1. Extract certificate signature and algorithm")?;
    println("     2. Extract signed certificate data (TBS Certificate)")?;
    println("     3. Verify signature using CA public key")?;
    println("     4. Check certificate chain validity")?;
    println("   ✅ Signature verification ensures certificate authenticity")?;
}

// Demo 6: Certificate Signing Policies
sus demo_signing_policies() {
    println("\n📜 Demo 6: Certificate Signing Policies")?;
    
    // Configure signing policy
    sus signing_policy = crypto_pki::CertificateSigningPolicy {
        max_validity_period: Duration::from_secs(5 * 365 * 24 * 60 * 60), // 5 years max
        min_validity_period: Duration::from_secs(7 * 24 * 60 * 60), // 7 days min
        allowed_key_usages: vec![
            crypto_pki::KeyUsage {
                digital_signature: true,
                key_encipherment: true,
                data_encipherment: false,
                key_agreement: false,
                key_cert_sign: false,
                crl_sign: false,
                non_repudiation: false,
                encipher_only: false,
                decipher_only: false,
            }
        ],
        allowed_extended_key_usages: vec![
            crypto_pki::ExtendedKeyUsage {
                server_auth: true,
                client_auth: true,
                code_signing: false,
                email_protection: false,
                time_stamping: false,
                ocsp_signing: false,
                custom_purposes: vec![],
            }
        ],
        require_san: true,
        max_san_entries: 10,
        allowed_subject_fields: vec![
            "CN".to_string(),
            "O".to_string(),
            "OU".to_string(),
            "C".to_string(),
            "ST".to_string(),
            "L".to_string(),
        ],
        serial_number_policy: crypto_pki::SerialNumberPolicy {
            length: 16,
            secure_random: true,
            prefix: None,
            track_uniqueness: true,
        },
        extension_policy: crypto_pki::ExtensionPolicy {
            required_extensions: vec!["2.5.29.15".to_string()], // Key Usage required
            forbidden_extensions: vec![],
            auto_basic_constraints: true,
            auto_key_identifiers: true,
        },
    };
    
    println("   ✅ Configured comprehensive signing policy")?;
    println("   📏 Validity period: 7 days minimum, 5 years maximum")?;
    println("   🔑 Required key usage: Digital Signature + Key Encipherment")?;
    println("   🌐 Required SAN entries (max 10)")?;
    println("   🔢 16-byte secure random serial numbers")?;
    println("   📋 Auto-generated Basic Constraints and Key Identifiers")?;
    
    // Policy enforcement examples
    println("   🛡️  Policy enforcement ensures:")?;
    println("     - Certificate validity periods within acceptable range")?;
    println("     - Appropriate key usage for certificate purpose")?;
    println("     - Required extensions present")?;
    println("     - Subject DN field validation")?;
    println("     - Unique serial number generation")?;
}

// Certificate purpose examples
sus demonstrate_certificate_purposes() {
    println("\n🎯 Certificate Purpose Classifications:")?;
    
    println("   🌐 ServerAuth: TLS/SSL server certificates")?;
    println("   👤 ClientAuth: TLS/SSL client certificates")?;
    println("   📦 CodeSigning: Software and application signing")?;
    println("   ✉️  EmailProtection: S/MIME email certificates")?;
    println("   🏛️  IntermediateCA: Intermediate Certificate Authority")?;
    println("   🌳 RootCA: Root Certificate Authority (self-signed)")?;
    println("   ⏰ TimeStamping: Trusted timestamping authority")?;
    println("   🔍 OcspResponder: OCSP response signing")?;
    println("   🎭 Custom: Application-specific purposes")?;
}

// Signature algorithm showcase
sus demonstrate_signature_algorithms() {
    println("\n🔐 Supported Signature Algorithms:")?;
    
    println("   RSA with SHA-256: Industry standard, widely supported")?;
    println("   RSA with SHA-384: Higher security RSA variant")?;
    println("   RSA with SHA-512: Maximum security RSA variant")?;
    println("   ECDSA with SHA-256: Efficient elliptic curve signatures")?;
    println("   ECDSA with SHA-384: Higher security ECDSA variant")?;
    println("   ECDSA with SHA-512: Maximum security ECDSA variant")?;
    println("   Ed25519: Modern, fast, secure EdDSA signatures")?;
    println("   Ed448: Higher security EdDSA variant")?;
    
    println("\n   💡 Algorithm Selection Guidelines:")?;
    println("     • RSA: Broad compatibility, larger signatures")?;
    println("     • ECDSA: Smaller signatures, good performance")?;
    println("     • EdDSA: Best security and performance (modern)")?;
}
