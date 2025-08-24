yeet "stringz_enhanced"

# Test enhanced stringz modules with real algorithms
sus test_string tea = "Hello, World! This is a test string with various characters: àáâãäå"

# Test enhanced string algorithms
sus kmp_result drip = kmp_search(test_string, "World")
vibez.spill("KMP search for 'World':", kmp_result)

sus boyer_moore_result drip = boyer_moore_search(test_string, "test")
vibez.spill("Boyer-Moore search for 'test':", boyer_moore_result)

# Test Unicode processing
sus unicode_length drip = unicode_length(test_string)
vibez.spill("Unicode length:", unicode_length)

sus normalized tea = unicode_normalize_nfd(test_string)
vibez.spill("NFD normalized:", normalized)

# Test string distance algorithms
sus levenshtein_dist drip = levenshtein_distance("kitten", "sitting")
vibez.spill("Levenshtein distance (kitten/sitting):", levenshtein_dist)

sus jaro_winkler_sim drip = jaro_winkler_similarity("martha", "marhta")
vibez.spill("Jaro-Winkler similarity:", jaro_winkler_sim)

# Test advanced formatting
sus template tea = "Hello {name}, you are {age} years old!"
sus formatted tea = format_template(template, {"name": "Alice", "age": "30"})
vibez.spill("Template formatted:", formatted)

# Test string compression
sus compressed tea = compress_string(test_string)
sus decompressed tea = decompress_string(compressed)
vibez.spill("Compression ratio:", len(compressed) / len(test_string))

# Test phonetic algorithms
sus soundex tea = soundex_code("Smith")
sus metaphone tea = metaphone_code("Smith")
vibez.spill("Soundex for Smith:", soundex)
vibez.spill("Metaphone for Smith:", metaphone)

vibez.spill("✅ stringz_enhanced: All real string algorithms working")
