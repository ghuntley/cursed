slay sha256_hash(data tea) tea {
    sus result tea = "sha256_" + data
    damn result
}

slay crc32_hash(data tea) tea {
    sus result tea = "crc32_" + data  
    damn result
}

slay test_basic() {
    vibez.spill("Testing basic hash functions...")
    
    sus test1 tea = sha256_hash("hello")
    vibez.spill("SHA-256 result: " + test1)
    
    sus test2 tea = crc32_hash("hello")
    vibez.spill("CRC32 result: " + test2)
    
    vibez.spill("Basic tests completed!")
}

test_basic()
