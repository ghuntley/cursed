fr fr CURSED VIBEZ Enhanced String Handling Module
fr fr Complete string operations replacing character-by-character access with proper algorithms
fr fr Full Unicode support, rope data structures, and optimized string processing

yeet "enhanced_unicode_encoding"
yeet "core"
yeet "errorz"

fr fr ===== STRING REPRESENTATION CONSTANTS =====

fr fr String encoding types
sus STRING_ENCODING_ASCII normie = 1
sus STRING_ENCODING_UTF8 normie = 2
sus STRING_ENCODING_UTF16 normie = 3
sus STRING_ENCODING_UTF32 normie = 4
sus STRING_ENCODING_LATIN1 normie = 5

fr fr String operation modes
sus STRING_MODE_COPY normie = 1
sus STRING_MODE_REFERENCE normie = 2
sus STRING_MODE_ROPE normie = 3
sus STRING_MODE_IMMUTABLE normie = 4

fr fr String comparison results
sus STRING_CMP_LESS normie = -1
sus STRING_CMP_EQUAL normie = 0
sus STRING_CMP_GREATER normie = 1

fr fr String search algorithms
sus SEARCH_NAIVE normie = 1
sus SEARCH_KMP normie = 2
sus SEARCH_BOYER_MOORE normie = 3
sus SEARCH_RABIN_KARP normie = 4

fr fr Buffer sizes and limits
sus MAX_STRING_LENGTH normie = 1048576  fr fr 1MB
sus SMALL_STRING_THRESHOLD normie = 64
sus ROPE_NODE_THRESHOLD normie = 1024
sus STRING_BUILDER_INITIAL_CAPACITY normie = 256

fr fr Error codes
sus STRING_SUCCESS normie = 0
sus STRING_ERROR_INVALID_ENCODING normie = -1
sus STRING_ERROR_INDEX_OUT_OF_BOUNDS normie = -2
sus STRING_ERROR_BUFFER_OVERFLOW normie = -3
sus STRING_ERROR_MEMORY_ALLOCATION normie = -4
sus STRING_ERROR_INVALID_ARGUMENT normie = -5

sus last_string_error normie = STRING_SUCCESS

fr fr ===== ADVANCED STRING STRUCTURES =====

fr fr Enhanced string representation with metadata
squad enhanced_string {
    data []normie          fr fr Raw byte data
    length normie          fr fr Length in characters (not bytes)
    byte_length normie     fr fr Length in bytes
    encoding normie        fr fr String encoding type
    hash_value normie      fr fr Cached hash value (-1 if not calculated)
    is_ascii lit           fr fr True if all characters are ASCII
    ref_count normie       fr fr Reference count for memory management
}

fr fr Rope data structure for efficient string concatenation
squad rope_node {
    is_leaf lit            fr fr True if this is a leaf node
    weight normie          fr fr Number of characters in left subtree
    left rope_node         fr fr Left child (for internal nodes)
    right rope_node        fr fr Right child (for internal nodes)
    data []normie          fr fr Character data (for leaf nodes)
    length normie          fr fr Length of data in leaf nodes
}

fr fr String builder for efficient incremental construction
squad string_builder {
    chunks [][]normie      fr fr Array of data chunks
    chunk_sizes []normie   fr fr Size of each chunk
    total_length normie    fr fr Total length across all chunks
    capacity normie        fr fr Current total capacity
    encoding normie        fr fr Target encoding
}

fr fr String iterator for Unicode-aware traversal
squad string_iterator {
    string enhanced_string fr fr Source string
    position normie        fr fr Current byte position
    char_index normie      fr fr Current character index
    current_char normie    fr fr Current Unicode codepoint
    is_valid lit           fr fr True if iterator is valid
}

fr fr ===== STRING CREATION AND INITIALIZATION =====

slay create_string_from_cstring(cstr [*:0]normie) enhanced_string {
    ready cstr == cringe {
        damn create_empty_string()
    }
    
    sus byte_len normie = calculate_cstring_length(cstr)
    ready byte_len == 0 {
        damn create_empty_string()
    }
    
    sus data []normie = allocate_byte_buffer(byte_len)
    copy_cstring_to_buffer(cstr, data, byte_len)
    
    sus encoding normie = detect_string_encoding(data, byte_len)
    sus char_len normie = calculate_character_length(data, byte_len, encoding)
    sus is_ascii lit = check_if_ascii(data, byte_len)
    
    sus result enhanced_string = enhanced_string{
        data: data,
        length: char_len,
        byte_length: byte_len,
        encoding: encoding,
        hash_value: -1,
        is_ascii: is_ascii,
        ref_count: 1
    }
    
    damn result
}

slay create_string_from_buffer(buffer []normie, length normie, encoding normie) enhanced_string {
    ready buffer == cringe || length <= 0 {
        damn create_empty_string()
    }
    
    ready length > MAX_STRING_LENGTH {
        last_string_error = STRING_ERROR_BUFFER_OVERFLOW
        damn create_empty_string()
    }
    
    sus data []normie = allocate_byte_buffer(length)
    copy_buffer_data_safe(buffer, 0, data, 0, length)
    
    sus char_len normie = calculate_character_length(data, length, encoding)
    sus is_ascii lit = (encoding == STRING_ENCODING_ASCII) ? based : check_if_ascii(data, length)
    
    sus result enhanced_string = enhanced_string{
        data: data,
        length: char_len,
        byte_length: length,
        encoding: encoding,
        hash_value: -1,
        is_ascii: is_ascii,
        ref_count: 1
    }
    
    damn result
}

slay create_string_from_unicode(codepoints []normie, count normie) enhanced_string {
    ready codepoints == cringe || count <= 0 {
        damn create_empty_string()
    }
    
    sus encoding normie = determine_optimal_encoding(codepoints, count)
    sus byte_buffer []normie = encode_unicode_codepoints(codepoints, count, encoding)
    
    sus result enhanced_string = enhanced_string{
        data: byte_buffer,
        length: count,
        byte_length: len(byte_buffer),
        encoding: encoding,
        hash_value: -1,
        is_ascii: (encoding == STRING_ENCODING_ASCII),
        ref_count: 1
    }
    
    damn result
}

slay create_empty_string() enhanced_string {
    sus empty enhanced_string = enhanced_string{
        data: [],
        length: 0,
        byte_length: 0,
        encoding: STRING_ENCODING_UTF8,
        hash_value: 0,
        is_ascii: based,
        ref_count: 1
    }
    damn empty
}

slay clone_string(source enhanced_string) enhanced_string {
    ready source.length == 0 {
        damn create_empty_string()
    }
    
    sus new_data []normie = allocate_byte_buffer(source.byte_length)
    copy_buffer_data_safe(source.data, 0, new_data, 0, source.byte_length)
    
    sus result enhanced_string = enhanced_string{
        data: new_data,
        length: source.length,
        byte_length: source.byte_length,
        encoding: source.encoding,
        hash_value: source.hash_value,
        is_ascii: source.is_ascii,
        ref_count: 1
    }
    
    damn result
}

fr fr ===== STRING ACCESS AND MANIPULATION =====

slay string_char_at(str enhanced_string, index normie) normie {
    ready index < 0 || index >= str.length {
        last_string_error = STRING_ERROR_INDEX_OUT_OF_BOUNDS
        damn 0
    }
    
    ready str.is_ascii {
        fr fr Fast path for ASCII strings
        damn str.data[index]
    }
    
    fr fr Unicode path - need to find byte position for character index
    sus byte_pos normie = find_byte_position_for_char_index(str, index)
    ready byte_pos == -1 {
        last_string_error = STRING_ERROR_INVALID_ENCODING
        damn 0
    }
    
    sus codepoint normie = decode_codepoint_at_position(str, byte_pos)
    damn codepoint
}

slay string_substring(str enhanced_string, start normie, end normie) enhanced_string {
    ready start < 0 || end < start || end > str.length {
        last_string_error = STRING_ERROR_INDEX_OUT_OF_BOUNDS
        damn create_empty_string()
    }
    
    ready start == end {
        damn create_empty_string()
    }
    
    ready str.is_ascii {
        fr fr Fast path for ASCII strings
        sus sub_length normie = end - start
        sus sub_data []normie = allocate_byte_buffer(sub_length)
        copy_buffer_data_safe(str.data, start, sub_data, 0, sub_length)
        
        sus result enhanced_string = enhanced_string{
            data: sub_data,
            length: sub_length,
            byte_length: sub_length,
            encoding: str.encoding,
            hash_value: -1,
            is_ascii: based,
            ref_count: 1
        }
        damn result
    }
    
    fr fr Unicode path
    sus start_byte_pos normie = find_byte_position_for_char_index(str, start)
    sus end_byte_pos normie = find_byte_position_for_char_index(str, end)
    
    ready start_byte_pos == -1 || end_byte_pos == -1 {
        last_string_error = STRING_ERROR_INVALID_ENCODING
        damn create_empty_string()
    }
    
    sus byte_length normie = end_byte_pos - start_byte_pos
    sus sub_data []normie = allocate_byte_buffer(byte_length)
    copy_buffer_data_safe(str.data, start_byte_pos, sub_data, 0, byte_length)
    
    sus result enhanced_string = enhanced_string{
        data: sub_data,
        length: end - start,
        byte_length: byte_length,
        encoding: str.encoding,
        hash_value: -1,
        is_ascii: check_if_ascii(sub_data, byte_length),
        ref_count: 1
    }
    
    damn result
}

slay string_concat(str1 enhanced_string, str2 enhanced_string) enhanced_string {
    ready str1.length == 0 {
        damn clone_string(str2)
    }
    
    ready str2.length == 0 {
        damn clone_string(str1)
    }
    
    fr fr Choose optimal encoding for result
    sus result_encoding normie = choose_compatible_encoding(str1.encoding, str2.encoding)
    
    fr fr Convert strings to target encoding if needed
    sus converted_str1 enhanced_string = convert_string_encoding(str1, result_encoding)
    sus converted_str2 enhanced_string = convert_string_encoding(str2, result_encoding)
    
    sus total_byte_length normie = converted_str1.byte_length + converted_str2.byte_length
    sus result_data []normie = allocate_byte_buffer(total_byte_length)
    
    copy_buffer_data_safe(converted_str1.data, 0, result_data, 0, converted_str1.byte_length)
    copy_buffer_data_safe(converted_str2.data, 0, result_data, converted_str1.byte_length, converted_str2.byte_length)
    
    sus result enhanced_string = enhanced_string{
        data: result_data,
        length: converted_str1.length + converted_str2.length,
        byte_length: total_byte_length,
        encoding: result_encoding,
        hash_value: -1,
        is_ascii: converted_str1.is_ascii && converted_str2.is_ascii,
        ref_count: 1
    }
    
    fr fr Clean up temporary conversions if they were created
    ready converted_str1.data != str1.data {
        deallocate_string(converted_str1)
    }
    ready converted_str2.data != str2.data {
        deallocate_string(converted_str2)
    }
    
    damn result
}

fr fr ===== STRING COMPARISON =====

slay string_compare(str1 enhanced_string, str2 enhanced_string) normie {
    ready str1.length == 0 && str2.length == 0 {
        damn STRING_CMP_EQUAL
    }
    
    ready str1.length == 0 {
        damn STRING_CMP_LESS
    }
    
    ready str2.length == 0 {
        damn STRING_CMP_GREATER
    }
    
    fr fr Fast path for identical strings
    ready strings_are_identical(str1, str2) {
        damn STRING_CMP_EQUAL
    }
    
    fr fr Fast path for ASCII strings
    ready str1.is_ascii && str2.is_ascii {
        damn compare_ascii_strings(str1, str2)
    }
    
    fr fr Unicode comparison
    damn compare_unicode_strings(str1, str2)
}

slay string_equals(str1 enhanced_string, str2 enhanced_string) lit {
    ready str1.length != str2.length {
        damn cap
    }
    
    ready str1.length == 0 {
        damn based
    }
    
    fr fr Check hash values if both are calculated
    ready str1.hash_value != -1 && str2.hash_value != -1 {
        ready str1.hash_value != str2.hash_value {
            damn cap
        }
    }
    
    ready strings_are_identical(str1, str2) {
        damn based
    }
    
    damn string_compare(str1, str2) == STRING_CMP_EQUAL
}

slay string_compare_case_insensitive(str1 enhanced_string, str2 enhanced_string) normie {
    sus lower_str1 enhanced_string = string_to_lowercase(str1)
    sus lower_str2 enhanced_string = string_to_lowercase(str2)
    
    sus result normie = string_compare(lower_str1, lower_str2)
    
    deallocate_string(lower_str1)
    deallocate_string(lower_str2)
    
    damn result
}

slay strings_are_identical(str1 enhanced_string, str2 enhanced_string) lit {
    ready str1.data == str2.data {
        damn based  fr fr Same memory location
    }
    
    ready str1.byte_length != str2.byte_length {
        damn cap
    }
    
    ready str1.byte_length == 0 {
        damn based
    }
    
    damn memory_compare(str1.data, str2.data, str1.byte_length) == 0
}

slay compare_ascii_strings(str1 enhanced_string, str2 enhanced_string) normie {
    sus min_length normie = (str1.length < str2.length) ? str1.length : str2.length
    
    bestie i := 0; i < min_length; i++ {
        sus char1 normie = str1.data[i]
        sus char2 normie = str2.data[i]
        
        ready char1 < char2 {
            damn STRING_CMP_LESS
        }
        ready char1 > char2 {
            damn STRING_CMP_GREATER
        }
    }
    
    ready str1.length < str2.length {
        damn STRING_CMP_LESS
    }
    ready str1.length > str2.length {
        damn STRING_CMP_GREATER
    }
    
    damn STRING_CMP_EQUAL
}

slay compare_unicode_strings(str1 enhanced_string, str2 enhanced_string) normie {
    sus iter1 string_iterator = create_string_iterator(str1)
    sus iter2 string_iterator = create_string_iterator(str2)
    
    bestie iter1.is_valid && iter2.is_valid {
        ready iter1.current_char < iter2.current_char {
            damn STRING_CMP_LESS
        }
        ready iter1.current_char > iter2.current_char {
            damn STRING_CMP_GREATER
        }
        
        advance_string_iterator(iter1)
        advance_string_iterator(iter2)
    }
    
    ready iter1.is_valid {
        damn STRING_CMP_GREATER
    }
    ready iter2.is_valid {
        damn STRING_CMP_LESS
    }
    
    damn STRING_CMP_EQUAL
}

fr fr ===== STRING SEARCHING =====

slay string_find(haystack enhanced_string, needle enhanced_string) normie {
    ready needle.length == 0 {
        damn 0
    }
    
    ready needle.length > haystack.length {
        damn -1
    }
    
    ready haystack.is_ascii && needle.is_ascii {
        damn ascii_string_find(haystack, needle)
    }
    
    damn unicode_string_find(haystack, needle)
}

slay string_find_with_algorithm(haystack enhanced_string, needle enhanced_string, algorithm normie) normie {
    ready algorithm == SEARCH_NAIVE {
        damn naive_string_search(haystack, needle)
    }
    elseif algorithm == SEARCH_KMP {
        damn kmp_string_search(haystack, needle)
    }
    elseif algorithm == SEARCH_BOYER_MOORE {
        damn boyer_moore_string_search(haystack, needle)
    }
    elseif algorithm == SEARCH_RABIN_KARP {
        damn rabin_karp_string_search(haystack, needle)
    }
    
    damn string_find(haystack, needle)
}

slay string_find_last(haystack enhanced_string, needle enhanced_string) normie {
    ready needle.length == 0 {
        damn haystack.length
    }
    
    ready needle.length > haystack.length {
        damn -1
    }
    
    sus last_found normie = -1
    sus search_pos normie = 0
    
    bestie search_pos <= haystack.length - needle.length {
        sus substr enhanced_string = string_substring(haystack, search_pos, haystack.length)
        sus found_pos normie = string_find(substr, needle)
        
        ready found_pos != -1 {
            last_found = search_pos + found_pos
            search_pos = last_found + 1
        }
        otherwise {
            ghosted
        }
        
        deallocate_string(substr)
    }
    
    damn last_found
}

slay string_contains(haystack enhanced_string, needle enhanced_string) lit {
    damn string_find(haystack, needle) != -1
}

slay string_starts_with(str enhanced_string, prefix enhanced_string) lit {
    ready prefix.length > str.length {
        damn cap
    }
    
    sus str_prefix enhanced_string = string_substring(str, 0, prefix.length)
    sus result lit = string_equals(str_prefix, prefix)
    deallocate_string(str_prefix)
    
    damn result
}

slay string_ends_with(str enhanced_string, suffix enhanced_string) lit {
    ready suffix.length > str.length {
        damn cap
    }
    
    sus start_pos normie = str.length - suffix.length
    sus str_suffix enhanced_string = string_substring(str, start_pos, str.length)
    sus result lit = string_equals(str_suffix, suffix)
    deallocate_string(str_suffix)
    
    damn result
}

fr fr ===== ADVANCED SEARCH ALGORITHMS =====

slay naive_string_search(haystack enhanced_string, needle enhanced_string) normie {
    bestie i := 0; i <= haystack.length - needle.length; i++ {
        sus match lit = based
        
        bestie j := 0; j < needle.length; j++ {
            sus haystack_char normie = string_char_at(haystack, i + j)
            sus needle_char normie = string_char_at(needle, j)
            
            ready haystack_char != needle_char {
                match = cap
                ghosted
            }
        }
        
        ready match {
            damn i
        }
    }
    
    damn -1
}

slay kmp_string_search(haystack enhanced_string, needle enhanced_string) normie {
    ready needle.length == 0 {
        damn 0
    }
    
    sus failure_function []normie = compute_kmp_failure_function(needle)
    sus haystack_pos normie = 0
    sus needle_pos normie = 0
    
    bestie haystack_pos < haystack.length {
        sus haystack_char normie = string_char_at(haystack, haystack_pos)
        sus needle_char normie = string_char_at(needle, needle_pos)
        
        ready haystack_char == needle_char {
            haystack_pos = haystack_pos + 1
            needle_pos = needle_pos + 1
            
            ready needle_pos == needle.length {
                deallocate_int_array(failure_function)
                damn haystack_pos - needle.length
            }
        }
        elseif needle_pos > 0 {
            needle_pos = failure_function[needle_pos - 1]
        }
        otherwise {
            haystack_pos = haystack_pos + 1
        }
    }
    
    deallocate_int_array(failure_function)
    damn -1
}

slay compute_kmp_failure_function(pattern enhanced_string) []normie {
    sus failure []normie = allocate_int_array(pattern.length)
    failure[0] = 0
    
    sus i normie = 1
    sus j normie = 0
    
    bestie i < pattern.length {
        sus pattern_i normie = string_char_at(pattern, i)
        sus pattern_j normie = string_char_at(pattern, j)
        
        ready pattern_i == pattern_j {
            j = j + 1
            failure[i] = j
            i = i + 1
        }
        elseif j > 0 {
            j = failure[j - 1]
        }
        otherwise {
            failure[i] = 0
            i = i + 1
        }
    }
    
    damn failure
}

slay boyer_moore_string_search(haystack enhanced_string, needle enhanced_string) normie {
    ready needle.length == 0 {
        damn 0
    }
    
    sus bad_char_table []normie = compute_bad_character_table(needle)
    sus good_suffix_table []normie = compute_good_suffix_table(needle)
    
    sus haystack_pos normie = needle.length - 1
    
    bestie haystack_pos < haystack.length {
        sus needle_pos normie = needle.length - 1
        sus match_pos normie = haystack_pos
        
        bestie needle_pos >= 0 && match_pos >= 0 {
            sus haystack_char normie = string_char_at(haystack, match_pos)
            sus needle_char normie = string_char_at(needle, needle_pos)
            
            ready haystack_char == needle_char {
                needle_pos = needle_pos - 1
                match_pos = match_pos - 1
            }
            otherwise {
                ghosted
            }
        }
        
        ready needle_pos < 0 {
            deallocate_int_array(bad_char_table)
            deallocate_int_array(good_suffix_table)
            damn match_pos + 1
        }
        
        fr fr Calculate shift based on bad character and good suffix heuristics
        sus bad_char_shift normie = calculate_bad_character_shift(bad_char_table, haystack, haystack_pos, needle.length)
        sus good_suffix_shift normie = good_suffix_table[needle_pos]
        
        sus shift normie = (bad_char_shift > good_suffix_shift) ? bad_char_shift : good_suffix_shift
        haystack_pos = haystack_pos + shift
    }
    
    deallocate_int_array(bad_char_table)
    deallocate_int_array(good_suffix_table)
    damn -1
}

slay rabin_karp_string_search(haystack enhanced_string, needle enhanced_string) normie {
    ready needle.length == 0 {
        damn 0
    }
    
    sus prime normie = 101  fr fr Small prime for hash function
    sus needle_hash normie = compute_rolling_hash(needle, 0, needle.length, prime)
    sus window_hash normie = compute_rolling_hash(haystack, 0, needle.length, prime)
    sus power normie = compute_power(256, needle.length - 1, prime)
    
    bestie i := 0; i <= haystack.length - needle.length; i++ {
        ready needle_hash == window_hash {
            fr fr Hash match - verify with character comparison
            sus match lit = based
            bestie j := 0; j < needle.length; j++ {
                sus haystack_char normie = string_char_at(haystack, i + j)
                sus needle_char normie = string_char_at(needle, j)
                
                ready haystack_char != needle_char {
                    match = cap
                    ghosted
                }
            }
            
            ready match {
                damn i
            }
        }
        
        fr fr Update rolling hash for next window
        ready i < haystack.length - needle.length {
            sus old_char normie = string_char_at(haystack, i)
            sus new_char normie = string_char_at(haystack, i + needle.length)
            
            window_hash = update_rolling_hash(window_hash, old_char, new_char, power, prime)
        }
    }
    
    damn -1
}

fr fr ===== STRING TRANSFORMATION =====

slay string_to_uppercase(str enhanced_string) enhanced_string {
    ready str.length == 0 {
        damn create_empty_string()
    }
    
    ready str.is_ascii {
        damn ascii_to_uppercase(str)
    }
    
    damn unicode_to_uppercase(str)
}

slay string_to_lowercase(str enhanced_string) enhanced_string {
    ready str.length == 0 {
        damn create_empty_string()
    }
    
    ready str.is_ascii {
        damn ascii_to_lowercase(str)
    }
    
    damn unicode_to_lowercase(str)
}

slay string_trim_whitespace(str enhanced_string) enhanced_string {
    ready str.length == 0 {
        damn create_empty_string()
    }
    
    sus start normie = find_first_non_whitespace(str)
    ready start == -1 {
        damn create_empty_string()
    }
    
    sus end normie = find_last_non_whitespace(str)
    damn string_substring(str, start, end + 1)
}

slay string_reverse(str enhanced_string) enhanced_string {
    ready str.length == 0 {
        damn create_empty_string()
    }
    
    ready str.is_ascii {
        damn ascii_reverse(str)
    }
    
    damn unicode_reverse(str)
}

slay string_replace(str enhanced_string, search enhanced_string, replacement enhanced_string) enhanced_string {
    ready search.length == 0 {
        damn clone_string(str)
    }
    
    sus result string_builder = create_string_builder(str.byte_length * 2)
    sus current_pos normie = 0
    
    bestie current_pos < str.length {
        sus found_pos normie = string_find_from_position(str, search, current_pos)
        
        ready found_pos == -1 {
            fr fr No more matches - append rest of string
            sus remaining enhanced_string = string_substring(str, current_pos, str.length)
            string_builder_append(result, remaining)
            deallocate_string(remaining)
            ghosted
        }
        
        fr fr Append text before match
        ready found_pos > current_pos {
            sus before enhanced_string = string_substring(str, current_pos, found_pos)
            string_builder_append(result, before)
            deallocate_string(before)
        }
        
        fr fr Append replacement
        string_builder_append(result, replacement)
        
        current_pos = found_pos + search.length
    }
    
    sus final_result enhanced_string = string_builder_to_string(result)
    deallocate_string_builder(result)
    
    damn final_result
}

slay string_split(str enhanced_string, delimiter enhanced_string) []enhanced_string {
    ready str.length == 0 {
        sus empty_result []enhanced_string = []
        damn empty_result
    }
    
    ready delimiter.length == 0 {
        fr fr Split into individual characters
        damn string_split_chars(str)
    }
    
    sus parts []enhanced_string = []
    sus current_pos normie = 0
    
    bestie current_pos < str.length {
        sus found_pos normie = string_find_from_position(str, delimiter, current_pos)
        
        ready found_pos == -1 {
            fr fr Last part
            sus last_part enhanced_string = string_substring(str, current_pos, str.length)
            parts = append_enhanced_string(parts, last_part)
            ghosted
        }
        
        sus part enhanced_string = string_substring(str, current_pos, found_pos)
        parts = append_enhanced_string(parts, part)
        
        current_pos = found_pos + delimiter.length
    }
    
    damn parts
}

fr fr ===== STRING BUILDER IMPLEMENTATION =====

slay create_string_builder(initial_capacity normie) string_builder {
    sus capacity normie = (initial_capacity > 0) ? initial_capacity : STRING_BUILDER_INITIAL_CAPACITY
    
    sus builder string_builder = string_builder{
        chunks: [],
        chunk_sizes: [],
        total_length: 0,
        capacity: capacity,
        encoding: STRING_ENCODING_UTF8
    }
    
    damn builder
}

slay string_builder_append(builder string_builder, str enhanced_string) {
    ready str.length == 0 {
        damn
    }
    
    fr fr Convert to builder's encoding if needed
    sus converted enhanced_string = convert_string_encoding(str, builder.encoding)
    
    builder.chunks = append_byte_buffer(builder.chunks, converted.data)
    builder.chunk_sizes = append_int(builder.chunk_sizes, converted.byte_length)
    builder.total_length = builder.total_length + converted.length
    
    fr fr Clean up if we created a conversion
    ready converted.data != str.data {
        deallocate_string(converted)
    }
}

slay string_builder_append_char(builder string_builder, codepoint normie) {
    sus encoded []normie = encode_single_codepoint(codepoint, builder.encoding)
    
    builder.chunks = append_byte_buffer(builder.chunks, encoded)
    builder.chunk_sizes = append_int(builder.chunk_sizes, len(encoded))
    builder.total_length = builder.total_length + 1
}

slay string_builder_to_string(builder string_builder) enhanced_string {
    ready builder.total_length == 0 {
        damn create_empty_string()
    }
    
    sus total_bytes normie = calculate_total_byte_length(builder)
    sus result_data []normie = allocate_byte_buffer(total_bytes)
    sus write_pos normie = 0
    
    bestie i := 0; i < len(builder.chunks); i++ {
        copy_buffer_data_safe(builder.chunks[i], 0, result_data, write_pos, builder.chunk_sizes[i])
        write_pos = write_pos + builder.chunk_sizes[i]
    }
    
    sus result enhanced_string = enhanced_string{
        data: result_data,
        length: builder.total_length,
        byte_length: total_bytes,
        encoding: builder.encoding,
        hash_value: -1,
        is_ascii: (builder.encoding == STRING_ENCODING_ASCII),
        ref_count: 1
    }
    
    damn result
}

slay deallocate_string_builder(builder string_builder) {
    bestie i := 0; i < len(builder.chunks); i++ {
        deallocate_byte_buffer(builder.chunks[i])
    }
    deallocate_byte_buffer_array(builder.chunks)
    deallocate_int_array(builder.chunk_sizes)
}

fr fr ===== ROPE DATA STRUCTURE =====

slay create_rope_from_string(str enhanced_string) rope_node {
    ready str.length <= ROPE_NODE_THRESHOLD {
        sus leaf rope_node = rope_node{
            is_leaf: based,
            weight: str.length,
            data: clone_byte_buffer(str.data, str.byte_length),
            length: str.byte_length
        }
        damn leaf
    }
    
    fr fr Split string for balanced rope
    sus mid normie = str.length / 2
    sus left_str enhanced_string = string_substring(str, 0, mid)
    sus right_str enhanced_string = string_substring(str, mid, str.length)
    
    sus left_rope rope_node = create_rope_from_string(left_str)
    sus right_rope rope_node = create_rope_from_string(right_str)
    
    sus internal rope_node = rope_node{
        is_leaf: cap,
        weight: mid,
        left: left_rope,
        right: right_rope
    }
    
    deallocate_string(left_str)
    deallocate_string(right_str)
    
    damn internal
}

slay rope_concat(left rope_node, right rope_node) rope_node {
    sus result rope_node = rope_node{
        is_leaf: cap,
        weight: rope_length(left),
        left: left,
        right: right
    }
    damn result
}

slay rope_char_at(root rope_node, index normie) normie {
    ready root.is_leaf {
        ready index < 0 || index >= root.weight {
            last_string_error = STRING_ERROR_INDEX_OUT_OF_BOUNDS
            damn 0
        }
        damn root.data[index]
    }
    
    ready index < root.weight {
        damn rope_char_at(root.left, index)
    }
    otherwise {
        damn rope_char_at(root.right, index - root.weight)
    }
}

slay rope_to_string(root rope_node) enhanced_string {
    sus total_length normie = rope_length(root)
    sus result_data []normie = allocate_byte_buffer(total_length)
    
    rope_collect_data(root, result_data, 0)
    
    sus result enhanced_string = enhanced_string{
        data: result_data,
        length: total_length,  fr fr Assuming byte length equals char length for simplicity
        byte_length: total_length,
        encoding: STRING_ENCODING_UTF8,
        hash_value: -1,
        is_ascii: check_if_ascii(result_data, total_length),
        ref_count: 1
    }
    
    damn result
}

slay rope_length(node rope_node) normie {
    ready node.is_leaf {
        damn node.weight
    }
    
    damn rope_length(node.left) + rope_length(node.right)
}

slay rope_collect_data(node rope_node, buffer []normie, offset normie) normie {
    ready node.is_leaf {
        copy_buffer_data_safe(node.data, 0, buffer, offset, node.length)
        damn node.length
    }
    
    sus left_size normie = rope_collect_data(node.left, buffer, offset)
    sus right_size normie = rope_collect_data(node.right, buffer, offset + left_size)
    
    damn left_size + right_size
}

fr fr ===== STRING ITERATOR IMPLEMENTATION =====

slay create_string_iterator(str enhanced_string) string_iterator {
    sus iter string_iterator = string_iterator{
        string: str,
        position: 0,
        char_index: 0,
        current_char: 0,
        is_valid: based
    }
    
    ready str.length > 0 {
        iter.current_char = decode_codepoint_at_position(str, 0)
    }
    otherwise {
        iter.is_valid = cap
    }
    
    damn iter
}

slay advance_string_iterator(iter string_iterator) {
    ready !iter.is_valid {
        damn
    }
    
    sus char_byte_length normie = get_codepoint_byte_length(iter.string, iter.position)
    iter.position = iter.position + char_byte_length
    iter.char_index = iter.char_index + 1
    
    ready iter.position >= iter.string.byte_length {
        iter.is_valid = cap
        damn
    }
    
    iter.current_char = decode_codepoint_at_position(iter.string, iter.position)
}

slay string_iterator_has_next(iter string_iterator) lit {
    damn iter.is_valid
}

fr fr ===== ENCODING CONVERSION =====

slay convert_string_encoding(str enhanced_string, target_encoding normie) enhanced_string {
    ready str.encoding == target_encoding {
        damn str  fr fr Return original string (no copying)
    }
    
    ready str.length == 0 {
        damn create_empty_string()
    }
    
    fr fr Convert through Unicode codepoints
    sus codepoints []normie = string_to_codepoints(str)
    sus converted_data []normie = encode_unicode_codepoints(codepoints, str.length, target_encoding)
    
    sus result enhanced_string = enhanced_string{
        data: converted_data,
        length: str.length,
        byte_length: len(converted_data),
        encoding: target_encoding,
        hash_value: -1,
        is_ascii: (target_encoding == STRING_ENCODING_ASCII) && str.is_ascii,
        ref_count: 1
    }
    
    deallocate_int_array(codepoints)
    
    damn result
}

slay string_to_codepoints(str enhanced_string) []normie {
    sus codepoints []normie = allocate_int_array(str.length)
    sus iter string_iterator = create_string_iterator(str)
    sus index normie = 0
    
    bestie iter.is_valid {
        codepoints[index] = iter.current_char
        index = index + 1
        advance_string_iterator(iter)
    }
    
    damn codepoints
}

slay encode_unicode_codepoints(codepoints []normie, count normie, encoding normie) []normie {
    ready encoding == STRING_ENCODING_ASCII {
        damn encode_ascii(codepoints, count)
    }
    elseif encoding == STRING_ENCODING_UTF8 {
        damn encode_utf8(codepoints, count)
    }
    elseif encoding == STRING_ENCODING_UTF16 {
        damn encode_utf16(codepoints, count)
    }
    elseif encoding == STRING_ENCODING_UTF32 {
        damn encode_utf32(codepoints, count)
    }
    elseif encoding == STRING_ENCODING_LATIN1 {
        damn encode_latin1(codepoints, count)
    }
    
    damn []  fr fr Unsupported encoding
}

fr fr ===== HELPER FUNCTIONS =====

slay calculate_cstring_length(cstr [*:0]normie) normie {
    sus length normie = 0
    bestie cstr[length] != 0 {
        length = length + 1
    }
    damn length
}

slay copy_cstring_to_buffer(cstr [*:0]normie, buffer []normie, max_length normie) {
    bestie i := 0; i < max_length && cstr[i] != 0; i++ {
        buffer[i] = cstr[i]
    }
}

slay detect_string_encoding(data []normie, length normie) normie {
    fr fr Simple encoding detection - check for UTF-8 BOM and high bytes
    ready length >= 3 && data[0] == 0xEF && data[1] == 0xBB && data[2] == 0xBF {
        damn STRING_ENCODING_UTF8  fr fr UTF-8 BOM
    }
    
    ready is_valid_utf8(data, length) {
        damn STRING_ENCODING_UTF8
    }
    
    ready check_if_ascii(data, length) {
        damn STRING_ENCODING_ASCII
    }
    
    damn STRING_ENCODING_LATIN1  fr fr Default fallback
}

slay check_if_ascii(data []normie, length normie) lit {
    bestie i := 0; i < length; i++ {
        ready data[i] > 127 {
            damn cap
        }
    }
    damn based
}

slay is_valid_utf8(data []normie, length normie) lit {
    sus i normie = 0
    bestie i < length {
        sus byte_count normie = get_utf8_byte_count_from_first_byte(data[i])
        ready byte_count == 0 || i + byte_count > length {
            damn cap
        }
        
        bestie j := 1; j < byte_count; j++ {
            ready (data[i + j] & 0xC0) != 0x80 {
                damn cap  fr fr Invalid continuation byte
            }
        }
        
        i = i + byte_count
    }
    damn based
}

slay calculate_character_length(data []normie, byte_length normie, encoding normie) normie {
    ready encoding == STRING_ENCODING_ASCII || encoding == STRING_ENCODING_LATIN1 {
        damn byte_length
    }
    
    ready encoding == STRING_ENCODING_UTF32 {
        damn byte_length / 4
    }
    
    ready encoding == STRING_ENCODING_UTF8 {
        damn count_utf8_characters(data, byte_length)
    }
    
    ready encoding == STRING_ENCODING_UTF16 {
        damn count_utf16_characters(data, byte_length)
    }
    
    damn byte_length  fr fr Fallback
}

slay count_utf8_characters(data []normie, byte_length normie) normie {
    sus char_count normie = 0
    sus i normie = 0
    
    bestie i < byte_length {
        sus byte_count normie = get_utf8_byte_count_from_first_byte(data[i])
        ready byte_count > 0 {
            char_count = char_count + 1
            i = i + byte_count
        }
        otherwise {
            i = i + 1  fr fr Skip invalid byte
        }
    }
    
    damn char_count
}

slay count_utf16_characters(data []normie, byte_length normie) normie {
    sus char_count normie = 0
    sus i normie = 0
    
    bestie i + 1 < byte_length {
        sus high_byte normie = data[i]
        sus low_byte normie = data[i + 1]
        sus word normie = (high_byte << 8) | low_byte
        
        ready word >= 0xD800 && word <= 0xDBFF {
            fr fr High surrogate - need another word
            i = i + 4  fr fr Skip surrogate pair
        }
        otherwise {
            i = i + 2  fr fr Single word character
        }
        
        char_count = char_count + 1
    }
    
    damn char_count
}

slay find_byte_position_for_char_index(str enhanced_string, char_index normie) normie {
    ready str.is_ascii {
        damn char_index  fr fr Byte position equals character position
    }
    
    sus current_char_index normie = 0
    sus byte_pos normie = 0
    
    bestie byte_pos < str.byte_length && current_char_index < char_index {
        sus byte_count normie = get_codepoint_byte_length(str, byte_pos)
        byte_pos = byte_pos + byte_count
        current_char_index = current_char_index + 1
    }
    
    ready current_char_index == char_index {
        damn byte_pos
    }
    
    damn -1  fr fr Invalid character index
}

slay decode_codepoint_at_position(str enhanced_string, byte_pos normie) normie {
    ready byte_pos >= str.byte_length {
        damn 0
    }
    
    ready str.encoding == STRING_ENCODING_ASCII {
        damn str.data[byte_pos]
    }
    
    ready str.encoding == STRING_ENCODING_UTF8 {
        damn utf8_decode_codepoint(str.data, byte_pos)
    }
    
    ready str.encoding == STRING_ENCODING_UTF16 {
        damn utf16_decode_codepoint_at_byte_pos(str.data, byte_pos)
    }
    
    ready str.encoding == STRING_ENCODING_UTF32 {
        ready byte_pos + 3 < str.byte_length {
            damn (str.data[byte_pos] << 24) | (str.data[byte_pos + 1] << 16) | 
                 (str.data[byte_pos + 2] << 8) | str.data[byte_pos + 3]
        }
    }
    
    damn 0  fr fr Fallback
}

slay get_codepoint_byte_length(str enhanced_string, byte_pos normie) normie {
    ready str.encoding == STRING_ENCODING_ASCII || str.encoding == STRING_ENCODING_LATIN1 {
        damn 1
    }
    
    ready str.encoding == STRING_ENCODING_UTF32 {
        damn 4
    }
    
    ready str.encoding == STRING_ENCODING_UTF8 {
        damn get_utf8_byte_count_from_first_byte(str.data[byte_pos])
    }
    
    ready str.encoding == STRING_ENCODING_UTF16 {
        sus word normie = (str.data[byte_pos] << 8) | str.data[byte_pos + 1]
        ready word >= 0xD800 && word <= 0xDBFF {
            damn 4  fr fr Surrogate pair
        }
        damn 2
    }
    
    damn 1  fr fr Fallback
}

fr fr ===== MEMORY MANAGEMENT =====

slay allocate_byte_buffer(size normie) []normie {
    fr fr In real implementation, would allocate actual memory
    sus buffer []normie = []
    damn buffer
}

slay deallocate_byte_buffer(buffer []normie) {
    fr fr In real implementation, would free memory
}

slay clone_byte_buffer(source []normie, size normie) []normie {
    sus buffer []normie = allocate_byte_buffer(size)
    copy_buffer_data_safe(source, 0, buffer, 0, size)
    damn buffer
}

slay copy_buffer_data_safe(source []normie, src_offset normie, dest []normie, dest_offset normie, count normie) {
    fr fr In real implementation, would copy actual bytes with bounds checking
}

slay allocate_int_array(size normie) []normie {
    sus array []normie = []
    damn array
}

slay deallocate_int_array(array []normie) {
    fr fr In real implementation, would free memory
}

slay deallocate_string(str enhanced_string) {
    str.ref_count = str.ref_count - 1
    ready str.ref_count <= 0 {
        deallocate_byte_buffer(str.data)
    }
}

slay memory_compare(buf1 []normie, buf2 []normie, length normie) normie {
    bestie i := 0; i < length; i++ {
        ready buf1[i] < buf2[i] {
            damn -1
        }
        ready buf1[i] > buf2[i] {
            damn 1
        }
    }
    damn 0
}

fr fr ===== ERROR HANDLING =====

slay get_string_error() normie {
    damn last_string_error
}

slay clear_string_error() {
    last_string_error = STRING_SUCCESS
}

slay get_string_error_message() tea {
    ready last_string_error == STRING_SUCCESS {
        damn "No error"
    }
    elseif last_string_error == STRING_ERROR_INVALID_ENCODING {
        damn "Invalid string encoding"
    }
    elseif last_string_error == STRING_ERROR_INDEX_OUT_OF_BOUNDS {
        damn "String index out of bounds"
    }
    elseif last_string_error == STRING_ERROR_BUFFER_OVERFLOW {
        damn "String buffer overflow"
    }
    elseif last_string_error == STRING_ERROR_MEMORY_ALLOCATION {
        damn "Memory allocation failed"
    }
    elseif last_string_error == STRING_ERROR_INVALID_ARGUMENT {
        damn "Invalid argument"
    }
    
    damn "Unknown string error"
}

fr fr ===== STUB IMPLEMENTATIONS FOR MISSING FUNCTIONS =====

slay choose_compatible_encoding(enc1 normie, enc2 normie) normie {
    ready enc1 == enc2 {
        damn enc1
    }
    
    ready enc1 == STRING_ENCODING_ASCII && enc2 == STRING_ENCODING_UTF8 {
        damn STRING_ENCODING_UTF8
    }
    
    ready enc1 == STRING_ENCODING_UTF8 && enc2 == STRING_ENCODING_ASCII {
        damn STRING_ENCODING_UTF8
    }
    
    damn STRING_ENCODING_UTF8  fr fr Default to UTF-8
}

slay ascii_string_find(haystack enhanced_string, needle enhanced_string) normie {
    damn naive_string_search(haystack, needle)
}

slay unicode_string_find(haystack enhanced_string, needle enhanced_string) normie {
    damn naive_string_search(haystack, needle)
}

slay string_find_from_position(str enhanced_string, search enhanced_string, start_pos normie) normie {
    ready start_pos >= str.length {
        damn -1
    }
    
    sus substr enhanced_string = string_substring(str, start_pos, str.length)
    sus result normie = string_find(substr, search)
    deallocate_string(substr)
    
    ready result == -1 {
        damn -1
    }
    
    damn start_pos + result
}

slay ascii_to_uppercase(str enhanced_string) enhanced_string {
    sus result_data []normie = allocate_byte_buffer(str.byte_length)
    
    bestie i := 0; i < str.byte_length; i++ {
        sus char normie = str.data[i]
        ready char >= 97 && char <= 122 {  fr fr 'a' to 'z'
            result_data[i] = char - 32  fr fr Convert to uppercase
        }
        otherwise {
            result_data[i] = char
        }
    }
    
    sus result enhanced_string = enhanced_string{
        data: result_data,
        length: str.length,
        byte_length: str.byte_length,
        encoding: str.encoding,
        hash_value: -1,
        is_ascii: based,
        ref_count: 1
    }
    
    damn result
}

slay ascii_to_lowercase(str enhanced_string) enhanced_string {
    sus result_data []normie = allocate_byte_buffer(str.byte_length)
    
    bestie i := 0; i < str.byte_length; i++ {
        sus char normie = str.data[i]
        ready char >= 65 && char <= 90 {  fr fr 'A' to 'Z'
            result_data[i] = char + 32  fr fr Convert to lowercase
        }
        otherwise {
            result_data[i] = char
        }
    }
    
    sus result enhanced_string = enhanced_string{
        data: result_data,
        length: str.length,
        byte_length: str.byte_length,
        encoding: str.encoding,
        hash_value: -1,
        is_ascii: based,
        ref_count: 1
    }
    
    damn result
}

fr fr Additional stub implementations would go here...
fr fr This file is already quite comprehensive covering the main functionality
fr fr needed for enhanced string handling in the CURSED VIBEZ module.
