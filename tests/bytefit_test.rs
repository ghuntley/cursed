/// Comprehensive tests for the ByteFit byte manipulation library
use cursed::stdlib::bytefit::*;

#[cfg(test)]
mod tests {
    use super::*;

    // ================================
    // BASIC OPERATIONS TESTS
    // ================================

    #[test]
    fn test_basic_compare() {
        assert_eq!(compare(b"abc", b"abc"), 0);
        assert_eq!(compare(b"abc", b"def"), -1);
        assert_eq!(compare(b"def", b"abc"), 1);
        assert_eq!(compare(b"", b""), 0);
        assert_eq!(compare(b"a", b""), 1);
        assert_eq!(compare(b"", b"a"), -1);
    }

    #[test]
    fn test_basic_equal() {
        assert!(equal(b"hello", b"hello"));
        assert!(!equal(b"hello", b"world"));
        assert!(!equal(b"hello", b"hello!"));
        assert!(equal(b"", b""));
    }

    #[test]
    fn test_basic_equal_fold() {
        assert!(equal_fold(b"Hello", b"HELLO"));
        assert!(equal_fold(b"World", b"world"));
        assert!(!equal_fold(b"hello", b"world"));
        assert!(equal_fold(b"", b""));
    }

    #[test]
    fn test_basic_repeat() {
        assert_eq!(repeat(b"abc", 3), b"abcabcabc");
        assert_eq!(repeat(b"x", 5), b"xxxxx");
        assert_eq!(repeat(b"hello", 0), b"");
        assert_eq!(repeat(b"", 10), b"");
    }

    #[test]
    fn test_basic_runes() {
        assert_eq!(runes(b"hello").unwrap(), vec!['h', 'e', 'l', 'l', 'o']);
        assert_eq!(runes(b"").unwrap(), vec![]);
        assert_eq!(runes("🦀".as_bytes()).unwrap(), vec!['🦀']);
        
        // Test invalid UTF-8
        let invalid_utf8 = vec![0xFF, 0xFE];
        assert!(runes(&invalid_utf8).is_err());
    }

    // ================================
    // SEARCH FUNCTIONS TESTS
    // ================================

    #[test]
    fn test_search_contains() {
        assert!(contains(b"hello world", b"world"));
        assert!(contains(b"hello world", b"hello"));
        assert!(!contains(b"hello world", b"foo"));
        assert!(contains(b"hello world", b""));
        assert!(!contains(b"", b"hello"));
    }

    #[test]
    fn test_search_contains_any() {
        assert!(contains_any(b"hello", "aeiou"));
        assert!(!contains_any(b"bcdfg", "aeiou"));
        assert!(contains_any("café".as_bytes(), "é"));
    }

    #[test]
    fn test_search_contains_rune() {
        assert!(contains_rune(b"hello", 'e'));
        assert!(!contains_rune(b"hello", 'x'));
        assert!(contains_rune("café".as_bytes(), 'é'));
    }

    #[test]
    fn test_search_count() {
        assert_eq!(count(b"aaaa", b"aa"), 2);
        assert_eq!(count(b"hello world hello", b"hello"), 2);
        assert_eq!(count(b"hello", b"x"), 0);
        assert_eq!(count(b"hello", b""), 6);
    }

    #[test]
    fn test_search_prefix_suffix() {
        assert!(has_prefix(b"hello world", b"hello"));
        assert!(!has_prefix(b"hello world", b"world"));
        assert!(has_prefix(b"hello", b""));
        assert!(!has_prefix(b"", b"hello"));

        assert!(has_suffix(b"hello world", b"world"));
        assert!(!has_suffix(b"hello world", b"hello"));
        assert!(has_suffix(b"hello", b""));
        assert!(!has_suffix(b"", b"hello"));
    }

    #[test]
    fn test_search_index() {
        assert_eq!(index(b"hello world", b"world"), 6);
        assert_eq!(index(b"hello world", b"hello"), 0);
        assert_eq!(index(b"hello world", b"foo"), -1);
        assert_eq!(index(b"hello", b""), 0);
    }

    #[test]
    fn test_search_index_byte() {
        assert_eq!(index_byte(b"hello", b'e'), 1);
        assert_eq!(index_byte(b"hello", b'x'), -1);
        assert_eq!(index_byte(b"hello", b'h'), 0);
    }

    #[test]
    fn test_search_last_index() {
        assert_eq!(last_index(b"hello world hello", b"hello"), 12);
        assert_eq!(last_index(b"hello world", b"foo"), -1);
        assert_eq!(last_index(b"hello", b""), 5);
    }

    #[test]
    fn test_search_last_index_byte() {
        assert_eq!(last_index_byte(b"hello", b'l'), 3);
        assert_eq!(last_index_byte(b"hello", b'x'), -1);
        assert_eq!(last_index_byte(b"hello", b'h'), 0);
    }

    // ================================
    // TRANSFORMATION TESTS
    // ================================

    #[test]
    fn test_transform_join() {
        let parts = vec![b"hello", b"world", b"test"];
        assert_eq!(join(&parts, b", "), b"hello, world, test");
        
        let empty: Vec<&[u8]> = vec![];
        assert_eq!(join(&empty, b", "), b"");
        
        assert_eq!(join(&[b"single"], b", "), b"single");
    }

    #[test]
    fn test_transform_replace() {
        assert_eq!(replace(b"hello world hello", b"hello", b"hi", 1), b"hi world hello");
        assert_eq!(replace(b"hello world hello", b"hello", b"hi", 2), b"hi world hi");
        assert_eq!(replace(b"hello world", b"foo", b"bar", 1), b"hello world");
        assert_eq!(replace(b"", b"hello", b"hi", 1), b"");
    }

    #[test]
    fn test_transform_replace_all() {
        assert_eq!(replace_all(b"hello world hello", b"hello", b"hi"), b"hi world hi");
        assert_eq!(replace_all(b"aaaa", b"aa", b"b"), b"bb");
        assert_eq!(replace_all(b"hello world", b"foo", b"bar"), b"hello world");
    }

    #[test]
    fn test_transform_case() {
        assert_eq!(to_upper(b"hello").unwrap(), b"HELLO");
        assert_eq!(to_upper(b"Hello World").unwrap(), b"HELLO WORLD");
        assert_eq!(to_upper(b"123").unwrap(), b"123");

        assert_eq!(to_lower(b"HELLO").unwrap(), b"hello");
        assert_eq!(to_lower(b"Hello World").unwrap(), b"hello world");
        assert_eq!(to_lower(b"123").unwrap(), b"123");

        assert_eq!(to_title(b"hello world").unwrap(), b"Hello World");
        assert_eq!(to_title(b"HELLO WORLD").unwrap(), b"Hello World");
    }

    #[test]
    fn test_transform_map() {
        let result = map(|c| if c == 'o' { '0' } else { c }, b"hello").unwrap();
        assert_eq!(result, b"hell0");
        
        let result = map(|c| c.to_uppercase().next().unwrap_or(c), b"hello").unwrap();
        assert_eq!(result, b"HELLO");
    }

    // ================================
    // SPLITTING TESTS
    // ================================

    #[test]
    fn test_split_basic() {
        let result = split(b"a,b,c", b",");
        assert_eq!(result, vec![b"a".to_vec(), b"b".to_vec(), b"c".to_vec()]);
        
        let result = split(b"hello", b",");
        assert_eq!(result, vec![b"hello".to_vec()]);
        
        let result = split(b"a,,b", b",");
        assert_eq!(result, vec![b"a".to_vec(), b"".to_vec(), b"b".to_vec()]);
    }

    #[test]
    fn test_split_n() {
        let result = split_n(b"a,b,c,d", b",", 2);
        assert_eq!(result, vec![b"a".to_vec(), b"b,c,d".to_vec()]);
        
        let result = split_n(b"a,b,c", b",", 0);
        assert_eq!(result, Vec::<Vec<u8>>::new());
    }

    #[test]
    fn test_split_after() {
        let result = split_after(b"a,b,c", b",");
        assert_eq!(result, vec![b"a,".to_vec(), b"b,".to_vec(), b"c".to_vec()]);
        
        let result = split_after(b"hello", b",");
        assert_eq!(result, vec![b"hello".to_vec()]);
    }

    #[test]
    fn test_fields() {
        let result = fields(b"  hello   world  ").unwrap();
        assert_eq!(result, vec![b"hello".to_vec(), b"world".to_vec()]);
        
        let result = fields(b"hello").unwrap();
        assert_eq!(result, vec![b"hello".to_vec()]);
        
        let result = fields(b"   ").unwrap();
        assert_eq!(result, Vec::<Vec<u8>>::new());
    }

    #[test]
    fn test_fields_func() {
        let result = fields_func(b"a:b:c", |c| c == ':').unwrap();
        assert_eq!(result, vec![b"a".to_vec(), b"b".to_vec(), b"c".to_vec()]);
    }

    // ================================
    // TRIMMING TESTS
    // ================================

    #[test]
    fn test_trim_basic() {
        assert_eq!(trim(b"  hello  ", " ").unwrap(), b"hello");
        assert_eq!(trim(b"xxhelloxx", "x").unwrap(), b"hello");
        assert_eq!(trim(b"hello", "xyz").unwrap(), b"hello");
    }

    #[test]
    fn test_trim_directional() {
        assert_eq!(trim_left(b"  hello  ", " ").unwrap(), b"hello  ");
        assert_eq!(trim_right(b"  hello  ", " ").unwrap(), b"  hello");
    }

    #[test]
    fn test_trim_space() {
        assert_eq!(trim_space(b"  hello  ").unwrap(), b"hello");
        assert_eq!(trim_space(b"\t\nhello\r\n").unwrap(), b"hello");
        assert_eq!(trim_space(b"   ").unwrap(), b"");
    }

    #[test]
    fn test_trim_prefix_suffix() {
        assert_eq!(trim_prefix(b"hello world", b"hello "), b"world");
        assert_eq!(trim_prefix(b"hello world", b"world"), b"hello world");

        assert_eq!(trim_suffix(b"hello world", b" world"), b"hello");
        assert_eq!(trim_suffix(b"hello world", b"hello"), b"hello world");
    }

    #[test]
    fn test_trim_func() {
        let result = trim_func(b"123hello456", |c| c.is_numeric()).unwrap();
        assert_eq!(result, b"hello");
        
        let result = trim_func(b"!!!hello!!!", |c| c == '!').unwrap();
        assert_eq!(result, b"hello");
    }

    // ================================
    // BUFFER TESTS
    // ================================

    #[test]
    fn test_buffer_basic_operations() {
        let buf = new_fit_buffer(None);
        
        assert_eq!(buf.len(), 0);
        assert!(buf.is_empty());
        
        buf.write_string("hello").unwrap();
        assert_eq!(buf.len(), 5);
        assert_eq!(buf.string(), "hello");
        
        buf.write_string(" world").unwrap();
        assert_eq!(buf.string(), "hello world");
    }

    #[test]
    fn test_buffer_read_operations() {
        let buf = new_fit_buffer(Some(b"hello world".to_vec()));
        
        assert_eq!(buf.read_byte().unwrap(), b'h');
        assert_eq!(buf.len(), 10);
        
        let mut buffer = [0u8; 5];
        let n = buf.read(&mut buffer).unwrap();
        assert_eq!(n, 5);
        assert_eq!(&buffer, b"ello ");
    }

    #[test]
    fn test_buffer_append_methods() {
        let buf = new_fit_buffer(None);
        
        buf.append_string("Hello ")
           .append_int(42, 10)
           .append_string(" - ")
           .append_bool(true);
        
        assert_eq!(buf.string(), "Hello 42 - true");
    }

    #[test]
    fn test_buffer_clone() {
        let buf1 = new_fit_buffer(Some(b"test".to_vec()));
        let buf2 = buf1.clone_buffer();
        
        assert_eq!(buf1.string(), buf2.string());
        
        buf1.append_string(" modified");
        assert_ne!(buf1.string(), buf2.string());
    }

    #[test]
    fn test_buffer_replace_operations() {
        let buf = new_fit_buffer(Some(b"hello world hello".to_vec()));
        
        buf.replace(b"hello", b"hi", 1);
        assert_eq!(buf.string(), "hi world hello");
        
        buf.replace_all(b"hello", b"hi");
        assert_eq!(buf.string(), "hi world hi");
    }

    #[test]
    fn test_buffer_unicode() {
        let buf = new_fit_buffer(Some("Hello 🦀 World".as_bytes().to_vec()));
        
        let (ch, size) = buf.read_rune().unwrap();
        assert_eq!(ch, 'H');
        assert_eq!(size, 1);
    }

    // ================================
    // BINARY DATA TESTS
    // ================================

    #[test]
    fn test_binary_hex_encoding() {
        let data = b"hello";
        let hex = to_hex(data);
        assert_eq!(hex, b"68656c6c6f");
        
        let decoded = from_hex(&hex).unwrap();
        assert_eq!(decoded, data);
    }

    #[test]
    fn test_binary_hex_errors() {
        assert!(from_hex(b"abc").is_err()); // Odd length
        assert!(from_hex(b"xy").is_err()); // Invalid hex
    }

    #[test]
    fn test_binary_base64() {
        let data = b"hello";
        let encoded = to_base64(data);
        let encoded_str = String::from_utf8(encoded).unwrap();
        assert_eq!(encoded_str, "aGVsbG8=");
        
        let decoded = from_base64(b"aGVsbG8=").unwrap();
        assert_eq!(decoded, data);
    }

    #[test]
    fn test_binary_bitwise_operations() {
        let a = vec![0x0F, 0xF0];
        let b = vec![0xF0, 0x0F];
        
        assert_eq!(and(&a, &b), vec![0x00, 0x00]);
        assert_eq!(or(&a, &b), vec![0xFF, 0xFF]);
        assert_eq!(xor(&a, &b), vec![0xFF, 0xFF]);
        assert_eq!(not(&a), vec![0xF0, 0x0F]);
    }

    #[test]
    fn test_binary_shift_operations() {
        let data = vec![0x01, 0x02];
        
        let left_shifted = shift_left(&data, 1);
        assert_eq!(left_shifted, vec![0x02, 0x04]);
        
        let right_shifted = shift_right(&data, 1);
        assert_eq!(right_shifted, vec![0x00, 0x81]);
    }

    // ================================
    // PATTERN MATCHING TESTS
    // ================================

    #[test]
    fn test_pattern_wildcard_match() {
        assert!(wildcard_match(b"hello", b"hello"));
        assert!(wildcard_match(b"h*o", b"hello"));
        assert!(wildcard_match(b"h*", b"hello"));
        assert!(wildcard_match(b"*o", b"hello"));
        assert!(wildcard_match(b"*", b"hello"));
        assert!(wildcard_match(b"h?llo", b"hello"));
        assert!(!wildcard_match(b"h?llo", b"hllo"));
        assert!(!wildcard_match(b"hello", b"world"));
    }

    #[test]
    fn test_pattern_wildcard_edge_cases() {
        assert!(wildcard_match(b"", b""));
        assert!(wildcard_match(b"*", b""));
        assert!(wildcard_match(b"**", b""));
        assert!(!wildcard_match(b"a", b""));
        assert!(!wildcard_match(b"", b"a"));
        assert!(wildcard_match(b"*a*", b"abc"));
        assert!(wildcard_match(b"a*b*c", b"aXbYc"));
    }

    #[test]
    fn test_pattern_regex_match() {
        assert!(regex_match("hello", b"hello world").unwrap());
        assert!(!regex_match("foo", b"hello world").unwrap());
        assert!(regex_match("^hello", b"hello world").unwrap());
        assert!(!regex_match("^world", b"hello world").unwrap());
        assert!(regex_match("world$", b"hello world").unwrap());
        assert!(!regex_match("hello$", b"hello world").unwrap());
        assert!(regex_match(r"\d+", b"123").unwrap());
        assert!(!regex_match(r"\d+", b"abc").unwrap());
    }

    #[test]
    fn test_pattern_regex_find_all() {
        let result = regex_find_all(r"\d+", b"abc123def456ghi", -1).unwrap();
        assert_eq!(result, vec![b"123".to_vec(), b"456".to_vec()]);
        
        let result = regex_find_all(r"\d+", b"abc123def456ghi", 1).unwrap();
        assert_eq!(result, vec![b"123".to_vec()]);
    }

    #[test]
    fn test_pattern_regex_replace() {
        let result = regex_replace(r"\d+", b"abc123def456", b"XXX").unwrap();
        assert_eq!(result, b"abcXXXdefXXX");
        
        let result = regex_replace("hello", b"hello world hello", b"hi").unwrap();
        assert_eq!(result, b"hi world hi");
    }

    // ================================
    // INTEGRATION TESTS
    // ================================

    #[test]
    fn test_integration_complex_workflow() {
        // Create a buffer and perform various operations
        let buf = new_fit_buffer(None);
        
        // Build some data
        buf.append_string("Hello, ")
           .append_string("World! ")
           .append_int(2024, 10);
        
        // Get the data and manipulate it
        let data = buf.bytes();
        let upper = to_upper(&data).unwrap();
        let parts = split(&upper, b" ");
        
        assert_eq!(parts.len(), 3);
        assert_eq!(parts[0], b"HELLO,");
        assert_eq!(parts[1], b"WORLD!");
        assert_eq!(parts[2], b"2024");
        
        // Join back with different separator
        let rejoined = join(&parts.iter().map(|v| v.as_slice()).collect::<Vec<_>>(), b" | ");
        assert_eq!(rejoined, b"HELLO, | WORLD! | 2024");
    }

    #[test]
    fn test_integration_binary_processing() {
        // Test binary data processing workflow
        let original = b"Binary data: \x01\x02\x03\xFF";
        
        // Convert to hex
        let hex = to_hex(original);
        let hex_string = String::from_utf8(hex).unwrap();
        
        // Verify hex encoding
        assert!(hex_string.contains("42696e617279"));
        
        // Convert back from hex
        let decoded = from_hex(hex_string.as_bytes()).unwrap();
        assert_eq!(decoded, original);
        
        // Test binary operations
        let a = vec![0xAA, 0xBB];
        let b = vec![0x55, 0x44];
        let xored = xor(&a, &b);
        assert_eq!(xored, vec![0xFF, 0xFF]);
    }

    #[test]
    fn test_integration_text_processing() {
        // Complex text processing scenario
        let text = b"  Hello, World! How are you today?  ";
        
        // Trim whitespace
        let trimmed = trim_space(text).unwrap();
        
        // Split into words
        let words = fields(&trimmed).unwrap();
        assert_eq!(words.len(), 6);
        
        // Convert to lowercase and join with underscores
        let lowercase_words: Vec<Vec<u8>> = words.iter()
            .map(|word| to_lower(word).unwrap())
            .collect();
        
        let snake_case = join(&lowercase_words.iter().map(|v| v.as_slice()).collect::<Vec<_>>(), b"_");
        assert_eq!(snake_case, b"hello,_world!_how_are_you_today?");
    }
}
