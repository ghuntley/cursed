// Test advanced compression algorithms in archivez
yeet "archivez"
yeet "vibez"

vibez.spill("=== Testing Advanced Archivez Compression ===")

// Test LZMA compression
sus data tea = "This is a test string for LZMA compression that should compress well because it has repeated patterns and common words."
sus lzma_compressed []drip = lzma_compress(data, 6)  // Level 6 compression
sus lzma_decompressed tea = lzma_decompress(lzma_compressed)

ready (lzma_decompressed == data) {
    vibez.spill("✅ LZMA compression/decompression: PASSED")
} otherwise {
    vibez.spill("❌ LZMA compression/decompression: FAILED")
}

// Test Brotli compression
sus brotli_compressed []drip = brotli_compress(data, 4)  // Quality 4
sus brotli_decompressed tea = brotli_decompress(brotli_compressed)

ready (brotli_decompressed == data) {
    vibez.spill("✅ Brotli compression/decompression: PASSED")
} otherwise {
    vibez.spill("❌ Brotli compression/decompression: FAILED")
}

// Test LZ4 compression
sus lz4_compressed []drip = lz4_compress(data)
sus lz4_decompressed tea = lz4_decompress(lz4_compressed)

ready (lz4_decompressed == data) {
    vibez.spill("✅ LZ4 compression/decompression: PASSED")
} otherwise {
    vibez.spill("❌ LZ4 compression/decompression: FAILED")
}

// Test compression ratio comparison
sus original_size drip = len(data)
sus lzma_ratio drip = (len(lzma_compressed) * 100) / original_size
sus brotli_ratio drip = (len(brotli_compressed) * 100) / original_size
sus lz4_ratio drip = (len(lz4_compressed) * 100) / original_size

vibez.spill("Compression ratios:")
vibez.spill("- LZMA:", lzma_ratio, "%")
vibez.spill("- Brotli:", brotli_ratio, "%")
vibez.spill("- LZ4:", lz4_ratio, "%")

vibez.spill("=== Archivez Advanced Testing Complete ===")
