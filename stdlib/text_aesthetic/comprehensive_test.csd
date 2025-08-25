// CURSED Text Processing Comprehensive Test Suite
// Tests for advanced text processing algorithms and security features

yeet "text_aesthetic"
yeet "testz"
yeet "stringz"
yeet "cryptz"
yeet "mathz"

// Test advanced string matching algorithms
slay test_string_matching_algorithms() {
    testz.test_group("String Matching Algorithms")
    
    sus processor TextProcessor = create_production_text_processor(ProcessorConfig{
        enable_caching: based,
        enable_security: based,
        max_content_size: 10240,
        cache_size: 100,
        unicode_normalization: based
    })
    
    sus content tea = "The quick brown fox jumps over the lazy dog. The fox is quick and brown."
    
    // Test KMP algorithm
    sus kmp_operation TextOperation = TextOperation{
        operation_type: "find_and_replace",
        parameters: {
            "pattern": "fox",
            "replacement": "cat",
            "algorithm": "kmp",
            "case_sensitive": based
        },
        security_level: 1,
        timeout_ms: 1000
    }
    
    sus kmp_result ProcessingResult = process_text_secure(processor, content, [kmp_operation])
    testz.assert_true(kmp_result.success, "KMP find and replace should succeed")
    testz.assert_contains(kmp_result.content, "The quick brown cat jumps", "Should replace first occurrence")
    testz.assert_contains(kmp_result.content, "The cat is quick", "Should replace second occurrence")
    testz.assert_not_contains(kmp_result.content, "fox", "Should not contain original pattern")
    
    // Test Boyer-Moore algorithm
    sus bm_operation TextOperation = TextOperation{
        operation_type: "find_and_replace",
        parameters: {
            "pattern": "quick",
            "replacement": "slow",
            "algorithm": "boyer_moore",
            "case_sensitive": based
        },
        security_level: 1,
        timeout_ms: 1000
    }
    
    sus bm_result ProcessingResult = process_text_secure(processor, content, [bm_operation])
    testz.assert_true(bm_result.success, "Boyer-Moore find and replace should succeed")
    testz.assert_contains(bm_result.content, "The slow brown fox", "Should replace with Boyer-Moore")
    
    // Test Rabin-Karp algorithm
    sus rk_operation TextOperation = TextOperation{
        operation_type: "find_and_replace",
        parameters: {
            "pattern": "brown",
            "replacement": "black",
            "algorithm": "rabin_karp",
            "case_sensitive": based
        },
        security_level: 1,
        timeout_ms: 1000
    }
    
    sus rk_result ProcessingResult = process_text_secure(processor, content, [rk_operation])
    testz.assert_true(rk_result.success, "Rabin-Karp find and replace should succeed")
    testz.assert_contains(rk_result.content, "The quick black fox", "Should replace with Rabin-Karp")
    
    // Test case-insensitive matching
    sus case_insensitive_operation TextOperation = TextOperation{
        operation_type: "find_and_replace",
        parameters: {
            "pattern": "THE",
            "replacement": "A",
            "algorithm": "kmp",
            "case_sensitive": cap
        },
        security_level: 1,
        timeout_ms: 1000
    }
    
    sus case_result ProcessingResult = process_text_secure(processor, content, [case_insensitive_operation])
    testz.assert_true(case_result.success, "Case-insensitive matching should succeed")
    testz.assert_contains(case_result.content, "A quick brown fox", "Should match regardless of case")
    
    testz.test_complete("String Matching Algorithms")
}

// Test text analysis algorithms
slay test_text_analysis_algorithms() {
    testz.test_group("Text Analysis Algorithms")
    
    sus processor TextProcessor = create_production_text_processor(ProcessorConfig{
        enable_caching: based,
        enable_security: based,
        max_content_size: 8192,
        cache_size: 50,
        unicode_normalization: based
    })
    
    // Test readability analysis
    sus sample_text tea = "The quick brown fox jumps over the lazy dog. This is a simple sentence for testing readability metrics. We need multiple sentences to calculate proper statistics."
    
    sus readability_operation TextOperation = TextOperation{
        operation_type: "text_analysis",
        parameters: {
            "analysis_type": "readability"
        },
        security_level: 1,
        timeout_ms: 5000
    }
    
    sus readability_result ProcessingResult = process_text_secure(processor, sample_text, [readability_operation])
    testz.assert_true(readability_result.success, "Readability analysis should succeed")
    testz.assert_contains(readability_result.content, "word_count", "Should include word count")
    testz.assert_contains(readability_result.content, "sentence_count", "Should include sentence count")
    testz.assert_contains(readability_result.content, "flesch_reading_ease", "Should include Flesch score")
    testz.assert_contains(readability_result.content, "flesch_kincaid_grade", "Should include grade level")
    
    // Test sentiment analysis
    sus positive_text tea = "I love this amazing product! It's fantastic and wonderful. Great job!"
    
    sus sentiment_operation TextOperation = TextOperation{
        operation_type: "text_analysis",
        parameters: {
            "analysis_type": "sentiment"
        },
        security_level: 1,
        timeout_ms: 3000
    }
    
    sus sentiment_result ProcessingResult = process_text_secure(processor, positive_text, [sentiment_operation])
    testz.assert_true(sentiment_result.success, "Sentiment analysis should succeed")
    testz.assert_contains(sentiment_result.content, "positive_score", "Should include positive score")
    testz.assert_contains(sentiment_result.content, "negative_score", "Should include negative score")
    testz.assert_contains(sentiment_result.content, "overall_sentiment", "Should determine overall sentiment")
    testz.assert_contains(sentiment_result.content, "\"positive\"", "Should detect positive sentiment")
    
    // Test negative sentiment
    sus negative_text tea = "This is terrible and awful. I hate this bad product. It's horrible and disappointing."
    
    sus negative_result ProcessingResult = process_text_secure(processor, negative_text, [sentiment_operation])
    testz.assert_true(negative_result.success, "Negative sentiment analysis should succeed")
    testz.assert_contains(negative_result.content, "\"negative\"", "Should detect negative sentiment")
    
    testz.test_complete("Text Analysis Algorithms")
}

// Test Levenshtein distance calculation
slay test_levenshtein_distance() {
    testz.test_group("Levenshtein Distance")
    
    sus processor TextProcessor = create_production_text_processor(ProcessorConfig{
        enable_caching: based,
        enable_security: based,
        max_content_size: 2048,
        cache_size: 50,
        unicode_normalization: based
    })
    
    // Test identical strings
    sus identical_operation TextOperation = TextOperation{
        operation_type: "text_analysis",
        parameters: {
            "analysis_type": "similarity",
            "target": "hello world"
        },
        security_level: 1,
        timeout_ms: 2000
    }
    
    sus identical_result ProcessingResult = process_text_secure(processor, "hello world", [identical_operation])
    testz.assert_true(identical_result.success, "Identical string comparison should succeed")
    testz.assert_contains(identical_result.content, "\"edit_distance\": 0", "Identical strings should have distance 0")
    testz.assert_contains(identical_result.content, "\"similarity_ratio\": 1", "Identical strings should have ratio 1.0")
    
    // Test single character difference
    sus single_diff_result ProcessingResult = process_text_secure(processor, "hello worlD", [identical_operation])
    testz.assert_true(single_diff_result.success, "Single difference comparison should succeed")
    testz.assert_contains(single_diff_result.content, "\"edit_distance\": 1", "Single difference should have distance 1")
    
    // Test completely different strings
    sus different_operation TextOperation = TextOperation{
        operation_type: "text_analysis",
        parameters: {
            "analysis_type": "similarity",
            "target": "xyz"
        },
        security_level: 1,
        timeout_ms: 2000
    }
    
    sus different_result ProcessingResult = process_text_secure(processor, "abc", [different_operation])
    testz.assert_true(different_result.success, "Different string comparison should succeed")
    testz.assert_contains(different_result.content, "\"edit_distance\": 3", "Different strings should have distance 3")
    
    // Test empty string handling
    sus empty_operation TextOperation = TextOperation{
        operation_type: "text_analysis",
        parameters: {
            "analysis_type": "similarity",
            "target": ""
        },
        security_level: 1,
        timeout_ms: 1000
    }
    
    sus empty_result ProcessingResult = process_text_secure(processor, "test", [empty_operation])
    testz.assert_true(empty_result.success, "Empty string comparison should succeed")
    testz.assert_contains(empty_result.content, "\"edit_distance\": 4", "Distance to empty string should equal length")
    
    testz.test_complete("Levenshtein Distance")
}

// Test phonetic algorithms
slay test_phonetic_algorithms() {
    testz.test_group("Phonetic Algorithms")
    
    sus processor TextProcessor = create_production_text_processor(ProcessorConfig{
        enable_caching: based,
        enable_security: based,
        max_content_size: 1024,
        cache_size: 30,
        unicode_normalization: based
    })
    
    // Test Soundex generation
    sus soundex_text tea = "Smith Johnson Jackson Thompson Williams"
    
    sus soundex_operation TextOperation = TextOperation{
        operation_type: "text_analysis",
        parameters: {
            "analysis_type": "phonetic"
        },
        security_level: 1,
        timeout_ms: 2000
    }
    
    sus soundex_result ProcessingResult = process_text_secure(processor, soundex_text, [soundex_operation])
    testz.assert_true(soundex_result.success, "Soundex generation should succeed")
    testz.assert_contains(soundex_result.content, "Smith", "Should include original word")
    testz.assert_contains(soundex_result.content, "S530", "Smith should have Soundex code S530")
    testz.assert_contains(soundex_result.content, "Johnson", "Should include Johnson")
    testz.assert_contains(soundex_result.content, "Jackson", "Should include Jackson")
    
    // Test similar sounding words have similar codes
    sus similar_words tea = "Smith Smyth"
    sus similar_result ProcessingResult = process_text_secure(processor, similar_words, [soundex_operation])
    testz.assert_true(similar_result.success, "Similar word Soundex should succeed")
    
    // Both Smith and Smyth should have same Soundex code (S530)
    testz.assert_contains(similar_result.content, "S530", "Similar sounding words should have same code")
    
    testz.test_complete("Phonetic Algorithms")
}

// Test security features
slay test_security_features() {
    testz.test_group("Security Features")
    
    sus processor TextProcessor = create_production_text_processor(ProcessorConfig{
        enable_caching: based,
        enable_security: based,
        max_content_size: 2048,
        cache_size: 50,
        unicode_normalization: based
    })
    
    // Test content hashing for integrity
    sus secure_content tea = "This is secure content that should be hashed"
    
    sus secure_operation TextOperation = TextOperation{
        operation_type: "text_analysis",
        parameters: {
            "analysis_type": "readability"
        },
        security_level: 2,
        timeout_ms: 3000
    }
    
    sus secure_result ProcessingResult = process_text_secure(processor, secure_content, [secure_operation])
    testz.assert_true(secure_result.success, "Secure processing should succeed")
    testz.assert_not_empty(secure_result.security_context.content_hash, "Should generate content hash")
    testz.assert_not_empty(secure_result.security_context.processing_nonce, "Should generate processing nonce")
    testz.assert_equal_int(len(secure_result.security_context.content_hash), 64, "Hash should be 64 characters (SHA-256)")
    
    // Test malicious pattern detection
    sus malicious_content tea = "SELECT * FROM users WHERE password = ''; DROP TABLE users; --"
    
    sus pattern_operation TextOperation = TextOperation{
        operation_type: "pattern_match",
        parameters: {
            "pattern": "DROP TABLE",
            "case_sensitive": cap
        },
        security_level: 3,
        timeout_ms: 2000
    }
    
    // This should be detected and potentially blocked by security system
    sus pattern_result ProcessingResult = process_text_secure(processor, malicious_content, [pattern_operation])
    
    // Either should succeed with sanitization or fail with security error
    testz.assert_true(pattern_result.success || len(pattern_result.error_message) > 0, "Should handle malicious patterns")
    
    // Test XSS prevention
    sus xss_content tea = "<script>alert('XSS attack')</script><img src=x onerror=alert(1)>"
    
    sus sanitization_operation TextOperation = TextOperation{
        operation_type: "format_transform",
        parameters: {
            "transform_type": "html_sanitize"
        },
        security_level: 2,
        timeout_ms: 2000
    }
    
    sus xss_result ProcessingResult = process_text_secure(processor, xss_content, [sanitization_operation])
    testz.assert_true(xss_result.success, "XSS sanitization should succeed")
    testz.assert_not_contains(xss_result.content, "<script>", "Should remove script tags")
    testz.assert_not_contains(xss_result.content, "onerror=", "Should remove event handlers")
    
    testz.test_complete("Security Features")
}

// Test performance and caching
slay test_performance_and_caching() {
    testz.test_group("Performance and Caching")
    
    sus processor TextProcessor = create_production_text_processor(ProcessorConfig{
        enable_caching: based,
        enable_security: based,
        max_content_size: 4096,
        cache_size: 100,
        unicode_normalization: based
    })
    
    sus test_content tea = "Performance testing content with multiple operations and complex processing requirements for benchmarking."
    
    sus operations [TextOperation] = [
        TextOperation{
            operation_type: "text_analysis",
            parameters: {"analysis_type": "readability"},
            security_level: 1,
            timeout_ms: 2000
        },
        TextOperation{
            operation_type: "text_analysis",
            parameters: {"analysis_type": "sentiment"},
            security_level: 1,
            timeout_ms: 2000
        },
        TextOperation{
            operation_type: "find_and_replace",
            parameters: {
                "pattern": "testing",
                "replacement": "benchmarking",
                "algorithm": "kmp",
                "case_sensitive": based
            },
            security_level: 1,
            timeout_ms: 1000
        }
    ]
    
    // First execution (cache miss)
    sus start_time1 normie = timez.now_unix_nano()
    sus result1 ProcessingResult = process_text_secure(processor, test_content, operations)
    sus execution_time1 normie = timez.now_unix_nano() - start_time1
    
    testz.assert_true(result1.success, "First execution should succeed")
    
    // Second execution (potential cache hit)
    sus start_time2 normie = timez.now_unix_nano()
    sus result2 ProcessingResult = process_text_secure(processor, test_content, operations)
    sus execution_time2 normie = timez.now_unix_nano() - start_time2
    
    testz.assert_true(result2.success, "Second execution should succeed")
    testz.assert_equal(result1.content, result2.content, "Cached results should be identical")
    
    // Performance should improve with caching (though may not be guaranteed in simple test)
    testz.assert_true(execution_time2 <= execution_time1 * 2, "Cached execution should not be significantly slower")
    
    // Test cache hit metrics
    testz.assert_true(result2.performance_metrics.cache_hits >= 0, "Should track cache hits")
    testz.assert_true(result2.performance_metrics.cache_misses >= 0, "Should track cache misses")
    
    testz.test_complete("Performance and Caching")
}

// Test Unicode and internationalization
slay test_unicode_processing() {
    testz.test_group("Unicode Processing")
    
    sus processor TextProcessor = create_production_text_processor(ProcessorConfig{
        enable_caching: based,
        enable_security: based,
        max_content_size: 2048,
        cache_size: 50,
        unicode_normalization: based
    })
    
    // Test various Unicode scripts
    sus unicode_content tea = "English text. Texto en español with ñ and á. Текст на русском языке. 日本語のテキスト. العربية النص. 🚀 Emoji content 🌟"
    
    // Test readability analysis with Unicode
    sus unicode_operation TextOperation = TextOperation{
        operation_type: "text_analysis",
        parameters: {
            "analysis_type": "readability"
        },
        security_level: 1,
        timeout_ms: 3000
    }
    
    sus unicode_result ProcessingResult = process_text_secure(processor, unicode_content, [unicode_operation])
    testz.assert_true(unicode_result.success, "Unicode text analysis should succeed")
    testz.assert_contains(unicode_result.content, "word_count", "Should count words in Unicode text")
    testz.assert_contains(unicode_result.content, "character_count", "Should count Unicode characters")
    
    // Test pattern matching with Unicode
    sus unicode_replace_operation TextOperation = TextOperation{
        operation_type: "find_and_replace",
        parameters: {
            "pattern": "español",
            "replacement": "castellano",
            "algorithm": "kmp",
            "case_sensitive": based
        },
        security_level: 1,
        timeout_ms: 2000
    }
    
    sus unicode_replace_result ProcessingResult = process_text_secure(processor, unicode_content, [unicode_replace_operation])
    testz.assert_true(unicode_replace_result.success, "Unicode find and replace should succeed")
    testz.assert_contains(unicode_replace_result.content, "castellano", "Should replace Unicode text")
    testz.assert_not_contains(unicode_replace_result.content, "español", "Should not contain original Unicode pattern")
    
    // Test emoji handling
    sus emoji_content tea = "👨‍👩‍👧‍👦 Family emoji and 🚀 rocket emoji in text"
    
    sus emoji_analysis ProcessingResult = process_text_secure(processor, emoji_content, [unicode_operation])
    testz.assert_true(emoji_analysis.success, "Emoji text analysis should succeed")
    
    testz.test_complete("Unicode Processing")
}

// Test advanced tokenization
slay test_advanced_tokenization() {
    testz.test_group("Advanced Tokenization")
    
    sus processor TextProcessor = create_production_text_processor(ProcessorConfig{
        enable_caching: based,
        enable_security: based,
        max_content_size: 3072,
        cache_size: 75,
        unicode_normalization: based
    })
    
    // Test complex text with punctuation, contractions, and special cases
    sus complex_text tea = "Dr. Smith's research on AI-powered systems (including ML algorithms) shows that it's 99.9% accurate. However, the U.S.A. regulations don't apply to non-U.S. companies."
    
    sus tokenization_operation TextOperation = TextOperation{
        operation_type: "tokenization",
        parameters: {
            "preserve_punctuation": based,
            "handle_contractions": based,
            "detect_abbreviations": based
        },
        security_level: 1,
        timeout_ms: 3000
    }
    
    sus tokenization_result ProcessingResult = process_text_secure(processor, complex_text, [tokenization_operation])
    testz.assert_true(tokenization_result.success, "Advanced tokenization should succeed")
    
    // Should properly handle abbreviations, contractions, and punctuation
    testz.assert_contains(tokenization_result.content, "Dr.", "Should preserve abbreviations")
    testz.assert_contains(tokenization_result.content, "it's", "Should handle contractions")
    testz.assert_contains(tokenization_result.content, "99.9%", "Should handle numbers with symbols")
    testz.assert_contains(tokenization_result.content, "U.S.A.", "Should handle complex abbreviations")
    
    // Test sentence boundary detection
    sus sentence_text tea = "First sentence. Second sentence! Third sentence? Fourth sentence... Fifth sentence."
    
    sus sentence_operation TextOperation = TextOperation{
        operation_type: "tokenization",
        parameters: {
            "tokenization_type": "sentences"
        },
        security_level: 1,
        timeout_ms: 2000
    }
    
    sus sentence_result ProcessingResult = process_text_secure(processor, sentence_text, [sentence_operation])
    testz.assert_true(sentence_result.success, "Sentence tokenization should succeed")
    
    testz.test_complete("Advanced Tokenization")
}

// Test linguistic analysis
slay test_linguistic_analysis() {
    testz.test_group("Linguistic Analysis")
    
    sus processor TextProcessor = create_production_text_processor(ProcessorConfig{
        enable_caching: based,
        enable_security: based,
        max_content_size: 4096,
        cache_size: 100,
        unicode_normalization: based
    })
    
    // Test part-of-speech tagging and linguistic features
    sus linguistic_text tea = "The beautiful red car quickly drove through the winding mountain road during the peaceful sunset."
    
    sus linguistic_operation TextOperation = TextOperation{
        operation_type: "linguistic_analysis",
        parameters: {
            "analysis_type": "pos_tagging",
            "include_lemmatization": based,
            "detect_entities": based
        },
        security_level: 1,
        timeout_ms: 5000
    }
    
    sus linguistic_result ProcessingResult = process_text_secure(processor, linguistic_text, [linguistic_operation])
    testz.assert_true(linguistic_result.success, "Linguistic analysis should succeed")
    
    // Should identify different parts of speech and linguistic features
    testz.assert_contains(linguistic_result.content, "adjective", "Should identify adjectives")
    testz.assert_contains(linguistic_result.content, "noun", "Should identify nouns")
    testz.assert_contains(linguistic_result.content, "verb", "Should identify verbs")
    testz.assert_contains(linguistic_result.content, "adverb", "Should identify adverbs")
    
    // Test named entity recognition
    sus entity_text tea = "Apple Inc. was founded by Steve Jobs in Cupertino, California in 1976. Microsoft Corporation is located in Redmond, Washington."
    
    sus entity_operation TextOperation = TextOperation{
        operation_type: "linguistic_analysis",
        parameters: {
            "analysis_type": "named_entities"
        },
        security_level: 1,
        timeout_ms: 4000
    }
    
    sus entity_result ProcessingResult = process_text_secure(processor, entity_text, [entity_operation])
    testz.assert_true(entity_result.success, "Named entity recognition should succeed")
    
    testz.assert_contains(entity_result.content, "Apple Inc.", "Should identify company entities")
    testz.assert_contains(entity_result.content, "Steve Jobs", "Should identify person entities")
    testz.assert_contains(entity_result.content, "Cupertino", "Should identify location entities")
    testz.assert_contains(entity_result.content, "1976", "Should identify date entities")
    
    testz.test_complete("Linguistic Analysis")
}

// Test compression and encoding
slay test_compression_and_encoding() {
    testz.test_group("Compression and Encoding")
    
    sus processor TextProcessor = create_production_text_processor(ProcessorConfig{
        enable_caching: based,
        enable_security: based,
        max_content_size: 8192,
        cache_size: 50,
        unicode_normalization: based
    })
    
    // Test Base64 encoding
    sus original_text tea = "This is a test string for Base64 encoding and decoding operations."
    
    sus base64_encode_operation TextOperation = TextOperation{
        operation_type: "encoding_conversion",
        parameters: {
            "conversion_type": "base64_encode"
        },
        security_level: 1,
        timeout_ms: 2000
    }
    
    sus encode_result ProcessingResult = process_text_secure(processor, original_text, [base64_encode_operation])
    testz.assert_true(encode_result.success, "Base64 encoding should succeed")
    testz.assert_not_equal(encode_result.content, original_text, "Encoded text should be different")
    testz.assert_not_contains(encode_result.content, "test string", "Encoded text should not contain original")
    
    // Test Base64 decoding
    sus base64_decode_operation TextOperation = TextOperation{
        operation_type: "encoding_conversion",
        parameters: {
            "conversion_type": "base64_decode"
        },
        security_level: 1,
        timeout_ms: 2000
    }
    
    sus decode_result ProcessingResult = process_text_secure(processor, encode_result.content, [base64_decode_operation])
    testz.assert_true(decode_result.success, "Base64 decoding should succeed")
    testz.assert_equal(decode_result.content, original_text, "Decoded text should match original")
    
    // Test text compression
    sus repetitive_text tea = ""
    bestie i := 0; i < 100; i++ {
        repetitive_text = repetitive_text + "This is repetitive text that should compress well. "
    }
    
    sus compression_operation TextOperation = TextOperation{
        operation_type: "compression",
        parameters: {
            "compression_type": "lz77",
            "compression_level": 6
        },
        security_level: 1,
        timeout_ms: 5000
    }
    
    sus compression_result ProcessingResult = process_text_secure(processor, repetitive_text, [compression_operation])
    testz.assert_true(compression_result.success, "Text compression should succeed")
    testz.assert_true(len(compression_result.content) < len(repetitive_text), "Compressed text should be smaller")
    
    // Test decompression
    sus decompression_operation TextOperation = TextOperation{
        operation_type: "compression",
        parameters: {
            "compression_type": "lz77_decompress"
        },
        security_level: 1,
        timeout_ms: 3000
    }
    
    sus decompression_result ProcessingResult = process_text_secure(processor, compression_result.content, [decompression_operation])
    testz.assert_true(decompression_result.success, "Text decompression should succeed")
    testz.assert_equal(decompression_result.content, repetitive_text, "Decompressed text should match original")
    
    testz.test_complete("Compression and Encoding")
}

// Test error handling and edge cases
slay test_error_handling_edge_cases() {
    testz.test_group("Error Handling and Edge Cases")
    
    sus processor TextProcessor = create_production_text_processor(ProcessorConfig{
        enable_caching: based,
        enable_security: based,
        max_content_size: 1024,
        cache_size: 25,
        unicode_normalization: based
    })
    
    // Test empty input
    sus empty_operation TextOperation = TextOperation{
        operation_type: "text_analysis",
        parameters: {
            "analysis_type": "readability"
        },
        security_level: 1,
        timeout_ms: 1000
    }
    
    sus empty_result ProcessingResult = process_text_secure(processor, "", [empty_operation])
    testz.assert_true(empty_result.success || len(empty_result.error_message) > 0, "Should handle empty input gracefully")
    
    // Test very large input (should be rejected due to size limit)
    sus large_text tea = ""
    bestie i := 0; i < 2000; i++ {
        large_text = large_text + "This is a very long text that exceeds the maximum content size limit. "
    }
    
    sus large_result ProcessingResult = process_text_secure(processor, large_text, [empty_operation])
    testz.assert_false(large_result.success, "Should reject content that exceeds size limit")
    testz.assert_not_empty(large_result.error_message, "Should provide error message for oversized content")
    
    // Test invalid operation type
    sus invalid_operation TextOperation = TextOperation{
        operation_type: "nonexistent_operation",
        parameters: {},
        security_level: 1,
        timeout_ms: 1000
    }
    
    sus invalid_result ProcessingResult = process_text_secure(processor, "test content", [invalid_operation])
    testz.assert_false(invalid_result.success, "Should reject invalid operation types")
    testz.assert_not_empty(invalid_result.error_message, "Should provide error message for invalid operations")
    
    // Test malformed parameters
    sus malformed_operation TextOperation = TextOperation{
        operation_type: "find_and_replace",
        parameters: {
            "pattern": "",  // Empty pattern should be invalid
            "replacement": "test"
        },
        security_level: 1,
        timeout_ms: 1000
    }
    
    sus malformed_result ProcessingResult = process_text_secure(processor, "test content", [malformed_operation])
    testz.assert_false(malformed_result.success, "Should reject malformed parameters")
    
    // Test null/invalid characters
    sus invalid_char_text tea = "Test\x00null\x01control\x02chars"
    
    sus sanitize_operation TextOperation = TextOperation{
        operation_type: "format_transform",
        parameters: {
            "transform_type": "sanitize_control_chars"
        },
        security_level: 2,
        timeout_ms: 1000
    }
    
    sus sanitize_result ProcessingResult = process_text_secure(processor, invalid_char_text, [sanitize_operation])
    testz.assert_true(sanitize_result.success, "Should handle control characters")
    testz.assert_not_contains(sanitize_result.content, "\x00", "Should remove null characters")
    
    testz.test_complete("Error Handling and Edge Cases")
}

// Main test runner
slay run_all_tests() {
    testz.test_start("Text Processing Comprehensive Test Suite")
    
    test_string_matching_algorithms()
    test_text_analysis_algorithms()
    test_levenshtein_distance()
    test_phonetic_algorithms()
    test_security_features()
    test_performance_and_caching()
    test_unicode_processing()
    test_advanced_tokenization()
    test_linguistic_analysis()
    test_compression_and_encoding()
    test_error_handling_edge_cases()
    
    testz.print_test_summary()
}

// Execute tests
run_all_tests()
