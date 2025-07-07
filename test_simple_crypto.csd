vibez.spill("Starting crypto test...")

// Test SHA-256 - should work
vibez.spill("Testing SHA-256...")
sus sha_result tea = crypto_sha256("hello")
vibez.spill("SHA-256 result: " + sha_result)

// Test MD5 - should fail
vibez.spill("Testing MD5 (should fail)...")
sus md5_result tea = crypto_md5("hello")
vibez.spill("MD5 result: " + md5_result)

vibez.spill("Test complete.")
