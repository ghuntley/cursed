fr fr Simple Compression Test

yeet "compressz"
yeet "vibez"

slay main() {
    vibez.spill("Testing compression implementation...")
    
    sus test_data tea = "Hello, World! This is a test."
    vibez.spill("Test data: Hello, World! This is a test.")
    
    fr fr Test basic compression
    sus result CompressedData = compressz.gzip_compress(test_data, 6)
    
    vibez.spill("Compression completed")
    vibez.spill("Algorithm: gzip")
    
    fr fr Test decompression
    sus decompressed tea = compressz.gzip_decompress(result)
    
    vibez.spill("Decompression completed")
    
    ready (test_data == decompressed) {
        vibez.spill("SUCCESS: Compression roundtrip works!")
    } otherwise {
        vibez.spill("FAILURE: Compression roundtrip failed")
    }
    
    fr fr Test utility functions directly
    sus test_crc drip = compressz.calculate_crc32("Hello")
    vibez.spill("CRC32 calculation works")
    
    fr fr Test frequency calculation
    sus frequencies []drip = compressz.calculate_frequencies("aabbcc")
    vibez.spill("Frequency calculation works")
    
    vibez.spill("Compression implementation test complete")
}
