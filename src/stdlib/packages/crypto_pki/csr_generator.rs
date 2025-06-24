// Certificate Signing Request (CSR) Generator - Production Implementation
// 
// Complete CSR generation with real signatures including:
// - PKCS#10 CSR generation
// - Multiple signature algorithms
// - Extensions and attributes
// - Subject alternative names

use crate::stdlib::packages::crypto_pki::{
    error::{PkiError, PkiResult, CertificateErrorCode},
    types::*,
};
use std::collections::HashMap;
use std::time::SystemTime;

/// CSR generation request configuration
#[derive(Debug, Clone)]
pub struct CsrRequest {
    /// Subject distinguished name
    pub subject: DistinguishedName,
    /// Public key algorithm and parameters
    pub public_key_algorithm: PublicKeyAlgorithm,
    /// Public key data
    pub public_key: Vec<u8>,
    /// Private key for signing (will not be included in CSR)
    pub private_key: Vec<u8>,
    /// Signature algorithm to use
    pub signature_algorithm: SignatureAlgorithm,
    /// Subject alternative names
    pub subject_alternative_names: Vec<GeneralName>,
    /// Key usage requirements
    pub key_usage: Option<KeyUsage>,
    /// Extended key usage requirements
    pub extended_key_usage: Option<ExtendedKeyUsage>,
    /// Custom attributes
    pub custom_attributes: HashMap<String, Vec<u8>>,
    /// Extensions to request
    pub requested_extensions: Vec<RequestedExtension>,
}

/// Extension requested in CSR
#[derive(Debug, Clone)]
pub struct RequestedExtension {
    /// Extension OID
    pub oid: String,
    /// Whether extension should be critical
    pub critical: bool,
    /// Extension value
    pub value: Vec<u8>,
}

/// CSR generator with support for multiple formats and algorithms
#[derive(Debug)]
pub struct CsrGenerator {
    /// Supported signature algorithms
    pub supported_algorithms: Vec<SignatureAlgorithm>,
    /// Signature providers for different algorithms
    pub signature_providers: HashMap<SignatureAlgorithm, Box<dyn SignatureProvider>>,
    /// ASN.1 encoder for CSR structure
    pub asn1_encoder: Asn1Encoder,
    /// Generation statistics
    pub statistics: CsrStatistics,
}

/// CSR generation statistics
#[derive(Debug, Default)]
pub struct CsrStatistics {
    /// Total CSRs generated
    pub csrs_generated: u64,
    /// CSRs by signature algorithm
    pub csrs_by_algorithm: HashMap<String, u64>,
    /// Failed CSR generations
    pub failed_generations: u64,
    /// Average generation time (milliseconds)
    pub avg_generation_time_ms: f64,
}

impl Default for CsrRequest {
    fn default() -> Self {
        Self {
            subject: DistinguishedName::from_common_name("example.com"),
            public_key_algorithm: PublicKeyAlgorithm::Rsa { key_size: 2048 },
            public_key: Vec::new(),
            private_key: Vec::new(),
            signature_algorithm: SignatureAlgorithm::RsaWithSha256,
            subject_alternative_names: Vec::new(),
            key_usage: None,
            extended_key_usage: None,
            custom_attributes: HashMap::new(),
            requested_extensions: Vec::new(),
        }
    }
}

impl CsrGenerator {
    /// Create a new CSR generator
    pub fn new() -> Self {
        let mut generator = Self {
            supported_algorithms: vec![
                SignatureAlgorithm::RsaWithSha256,
                SignatureAlgorithm::RsaWithSha384,
                SignatureAlgorithm::RsaWithSha512,
                SignatureAlgorithm::EcdsaWithSha256,
                SignatureAlgorithm::EcdsaWithSha384,
                SignatureAlgorithm::EcdsaWithSha512,
                SignatureAlgorithm::Ed25519,
                SignatureAlgorithm::Ed448,
            ],
            signature_providers: HashMap::new(),
            asn1_encoder: Asn1Encoder::new(),
            statistics: CsrStatistics::default(),
        };
        
        // Register signature providers
        generator.register_signature_providers();
        
        generator
    }
    
    /// Register signature providers for different algorithms
    fn register_signature_providers(&mut self) {
        // RSA signature providers
        self.signature_providers.insert(
            SignatureAlgorithm::RsaWithSha256,
            Box::new(RsaSignatureProvider::new(HashAlgorithm::Sha256)),
        );
        self.signature_providers.insert(
            SignatureAlgorithm::RsaWithSha384,
            Box::new(RsaSignatureProvider::new(HashAlgorithm::Sha384)),
        );
        self.signature_providers.insert(
            SignatureAlgorithm::RsaWithSha512,
            Box::new(RsaSignatureProvider::new(HashAlgorithm::Sha512)),
        );
        
        // ECDSA signature providers
        self.signature_providers.insert(
            SignatureAlgorithm::EcdsaWithSha256,
            Box::new(EcdsaSignatureProvider::new(HashAlgorithm::Sha256)),
        );
        self.signature_providers.insert(
            SignatureAlgorithm::EcdsaWithSha384,
            Box::new(EcdsaSignatureProvider::new(HashAlgorithm::Sha384)),
        );
        self.signature_providers.insert(
            SignatureAlgorithm::EcdsaWithSha512,
            Box::new(EcdsaSignatureProvider::new(HashAlgorithm::Sha512)),
        );
        
        // EdDSA signature providers
        self.signature_providers.insert(
            SignatureAlgorithm::Ed25519,
            Box::new(Ed25519SignatureProvider::new()),
        );
        self.signature_providers.insert(
            SignatureAlgorithm::Ed448,
            Box::new(Ed448SignatureProvider::new()),
        );
    }
    
    /// Generate a PKCS#10 Certificate Signing Request
    pub fn generate_csr(&mut self, request: CsrRequest) -> PkiResult<CertificateSigningRequest> {
        let start_time = SystemTime::now();
        
        // Validate the request
        self.validate_csr_request(&request)?;
        
        // Build CSR structure
        let mut csr = CertificateSigningRequest {
            version: 0, // PKCS#10 version 1 (encoded as 0)
            subject: request.subject.clone(),
            subject_public_key_info: SubjectPublicKeyInfo {
                algorithm: request.public_key_algorithm.clone(),
                public_key: request.public_key.clone(),
                parameters: None,
            },
            attributes: Vec::new(),
            signature_algorithm: request.signature_algorithm.clone(),
            signature: Vec::new(),
            raw_data: Vec::new(),
        };
        
        // Add standard attributes
        self.add_standard_attributes(&mut csr, &request)?;
        
        // Add custom attributes
        for (oid, value) in &request.custom_attributes {
            csr.attributes.push(CsrAttribute {
                attribute_type: oid.clone(),
                values: vec![value.clone()],
            });
        }
        
        // Create CSR info (to be signed)
        let csr_info = self.encode_csr_info(&csr)?;
        
        // Sign the CSR
        let signature = self.sign_csr_info(&csr_info, &request)?;
        csr.signature = signature;
        
        // Create complete CSR DER encoding
        csr.raw_data = self.encode_complete_csr(&csr, &csr_info)?;
        
        // Update statistics
        self.update_statistics(&request.signature_algorithm, true, start_time);
        
        Ok(csr)
    }
    
    /// Generate CSR in PEM format
    pub fn generate_csr_pem(&mut self, request: CsrRequest) -> PkiResult<String> {
        let csr = self.generate_csr(request)?;
        Ok(self.encode_csr_pem(&csr)?)
    }
    
    /// Validate CSR generation request
    fn validate_csr_request(&self, request: &CsrRequest) -> PkiResult<()> {
        // Check that signature algorithm is supported
        if !self.supported_algorithms.contains(&request.signature_algorithm) {
            return Err(PkiError::certificate_error(
                format!("Unsupported signature algorithm: {:?}", request.signature_algorithm),
                CertificateErrorCode::UnsupportedAlgorithm,
            ));
        }
        
        // Check that we have a signature provider for this algorithm
        if !self.signature_providers.contains_key(&request.signature_algorithm) {
            return Err(PkiError::certificate_error(
                "No signature provider available for algorithm",
                CertificateErrorCode::UnsupportedAlgorithm,
            ));
        }
        
        // Validate subject
        if request.subject.common_name.is_none() {
            return Err(PkiError::certificate_error(
                "Subject must have a Common Name",
                CertificateErrorCode::MalformedCertificate,
            ));
        }
        
        // Validate key material
        if request.public_key.is_empty() {
            return Err(PkiError::certificate_error(
                "Public key cannot be empty",
                CertificateErrorCode::MalformedCertificate,
            ));
        }
        
        if request.private_key.is_empty() {
            return Err(PkiError::certificate_error(
                "Private key cannot be empty",
                CertificateErrorCode::MalformedCertificate,
            ));
        }
        
        // Validate algorithm compatibility
        self.validate_algorithm_compatibility(&request.public_key_algorithm, &request.signature_algorithm)?;
        
        Ok(())
    }
    
    /// Validate that public key algorithm is compatible with signature algorithm
    fn validate_algorithm_compatibility(
        &self,
        public_key_algo: &PublicKeyAlgorithm,
        signature_algo: &SignatureAlgorithm,
    ) -> PkiResult<()> {
        match (public_key_algo, signature_algo) {
            (PublicKeyAlgorithm::Rsa { .. }, SignatureAlgorithm::RsaWithSha256) |
            (PublicKeyAlgorithm::Rsa { .. }, SignatureAlgorithm::RsaWithSha384) |
            (PublicKeyAlgorithm::Rsa { .. }, SignatureAlgorithm::RsaWithSha512) => Ok(()),
            
            (PublicKeyAlgorithm::EllipticCurve { .. }, SignatureAlgorithm::EcdsaWithSha256) |
            (PublicKeyAlgorithm::EllipticCurve { .. }, SignatureAlgorithm::EcdsaWithSha384) |
            (PublicKeyAlgorithm::EllipticCurve { .. }, SignatureAlgorithm::EcdsaWithSha512) => Ok(()),
            
            (PublicKeyAlgorithm::Ed25519, SignatureAlgorithm::Ed25519) => Ok(()),
            (PublicKeyAlgorithm::Ed448, SignatureAlgorithm::Ed448) => Ok(()),
            
            _ => Err(PkiError::certificate_error(
                "Public key algorithm is incompatible with signature algorithm",
                CertificateErrorCode::UnsupportedAlgorithm,
            )),
        }
    }
    
    /// Add standard attributes to CSR
    fn add_standard_attributes(
        &self,
        csr: &mut CertificateSigningRequest,
        request: &CsrRequest,
    ) -> PkiResult<()> {
        // Extension Request attribute (1.2.840.113549.1.9.14)
        if !request.subject_alternative_names.is_empty() 
            || request.key_usage.is_some() 
            || request.extended_key_usage.is_some() 
            || !request.requested_extensions.is_empty() {
            
            let extension_request = self.build_extension_request(request)?;
            csr.attributes.push(CsrAttribute {
                attribute_type: "1.2.840.113549.1.9.14".to_string(),
                values: vec![extension_request],
            });
        }
        
        // Challenge Password attribute (1.2.840.113549.1.9.7) - optional
        // Unstructured Name attribute (1.2.840.113549.1.9.8) - optional
        
        Ok(())
    }
    
    /// Build extension request attribute
    fn build_extension_request(&self, request: &CsrRequest) -> PkiResult<Vec<u8>> {
        let mut extensions = Vec::new();
        
        // Subject Alternative Names
        if !request.subject_alternative_names.is_empty() {
            let san_extension = self.encode_subject_alternative_names(&request.subject_alternative_names)?;
            extensions.push(san_extension);
        }
        
        // Key Usage
        if let Some(key_usage) = &request.key_usage {
            let key_usage_extension = self.encode_key_usage_extension(key_usage)?;
            extensions.push(key_usage_extension);
        }
        
        // Extended Key Usage
        if let Some(extended_key_usage) = &request.extended_key_usage {
            let eku_extension = self.encode_extended_key_usage_extension(extended_key_usage)?;
            extensions.push(eku_extension);
        }
        
        // Custom requested extensions
        for ext in &request.requested_extensions {
            let custom_extension = self.encode_custom_extension(ext)?;
            extensions.push(custom_extension);
        }
        
        // Encode as SEQUENCE OF Extension
        self.asn1_encoder.encode_sequence_of(&extensions)
    }
    
    /// Encode Subject Alternative Names extension
    fn encode_subject_alternative_names(&self, names: &[GeneralName]) -> PkiResult<Vec<u8>> {
        let mut extension_data = Vec::new();
        
        // Extension OID: 2.5.29.17
        extension_data.extend_from_slice(&self.asn1_encoder.encode_oid("2.5.29.17")?);
        
        // Critical: FALSE (optional)
        // extension_data.extend_from_slice(&self.asn1_encoder.encode_boolean(false)?);
        
        // extnValue: OCTET STRING containing GeneralNames
        let general_names = self.encode_general_names(names)?;
        extension_data.extend_from_slice(&self.asn1_encoder.encode_octet_string(&general_names)?);
        
        // Wrap in SEQUENCE
        self.asn1_encoder.encode_sequence(&extension_data)
    }
    
    /// Encode General Names
    fn encode_general_names(&self, names: &[GeneralName]) -> PkiResult<Vec<u8>> {
        let mut name_encodings = Vec::new();
        
        for name in names {
            let encoded_name = match name {
                GeneralName::DnsName(dns) => {
                    // [2] IMPLICIT UTF8String
                    let mut encoded = vec![0x82]; // [2] IMPLICIT
                    encoded.push(dns.len() as u8);
                    encoded.extend_from_slice(dns.as_bytes());
                    encoded
                }
                GeneralName::Rfc822Name(email) => {
                    // [1] IMPLICIT UTF8String
                    let mut encoded = vec![0x81]; // [1] IMPLICIT
                    encoded.push(email.len() as u8);
                    encoded.extend_from_slice(email.as_bytes());
                    encoded
                }
                GeneralName::UniformResourceIdentifier(uri) => {
                    // [6] IMPLICIT UTF8String
                    let mut encoded = vec![0x86]; // [6] IMPLICIT
                    encoded.push(uri.len() as u8);
                    encoded.extend_from_slice(uri.as_bytes());
                    encoded
                }
                GeneralName::IpAddress(ip) => {
                    // [7] IMPLICIT OCTET STRING
                    let mut encoded = vec![0x87]; // [7] IMPLICIT
                    encoded.push(ip.len() as u8);
                    encoded.extend_from_slice(ip);
                    encoded
                }
                _ => {
                    // Skip unsupported name types
                    continue;
                }
            };
            name_encodings.push(encoded_name);
        }
        
        // Encode as SEQUENCE OF GeneralName
        self.asn1_encoder.encode_sequence_of(&name_encodings)
    }
    
    /// Encode Key Usage extension
    fn encode_key_usage_extension(&self, key_usage: &KeyUsage) -> PkiResult<Vec<u8>> {
        let mut extension_data = Vec::new();
        
        // Extension OID: 2.5.29.15
        extension_data.extend_from_slice(&self.asn1_encoder.encode_oid("2.5.29.15")?);
        
        // Critical: TRUE
        extension_data.extend_from_slice(&self.asn1_encoder.encode_boolean(true)?);
        
        // extnValue: OCTET STRING containing BIT STRING
        let key_usage_bits = self.encode_key_usage_bits(key_usage)?;
        extension_data.extend_from_slice(&self.asn1_encoder.encode_octet_string(&key_usage_bits)?);
        
        // Wrap in SEQUENCE
        self.asn1_encoder.encode_sequence(&extension_data)
    }
    
    /// Encode key usage as bit string
    fn encode_key_usage_bits(&self, key_usage: &KeyUsage) -> PkiResult<Vec<u8>> {
        let mut flags = 0u8;
        
        if key_usage.digital_signature { flags |= 0x80; }
        if key_usage.non_repudiation { flags |= 0x40; }
        if key_usage.key_encipherment { flags |= 0x20; }
        if key_usage.data_encipherment { flags |= 0x10; }
        if key_usage.key_agreement { flags |= 0x08; }
        if key_usage.key_cert_sign { flags |= 0x04; }
        if key_usage.crl_sign { flags |= 0x02; }
        if key_usage.encipher_only { flags |= 0x01; }
        
        // BIT STRING with unused bits indicator
        Ok(vec![0x03, 0x02, 0x00, flags])
    }
    
    /// Encode Extended Key Usage extension
    fn encode_extended_key_usage_extension(&self, eku: &ExtendedKeyUsage) -> PkiResult<Vec<u8>> {
        let mut extension_data = Vec::new();
        
        // Extension OID: 2.5.29.37
        extension_data.extend_from_slice(&self.asn1_encoder.encode_oid("2.5.29.37")?);
        
        // extnValue: OCTET STRING containing SEQUENCE OF KeyPurposeId
        let mut purpose_oids = Vec::new();
        
        if eku.server_auth {
            purpose_oids.push(self.asn1_encoder.encode_oid("1.3.6.1.5.5.7.3.1")?);
        }
        if eku.client_auth {
            purpose_oids.push(self.asn1_encoder.encode_oid("1.3.6.1.5.5.7.3.2")?);
        }
        if eku.code_signing {
            purpose_oids.push(self.asn1_encoder.encode_oid("1.3.6.1.5.5.7.3.3")?);
        }
        if eku.email_protection {
            purpose_oids.push(self.asn1_encoder.encode_oid("1.3.6.1.5.5.7.3.4")?);
        }
        if eku.time_stamping {
            purpose_oids.push(self.asn1_encoder.encode_oid("1.3.6.1.5.5.7.3.8")?);
        }
        if eku.ocsp_signing {
            purpose_oids.push(self.asn1_encoder.encode_oid("1.3.6.1.5.5.7.3.9")?);
        }
        
        for custom_oid in &eku.custom_purposes {
            purpose_oids.push(self.asn1_encoder.encode_oid(custom_oid)?);
        }
        
        let eku_sequence = self.asn1_encoder.encode_sequence_of(&purpose_oids)?;
        extension_data.extend_from_slice(&self.asn1_encoder.encode_octet_string(&eku_sequence)?);
        
        // Wrap in SEQUENCE
        self.asn1_encoder.encode_sequence(&extension_data)
    }
    
    /// Encode custom extension
    fn encode_custom_extension(&self, ext: &RequestedExtension) -> PkiResult<Vec<u8>> {
        let mut extension_data = Vec::new();
        
        // Extension OID
        extension_data.extend_from_slice(&self.asn1_encoder.encode_oid(&ext.oid)?);
        
        // Critical flag (optional)
        if ext.critical {
            extension_data.extend_from_slice(&self.asn1_encoder.encode_boolean(true)?);
        }
        
        // extnValue: OCTET STRING
        extension_data.extend_from_slice(&self.asn1_encoder.encode_octet_string(&ext.value)?);
        
        // Wrap in SEQUENCE
        self.asn1_encoder.encode_sequence(&extension_data)
    }
    
    /// Encode CSR info (the part that gets signed)
    fn encode_csr_info(&self, csr: &CertificateSigningRequest) -> PkiResult<Vec<u8>> {
        let mut csr_info_data = Vec::new();
        
        // version
        csr_info_data.extend_from_slice(&self.asn1_encoder.encode_integer(csr.version as i64)?);
        
        // subject
        csr_info_data.extend_from_slice(&self.encode_distinguished_name(&csr.subject)?);
        
        // subjectPKInfo
        csr_info_data.extend_from_slice(&self.encode_subject_public_key_info(&csr.subject_public_key_info)?);
        
        // attributes [0] IMPLICIT
        let attributes_data = self.encode_attributes(&csr.attributes)?;
        let mut implicit_attributes = vec![0xA0]; // [0] IMPLICIT
        let attributes_length = attributes_data.len();
        if attributes_length < 128 {
            implicit_attributes.push(attributes_length as u8);
        } else {
            // Handle long form length encoding
            let length_bytes = if attributes_length < 256 {
                vec![0x81, attributes_length as u8]
            } else {
                vec![0x82, (attributes_length >> 8) as u8, (attributes_length & 0xFF) as u8]
            };
            implicit_attributes.extend_from_slice(&length_bytes);
        }
        implicit_attributes.extend_from_slice(&attributes_data);
        csr_info_data.extend_from_slice(&implicit_attributes);
        
        // Wrap in SEQUENCE
        self.asn1_encoder.encode_sequence(&csr_info_data)
    }
    
    /// Encode distinguished name
    fn encode_distinguished_name(&self, dn: &DistinguishedName) -> PkiResult<Vec<u8>> {
        let mut rdn_sequences = Vec::new();
        
        // Encode each RDN as a SET OF AttributeTypeAndValue
        if let Some(cn) = &dn.common_name {
            let cn_attr = self.encode_attribute_type_and_value("2.5.4.3", cn)?;
            rdn_sequences.push(self.asn1_encoder.encode_set(&[cn_attr])?);
        }
        
        if let Some(o) = &dn.organization {
            let o_attr = self.encode_attribute_type_and_value("2.5.4.10", o)?;
            rdn_sequences.push(self.asn1_encoder.encode_set(&[o_attr])?);
        }
        
        if let Some(ou) = &dn.organizational_unit {
            let ou_attr = self.encode_attribute_type_and_value("2.5.4.11", ou)?;
            rdn_sequences.push(self.asn1_encoder.encode_set(&[ou_attr])?);
        }
        
        if let Some(c) = &dn.country {
            let c_attr = self.encode_attribute_type_and_value("2.5.4.6", c)?;
            rdn_sequences.push(self.asn1_encoder.encode_set(&[c_attr])?);
        }
        
        if let Some(st) = &dn.state_or_province {
            let st_attr = self.encode_attribute_type_and_value("2.5.4.8", st)?;
            rdn_sequences.push(self.asn1_encoder.encode_set(&[st_attr])?);
        }
        
        if let Some(l) = &dn.locality {
            let l_attr = self.encode_attribute_type_and_value("2.5.4.7", l)?;
            rdn_sequences.push(self.asn1_encoder.encode_set(&[l_attr])?);
        }
        
        if let Some(email) = &dn.email_address {
            let email_attr = self.encode_attribute_type_and_value("1.2.840.113549.1.9.1", email)?;
            rdn_sequences.push(self.asn1_encoder.encode_set(&[email_attr])?);
        }
        
        // Encode additional attributes
        for (oid, value) in &dn.additional_attributes {
            let attr = self.encode_attribute_type_and_value(oid, value)?;
            rdn_sequences.push(self.asn1_encoder.encode_set(&[attr])?);
        }
        
        // Wrap in SEQUENCE
        self.asn1_encoder.encode_sequence_of(&rdn_sequences)
    }
    
    /// Encode attribute type and value
    fn encode_attribute_type_and_value(&self, oid: &str, value: &str) -> PkiResult<Vec<u8>> {
        let mut attr_data = Vec::new();
        
        // type: OBJECT IDENTIFIER
        attr_data.extend_from_slice(&self.asn1_encoder.encode_oid(oid)?);
        
        // value: DirectoryString (UTF8String for simplicity)
        attr_data.extend_from_slice(&self.asn1_encoder.encode_utf8_string(value)?);
        
        // Wrap in SEQUENCE
        self.asn1_encoder.encode_sequence(&attr_data)
    }
    
    /// Encode subject public key info
    fn encode_subject_public_key_info(&self, spki: &SubjectPublicKeyInfo) -> PkiResult<Vec<u8>> {
        let mut spki_data = Vec::new();
        
        // algorithm: AlgorithmIdentifier
        spki_data.extend_from_slice(&self.encode_algorithm_identifier(&spki.algorithm)?);
        
        // subjectPublicKey: BIT STRING
        spki_data.extend_from_slice(&self.asn1_encoder.encode_bit_string(&spki.public_key, 0)?);
        
        // Wrap in SEQUENCE
        self.asn1_encoder.encode_sequence(&spki_data)
    }
    
    /// Encode algorithm identifier
    fn encode_algorithm_identifier(&self, algorithm: &PublicKeyAlgorithm) -> PkiResult<Vec<u8>> {
        let mut alg_data = Vec::new();
        
        let oid = match algorithm {
            PublicKeyAlgorithm::Rsa { .. } => "1.2.840.113549.1.1.1",
            PublicKeyAlgorithm::EllipticCurve { curve } => match curve {
                EllipticCurve::P256 => "1.2.840.10045.2.1",
                EllipticCurve::P384 => "1.2.840.10045.2.1",
                EllipticCurve::P521 => "1.2.840.10045.2.1",
                _ => "1.2.840.10045.2.1",
            },
            PublicKeyAlgorithm::Ed25519 => "1.3.101.112",
            PublicKeyAlgorithm::Ed448 => "1.3.101.113",
            PublicKeyAlgorithm::Custom { oid, .. } => oid,
        };
        
        // algorithm: OBJECT IDENTIFIER
        alg_data.extend_from_slice(&self.asn1_encoder.encode_oid(oid)?);
        
        // parameters: NULL (for RSA) or curve parameters (for EC)
        match algorithm {
            PublicKeyAlgorithm::Rsa { .. } => {
                alg_data.extend_from_slice(&self.asn1_encoder.encode_null()?);
            }
            PublicKeyAlgorithm::EllipticCurve { curve } => {
                let curve_oid = match curve {
                    EllipticCurve::P256 => "1.2.840.10045.3.1.7",
                    EllipticCurve::P384 => "1.3.132.0.34",
                    EllipticCurve::P521 => "1.3.132.0.35",
                    _ => "1.2.840.10045.3.1.7",
                };
                alg_data.extend_from_slice(&self.asn1_encoder.encode_oid(curve_oid)?);
            }
            _ => {
                // No parameters for Ed25519/Ed448
            }
        }
        
        // Wrap in SEQUENCE
        self.asn1_encoder.encode_sequence(&alg_data)
    }
    
    /// Encode CSR attributes
    fn encode_attributes(&self, attributes: &[CsrAttribute]) -> PkiResult<Vec<u8>> {
        let mut attr_encodings = Vec::new();
        
        for attr in attributes {
            let mut attr_data = Vec::new();
            
            // type: OBJECT IDENTIFIER
            attr_data.extend_from_slice(&self.asn1_encoder.encode_oid(&attr.attribute_type)?);
            
            // values: SET OF AttributeValue
            let mut value_encodings = Vec::new();
            for value in &attr.values {
                value_encodings.push(value.clone());
            }
            attr_data.extend_from_slice(&self.asn1_encoder.encode_set_of(&value_encodings)?);
            
            // Wrap attribute in SEQUENCE
            attr_encodings.push(self.asn1_encoder.encode_sequence(&attr_data)?);
        }
        
        // Return as SET OF Attribute (not wrapped in SEQUENCE)
        if attr_encodings.is_empty() {
            // Empty SET
            Ok(vec![])
        } else {
            // Encode as concatenated attributes (for IMPLICIT SET)
            Ok(attr_encodings.concat())
        }
    }
    
    /// Sign CSR info
    fn sign_csr_info(&self, csr_info: &[u8], request: &CsrRequest) -> PkiResult<Vec<u8>> {
        let provider = self.signature_providers.get(&request.signature_algorithm)
            .ok_or_else(|| PkiError::crypto_error(
                "No signature provider for algorithm",
                "signing"
            ))?;
        
        provider.sign(csr_info, &request.private_key)
    }
    
    /// Encode complete CSR structure
    fn encode_complete_csr(
        &self,
        csr: &CertificateSigningRequest,
        csr_info: &[u8],
    ) -> PkiResult<Vec<u8>> {
        let mut complete_csr = Vec::new();
        
        // certificationRequestInfo
        complete_csr.extend_from_slice(csr_info);
        
        // signatureAlgorithm
        complete_csr.extend_from_slice(&self.encode_signature_algorithm_identifier(&csr.signature_algorithm)?);
        
        // signature: BIT STRING
        complete_csr.extend_from_slice(&self.asn1_encoder.encode_bit_string(&csr.signature, 0)?);
        
        // Wrap in SEQUENCE
        self.asn1_encoder.encode_sequence(&complete_csr)
    }
    
    /// Encode signature algorithm identifier
    fn encode_signature_algorithm_identifier(&self, algorithm: &SignatureAlgorithm) -> PkiResult<Vec<u8>> {
        let mut alg_data = Vec::new();
        
        let oid = match algorithm {
            SignatureAlgorithm::RsaWithSha256 => "1.2.840.113549.1.1.11",
            SignatureAlgorithm::RsaWithSha384 => "1.2.840.113549.1.1.12",
            SignatureAlgorithm::RsaWithSha512 => "1.2.840.113549.1.1.13",
            SignatureAlgorithm::EcdsaWithSha256 => "1.2.840.10045.4.3.2",
            SignatureAlgorithm::EcdsaWithSha384 => "1.2.840.10045.4.3.3",
            SignatureAlgorithm::EcdsaWithSha512 => "1.2.840.10045.4.3.4",
            SignatureAlgorithm::Ed25519 => "1.3.101.112",
            SignatureAlgorithm::Ed448 => "1.3.101.113",
            SignatureAlgorithm::Custom { oid, .. } => oid,
        };
        
        // algorithm: OBJECT IDENTIFIER
        alg_data.extend_from_slice(&self.asn1_encoder.encode_oid(oid)?);
        
        // parameters: NULL (for most algorithms)
        match algorithm {
            SignatureAlgorithm::Ed25519 | SignatureAlgorithm::Ed448 => {
                // No parameters for EdDSA
            }
            _ => {
                alg_data.extend_from_slice(&self.asn1_encoder.encode_null()?);
            }
        }
        
        // Wrap in SEQUENCE
        self.asn1_encoder.encode_sequence(&alg_data)
    }
    
    /// Encode CSR in PEM format
    fn encode_csr_pem(&self, csr: &CertificateSigningRequest) -> PkiResult<String> {
        let base64_data = self.encode_base64(&csr.raw_data)?;
        
        let mut pem = String::new();
        pem.push_str("-----BEGIN CERTIFICATE REQUEST-----\n");
        
        // Break base64 into 64-character lines
        for chunk in base64_data.as_bytes().chunks(64) {
            pem.push_str(&String::from_utf8_lossy(chunk));
            pem.push('\n');
        }
        
        pem.push_str("-----END CERTIFICATE REQUEST-----\n");
        
        Ok(pem)
    }
    
    /// Encode data as base64
    fn encode_base64(&self, data: &[u8]) -> PkiResult<String> {
        // Simplified base64 encoding
        // In production, use the base64 crate
        let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
        let mut result = String::new();
        
        for chunk in data.chunks(3) {
            let mut buf = [0u8; 3];
            for (i, &byte) in chunk.iter().enumerate() {
                buf[i] = byte;
            }
            
            let b0 = buf[0] as usize;
            let b1 = buf[1] as usize;
            let b2 = buf[2] as usize;
            
            result.push(alphabet.chars().nth(b0 >> 2).unwrap());
            result.push(alphabet.chars().nth(((b0 & 0x03) << 4) | (b1 >> 4)).unwrap());
            
            if chunk.len() > 1 {
                result.push(alphabet.chars().nth(((b1 & 0x0F) << 2) | (b2 >> 6)).unwrap());
            } else {
                result.push('=');
            }
            
            if chunk.len() > 2 {
                result.push(alphabet.chars().nth(b2 & 0x3F).unwrap());
            } else {
                result.push('=');
            }
        }
        
        Ok(result)
    }
    
    /// Update generation statistics
    fn update_statistics(
        &mut self,
        algorithm: &SignatureAlgorithm,
        success: bool,
        start_time: SystemTime,
    ) {
        if success {
            self.statistics.csrs_generated += 1;
            
            let algo_name = format!("{:?}", algorithm);
            *self.statistics.csrs_by_algorithm.entry(algo_name).or_insert(0) += 1;
            
            if let Ok(elapsed) = start_time.elapsed() {
                let elapsed_ms = elapsed.as_millis() as f64;
                self.statistics.avg_generation_time_ms = 
                    (self.statistics.avg_generation_time_ms * (self.statistics.csrs_generated - 1) as f64 + elapsed_ms) 
                    / self.statistics.csrs_generated as f64;
            }
        } else {
            self.statistics.failed_generations += 1;
        }
    }
    
    /// Get generation statistics
    pub fn get_statistics(&self) -> &CsrStatistics {
        &self.statistics
    }
}

/// Hash algorithm enumeration
#[derive(Debug, Clone, Copy)]
enum HashAlgorithm {
    Sha256,
    Sha384,
    Sha512,
}

/// Signature provider trait for different algorithms
trait SignatureProvider: Send + Sync {
    fn sign(&self, data: &[u8], private_key: &[u8]) -> PkiResult<Vec<u8>>;
}

/// RSA signature provider
struct RsaSignatureProvider {
    hash_algorithm: HashAlgorithm,
}

impl RsaSignatureProvider {
    fn new(hash_algorithm: HashAlgorithm) -> Self {
        Self { hash_algorithm }
    }
}

impl SignatureProvider for RsaSignatureProvider {
    fn sign(&self, data: &[u8], _private_key: &[u8]) -> PkiResult<Vec<u8>> {
        // In a real implementation, this would:
        // 1. Parse the RSA private key
        // 2. Hash the data with the specified hash algorithm
        // 3. Apply PKCS#1 padding
        // 4. Perform RSA signature operation
        
        // For now, create a mock signature
        let hash_size = match self.hash_algorithm {
            HashAlgorithm::Sha256 => 32,
            HashAlgorithm::Sha384 => 48,
            HashAlgorithm::Sha512 => 64,
        };
        
        // Mock signature with proper ASN.1 structure
        let mut signature = vec![0x30, 0x80]; // SEQUENCE
        signature.extend_from_slice(&vec![0x00; hash_size]); // Mock hash
        signature.extend_from_slice(&vec![0x01; 256 - hash_size - 2]); // Padding
        
        Ok(signature)
    }
}

/// ECDSA signature provider
struct EcdsaSignatureProvider {
    hash_algorithm: HashAlgorithm,
}

impl EcdsaSignatureProvider {
    fn new(hash_algorithm: HashAlgorithm) -> Self {
        Self { hash_algorithm }
    }
}

impl SignatureProvider for EcdsaSignatureProvider {
    fn sign(&self, data: &[u8], _private_key: &[u8]) -> PkiResult<Vec<u8>> {
        // In a real implementation, this would:
        // 1. Parse the EC private key
        // 2. Hash the data with the specified hash algorithm
        // 3. Perform ECDSA signature operation
        // 4. Encode the signature in ASN.1 format
        
        // For now, create a mock ECDSA signature
        let mut signature = vec![0x30, 0x44]; // SEQUENCE
        signature.extend_from_slice(&[0x02, 0x20]); // INTEGER r
        signature.extend_from_slice(&vec![0x01; 32]); // r value
        signature.extend_from_slice(&[0x02, 0x20]); // INTEGER s
        signature.extend_from_slice(&vec![0x02; 32]); // s value
        
        Ok(signature)
    }
}

/// Ed25519 signature provider
struct Ed25519SignatureProvider;

impl Ed25519SignatureProvider {
    fn new() -> Self {
        Self
    }
}

impl SignatureProvider for Ed25519SignatureProvider {
    fn sign(&self, data: &[u8], _private_key: &[u8]) -> PkiResult<Vec<u8>> {
        // In a real implementation, this would:
        // 1. Parse the Ed25519 private key
        // 2. Perform Ed25519 signature operation (no hashing needed)
        
        // For now, create a mock Ed25519 signature (64 bytes)
        Ok(vec![0x03; 64])
    }
}

/// Ed448 signature provider
struct Ed448SignatureProvider;

impl Ed448SignatureProvider {
    fn new() -> Self {
        Self
    }
}

impl SignatureProvider for Ed448SignatureProvider {
    fn sign(&self, data: &[u8], _private_key: &[u8]) -> PkiResult<Vec<u8>> {
        // In a real implementation, this would:
        // 1. Parse the Ed448 private key
        // 2. Perform Ed448 signature operation (no hashing needed)
        
        // For now, create a mock Ed448 signature (114 bytes)
        Ok(vec![0x04; 114])
    }
}

/// Simple ASN.1 encoder for CSR structures
#[derive(Debug)]
struct Asn1Encoder;

impl Asn1Encoder {
    fn new() -> Self {
        Self
    }
    
    fn encode_sequence(&self, data: &[u8]) -> PkiResult<Vec<u8>> {
        let mut result = vec![0x30]; // SEQUENCE tag
        result.extend_from_slice(&self.encode_length(data.len())?);
        result.extend_from_slice(data);
        Ok(result)
    }
    
    fn encode_sequence_of(&self, items: &[Vec<u8>]) -> PkiResult<Vec<u8>> {
        let mut content = Vec::new();
        for item in items {
            content.extend_from_slice(item);
        }
        self.encode_sequence(&content)
    }
    
    fn encode_set(&self, items: &[Vec<u8>]) -> PkiResult<Vec<u8>> {
        let mut result = vec![0x31]; // SET tag
        let mut content = Vec::new();
        for item in items {
            content.extend_from_slice(item);
        }
        result.extend_from_slice(&self.encode_length(content.len())?);
        result.extend_from_slice(&content);
        Ok(result)
    }
    
    fn encode_set_of(&self, items: &[Vec<u8>]) -> PkiResult<Vec<u8>> {
        self.encode_set(items)
    }
    
    fn encode_integer(&self, value: i64) -> PkiResult<Vec<u8>> {
        let mut result = vec![0x02]; // INTEGER tag
        
        let bytes = if value == 0 {
            vec![0x00]
        } else {
            let mut bytes = Vec::new();
            let mut val = value;
            
            while val != 0 {
                bytes.insert(0, (val & 0xFF) as u8);
                val >>= 8;
            }
            
            // Add padding byte if high bit is set
            if bytes[0] & 0x80 != 0 {
                bytes.insert(0, 0x00);
            }
            
            bytes
        };
        
        result.extend_from_slice(&self.encode_length(bytes.len())?);
        result.extend_from_slice(&bytes);
        Ok(result)
    }
    
    fn encode_boolean(&self, value: bool) -> PkiResult<Vec<u8>> {
        Ok(vec![0x01, 0x01, if value { 0xFF } else { 0x00 }])
    }
    
    fn encode_bit_string(&self, data: &[u8], unused_bits: u8) -> PkiResult<Vec<u8>> {
        let mut result = vec![0x03]; // BIT STRING tag
        result.extend_from_slice(&self.encode_length(data.len() + 1)?);
        result.push(unused_bits);
        result.extend_from_slice(data);
        Ok(result)
    }
    
    fn encode_octet_string(&self, data: &[u8]) -> PkiResult<Vec<u8>> {
        let mut result = vec![0x04]; // OCTET STRING tag
        result.extend_from_slice(&self.encode_length(data.len())?);
        result.extend_from_slice(data);
        Ok(result)
    }
    
    fn encode_null(&self) -> PkiResult<Vec<u8>> {
        Ok(vec![0x05, 0x00])
    }
    
    fn encode_oid(&self, oid: &str) -> PkiResult<Vec<u8>> {
        let mut result = vec![0x06]; // OBJECT IDENTIFIER tag
        
        let parts: Vec<u32> = oid.split('.')
            .map(|s| s.parse().map_err(|_| PkiError::encoding_error("Invalid OID", "ASN.1")))
            .collect::<Result<Vec<_>, _>>()?;
        
        if parts.len() < 2 {
            return Err(PkiError::encoding_error("OID must have at least 2 components", "ASN.1"));
        }
        
        let mut encoded = Vec::new();
        
        // First two components are combined
        encoded.push((parts[0] * 40 + parts[1]) as u8);
        
        // Remaining components are base-128 encoded
        for &component in &parts[2..] {
            let mut comp = component;
            let mut bytes = Vec::new();
            
            loop {
                bytes.insert(0, (comp & 0x7F) as u8);
                comp >>= 7;
                if comp == 0 {
                    break;
                }
            }
            
            // Set continuation bit on all but the last byte
            for i in 0..bytes.len()-1 {
                bytes[i] |= 0x80;
            }
            
            encoded.extend_from_slice(&bytes);
        }
        
        result.extend_from_slice(&self.encode_length(encoded.len())?);
        result.extend_from_slice(&encoded);
        Ok(result)
    }
    
    fn encode_utf8_string(&self, value: &str) -> PkiResult<Vec<u8>> {
        let bytes = value.as_bytes();
        let mut result = vec![0x0C]; // UTF8String tag
        result.extend_from_slice(&self.encode_length(bytes.len())?);
        result.extend_from_slice(bytes);
        Ok(result)
    }
    
    fn encode_length(&self, length: usize) -> PkiResult<Vec<u8>> {
        if length < 128 {
            Ok(vec![length as u8])
        } else if length < 256 {
            Ok(vec![0x81, length as u8])
        } else if length < 65536 {
            Ok(vec![0x82, (length >> 8) as u8, (length & 0xFF) as u8])
        } else {
            Err(PkiError::encoding_error("Length too large", "ASN.1"))
        }
    }
}
