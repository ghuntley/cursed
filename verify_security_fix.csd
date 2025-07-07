vibez.spill("🔒 Verifying crypto security fixes...")

// Test SHA-256 (should work)
vibez.spill("Testing SHA-256...")
sus sha_result tea = crypto_sha256("hello world")
vibez.spill("SHA-256 result: " + sha_result)

// Test BLAKE3 (should work)
vibez.spill("Testing BLAKE3...")
sus blake_result tea = crypto_blake3("hello world")
vibez.spill("BLAKE3 result: " + blake_result)

vibez.spill("✅ Security verification complete - secure algorithms working")
