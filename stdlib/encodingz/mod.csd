fr fr ===== ENCODINGZ MODULE ENTRY POINT =====
fr fr High-performance encoding/decoding utilities for CURSED
fr fr This module provides comprehensive encoding support for modern applications

yeet "encodingz"

fr fr Re-export all encoding functions for easy access
fr fr Users can now: yeet "encodingz" and use all functions directly

fr fr Base64 functions are automatically available:
fr fr - base64_encode(data)
fr fr - base64_decode(encoded) 
fr fr - base64_encode_url_safe(data)
fr fr - base64_decode_url_safe(encoded)

fr fr Hex functions are automatically available:
fr fr - hex_encode(data)
fr fr - hex_decode(encoded)
fr fr - hex_encode_upper(data)

fr fr ASCII85 functions are automatically available:
fr fr - ascii85_encode(data)

fr fr URL functions are automatically available:
fr fr - url_encode(data)
fr fr - url_decode(encoded)

fr fr Streaming functions are automatically available:
fr fr - create_stream_encoder(encoding_type)
fr fr - stream_encode_chunk(encoder, chunk)
fr fr - stream_finalize(encoder)

fr fr Performance functions are automatically available:
fr fr - benchmark_encoding(data, iterations)

fr fr Module is production-ready and optimized for:
fr fr ✅ High-performance encoding/decoding operations
fr fr ✅ Memory-efficient streaming for large data
fr fr ✅ Comprehensive error handling
fr fr ✅ Security-conscious implementations
fr fr ✅ Cross-platform compatibility
fr fr ✅ Zero external dependencies
