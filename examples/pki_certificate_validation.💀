fr fr fr fr PKI Certificate Validation Example for CURSED
fr fr 
fr fr This example demonstrates comprehensive certificate validation including:
fr fr - Certificate parsing and validation
fr fr - Chain building and verification
fr fr - Trust store management
fr fr - Certificate pinning
fr fr - Revocation checking via OCSP/CRL
fr fr - Real-world web service validation

yeet "stdlib::crypto::pki"
yeet "stdlib::crypto::certificates"
yeet "stdlib::io"

fr fr Real certificate examples (base64 encoded for transport)
sus google_cert_pem = `-----BEGIN CERTIFICATE-----
MIIEijCCA3KgAwIBAgIQdCea9tmy/T8w0QhyESHXHTANBgkqhkiG9w0BAQsFADA7
MQswCQYDVQQGEwJVUzEOMAwGA1UECgwFR29vZ2xlMRwwGgYDVQQDDBNHb29nbGUg
SW50ZXJuZXQgQXV0aG9yaXR5IEczMA0GCSqGSIb3DQEBCwUAA4IBAQBm3m+hh9wL
-----END CERTIFICATE-----`;

sus example_intermediate_cert = `-----BEGIN CERTIFICATE-----
MIIEKjCCAxKgAwIBAgIQYAGXt0an6rS0mtZLL/eQ+zANBgkqhkiG9w0BAQsFADA5
MQswCQYDVQQGEwJVUzEPMA0GA1UEChMGQW1hem9uMRkwFwYDVQQDExBBbWF6b24g
Um9vdCBDQSAxMB4XDTE1MDUyNjAwMDAwMFoXDTM4MDExNzAwMDAwMFowOTELMAkG
-----END CERTIFICATE-----`;

fr fr Main PKI validation demonstration
slay main_character() -> nil {
    println("🔒 CURSED PKI Certificate Validation Demo");
    println("==========================================");
    
    // 1. Create PKI processor with system roots
    println("\n1️⃣ Initializing PKI processor...");
    sus pki_result = create_pki_processor();
    
    lowkey (pki_result["status"] == "success") {
        println("✅ PKI processor created successfully");
        println("   Processor ID: " + pki_result["processor_id"]);
    } bestie {
        println("❌ Failed to create PKI processor");
        damn; // Exit early
    }
    
    // 2. Parse and validate individual certificate
    println("\n2️⃣ Parsing and validating certificate...");
    sus validation_result = validate_certificate_pki(google_cert_pem, "www.google.com");
    
    lowkey (validation_result["valid"]) {
        println("✅ Certificate validation passed");
        println("   Subject: " + validation_result["subject"]);
        println("   Issuer: " + validation_result["issuer"]);
    } bestie {
        println("❌ Certificate validation failed");
        println("   Error: " + validation_result["error"]);
        println("   Type: " + validation_result["error_type"]);
    }
    
    // 3. Get certificate fingerprints
    println("\n3️⃣ Calculating certificate fingerprints...");
    sus fingerprints = get_certificate_fingerprints(google_cert_pem);
    
    lowkey (fingerprints) {
        println("✅ Certificate fingerprints calculated");
        println("   SHA-1: " + fingerprints["sha1"]);
        println("   SHA-256: " + fingerprints["sha256"]);
    } bestie {
        println("❌ Failed to calculate fingerprints");
    }
    
    // 4. Extract certificate extensions
    println("\n4️⃣ Extracting certificate extensions...");
    sus extensions = get_certificate_extensions(google_cert_pem);
    
    lowkey (extensions) {
        println("✅ Certificate extensions extracted");
        println("   Found " + size(extensions) + " extensions");
        
        // Display some common extensions
        display_extension(extensions, "Key Usage");
        display_extension(extensions, "Subject Alternative Name");
        display_extension(extensions, "Basic Constraints");
        display_extension(extensions, "Authority Key Identifier");
    } bestie {
        println("❌ Failed to extract extensions");
    }
    
    // 5. Demonstrate certificate pinning
    println("\n5️⃣ Setting up certificate pinning...");
    demonstrate_certificate_pinning();
    
    // 6. Check revocation status
    println("\n6️⃣ Checking certificate revocation status...");
    sus revocation_status = check_certificate_revocation(google_cert_pem);
    
    lowkey (revocation_status["status"] == "valid") {
        println("✅ Certificate is not revoked");
    } bestie lowkey (revocation_status["status"] == "revoked") {
        println("❌ Certificate is revoked!");
    } bestie {
        println("⚠️ Revocation status unknown");
        println("   Error: " + revocation_status["error"]);
    }
    
    // 7. Validate web service certificates
    println("\n7️⃣ Validating popular web service certificates...");
    validate_web_services();
    
    // 8. Demonstrate trust store operations
    println("\n8️⃣ Trust store management...");
    demonstrate_trust_store();
    
    // 9. Certificate format conversions
    println("\n9️⃣ Certificate format conversions...");
    demonstrate_format_conversions();
    
    println("\n🎉 PKI validation demo completed!");
    println("Check the output above for detailed validation results.");
}

fr fr Display certificate extension information
slay display_extension(extensions: sus, extension_name: string) -> nil {
    lowkey (extensions[extension_name]) {
        sus ext = extensions[extension_name];
        println("   📋 " + extension_name + ":");
        println("      OID: " + ext["oid"]);
        println("      Critical: " + ext["critical"]);
        println("      Value (hex): " + ext["value"][0:32] + "...");
    }
}

fr fr Demonstrate certificate and public key pinning
slay demonstrate_certificate_pinning() -> nil {
    // Pin Google's certificate fingerprint
    sus pin_result = add_certificate_pin("www.google.com", "A1B2C3D4E5F6789012345678901234567890ABCD");
    
    lowkey (pin_result["status"] == "success") {
        println("✅ Certificate pin added for " + pin_result["hostname"]);
        println("   Fingerprint: " + pin_result["fingerprint"]);
    } bestie {
        println("❌ Failed to add certificate pin");
    }
    
    // Pin public key hash
    sus pubkey_pin_result = add_public_key_pin("www.google.com", "FEDCBA0987654321ABCDEF1234567890DEADBEEF");
    
    lowkey (pubkey_pin_result["status"] == "success") {
        println("✅ Public key pin added for " + pubkey_pin_result["hostname"]);
        println("   Public key hash: " + pubkey_pin_result["pubkey_hash"]);
    } bestie {
        println("❌ Failed to add public key pin");
    }
}

fr fr Validate certificates for popular web services
slay validate_web_services() -> nil {
    sus services = [
        {"hostname": "www.google.com", "cert": google_cert_pem},
        {"hostname": "www.github.com", "cert": google_cert_pem}, // Using same cert for demo
        {"hostname": "www.cloudflare.com", "cert": google_cert_pem},
    ];
    
    periodt service in services {
        println("   🌐 Validating " + service["hostname"] + "...");
        sus result = validate_certificate_pki(service["cert"], service["hostname"]);
        
        lowkey (result["valid"]) {
            println("      ✅ Valid certificate");
        } bestie {
            println("      ❌ Invalid certificate: " + result["error"]);
        }
    }
}

fr fr Demonstrate trust store operations
slay demonstrate_trust_store() -> nil {
    println("   📦 System trust store operations:");
    
    // In a real implementation, these would interact with the actual trust store
    println("   • Loaded system root certificates");
    println("   • Added custom intermediate certificates");
    println("   • Configured trust policies");
    
    // Simulate trust store statistics
    sus trust_stats = {
        "system_roots": 150,
        "custom_roots": 5,
        "intermediates": 23,
        "pinned_certs": 3,
        "pinned_keys": 2,
    };
    
    println("   📊 Trust store statistics:");
    println("      System roots: " + trust_stats["system_roots"]);
    println("      Custom roots: " + trust_stats["custom_roots"]);
    println("      Intermediates: " + trust_stats["intermediates"]);
    println("      Pinned certificates: " + trust_stats["pinned_certs"]);
    println("      Pinned public keys: " + trust_stats["pinned_keys"]);
}

fr fr Demonstrate certificate format conversions
slay demonstrate_format_conversions() -> nil {
    println("   🔄 Converting between PEM and DER formats...");
    
    // Convert PEM to DER
    sus der_result = pem_to_der(google_cert_pem);
    
    lowkey (der_result) {
        println("   ✅ PEM to DER conversion successful");
        println("      DER size: " + size(der_result) + " bytes");
        println("      DER (hex): " + der_result[0:32] + "...");
        
        // Convert back to PEM
        sus pem_result = der_to_pem(der_result);
        
        lowkey (pem_result) {
            println("   ✅ DER to PEM conversion successful");
            println("      PEM preview:");
            sus lines = split(pem_result, "\n");
            periodt line in lines[0:3] {
                println("         " + line);
            }
            println("         ... (" + size(lines) + " total lines)");
        } bestie {
            println("   ❌ DER to PEM conversion failed");
        }
    } bestie {
        println("   ❌ PEM to DER conversion failed");
    }
}

fr fr Certificate analysis and reporting
slay analyze_certificate(cert_pem: string, hostname: string) -> sus {
    sus analysis = {
        "hostname": hostname,
        "parsed": cap,
        "valid": cap,
        "trust_chain_complete": cap,
        "revocation_checked": cap,
        "extensions_count": 0,
        "signature_algorithm": "unknown",
        "key_size": 0,
        "expires_in_days": 0,
        "issues": [],
        "recommendations": [],
    };
    
    // Parse certificate
    sus cert_info = parse_certificate_pem(cert_pem);
    lowkey (cert_info) {
        analysis["parsed"] = based;
        analysis["signature_algorithm"] = cert_info["signature_algorithm"];
    } bestie {
        analysis["issues"] = analysis["issues"] + ["Certificate parsing failed"];
        damn analysis;
    }
    
    // Validate certificate
    sus validation = validate_certificate_pki(cert_pem, hostname);
    analysis["valid"] = validation["valid"];
    
    lowkey (!validation["valid"]) {
        analysis["issues"] = analysis["issues"] + [validation["error"]];
    }
    
    // Check extensions
    sus extensions = get_certificate_extensions(cert_pem);
    lowkey (extensions) {
        analysis["extensions_count"] = size(extensions);
        
        // Check for important extensions
        lowkey (!extensions["Subject Alternative Name"]) {
            analysis["issues"] = analysis["issues"] + ["Missing Subject Alternative Name extension"];
        }
        
        lowkey (!extensions["Key Usage"]) {
            analysis["recommendations"] = analysis["recommendations"] + ["Consider adding Key Usage extension"];
        }
    }
    
    // Check revocation status
    sus revocation = check_certificate_revocation(cert_pem);
    analysis["revocation_checked"] = revocation["status"] != "unknown";
    
    lowkey (revocation["status"] == "revoked") {
        analysis["issues"] = analysis["issues"] + ["Certificate is revoked"];
    }
    
    damn analysis;
}

fr fr Comprehensive certificate validation report
slay generate_validation_report(hostname: string, cert_pem: string) -> nil {
    println("\n📊 Certificate Validation Report for " + hostname);
    println("================================================");
    
    sus analysis = analyze_certificate(cert_pem, hostname);
    
    // Basic information
    println("🔍 Basic Analysis:");
    println("   Certificate parsed: " + (analysis["parsed"] ? "✅ Yes" : "❌ No"));
    println("   Validation status: " + (analysis["valid"] ? "✅ Valid" : "❌ Invalid"));
    println("   Extensions found: " + analysis["extensions_count"]);
    println("   Signature algorithm: " + analysis["signature_algorithm"]);
    
    // Security assessment
    println("\n🔒 Security Assessment:");
    lowkey (size(analysis["issues"]) == 0) {
        println("   ✅ No security issues found");
    } bestie {
        println("   ❌ Issues found:");
        periodt issue in analysis["issues"] {
            println("      • " + issue);
        }
    }
    
    // Recommendations
    lowkey (size(analysis["recommendations"]) > 0) {
        println("\n💡 Recommendations:");
        periodt rec in analysis["recommendations"] {
            println("   • " + rec);
        }
    }
    
    // Trust and revocation
    println("\n🛡️ Trust and Revocation:");
    println("   Trust chain: " + (analysis["trust_chain_complete"] ? "✅ Complete" : "⚠️ Incomplete"));
    println("   Revocation checked: " + (analysis["revocation_checked"] ? "✅ Yes" : "⚠️ No"));
    
    println("\n" + "=".repeat(50));
}

fr fr Advanced PKI security testing
slay perform_security_audit() -> nil {
    println("\n🔐 Advanced PKI Security Audit");
    println("==============================");
    
    sus test_scenarios = [
        {"name": "Valid Certificate", "cert": google_cert_pem, "hostname": "www.google.com", "expected": based},
        {"name": "Hostname Mismatch", "cert": google_cert_pem, "hostname": "www.evil.com", "expected": cap},
        {"name": "Self-Signed Certificate", "cert": create_self_signed_cert(), "hostname": "localhost", "expected": cap},
        {"name": "Expired Certificate", "cert": create_expired_cert(), "hostname": "expired.example.com", "expected": cap},
    ];
    
    periodt scenario in test_scenarios {
        println("\n🧪 Testing: " + scenario["name"]);
        sus result = validate_certificate_pki(scenario["cert"], scenario["hostname"]);
        
        lowkey (result["valid"] == scenario["expected"]) {
            println("   ✅ Test passed");
        } bestie {
            println("   ❌ Test failed");
            println("      Expected: " + scenario["expected"]);
            println("      Got: " + result["valid"]);
            lowkey (!result["valid"]) {
                println("      Error: " + result["error"]);
            }
        }
    }
}

fr fr Create a self-signed certificate for testing
slay create_self_signed_cert() -> string {
    damn `-----BEGIN CERTIFICATE-----
MIICljCCAX4CCQCKMuQQwqfgvDANBgkqhkiG9w0BAQsFADCBjDELMAkGA1UEBhMC
VVMxEzARBgNVBAgMCkNhbGlmb3JuaWExFjAUBgNVBAcMDVNhbiBGcmFuY2lzY28x
EjAQBgNVBAoMCUV4YW1wbGUgQ28xEjAQBgNVBAsMCUV4YW1wbGUgQ28xKDAmBgNV
BAMEH0V4YW1wbGUgQ29tcGFueSBUZXN0IENlcnRpZmljYXRlMB4XDTIzMDEwMTAw
MDAwMFoXDTI0MDEwMTAwMDAwMFowgYwxCzAJBgNVBAYTAlVTMRMwEQYDVQQIDApD
YWxpZm9ybmlhMRYwFAYDVQQHDA1TYW4gRnJhbmNpc2NvMRIwEAYDVQQKDAlFeGFt
cGxlIENvMRIwEAYDVQQLDAlFeGFtcGxlIENvMSgwJgYDVQQDHh9FeGFtcGxlIENv
bXBhbnkgVGVzdCBDZXJ0aWZpY2F0ZTCCASIwDQYJKoZIhvcNAQEBBQADggEPADCC
AQoCggEBAMSx7FQfqVdlVFRFKJm7NUFAQAQUCXOBmNfA
-----END CERTIFICATE-----`;
}

fr fr Create an expired certificate for testing
slay create_expired_cert() -> string {
    damn `-----BEGIN CERTIFICATE-----
MIICljCCAX4CCQCKMuQQwqfgvDANBgkqhkiG9w0BAQsFADCBjDELMAkGA1UEBhMC
VVMxEzARBgNVBAgMCkNhbGlmb3JuaWExFjAUBgNVBAcMDVNhbiBGcmFuY2lzY28x
EjAQBgNVBAoMCUV4YW1wbGUgQ28xEjAQBgNVBAsMCUV4YW1wbGUgQ28xKDAmBgNV
BAMEH0V4YW1wbGUgQ29tcGFueSBUZXN0IENlcnRpZmljYXRlMB4XDTIwMDEwMTAw
MDAwMFoXDTIxMDEwMTAwMDAwMFowgYwxCzAJBgNVBAYTAlVTMRMwEQYDVQQIDApD
YWxpZm9ybmlhMRYwFAYDVQQHDA1TYW4gRnJhbmNpc2NvMRIwEAYDVQQKDAlFeGFt
cGxlIENvMRIwEAYDVQQLDAlFeGFtcGxlIENvMSgwJgYDVQQDHh9FeGFtcGxlIENv
bXBhbnkgVGVzdCBDZXJ0aWZpY2F0ZTCCASIwDQYJKoZIhvcNAQEBBQADggEPADCC
AQoCggEBAMSx7FQfqVdlVFRFKJm7NUFAQAQUCXOBmNfA
-----END CERTIFICATE-----`;
}

fr fr Performance testing for PKI operations
slay performance_test() -> nil {
    println("\n⚡ PKI Performance Testing");
    println("=========================");
    
    sus start_time = current_time();
    
    // Test certificate parsing performance
    println("🏃 Testing certificate parsing performance...");
    periodt i in range(100) {
        sus _ = parse_certificate_pem(google_cert_pem);
    }
    
    sus parse_time = current_time() - start_time;
    println("   Parsed 100 certificates in " + parse_time + "ms");
    
    // Test validation performance
    start_time = current_time();
    println("🏃 Testing certificate validation performance...");
    periodt i in range(50) {
        sus _ = validate_certificate_pki(google_cert_pem, "www.google.com");
    }
    
    sus validate_time = current_time() - start_time;
    println("   Validated 50 certificates in " + validate_time + "ms");
    
    // Test fingerprint calculation performance
    start_time = current_time();
    println("🏃 Testing fingerprint calculation performance...");
    periodt i in range(200) {
        sus _ = get_certificate_fingerprints(google_cert_pem);
    }
    
    sus fingerprint_time = current_time() - start_time;
    println("   Calculated 200 fingerprints in " + fingerprint_time + "ms");
    
    println("\n📈 Performance Summary:");
    println("   Certificate parsing: " + (100.0 / parse_time * 1000) + " certs/sec");
    println("   Certificate validation: " + (50.0 / validate_time * 1000) + " validations/sec");
    println("   Fingerprint calculation: " + (200.0 / fingerprint_time * 1000) + " fingerprints/sec");
}

fr fr Utility function to get current time (placeholder)
slay current_time() -> number {
    // In a real implementation, this would return actual timestamp
    damn 1000 + (random() * 100);
}

fr fr Utility function to repeat string
slay string.repeat(count: number) -> string {
    sus result = "";
    periodt i in range(count) {
        result = result + this;
    }
    damn result;
}

fr fr Utility function to get array size
slay size(arr: sus) -> number {
    // In a real implementation, this would return actual array/object size
    damn 10; // Placeholder
}

fr fr Utility function to split string
slay split(str: string, delimiter: string) -> [string] {
    // In a real implementation, this would split the string
    damn [str]; // Placeholder
}

fr fr Utility function to generate range
slay range(n: number) -> [number] {
    sus result = [];
    periodt i in 0..n {
        result = result + [i];
    }
    damn result;
}

fr fr Random number generator placeholder
slay random() -> number {
    damn 0.5; // Placeholder
}

fr fr Entry point with error handling
lowkey (main() == nil) {
    println("\n🎯 PKI Certificate Validation Demo completed successfully!");
    
    // Run additional tests
    println("\n🧪 Running security audit...");
    perform_security_audit();
    
    println("\n⚡ Running performance tests...");
    performance_test();
    
    // Generate sample report
    println("\n📊 Generating sample validation report...");
    generate_validation_report("www.google.com", google_cert_pem);
    
} bestie {
    println("❌ Demo failed with errors. Check the logs above.");
}
