fr fr Simple crypto test for compilation
yeet "vibez"

slay main() {
    vibez.spill("🔐 Testing crypto in compilation mode...")
    
    fr fr Test SHA256 
    sus hash tea = crypto_sha256("hello")
    vibez.spill("SHA256 hash: " + hash)
    
    fr fr Test Base64
    sus encoded tea = crypto_base64_encode("hello")
    vibez.spill("Base64 encoded: " + encoded)
    
    sus decoded tea = crypto_base64_decode(encoded)
    vibez.spill("Base64 decoded: " + decoded)
    
    fr fr Test random
    sus rand_num normie = crypto_random_int(1, 10)
    vibez.spill("Random number: " + rand_num)
    
    fr fr Test HMAC
    sus hmac tea = crypto_hmac_sha256("data", "key")
    vibez.spill("HMAC result: " + hmac)
    
    vibez.spill("🎉 Crypto compilation test completed!")
}

main()
