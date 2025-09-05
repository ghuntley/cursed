yeet "testz"
yeet "stringz_enhanced"

test_start("STRINGZ_ENHANCED Advanced String Processing Tests")

// Test Unicode normalization
sus unicode_str tea = "café"
sus normalized tea = normalize_unicode_nfc(unicode_str)
assert_not_eq_string(normalized, "")

sus decomposed tea = normalize_unicode_nfd(unicode_str)
assert_not_eq_string(decomposed, "")

// Test advanced pattern matching
sus text tea = "The quick brown fox jumps over the lazy dog"
sus pattern tea = "\\b\\w{5}\\b" // 5-letter words
sus matches tea[value] = regex_find_all(text, pattern)
assert_true(len(matches) >= 3) // "quick", "brown", "jumps"

// Test string similarity algorithms
sus str1 tea = "kitten"
sus str2 tea = "sitting"
sus levenshtein drip = levenshtein_distance(str1, str2)
assert_eq_int(levenshtein, 3)

sus similarity drip = jaro_winkler_similarity("martha", "marhta")
assert_true(similarity > 0.9)

// Test advanced string manipulation
sus camel_case tea = "thisIsCamelCase"
sus snake_case tea = camel_to_snake_case(camel_case)
assert_eq_string(snake_case, "this_is_camel_case")

sus kebab_case tea = snake_to_kebab_case(snake_case)
assert_eq_string(kebab_case, "this-is-camel-case")

// Test string parsing and extraction
sus csv_line tea = "name,age,city,\"quoted,value\""
sus csv_fields tea[value] = parse_csv_line(csv_line)
assert_eq_int(len(csv_fields), 4)
assert_eq_string(csv_fields[3], "quoted,value")

// Test string templating
sus template tea = "Hello {{name}}, you are {{age}} years old"
sus variables sus_map<tea, tea> = {"name": "Alice", "age": "25"}
sus rendered tea = render_template(template, variables)
assert_eq_string(rendered, "Hello Alice, you are 25 years old")

// Test string compression
sus large_text tea = repeat_string("This is a test string for compression. ", 100)
sus compressed tea = compress_string(large_text)
assert_true(len(compressed) < len(large_text))

sus decompressed tea = decompress_string(compressed)
assert_eq_string(decompressed, large_text)

// Test pluralization and humanization
sus singular tea = "child"
sus plural tea = pluralize(singular)
assert_eq_string(plural, "children")

sus number_text tea = humanize_number(1234567)
assert_eq_string(number_text, "1,234,567")

// Test string validation
sus email tea = "test@example.com"
assert_true(is_valid_email(email))
assert_false(is_valid_email("invalid-email"))

sus url tea = "https://www.example.com/path"
assert_true(is_valid_url(url))
assert_false(is_valid_url("not-a-url"))

// Test text analysis
sus sample_text tea = "This is a sample text for analysis. It contains multiple sentences."
sus word_count drip = count_words(sample_text)
assert_eq_int(word_count, 12)

sus sentence_count drip = count_sentences(sample_text)
assert_eq_int(sentence_count, 2)

sus readability drip = calculate_readability_score(sample_text)
assert_true(readability > 0.0)

// Test string encoding/decoding
sus base64_encoded tea = encode_base64("Hello World")
assert_not_eq_string(base64_encoded, "Hello World")

sus base64_decoded tea = decode_base64(base64_encoded)
assert_eq_string(base64_decoded, "Hello World")

sus url_encoded tea = url_encode("hello world!")
assert_eq_string(url_encoded, "hello%20world%21")

// Test phonetic algorithms
sus soundex1 tea = soundex("Smith")
sus soundex2 tea = soundex("Smyth")
assert_eq_string(soundex1, soundex2) // Should be same soundex code

sus metaphone_code tea = metaphone("Catherine")
assert_not_eq_string(metaphone_code, "")

// Test advanced string search
sus haystack tea = "The quick brown fox jumps over the lazy dog multiple times"
sus needle tea = "fox"
sus kmp_index drip = knuth_morris_pratt_search(haystack, needle)
assert_eq_int(kmp_index, 16)

sus boyer_moore_index drip = boyer_moore_search(haystack, needle)
assert_eq_int(boyer_moore_index, 16)

// Test string performance optimizations
sus perf_strings tea[value] = generate_test_strings(1000, 100)
sus concat_start drip = get_nanoseconds()
sus concatenated tea = bulk_string_concat(perf_strings, " ")
sus concat_end drip = get_nanoseconds()
sus concat_duration drip = concat_end - concat_start
assert_true(concat_duration < 50000000) // Less than 50ms
assert_true(len(concatenated) > 100000)

print_test_summary()
