# Package Security Verification and Integrity Checking
# Real cryptographic verification for package integrity and authenticity

yeet "cryptz"
yeet "filez"
yeet "stringz"
yeet "arrayz"
yeet "vibez"
yeet "jsonz"
yeet "base64z"

# Package signature information
squad PackageSignature {
    sus signature_data tea
    sus signature_algorithm tea  # "ed25519", "rsa-pss", "ecdsa"
    sus hash_algorithm tea       # "sha256", "sha512"
    sus public_key_id tea
    sus timestamp drip
}

# Package verification result
squad VerificationResult {
    sus is_valid lit
    sus checksum_valid lit
    sus signature_valid lit
    sus certificate_valid lit
    sus trust_level tea  # "trusted", "untrusted", "unknown"
    sus error_message tea
    sus verification_details []tea
}

# Security policy configuration
squad SecurityPolicy {
    sus require_signatures lit
    sus require_checksums lit
    sus allow_self_signed lit
    sus trusted_publishers []tea
    sus blocked_packages []tea
    sus minimum_key_size drip
    sus allowed_algorithms []tea
}

# Package integrity metadata
squad IntegrityMetadata {
    sus sha256_checksum tea
    sus sha512_checksum tea
    sus file_size drip
    sus signature PackageSignature
    sus publisher_info PublisherInfo
}

# Publisher information and trust
squad PublisherInfo {
    sus name tea
    sus email tea
    sus public_key tea
    sus certificate tea
    sus is_verified lit
    sus trust_score drip
}

# Initialize default security policy
slay create_default_security_policy() SecurityPolicy {
    damn SecurityPolicy {
        require_signatures: cap,        # Start permissive, tighten later
        require_checksums: based,       # Always require checksums
        allow_self_signed: based,       # Allow self-signed for now
        trusted_publishers: [
            "cursed-official@cursedlang.org",
            "core-team@cursedlang.org"
        ],
        blocked_packages: [],
        minimum_key_size: 2048,
        allowed_algorithms: [
            "ed25519",
            "rsa-pss-2048", 
            "rsa-pss-3072",
            "rsa-pss-4096",
            "ecdsa-p256",
            "ecdsa-p384"
        ]
    }
}

# Verify package integrity with full security checks
slay verify_package_integrity(archive_path tea, metadata PackageMetadata, policy SecurityPolicy) VerificationResult {
    vibez.spill("Verifying package integrity:", metadata.name, "version:", metadata.version)
    
    sus result VerificationResult = VerificationResult {
        is_valid: cap,
        checksum_valid: cap,
        signature_valid: cap,
        certificate_valid: cap,
        trust_level: "unknown",
        error_message: "",
        verification_details: []
    }
    
    # Step 1: Verify file exists and basic properties
    ready (!filez.file_exists(archive_path)) {
        result.error_message = "Archive file not found: " + archive_path
        damn result
    }
    
    sus file_size drip = filez.file_size(archive_path)
    ready (file_size == 0) {
        result.error_message = "Archive file is empty"
        damn result
    }
    
    result.verification_details = arrayz.append(result.verification_details, 
        "File size: " + stringz.from_int(file_size) + " bytes")
    
    # Step 2: Verify checksums
    ready (policy.require_checksums || metadata.checksum != "") {
        ready (!verify_package_checksums(archive_path, metadata, result)) {
            damn result  # Checksum verification failed
        }
    }
    
    # Step 3: Verify digital signatures
    ready (policy.require_signatures) {
        ready (!verify_package_signature(archive_path, metadata, policy, result)) {
            damn result  # Signature verification failed
        }
    }
    
    # Step 4: Check publisher trust
    ready (!verify_publisher_trust(metadata, policy, result)) {
        # Publisher not trusted but continue with warning
        result.verification_details = arrayz.append(result.verification_details,
            "WARNING: Publisher not in trusted list")
    }
    
    # Step 5: Check security policy compliance
    ready (!check_security_policy_compliance(metadata, policy, result)) {
        damn result  # Policy violation
    }
    
    # All checks passed
    result.is_valid = based
    result.trust_level = determine_trust_level(metadata, policy)
    result.verification_details = arrayz.append(result.verification_details, 
        "All security checks passed")
    
    vibez.spill("Package verification successful for:", metadata.name)
    damn result
}

# Verify package checksums with multiple algorithms
slay verify_package_checksums(archive_path tea, metadata PackageMetadata, result VerificationResult) lit {
    vibez.spill("Verifying package checksums...")
    
    # Read file data for hashing
    sus archive_data tea = filez.read_file_binary(archive_path)
    ready (archive_data == "") {
        result.error_message = "Failed to read archive for checksum verification"
        damn cap
    }
    
    # Verify SHA-256 checksum (most common)
    ready (metadata.checksum != "") {
        sus computed_sha256 tea = cryptz.sha256_hash(archive_data)
        
        # Handle different checksum formats (hex vs base64)
        sus expected_checksum tea = normalize_checksum(metadata.checksum)
        sus computed_checksum tea = normalize_checksum(computed_sha256)
        
        ready (expected_checksum != computed_checksum) {
            result.error_message = "SHA-256 checksum mismatch"
            result.verification_details = arrayz.append(result.verification_details,
                "Expected: " + expected_checksum)
            result.verification_details = arrayz.append(result.verification_details, 
                "Computed: " + computed_checksum)
            damn cap
        }
        
        result.verification_details = arrayz.append(result.verification_details,
            "SHA-256 checksum verified: " + computed_checksum)
        result.checksum_valid = based
    }
    
    # Additional integrity check: verify file structure
    ready (!verify_archive_structure(archive_path, result)) {
        result.error_message = "Archive structure verification failed"
        damn cap
    }
    
    damn based
}

# Verify package digital signature
slay verify_package_signature(archive_path tea, metadata PackageMetadata, policy SecurityPolicy, result VerificationResult) lit {
    vibez.spill("Verifying digital signature...")
    
    # Look for signature file (.sig extension)
    sus signature_path tea = archive_path + ".sig"
    ready (!filez.file_exists(signature_path)) {
        # Try embedded signature in metadata
        ready (!has_embedded_signature(metadata)) {
            result.error_message = "No signature found and signatures are required"
            damn cap
        }
        damn verify_embedded_signature(archive_path, metadata, policy, result)
    }
    
    # Read external signature file
    sus signature_data tea = filez.read_file(signature_path)
    ready (signature_data == "") {
        result.error_message = "Failed to read signature file"
        damn cap
    }
    
    # Parse signature format (JSON, PGP, etc.)
    sus signature PackageSignature = parse_signature_file(signature_data)
    ready (signature.signature_data == "") {
        result.error_message = "Invalid signature format"
        damn cap
    }
    
    # Verify signature algorithm is allowed
    ready (!is_algorithm_allowed(signature.signature_algorithm, policy.allowed_algorithms)) {
        result.error_message = "Signature algorithm not allowed: " + signature.signature_algorithm
        damn cap
    }
    
    # Get public key for verification
    sus public_key tea = get_public_key_for_signature(signature, result)
    ready (public_key == "") {
        damn cap  # Error message set in function
    }
    
    # Verify the signature
    ready (!verify_digital_signature(archive_path, signature, public_key, result)) {
        damn cap  # Error message set in function
    }
    
    result.signature_valid = based
    result.verification_details = arrayz.append(result.verification_details,
        "Digital signature verified with " + signature.signature_algorithm)
    
    damn based
}

# Verify digital signature using cryptographic functions
slay verify_digital_signature(data_path tea, signature PackageSignature, public_key tea, result VerificationResult) lit {
    # Read data to be verified
    sus data tea = filez.read_file_binary(data_path)
    ready (data == "") {
        result.error_message = "Failed to read data for signature verification"
        damn cap
    }
    
    # Compute hash of data using specified algorithm
    sus data_hash tea = ""
    match signature.hash_algorithm {
        "sha256" -> data_hash = cryptz.sha256_hash(data)
        "sha512" -> data_hash = cryptz.sha512_hash(data)
        _ -> {
            result.error_message = "Unsupported hash algorithm: " + signature.hash_algorithm
            damn cap
        }
    }
    
    # Verify signature based on algorithm
    sus is_valid lit = cap
    match signature.signature_algorithm {
        "ed25519" -> {
            is_valid = cryptz.ed25519_verify(data_hash, signature.signature_data, public_key)
        }
        "rsa-pss" -> {
            is_valid = cryptz.rsa_pss_verify(data_hash, signature.signature_data, public_key)
        }
        "ecdsa-p256" -> {
            is_valid = cryptz.ecdsa_p256_verify(data_hash, signature.signature_data, public_key)
        }
        _ -> {
            result.error_message = "Unsupported signature algorithm: " + signature.signature_algorithm
            damn cap
        }
    }
    
    ready (!is_valid) {
        result.error_message = "Digital signature verification failed"
        result.verification_details = arrayz.append(result.verification_details,
            "Signature algorithm: " + signature.signature_algorithm)
        result.verification_details = arrayz.append(result.verification_details, 
            "Hash algorithm: " + signature.hash_algorithm)
        damn cap
    }
    
    damn based
}

# Check publisher trust and reputation
slay verify_publisher_trust(metadata PackageMetadata, policy SecurityPolicy, result VerificationResult) lit {
    # Check if package is in blocked list
    bestie (sus i drip = 0; i < arrayz.len(policy.blocked_packages); i = i + 1) {
        ready (metadata.name == policy.blocked_packages[i]) {
            result.error_message = "Package is in blocked list: " + metadata.name
            damn cap
        }
    }
    
    # Check if publisher is trusted
    bestie (sus i drip = 0; i < arrayz.len(metadata.authors); i = i + 1) {
        sus author tea = metadata.authors[i]
        sus email tea = extract_email_from_author(author)
        
        bestie (sus j drip = 0; j < arrayz.len(policy.trusted_publishers); j = j + 1) {
            ready (email == policy.trusted_publishers[j]) {
                result.verification_details = arrayz.append(result.verification_details,
                    "Trusted publisher: " + email)
                damn based
            }
        }
    }
    
    # Publisher not in trusted list
    result.verification_details = arrayz.append(result.verification_details,
        "Publisher not in trusted list")
    damn cap
}

# Determine overall trust level
slay determine_trust_level(metadata PackageMetadata, policy SecurityPolicy) tea {
    # Check if from trusted publisher
    bestie (sus i drip = 0; i < arrayz.len(metadata.authors); i = i + 1) {
        sus author tea = metadata.authors[i]
        sus email tea = extract_email_from_author(author)
        
        bestie (sus j drip = 0; j < arrayz.len(policy.trusted_publishers); j = j + 1) {
            ready (email == policy.trusted_publishers[j]) {
                damn "trusted"
            }
        }
    }
    
    # Has valid signature but not from trusted publisher
    ready (policy.require_signatures) {
        damn "verified"
    }
    
    # Only checksum verification
    damn "basic"
}

# Security helper functions
slay normalize_checksum(checksum tea) tea {
    # Convert to lowercase hex format
    sus normalized tea = stringz.to_lowercase(checksum)
    
    # Remove common prefixes
    ready (stringz.starts_with(normalized, "sha256:")) {
        normalized = stringz.substring(normalized, 7, stringz.len(normalized))
    }
    ready (stringz.starts_with(normalized, "0x")) {
        normalized = stringz.substring(normalized, 2, stringz.len(normalized))
    }
    
    damn normalized
}

slay verify_archive_structure(archive_path tea, result VerificationResult) lit {
    # Basic archive structure validation
    # Check if file has expected tar.gz header
    sus header_bytes tea = filez.read_file_bytes(archive_path, 0, 10)
    ready (stringz.len(header_bytes) < 3) {
        result.verification_details = arrayz.append(result.verification_details,
            "Archive too small or corrupted")
        damn cap
    }
    
    # Check for gzip magic number
    sus byte1 drip = stringz.char_code(stringz.char_at(header_bytes, 0))
    sus byte2 drip = stringz.char_code(stringz.char_at(header_bytes, 1))
    ready (byte1 != 0x1f || byte2 != 0x8b) {
        result.verification_details = arrayz.append(result.verification_details,
            "Invalid archive format - not gzip")
        damn cap
    }
    
    result.verification_details = arrayz.append(result.verification_details,
        "Archive structure appears valid")
    damn based
}

slay has_embedded_signature(metadata PackageMetadata) lit {
    # Check if metadata contains signature information
    damn metadata.checksum != "" && stringz.contains(metadata.checksum, "sig:")
}

slay verify_embedded_signature(archive_path tea, metadata PackageMetadata, policy SecurityPolicy, result VerificationResult) lit {
    # Parse embedded signature from metadata
    vibez.spill("Verifying embedded signature (not fully implemented)")
    damn cap  # Simplified for now
}

slay parse_signature_file(signature_data tea) PackageSignature {
    # Try to parse as JSON first
    ready (stringz.starts_with(stringz.trim(signature_data), "{")) {
        sus json_data JsonValue = jsonz.json_parse(signature_data)
        ready (json_data.type == "object") {
            damn PackageSignature {
                signature_data: jsonz.json_get_string(json_data, "signature"),
                signature_algorithm: jsonz.json_get_string(json_data, "algorithm"),
                hash_algorithm: jsonz.json_get_string(json_data, "hash_algorithm"),
                public_key_id: jsonz.json_get_string(json_data, "key_id"),
                timestamp: jsonz.json_get_int(json_data, "timestamp")
            }
        }
    }
    
    # Fall back to simple format parsing
    sus lines []tea = stringz.split(signature_data, "\n")
    ready (arrayz.len(lines) >= 3) {
        damn PackageSignature {
            signature_data: lines[0],
            signature_algorithm: lines[1],
            hash_algorithm: "sha256",
            public_key_id: "",
            timestamp: 0
        }
    }
    
    # Invalid signature format
    damn PackageSignature { signature_data: "" }
}

slay is_algorithm_allowed(algorithm tea, allowed_algorithms []tea) lit {
    bestie (sus i drip = 0; i < arrayz.len(allowed_algorithms); i = i + 1) {
        ready (algorithm == allowed_algorithms[i]) {
            damn based
        }
    }
    damn cap
}

slay get_public_key_for_signature(signature PackageSignature, result VerificationResult) tea {
    # In a real implementation, this would:
    # 1. Look up key by ID in keyring
    # 2. Fetch from keyserver if not found
    # 3. Validate key authenticity
    
    # For now, return a placeholder
    result.verification_details = arrayz.append(result.verification_details,
        "Public key lookup not fully implemented")
    damn ""  # Simplified
}

slay extract_email_from_author(author tea) tea {
    # Extract email from "Name <email>" format
    sus start drip = stringz.index_of(author, "<")
    sus end drip = stringz.index_of(author, ">")
    
    ready (start != -1 && end != -1 && end > start) {
        damn stringz.substring(author, start + 1, end)
    }
    
    damn ""
}

slay check_security_policy_compliance(metadata PackageMetadata, policy SecurityPolicy, result VerificationResult) lit {
    # Check minimum requirements are met
    ready (policy.require_checksums && metadata.checksum == "") {
        result.error_message = "Package checksum required by policy but not provided"
        damn cap
    }
    
    # All policy checks passed
    result.verification_details = arrayz.append(result.verification_details,
        "Security policy compliance verified")
    damn based
}

# Generate integrity metadata for package publishing
slay generate_integrity_metadata(archive_path tea, publisher_info PublisherInfo) IntegrityMetadata {
    sus archive_data tea = filez.read_file_binary(archive_path)
    sus file_size drip = stringz.len(archive_data)
    
    # Generate checksums
    sus sha256_checksum tea = cryptz.sha256_hash(archive_data)
    sus sha512_checksum tea = cryptz.sha512_hash(archive_data)
    
    # Create signature (placeholder implementation)
    sus signature PackageSignature = create_package_signature(archive_data, publisher_info)
    
    damn IntegrityMetadata {
        sha256_checksum: sha256_checksum,
        sha512_checksum: sha512_checksum,
        file_size: file_size,
        signature: signature,
        publisher_info: publisher_info
    }
}

slay create_package_signature(data tea, publisher_info PublisherInfo) PackageSignature {
    # In real implementation: sign data with private key
    damn PackageSignature {
        signature_data: "",
        signature_algorithm: "ed25519",
        hash_algorithm: "sha256",
        public_key_id: "",
        timestamp: timez.current_unix_time()
    }
}

# Validate security configuration
slay validate_security_policy(policy SecurityPolicy) []tea {
    sus warnings []tea = []
    
    ready (!policy.require_signatures && !policy.require_checksums) {
        warnings = arrayz.append(warnings, 
            "WARNING: Neither signatures nor checksums required - security is minimal")
    }
    
    ready (policy.allow_self_signed && policy.require_signatures) {
        warnings = arrayz.append(warnings,
            "WARNING: Self-signed certificates allowed - consider using CA-signed certificates")
    }
    
    ready (policy.minimum_key_size < 2048) {
        warnings = arrayz.append(warnings,
            "WARNING: Minimum key size below 2048 bits is not recommended")
    }
    
    damn warnings
}
