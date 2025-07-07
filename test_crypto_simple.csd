fr fr Simple crypto test to verify functions work
yeet "vibez"

slay main() {
    vibez.spill("Testing crypto functions...")
    
    fr fr Test SHA256
    sus hash tea = crypto_sha256("hello world")
    vibez.spill("SHA256 hash: " + hash)
    
    fr fr Test base64 encoding
    sus encoded tea = crypto_base64_encode("hello world")
    vibez.spill("Base64 encoded: " + encoded)
    
    fr fr Test base64 decoding
    sus decoded tea = crypto_base64_decode(encoded)
    vibez.spill("Base64 decoded: " + decoded)
    
    fr fr Test random number
    sus random_num normie = crypto_random_int(1, 100)
    vibez.spill("Random number: " + random_num)
    
    vibez.spill("All crypto tests completed!")
}

main()
