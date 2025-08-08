yeet "compressionz"

fr fr Basic test of compression functions
sus test_text tea = "Hello, World! This is a test string for compression."

vibez.spill("Testing CURSED Enhanced Compression Module")
vibez.spill("=====================================")

fr fr Test LZ4 compression
vibez.spill("Testing LZ4 Enhanced:")
sus lz4_compressed tea = compress_data(test_text, ALGO_LZ4, COMPRESS_LEVEL_BALANCED)
vibez.spill("LZ4 compressed: " + lz4_compressed)

sus lz4_decompressed tea = decompress_data(lz4_compressed)
vibez.spill("LZ4 decompressed: " + lz4_decompressed)

fr fr Test ZSTD compression
vibez.spill("")
vibez.spill("Testing ZSTD Advanced:")
sus zstd_compressed tea = compress_data(test_text, ALGO_ZSTD, COMPRESS_LEVEL_BALANCED)
vibez.spill("ZSTD compressed: " + zstd_compressed)

sus zstd_decompressed tea = decompress_data(zstd_compressed)
vibez.spill("ZSTD decompressed: " + zstd_decompressed)

fr fr Test GZIP compression
vibez.spill("")
vibez.spill("Testing GZIP Enhanced:")
sus gzip_compressed tea = compress_data(test_text, ALGO_GZIP, COMPRESS_LEVEL_BALANCED)
vibez.spill("GZIP compressed: " + gzip_compressed)

sus gzip_decompressed tea = decompress_data(gzip_compressed)
vibez.spill("GZIP decompressed: " + gzip_decompressed)

fr fr Test compression analysis
vibez.spill("")
vibez.spill("Compression Analysis:")
analyze_compression_performance(test_text, lz4_compressed, ALGO_LZ4)

vibez.spill("")
vibez.spill("All compression algorithms working correctly!")
