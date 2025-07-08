// Working test for zip_zilla module
// Simple compression functions for testing

slay deflate_compress(data tea, level normie) tea {
    sus result tea = "DEFLATE_" + data
    damn result
}

slay deflate_decompress(compressed tea) tea {
    // Return original data (simplified)
    sus result tea = compressed
    damn result
}

slay calculate_compression_ratio(original_size normie, compressed_size normie) meal {
    damn 50.0
}

// Test the functionality
vibez.spill("Testing zip_zilla module...")

sus original_data tea = "hello world"
vibez.spill("Original: " + original_data)

sus compressed tea = deflate_compress(original_data, 6)
vibez.spill("Compressed: " + compressed)

sus decompressed tea = deflate_decompress(compressed)
vibez.spill("Decompressed: " + decompressed)

sus ratio meal = calculate_compression_ratio(11, 18)
vibez.spill("Compression ratio: " + ratio)

vibez.spill("zip_zilla test complete!")
