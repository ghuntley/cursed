# CURSED PKI (Public Key Infrastructure) - Comprehensive Guide

## Overview

The CURSED PKI module provides a complete, production-ready Public Key Infrastructure implementation with comprehensive certificate management, validation, and security features. This guide covers all aspects of the PKI system, from basic certificate operations to advanced CA management and trust chain validation.

## Table of Contents

1. [Architecture Overview](#architecture-overview)
2. [Key Features](#key-features)
3. [Core Components](#core-components)
4. [Certificate Management](#certificate-management)
5. [Certificate Authority Operations](#certificate-authority-operations)
6. [Chain Validation](#chain-validation)
7. [Trust Store Management](#trust-store-management)
8. [Key Management](#key-management)
9. [Certificate Revocation](#certificate-revocation)
10. [OCSP Support](#ocsp-support)
11. [Security Considerations](#security-considerations)
12. [API Reference](#api-reference)
13. [Examples](#examples)
14. [Best Practices](#best-practices)
15. [Troubleshooting](#troubleshooting)

## Architecture Overview

The CURSED PKI system follows a modular architecture with clear separation of concerns:

```
┌─────────────────────────────────────────────────────────────┐
│                    PKI Manager                              │
│  ┌─────────────────┐ ┌─────────────────┐ ┌────────────────┐ │
│  │ Certificate     │ │ Trust Store     │ │ Key Management │ │
│  │ Authorities     │ │ Management      │ │                │ │
│  └─────────────────┘ └─────────────────┘ └────────────────┘ │
└─────────────────────────────────────────────────────────────┘
┌─────────────────────────────────────────────────────────────┐
│                Core PKI Components                         │
│  ┌─────────────────┐ ┌─────────────────┐ ┌────────────────┐ │
│  │ X.509 Parser    │ │ Chain Validator │ │ CSR Generator  │ │
│  └─────────────────┘ └─────────────────┘ └────────────────┘ │
│  ┌─────────────────┐ ┌─────────────────┐ ┌────────────────┐ │
│  │ CRL Manager     │ │ OCSP Client     │ │ PEM/DER Codec  │ │
│  └─────────────────┘ └─────────────────┘ └────────────────┘ │
└─────────────────────────────────────────────────────────────┘
┌─────────────────────────────────────────────────────────────┐
│                 Foundation Layer                           │
│  ┌─────────────────┐ ┌─────────────────┐ ┌────────────────┐ │
│  │ Error Handling  │ │ Core Types      │ │ Crypto Backend │ │
│  └─────────────────┘ └─────────────────┘ └────────────────┘ │
└─────────────────────────────────────────────────────────────┘
```

## Key Features

### ✅ Comprehensive X.509 Support
- **Certificate Parsing**: DER and PEM format support
- **Multiple Versions**: X.509 v1, v2, and v3 certificates
- **Extension Support**: Basic constraints, key usage, SAN, AIA, CRL distribution points
- **Signature Algorithms**: RSA, ECDSA, Ed25519, Ed448

### ✅ Certificate Authority Operations
- **CA Creation**: Self-signed root CAs and subordinate CAs
- **Certificate Issuance**: Template-based certificate generation
- **Policy Management**: Configurable certificate policies and constraints
- **Hierarchical Trust**: Multi-level CA hierarchies with path length constraints

### ✅ Advanced Validation
- **Chain Validation**: Complete certificate chain verification
- **Path Building**: Automatic trust path discovery
- **Revocation Checking**: CRL and OCSP integration
- **Policy Validation**: Certificate policy and constraint checking

### ✅ Security Features
- **Cryptographic Algorithms**: Industry-standard algorithms
- **Key Management**: Secure key generation and storage
- **Trust Anchors**: Configurable trust anchor management
- **Validation Policies**: Flexible validation rule configuration

### ✅ Real-time Features
- **OCSP Client**: Online certificate status checking
- **CRL Management**: Certificate revocation list handling
- **Status Caching**: Intelligent caching for performance
- **Network Integration**: HTTP-based revocation checking

## Core Components

### PKI Manager

The central coordinator that manages all PKI operations:

```cursed
// Initialize PKI system
init_crypto_pki()?;

// Get PKI statistics
let stats = get_pki_statistics()?;
println("Certificates validated: {}", stats.certificates_validated);
```

### X.509 Parser

Comprehensive certificate parsing with extensive format support:

```cursed
// Parse PEM certificate
let cert = parse_certificate(pem_data.as_bytes(), Some("pem"))?;

// Parse DER certificate
let cert = parse_certificate(der_data, Some("der"))?;

// Auto-detect format
let cert = parse_certificate(cert_data, None)?;
```

### Certificate Authority

Complete CA functionality for certificate issuance:

```cursed
// Create CA configuration
let ca_config = CaConfig {
    name: "Root CA".to_string(),
    distinguished_name: root_dn,
    signature_algorithm: SignatureAlgorithm::RsaWithSha256,
    default_validity: Duration::from_secs(365 * 24 * 3600),
    // ... other configuration
};

// Create Certificate Authority
let ca_id = create_certificate_authority("root_ca".to_string(), ca_config)?;
```

### Chain Validator

Sophisticated chain validation with policy enforcement:

```cursed
// Validate certificate chain
let result = validate_certificate_chain(&cert_chain)?;

if result.is_valid {
    println("Certificate chain is valid");
} else {
    for error in result.errors {
        println("Validation error: {}", error);
    }
}
```

## Certificate Management

### Certificate Structure

X.509 certificates contain the following key components:

- **Version**: Certificate version (1, 2, or 3)
- **Serial Number**: Unique identifier within CA
- **Signature Algorithm**: Algorithm used to sign the certificate
- **Issuer**: Distinguished name of the certificate issuer
- **Validity**: Not before and not after dates
- **Subject**: Distinguished name of the certificate holder
- **Public Key**: Subject's public key and algorithm
- **Extensions**: Additional certificate information (v3 only)

### Distinguished Names

Distinguished Names (DNs) follow the X.500 standard:

```cursed
let dn = DistinguishedName {
    common_name: Some("www.example.com".to_string()),
    organization: Some("Example Corp".to_string()),
    organizational_unit: Some("IT Department".to_string()),
    country: Some("US".to_string()),
    state_or_province: Some("California".to_string()),
    locality: Some("San Francisco".to_string()),
    email_address: Some("admin@example.com".to_string()),
    additional_attributes: HashMap::new(),
};
```

### Certificate Extensions

The PKI system supports all standard X.509v3 extensions:

#### Basic Constraints
```cursed
ExtensionData::BasicConstraints {
    is_ca: true,
    path_length_constraint: Some(5),
}
```

#### Key Usage
```cursed
let key_usage = KeyUsage {
    digital_signature: true,
    key_encipherment: true,
    key_cert_sign: true,
    crl_sign: true,
    ..KeyUsage::default()
};
```

#### Subject Alternative Names
```cursed
let sans = vec![
    GeneralName::DnsName("www.example.com".to_string()),
    GeneralName::DnsName("example.com".to_string()),
    GeneralName::Rfc822Name("admin@example.com".to_string()),
    GeneralName::IpAddress(vec![192, 168, 1, 1]),
];
```

## Certificate Authority Operations

### CA Creation

Creating a Certificate Authority involves several steps:

1. **Generate CA Key Pair**
2. **Configure CA Parameters**
3. **Generate Self-Signed Certificate**
4. **Initialize CA Templates**

```cursed
// Step 1: Configure CA
let ca_config = CaConfig {
    name: "Enterprise Root CA".to_string(),
    distinguished_name: ca_dn,
    default_validity: Duration::from_secs(365 * 24 * 3600), // 1 year
    max_validity: Duration::from_secs(10 * 365 * 24 * 3600), // 10 years
    signature_algorithm: SignatureAlgorithm::RsaWithSha256,
    ca_key_usage: KeyUsage {
        key_cert_sign: true,
        crl_sign: true,
        digital_signature: true,
        ..KeyUsage::default()
    },
    basic_constraints: BasicConstraints {
        is_ca: true,
        path_length_constraint: Some(3),
        critical: true,
    },
    supported_key_algorithms: vec![
        PublicKeyAlgorithm::Rsa { key_size: 2048 },
        PublicKeyAlgorithm::EllipticCurve { curve: EllipticCurve::P256 },
        PublicKeyAlgorithm::Ed25519,
    ],
    // ... additional configuration
};

// Step 2: Create the CA
let ca_id = create_certificate_authority("enterprise_ca".to_string(), ca_config)?;
```

### Certificate Issuance

The CA can issue certificates using predefined templates:

```cursed
// Create issuance request
let issuance_request = CertificateIssuanceRequest {
    csr: certificate_signing_request,
    validity_period: Some(Duration::from_secs(365 * 24 * 3600)),
    template_name: Some("server_auth".to_string()),
    additional_sans: vec![
        GeneralName::DnsName("api.example.com".to_string()),
    ],
    custom_extensions: vec![],
    purpose: CertificatePurpose::ServerAuth,
    requestor_info: RequestorInfo {
        name: "IT Administrator".to_string(),
        email: Some("it@example.com".to_string()),
        organization: Some("Example Corp".to_string()),
        requested_at: SystemTime::now(),
        metadata: HashMap::new(),
    },
};

// Issue the certificate
let issued_cert = ca.issue_certificate(issuance_request)?;
```

### Certificate Templates

Templates standardize certificate issuance:

```cursed
// Server Authentication Template
let server_auth_template = CertificateTemplate {
    name: "server_auth".to_string(),
    description: "TLS Server Authentication".to_string(),
    key_usage: KeyUsage {
        digital_signature: true,
        key_encipherment: true,
        ..KeyUsage::default()
    },
    extended_key_usage: ExtendedKeyUsage {
        server_auth: true,
        ..ExtendedKeyUsage::default()
    },
    validity_period: Duration::from_secs(365 * 24 * 3600),
    allowed_key_algorithms: vec![
        PublicKeyAlgorithm::Rsa { key_size: 2048 },
        PublicKeyAlgorithm::EllipticCurve { curve: EllipticCurve::P256 },
    ],
    required_subject_fields: vec!["CN".to_string()],
    san_requirements: SanRequirements {
        require_dns_names: true,
        allowed_dns_patterns: vec!["*.example.com".to_string()],
        ..SanRequirements::default()
    },
    // ... additional template configuration
};
```

## Chain Validation

### Validation Process

Certificate chain validation follows RFC 5280 and includes:

1. **Basic Validation**: Format, signatures, dates
2. **Chain Building**: Construct trust paths
3. **Policy Checking**: Certificate policies and constraints
4. **Revocation Checking**: CRL and OCSP validation
5. **Trust Anchor Verification**: Root certificate validation

### Validation Policies

Customize validation behavior:

```cursed
let validation_policy = ValidationPolicy {
    check_validity_dates: true,
    check_revocation: true,
    max_chain_length: 10,
    require_basic_constraints: true,
    allow_self_signed: false,
    check_key_usage: true,
    check_extended_key_usage: true,
    min_rsa_key_size: 2048,
    allowed_signature_algorithms: vec![
        SignatureAlgorithm::RsaWithSha256,
        SignatureAlgorithm::EcdsaWithSha256,
        SignatureAlgorithm::Ed25519,
    ],
    // ... additional policy settings
};
```

### Path Building

Automatic trust path discovery:

```cursed
let path_result = chain_validator.build_paths(&end_entity_cert, &context)?;

for chain in path_result.chains {
    println("Found trust path with {} certificates", 
            1 + chain.intermediates.len() + 
            if chain.root.is_some() { 1 } else { 0 });
}
```

## Trust Store Management

### Trust Store Configuration

```cursed
let trust_store_config = TrustStoreConfig {
    allow_self_signed: false,
    max_chain_length: 10,
    check_validity_dates: true,
    check_revocation: true,
    ocsp_responders: vec!["http://ocsp.example.com".to_string()],
    crl_distribution_points: vec!["http://crl.example.com/ca.crl".to_string()],
    network_timeout: Duration::from_secs(30),
};
```

### Adding Trust Anchors

```cursed
let mut trust_store = TrustStore::new("enterprise");
trust_store.add_root_certificate(root_ca_cert);
trust_store.add_intermediate_certificate(intermediate_cert);
```

### Trust Store Operations

```cursed
// Check if certificate is trusted
if trust_store.is_trusted(&certificate) {
    println("Certificate is trusted");
}

// Add certificate fingerprint
trust_store.trusted_fingerprints.insert(
    certificate.fingerprint.unwrap(),
    "Enterprise CA".to_string()
);
```

## Key Management

### Key Generation

```cursed
let key_config = KeyGenerationConfig {
    algorithm: PublicKeyAlgorithm::Rsa { key_size: 4096 },
    key_usage: vec![KeyUsagePurpose::DigitalSignature],
    key_strength: KeyStrength::High,
    parameters: HashMap::new(),
};

let key_pair = key_manager.generate_key_pair(&key_config)?;
```

### Key Storage

```cursed
// Store key pair securely
let key_id = key_manager.store_key_pair(key_pair, true)?; // encrypted=true

// Retrieve key pair
let stored_key = key_manager.get_key_pair(&key_id)?;
```

### Key Lifecycle Management

```cursed
// Check key expiration
if key_pair.is_expired() {
    println("Key has expired, rotation required");
}

// Get key age
let age_days = key_pair.age_days();
if age_days > 365 {
    println("Key is {} days old, consider rotation", age_days);
}
```

## Certificate Revocation

### CRL Management

```cursed
// Generate Certificate Revocation List
let crl = ca.generate_crl()?;

// Add CRL to manager
crl_manager.add_crl(crl, Some("http://crl.example.com/ca.crl".to_string()))?;

// Check revocation status
let is_revoked = crl_manager.is_certificate_revoked(&serial_number, &issuer_dn)?;
```

### Certificate Revocation

```cursed
let revocation_request = CertificateRevocationRequest {
    serial_number: cert_serial,
    reason: RevocationReason::KeyCompromise,
    revocation_time: Some(SystemTime::now()),
    requestor: "Security Team".to_string(),
    context: "Private key suspected compromised".to_string(),
};

ca.revoke_certificate(revocation_request)?;
```

## OCSP Support

### OCSP Client Configuration

```cursed
let ocsp_config = OcspConfig {
    timeout: Duration::from_secs(30),
    default_responder_url: Some("http://ocsp.example.com".to_string()),
    use_nonce: true,
};

let ocsp_client = OcspClient::new(ocsp_config);
```

### Status Checking

```cursed
let ocsp_request = OcspRequest {
    certificate: end_entity_cert,
    issuer: issuer_cert,
    responder_url: "http://ocsp.example.com".to_string(),
};

let response = ocsp_client.check_certificate_status(&ocsp_request)?;

match response.response_status {
    OcspResponseStatus::Successful => {
        println("OCSP check successful");
    },
    _ => {
        println("OCSP check failed: {:?}", response.response_status);
    }
}
```

## Security Considerations

### Cryptographic Security

1. **Strong Algorithms**: Use modern, secure cryptographic algorithms
2. **Key Sizes**: Minimum 2048-bit RSA, 256-bit ECC
3. **Hash Functions**: SHA-256 or stronger
4. **Key Protection**: Encrypt private keys at rest

### Validation Security

1. **Complete Chain Validation**: Always validate entire certificate chains
2. **Revocation Checking**: Enable CRL and OCSP checking
3. **Time Validation**: Check certificate validity periods
4. **Policy Enforcement**: Implement and enforce certificate policies

### Operational Security

1. **CA Protection**: Secure CA private keys with HSMs
2. **Access Control**: Implement proper access controls
3. **Audit Logging**: Log all PKI operations
4. **Regular Updates**: Keep trust stores current

## API Reference

### Core Functions

```cursed
// Initialize PKI system
init_crypto_pki() -> Result<(), CursedError>

// Parse certificates
parse_certificate(data: &[u8], format: Option<&str>) -> Result<X509Certificate, CursedError>

// Validate chains
validate_certificate_chain(chain: &CertificateChain) -> Result<ValidationResult, CursedError>

// Create CAs
create_certificate_authority(name: String, config: CaConfig) -> Result<String, CursedError>

// Get statistics
get_pki_statistics() -> Result<PkiStatistics, CursedError>
```

### Key Types

- `X509Certificate`: Complete certificate representation
- `CertificateChain`: Certificate chain structure
- `ValidationResult`: Chain validation results
- `CaConfig`: Certificate Authority configuration
- `ValidationPolicy`: Validation rule configuration
- `TrustStore`: Trust anchor management
- `KeyPair`: Cryptographic key pair

## Examples

### Basic Certificate Parsing

```cursed
import "stdlib::packages::crypto_pki";

let pem_data = "-----BEGIN CERTIFICATE-----\n...\n-----END CERTIFICATE-----";
let certificate = parse_certificate(pem_data.as_bytes(), Some("pem"))?;

println("Subject: {}", certificate.subject.to_string());
println("Issuer: {}", certificate.issuer.to_string());
println("Valid: {}", certificate.is_currently_valid());
```

### Certificate Chain Validation

```cursed
import "stdlib::packages::crypto_pki";

// Build certificate chain
let chain = CertificateChain {
    end_entity: server_cert,
    intermediates: vec![intermediate_cert],
    root: Some(root_cert),
};

// Validate the chain
let result = validate_certificate_chain(&chain)?;

if result.is_valid {
    println("Certificate chain is valid");
} else {
    for error in result.errors {
        println("Error: {}", error);
    }
}
```

### Creating a Certificate Authority

```cursed
import "stdlib::packages::crypto_pki";

// Configure CA
let ca_config = CaConfig {
    name: "Test CA".to_string(),
    distinguished_name: DistinguishedName::from_common_name("Test CA"),
    signature_algorithm: SignatureAlgorithm::RsaWithSha256,
    // ... other configuration
};

// Create the CA
let ca_id = create_certificate_authority("test_ca".to_string(), ca_config)?;
println("Created CA: {}", ca_id);
```

## Best Practices

### Certificate Management

1. **Use Templates**: Standardize certificate issuance with templates
2. **Limit Validity**: Use appropriate certificate lifetimes
3. **Monitor Expiration**: Track certificate expiration dates
4. **Automate Renewal**: Implement automated certificate renewal

### Security Best Practices

1. **Secure Storage**: Protect private keys with encryption
2. **Access Control**: Implement role-based access control
3. **Audit Trail**: Maintain comprehensive audit logs
4. **Regular Review**: Periodically review issued certificates

### Operational Best Practices

1. **Backup Management**: Maintain secure backups of CA keys
2. **Disaster Recovery**: Plan for CA key compromise scenarios
3. **Performance Monitoring**: Monitor PKI system performance
4. **Capacity Planning**: Plan for certificate volume growth

### Validation Best Practices

1. **Complete Validation**: Always perform full chain validation
2. **Revocation Checking**: Enable real-time revocation checking
3. **Policy Enforcement**: Implement strict validation policies
4. **Error Handling**: Properly handle validation failures

## Troubleshooting

### Common Issues

#### Certificate Parsing Errors

```
Error: Invalid PEM format
Solution: Check PEM headers and base64 encoding
```

#### Chain Validation Failures

```
Error: Certificate chain incomplete
Solution: Ensure all intermediate certificates are provided
```

#### CA Creation Problems

```
Error: Invalid CA configuration
Solution: Verify all required configuration fields
```

### Debugging Tips

1. **Enable Logging**: Use tracing for detailed operation logs
2. **Check Formats**: Verify certificate formats (PEM vs DER)
3. **Validate Dates**: Check certificate validity periods
4. **Review Policies**: Ensure validation policies are appropriate

### Performance Issues

1. **Cache Utilization**: Monitor cache hit rates
2. **Network Timeouts**: Adjust network timeout settings
3. **Memory Usage**: Monitor memory consumption
4. **Concurrent Operations**: Optimize for concurrent access

## Advanced Topics

### Custom Extensions

The PKI system supports custom X.509 extensions:

```cursed
let custom_extension = X509Extension {
    oid: "1.2.3.4.5".to_string(),
    critical: false,
    value: custom_data,
    parsed_data: Some(ExtensionData::Custom(custom_data)),
};
```

### Policy Validation

Implement certificate policy validation:

```cursed
let policy_info = PolicyInformation {
    policy_identifier: "1.2.3.4.5.6".to_string(),
    policy_qualifiers: Some(vec![qualifier]),
};
```

### Hardware Security Modules

Integration with HSMs for enhanced security:

```cursed
let key_provenance = KeyProvenance::Hardware {
    device_id: "HSM-001".to_string(),
    attestation: Some(attestation_data),
};
```

## Conclusion

The CURSED PKI system provides a comprehensive, production-ready implementation of Public Key Infrastructure with extensive features for certificate management, validation, and security. This guide covers the essential aspects of using the PKI system effectively and securely.

For additional information, examples, and updates, refer to the CURSED documentation and community resources.
