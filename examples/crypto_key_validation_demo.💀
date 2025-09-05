#!/usr/bin/env cursed

fr fr CURSED Cryptographic Key Validation Demo
fr fr Demonstrates comprehensive key validation capabilities

yeet "stdlib::crypto_asymmetric"

squad KeyValidationDemo {
    // RSA key validation example
    method validate_rsa_keys() {
        print("🔑 RSA Key Validation Examples")
        print("=====================================")
        
        // Valid RSA-2048 key
        sus rsa_key = {
            "n": "25195908475657893494027183240048398571429282126204032027777137836043662020707595556264018525880784406918290641249515082189298559149176184502808489120072844992687392807287776735971418347270261896375014971824691165077613379859095700097330459748808428401797429100642458691817195118746121515172654632282216869987549182422433637259085141865462043576798423387184774447920739934236584823824281198163815010674810451660377306056201619676256133844143603833904414952634432190114657544454178424020924616515723350778707749817125772467962926386356373289912154831438167899885040445364023527381951378636564391212010397122822120720357",
            "e": "65537"
        }
        
        // Validate the key
        sus result = validate_key(rsa_key)?
        
        print("RSA-2048 Validation Result:")
        print("  Valid: " + result.valid)
        print("  Key Type: " + result.key_type)
        print("  Strength: " + result.strength_bits + " bits")
        print("  Warnings: " + result.warnings.length)
        print("  Errors: " + result.errors.length)
        print()
        
        // Test weak RSA key
        sus weak_rsa = {
            "n": "1024",  // Too small modulus
            "e": "65537"
        }
        
        sus weak_result = validate_key(weak_rsa)?
        print("Weak RSA Key Validation:")
        print("  Valid: " + weak_result.valid)
        print("  Errors: " + weak_result.errors)
        print()
        
        // Test invalid RSA key
        sus invalid_rsa = {
            "n": "2",     // Even modulus (invalid)
            "e": "2"      // Even exponent (invalid)
        }
        
        sus invalid_result = validate_key(invalid_rsa)?
        print("Invalid RSA Key Validation:")
        print("  Valid: " + invalid_result.valid)
        print("  Errors: " + invalid_result.errors)
        print()
    }
    
    // ECC key validation example
    method validate_ecc_keys() {
        print("🔐 ECC Key Validation Examples")
        print("=====================================")
        
        // Valid secp256r1 key
        sus ecc_key = {
            "curve": "secp256r1",
            "public_key": "04b17e5f5b3a5b7b2e3d4f1a2c5e6d8f9b0a1c2d3e4f5a6b7c8d9e0f1a2b3c4d5e6f708192a3b4c5d6e7f8091a2b3c4d5e6f7809",
            "private_key": "a1b2c3d4e5f6708192a3b4c5d6e7f8091a2b3c4d5e6f708192a3b4c5d6e7f809"
        }
        
        sus result = validate_key(ecc_key)?
        
        print("ECC secp256r1 Validation Result:")
        print("  Valid: " + result.valid)
        print("  Key Type: " + result.key_type)
        print("  Strength: " + result.strength_bits + " bits")
        print("  Curve: " + result.parameters.curve)
        print()
        
        // Test different curves
        facts curves = ["secp256r1", "secp384r1", "secp521r1", "secp256k1"]
        
        lowkey (sus curve : curves) {
            sus test_key = {
                "curve": curve,
                "public_key": "04b17e5f5b3a5b7b2e3d4f1a2c5e6d8f9b0a1c2d3e4f5a6b7c8d9e0f1a2b3c4d5e6f708192a3b4c5d6e7f8091a2b3c4d5e6f7809"
            }
            
            sus curve_result = validate_key(test_key)?
            print(curve + " strength: " + curve_result.strength_bits + " bits")
        }
        print()
        
        // Test unknown curve
        sus unknown_curve = {
            "curve": "unknown_curve",
            "public_key": "04b17e5f5b3a5b7b2e3d4f1a2c5e6d8f9b0a1c2d3e4f5a6b7c8d9e0f1a2b3c4d5e6f708192a3b4c5d6e7f8091a2b3c4d5e6f7809"
        }
        
        sus unknown_result = validate_key(unknown_curve)?
        print("Unknown Curve Validation:")
        print("  Valid: " + unknown_result.valid)
        print("  Warnings: " + unknown_result.warnings)
        print()
    }
    
    // Ed25519/X25519 key validation example
    method validate_eddsa_keys() {
        print("🗝️  EdDSA Key Validation Examples")
        print("=====================================")
        
        // Valid Ed25519 key
        sus ed25519_key = {
            "ed25519_public": "d75a980182b10ab7d54bfed3c964073a0ee172f3daa62325af021a68f707511a",
            "ed25519_private": "9d61b19deffd5a60ba844af492ec2cc44449c5697b326919703bac031cae7f60"
        }
        
        sus result = validate_key(ed25519_key)?
        
        print("Ed25519 Validation Result:")
        print("  Valid: " + result.valid)
        print("  Key Type: " + result.key_type)
        print("  Strength: " + result.strength_bits + " bits")
        print("  Format Valid: " + result.parameters.format_valid)
        print()
        
        // Valid X25519 key
        sus x25519_key = {
            "x25519_public": "d75a980182b10ab7d54bfed3c964073a0ee172f3daa62325af021a68f707511a",
            "x25519_private": "9d61b19deffd5a60ba844af492ec2cc44449c5697b326919703bac031cae7f60"
        }
        
        sus x25519_result = validate_key(x25519_key)?
        
        print("X25519 Validation Result:")
        print("  Valid: " + x25519_result.valid)
        print("  Key Type: " + x25519_result.key_type)
        print("  Strength: " + x25519_result.strength_bits + " bits")
        print()
        
        // Test invalid key length
        sus invalid_ed25519 = {
            "ed25519_public": "invalid_length"
        }
        
        sus invalid_result = validate_key(invalid_ed25519)?
        print("Invalid Ed25519 Length Validation:")
        print("  Valid: " + invalid_result.valid)
        print("  Errors: " + invalid_result.errors)
        print()
        
        // Test all-zero key (invalid)
        sus zero_key = {
            "ed25519_public": "0000000000000000000000000000000000000000000000000000000000000000"
        }
        
        sus zero_result = validate_key(zero_key)?
        print("All-Zero Ed25519 Key Validation:")
        print("  Valid: " + zero_result.valid)
        print("  Errors: " + zero_result.errors)
        print()
    }
    
    // PEM/DER format validation example
    method validate_encoded_keys() {
        print("📄 Encoded Key Format Validation")
        print("=====================================")
        
        // Valid PEM format example
        sus pem_key = "-----BEGIN RSA PRIVATE KEY-----
MIIEpAIBAAKCAQEA1234567890abcdef...
-----END RSA PRIVATE KEY-----"
        
        sus pem_result = validate_key(pem_key)?
        
        print("PEM Format Validation:")
        print("  Valid: " + pem_result.valid)
        print("  Format: " + pem_result.parameters.format)
        print("  Key Type: " + pem_result.key_type)
        print()
        
        // Invalid PEM format
        sus invalid_pem = "-----BEGIN RSA PRIVATE KEY-----
Invalid content without proper ending"
        
        sus invalid_pem_result = validate_key(invalid_pem)?
        print("Invalid PEM Format:")
        print("  Valid: " + invalid_pem_result.valid)
        print("  Errors: " + invalid_pem_result.errors)
        print()
        
        // DER format example (hex encoded)
        sus der_key = "308204a30201000282010100d75a980182b10ab7d54bfed3c964073a"
        
        sus der_result = validate_key(der_key)?
        print("DER Format Validation:")
        print("  Valid: " + der_result.valid)
        print("  Format: " + der_result.parameters.format)
        print("  Warnings: " + der_result.warnings)
        print()
    }
    
    // Key pair validation example
    method validate_key_pairs() {
        print("👥 Key Pair Validation Examples")
        print("=====================================")
        
        // RSA key pair
        sus private_key = {
            "n": "25195908475657893494027183240048398571429282126204032027777137836043662020707595556264018525880784406918290641249515082189298559149176184502808489120072844992687392807287776735971418347270261896375014971824691165077613379859095700097330459748808428401797429100642458691817195118746121515172654632282216869987549182422433637259085141865462043576798423387184774447920739934236584823824281198163815010674810451660377306056201619676256133844143603833904414952634432190114657544454178424020924616515723350778707749817125772467962926386356373289912154831438167899885040445364023527381951378636564391212010397122822120720357",
            "e": "65537",
            "d": "15118067030067780966248653440027677943026995720693346421726606428012764985948260893772551985020993802823825996928439049436077749360336949442593244978244615551234345263092884062009370142653324973423623751470073671988610616046551066829825194688329154700763353058265341194079705952293026598847663844901925914554297037161749404364833119999828084262166976509076294001693203425802966781976068234554169953655950088721978329802572653671063364806133159157749065639008065166244456416906616502950411005655653503978892979251847618689423072978847851049024506985569965977705779623655493952092063369102996076721050006027536999644067"
        }
        
        sus public_key = {
            "n": "25195908475657893494027183240048398571429282126204032027777137836043662020707595556264018525880784406918290641249515082189298559149176184502808489120072844992687392807287776735971418347270261896375014971824691165077613379859095700097330459748808428401797429100642458691817195118746121515172654632282216869987549182422433637259085141865462043576798423387184774447920739934236584823824281198163815010674810451660377306056201619676256133844143603833904414952634432190114657544454178424020924616515723350778707749817125772467962926386356373289912154831438167899885040445364023527381951378636564391212010397122822120720357",
            "e": "65537"
        }
        
        sus pair_result = validate_key_pair(private_key, public_key)?
        
        print("RSA Key Pair Validation:")
        print("  Valid: " + pair_result.valid)
        print("  Pair Validation: " + pair_result.parameters.pair_validation)
        print("  Errors: " + pair_result.errors)
        print()
    }
    
    // Key strength validation example
    method validate_key_strength() {
        print("💪 Key Strength Validation Examples")
        print("=====================================")
        
        // Test different key strengths
        facts test_keys = [
            {"name": "RSA-1024", "key": {"n": "123456789", "e": "65537"}},
            {"name": "RSA-2048", "key": {"n": "25195908475657893494027183240048398571429282126204032027777137836043662020707595556264018525880784406918290641249515082189298559149176184502808489120072844992687392807287776735971418347270261896375014971824691165077613379859095700097330459748808428401797429100642458691817195118746121515172654632282216869987549182422433637259085141865462043576798423387184774447920739934236584823824281198163815010674810451660377306056201619676256133844143603833904414952634432190114657544454178424020924616515723350778707749817125772467962926386356373289912154831438167899885040445364023527381951378636564391212010397122822120720357", "e": "65537"}},
            {"name": "ECC-P256", "key": {"curve": "secp256r1", "public_key": "04b17e5f5b3a5b7b2e3d4f1a2c5e6d8f9b0a1c2d3e4f5a6b7c8d9e0f1a2b3c4d5e6f708192a3b4c5d6e7f8091a2b3c4d5e6f7809"}},
            {"name": "Ed25519", "key": {"ed25519_public": "d75a980182b10ab7d54bfed3c964073a0ee172f3daa62325af021a68f707511a"}}
        ]
        
        lowkey (sus test_case : test_keys) {
            sus strength_result = validate_key_strength(test_case.key, 112)?
            
            print(test_case.name + " Strength Analysis:")
            print("  Meets Standard (112-bit): " + strength_result.meets_standard)
            print("  Actual Strength: " + strength_result.actual_strength + " bits")
            print("  Valid for Requirement: " + strength_result.valid_strength)
            print()
        }
    }
    
    // Security best practices demonstration
    method demonstrate_security_practices() {
        print("🛡️  Security Best Practices")
        print("=====================================")
        
        print("Key Validation Best Practices:")
        print("1. Always validate keys before use")
        print("2. Check key strength against current standards")
        print("3. Verify key pair consistency for asymmetric operations")
        print("4. Validate curve parameters for ECC keys")
        print("5. Check for weak or compromised key patterns")
        print("6. Ensure proper key formats and encoding")
        print("7. Monitor for deprecated algorithms and key sizes")
        print()
        
        print("Current Security Recommendations:")
        print("• RSA: Minimum 2048 bits (prefer 3072+ bits)")
        print("• ECC: P-256 minimum (prefer P-384+ for high security)")
        print("• Ed25519: Always 128-bit security level")
        print("• Hash functions: SHA-256 minimum")
        print("• Avoid: RSA-1024, MD5, SHA-1 for new applications")
        print()
        
        print("Key Validation Scenarios:")
        print("✅ Production use: Require 112+ bit security strength")
        print("⚠️  Legacy systems: May accept 80+ bit strength with warnings")
        print("❌ High security: Require 128+ bit security strength")
        print("🔍 Regular audits: Check for algorithm deprecation")
        print()
    }
}

fr fr Main demonstration function
method main() {
    print("🔐 CURSED Cryptographic Key Validation Demo")
    print("===========================================")
    print()
    
    sus demo = KeyValidationDemo{}
    
    // Run all validation examples
    demo.validate_rsa_keys()
    demo.validate_ecc_keys()
    demo.validate_eddsa_keys()
    demo.validate_encoded_keys()
    demo.validate_key_pairs()
    demo.validate_key_strength()
    demo.demonstrate_security_practices()
    
    print("🎉 Key validation demonstration completed!")
    print("All cryptographic key types validated with comprehensive security checks.")
}
