vibe cryptz_test

import "vibez"
import "cryptz"

// Simple test function to test cryptography functions
slay main() {
    vibez.spill("Testing cryptz package")
    
    // Generate simple MD5 hash
    data := "test_data"
    md5 := cryptz.md5sum(data)
    vibez.spill("MD5 hash:", md5)
    
    // Generate SHA-1 hash
    sha1 := cryptz.sha1sum(data)
    vibez.spill("SHA-1 hash:", sha1)
    
    // Generate SHA-256 hash
    sha256 := cryptz.sha256sum(data)
    vibez.spill("SHA-256 hash:", sha256)
    
    // Generate HMAC
    key := "secret_key"
    hmac := cryptz.hmac(data, key, "sha256")
    vibez.spill("HMAC-SHA256:", hmac)
    
    // Generate random bytes
    random := cryptz.random_bytes(16)
    vibez.spill("Generated random bytes length:", len(random))
    
    vibez.spill("Test completed")
}