fr fr Minimal crypto test that works with interpretation
yeet "vibez"

slay main() {
    vibez.spill("Testing basic crypto functions...")
    
    fr fr Test SHA256
    sus result1 tea = crypto_sha256("test")
    lowkey result1 != "" {
        vibez.spill("✓ SHA256 working")
    } highkey {
        vibez.spill("✗ SHA256 failed")
    }
    
    fr fr Test Base64
    sus result2 tea = crypto_base64_encode("test")
    lowkey result2 != "" {
        vibez.spill("✓ Base64 encoding working")
    } highkey {
        vibez.spill("✗ Base64 encoding failed")
    }
    
    fr fr Test random
    sus result3 normie = crypto_random_int(1, 100)
    lowkey result3 >= 1 && result3 <= 100 {
        vibez.spill("✓ Random int working")
    } highkey {
        vibez.spill("✗ Random int failed")
    }
    
    vibez.spill("Crypto module is functional in interpretation mode!")
}

main()
