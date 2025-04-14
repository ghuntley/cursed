fr fr Test file for ByteFit package

import {
    bytefit,
    vibez,
}

fr fr Test basic operations
slay test_basic_operations() {
    fr fr Test Compare
    test_compare := bytefit.Compare([]byte("abc"), []byte("abc"))
    vibez.assert_equal(test_compare, 0, "Expected equal comparison to return 0")
    
    test_compare = bytefit.Compare([]byte("abc"), []byte("abd"))
    vibez.assert_true(test_compare < 0, "Expected abc < abd")
    
    test_compare = bytefit.Compare([]byte("abd"), []byte("abc"))
    vibez.assert_true(test_compare > 0, "Expected abd > abc")
    
    fr fr Test Equal
    vibez.assert_true(bytefit.Equal([]byte("test"), []byte("test")), "Expected equality")
    vibez.assert_false(bytefit.Equal([]byte("test"), []byte("test2")), "Expected inequality")
    
    fr fr Test EqualFold
    vibez.assert_true(bytefit.EqualFold([]byte("TeSt"), []byte("tEsT")), "Expected case-insensitive equality")
    vibez.assert_false(bytefit.EqualFold([]byte("test"), []byte("best")), "Expected inequality")
    
    fr fr Test Repeat
    repeated := bytefit.Repeat([]byte("ab"), 3)
    vibez.assert_equal(tea(repeated), "ababab", "Expected repetition")
    
    fr fr Test Runes
    runes := bytefit.Runes([]byte("hello"))
    vibez.assert_equal(len(runes), 5, "Expected 5 runes")
    vibez.assert_equal(rune(runes[0]), 'h', "Expected first rune to be 'h'")
}

fr fr Test search functions
slay test_search_functions() {
    test_bytes := []byte("hello world")
    
    fr fr Test Contains
    vibez.assert_true(bytefit.Contains(test_bytes, []byte("world")), "Expected to find substring")
    vibez.assert_false(bytefit.Contains(test_bytes, []byte("universe")), "Expected to not find substring")
    
    fr fr Test ContainsAny
    vibez.assert_true(bytefit.ContainsAny(test_bytes, "abcdefgh"), "Expected to find characters")
    vibez.assert_false(bytefit.ContainsAny(test_bytes, "xyz"), "Expected to not find characters")
    
    fr fr Test ContainsRune
    vibez.assert_true(bytefit.ContainsRune(test_bytes, 'w'), "Expected to find rune")
    vibez.assert_false(bytefit.ContainsRune(test_bytes, 'z'), "Expected to not find rune")
    
    fr fr Test Count
    vibez.assert_equal(bytefit.Count([]byte("hello hello"), []byte("hello")), 2, "Expected to count 2 occurrences")
    
    fr fr Test Prefix/Suffix
    vibez.assert_true(bytefit.HasPrefix(test_bytes, []byte("hello")), "Expected to find prefix")
    vibez.assert_true(bytefit.HasSuffix(test_bytes, []byte("world")), "Expected to find suffix")
    
    fr fr Test Index functions
    vibez.assert_equal(bytefit.Index(test_bytes, []byte("world")), 6, "Expected index 6")
    vibez.assert_equal(bytefit.IndexByte(test_bytes, 'w'), 6, "Expected index 6")
    vibez.assert_equal(bytefit.IndexAny(test_bytes, "wxyz"), 6, "Expected index 6")
    vibez.assert_equal(bytefit.IndexRune(test_bytes, 'w'), 6, "Expected index 6")
    
    fr fr Test LastIndex functions
    test_bytes_multiple := []byte("hello world hello")
    vibez.assert_equal(bytefit.LastIndex(test_bytes_multiple, []byte("hello")), 12, "Expected last index 12")
    vibez.assert_equal(bytefit.LastIndexByte(test_bytes_multiple, 'h'), 12, "Expected last index 12")
    vibez.assert_equal(bytefit.LastIndexAny(test_bytes_multiple, "eh"), 12, "Expected last index 12")
}

fr fr Test transformation functions
slay test_transformation_functions() {
    fr fr Test Join
    slices := [][]byte{[]byte("hello"), []byte("world")}
    joined := bytefit.Join(slices, []byte(", "))
    vibez.assert_equal(tea(joined), "hello, world", "Expected joined string")
    
    fr fr Test Replace
    replaced := bytefit.Replace([]byte("hello hello hello"), []byte("hello"), []byte("hi"), 2)
    vibez.assert_equal(tea(replaced), "hi hi hello", "Expected first 2 replacements")
    
    fr fr Test ReplaceAll
    replaced_all := bytefit.ReplaceAll([]byte("hello hello hello"), []byte("hello"), []byte("hi"))
    vibez.assert_equal(tea(replaced_all), "hi hi hi", "Expected all replacements")
    
    fr fr Test Map
    slay mapping_func(r rune) rune {
        if r == 'e' {
            return 'E'
        }
        return r
    }
    mapped := bytefit.Map(mapping_func, []byte("hello"))
    vibez.assert_equal(tea(mapped), "hEllo", "Expected mapped string")
    
    fr fr Test case conversion
    vibez.assert_equal(tea(bytefit.ToUpper([]byte("hello"))), "HELLO", "Expected uppercase")
    vibez.assert_equal(tea(bytefit.ToLower([]byte("HELLO"))), "hello", "Expected lowercase")
    vibez.assert_equal(tea(bytefit.ToTitle([]byte("hello world"))), "Hello World", "Expected title case")
}

fr fr Test splitting functions
slay test_splitting_functions() {
    test_bytes := []byte("a,b,c,d,e")
    
    fr fr Test Split
    split := bytefit.Split(test_bytes, []byte(","))
    vibez.assert_equal(len(split), 5, "Expected 5 parts")
    vibez.assert_equal(tea(split[0]), "a", "Expected first part to be 'a'")
    
    fr fr Test SplitN
    split_n := bytefit.SplitN(test_bytes, []byte(","), 3)
    vibez.assert_equal(len(split_n), 3, "Expected 3 parts")
    vibez.assert_equal(tea(split_n[2]), "c,d,e", "Expected third part to contain the rest")
    
    fr fr Test SplitAfter
    split_after := bytefit.SplitAfter(test_bytes, []byte(","))
    vibez.assert_equal(len(split_after), 5, "Expected 5 parts")
    vibez.assert_equal(tea(split_after[0]), "a,", "Expected first part to be 'a,'")
    
    fr fr Test SplitAfterN
    split_after_n := bytefit.SplitAfterN(test_bytes, []byte(","), 3)
    vibez.assert_equal(len(split_after_n), 3, "Expected 3 parts")
    vibez.assert_equal(tea(split_after_n[2]), "c,d,e", "Expected third part to contain the rest")
    
    fr fr Test Fields
    fields := bytefit.Fields([]byte("hello   world  test"))
    vibez.assert_equal(len(fields), 3, "Expected 3 fields")
    vibez.assert_equal(tea(fields[1]), "world", "Expected second field to be 'world'")
    
    fr fr Test FieldsFunc
    slay is_comma(r rune) lit {
        return r == ','
    }
    fields_func := bytefit.FieldsFunc(test_bytes, is_comma)
    vibez.assert_equal(len(fields_func), 5, "Expected 5 fields")
    vibez.assert_equal(tea(fields_func[4]), "e", "Expected last field to be 'e'")
}

fr fr Test trimming functions
slay test_trimming_functions() {
    test_bytes := []byte("  hello world  ")
    
    fr fr Test Trim
    trimmed := bytefit.Trim(test_bytes, " ")
    vibez.assert_equal(tea(trimmed), "hello world", "Expected trimmed string")
    
    fr fr Test TrimLeft
    trimmed_left := bytefit.TrimLeft(test_bytes, " ")
    vibez.assert_equal(tea(trimmed_left), "hello world  ", "Expected left-trimmed string")
    
    fr fr Test TrimRight
    trimmed_right := bytefit.TrimRight(test_bytes, " ")
    vibez.assert_equal(tea(trimmed_right), "  hello world", "Expected right-trimmed string")
    
    fr fr Test TrimSpace
    trimmed_space := bytefit.TrimSpace(test_bytes)
    vibez.assert_equal(tea(trimmed_space), "hello world", "Expected space-trimmed string")
    
    fr fr Test TrimPrefix
    trimmed_prefix := bytefit.TrimPrefix([]byte("foobar"), []byte("foo"))
    vibez.assert_equal(tea(trimmed_prefix), "bar", "Expected prefix-trimmed string")
    
    fr fr Test TrimSuffix
    trimmed_suffix := bytefit.TrimSuffix([]byte("foobar"), []byte("bar"))
    vibez.assert_equal(tea(trimmed_suffix), "foo", "Expected suffix-trimmed string")
    
    fr fr Test TrimFunc
    slay is_space(r rune) lit {
        return r == ' '
    }
    trimmed_func := bytefit.TrimFunc(test_bytes, is_space)
    vibez.assert_equal(tea(trimmed_func), "hello world", "Expected func-trimmed string")
}

fr fr Test FitBuffer
slay test_fit_buffer() {
    fr fr Test basic buffer operations
    buffer := bytefit.NewFitBuffer([]byte("hello"))
    vibez.assert_equal(buffer.String(), "hello", "Expected initial string")
    vibez.assert_equal(buffer.Len(), 5, "Expected length 5")
    
    fr fr Test writing to buffer
    buffer.WriteString(" world")
    vibez.assert_equal(buffer.String(), "hello world", "Expected concatenated string")
    
    fr fr Test reading from buffer
    var b [5]byte
    n, err := buffer.Read(b[:])
    vibez.assert_equal(n, 5, "Expected to read 5 bytes")
    vibez.assert_nil(err, "Expected no error")
    vibez.assert_equal(tea(b[:]), "hello", "Expected to read 'hello'")
    
    fr fr Test append methods
    buffer.Reset()
    buffer.AppendString("Hello").AppendString(", ").AppendString("World!")
    vibez.assert_equal(buffer.String(), "Hello, World!", "Expected chained appends")
    
    fr fr Test clone
    cloned := buffer.Clone()
    vibez.assert_equal(cloned.String(), buffer.String(), "Expected identical clone")
    
    fr fr Test buffer transformations
    buffer.Replace([]byte("Hello"), []byte("Hi"), 1)
    vibez.assert_equal(buffer.String(), "Hi, World!", "Expected replacement")
    
    buffer.ReplaceAll([]byte("!"), []byte("!!"))
    vibez.assert_equal(buffer.String(), "Hi, World!!", "Expected replacement")
    
    buffer.Trim(" !")
    vibez.assert_equal(buffer.String(), "Hi, World", "Expected trimmed string")
}

fr fr Test binary operations
slay test_binary_operations() {
    fr fr Test hex conversion
    hex_bytes := []byte("48656c6c6f") fr fr "Hello" in hex
    decoded := bytefit.FromHex(hex_bytes)
    vibez.assert_equal(tea(decoded), "Hello", "Expected decoded hex")
    
    encoded := bytefit.ToHex([]byte("Hello"))
    vibez.assert_equal(tea(encoded), "48656c6c6f", "Expected encoded hex")
    
    fr fr Test base64
    b64_bytes := []byte("SGVsbG8=") fr fr "Hello" in base64
    decoded, err := bytefit.FromBase64(b64_bytes)
    vibez.assert_nil(err, "Expected no error")
    vibez.assert_equal(tea(decoded), "Hello", "Expected decoded base64")
    
    encoded = bytefit.ToBase64([]byte("Hello"))
    vibez.assert_equal(tea(encoded), "SGVsbG8=", "Expected encoded base64")
    
    fr fr Test bitwise operations
    a := []byte{0x01, 0x02, 0x03}
    b := []byte{0x10, 0x20, 0x30}
    
    and_result := bytefit.And(a, b)
    vibez.assert_equal(and_result[0], byte(0x00), "Expected bitwise AND")
    
    or_result := bytefit.Or(a, b)
    vibez.assert_equal(or_result[0], byte(0x11), "Expected bitwise OR")
    
    xor_result := bytefit.Xor(a, b)
    vibez.assert_equal(xor_result[0], byte(0x11), "Expected bitwise XOR")
    
    not_result := bytefit.Not(a)
    vibez.assert_equal(not_result[0], byte(0xFE), "Expected bitwise NOT")
    
    shift_left := bytefit.ShiftLeft(a, 1)
    vibez.assert_equal(shift_left[0], byte(0x02), "Expected left shift")
    
    shift_right := bytefit.ShiftRight(a, 1)
    vibez.assert_equal(shift_right[0], byte(0x00), "Expected right shift")
}

fr fr Test pattern matching
slay test_pattern_matching() {
    test_bytes := []byte("hello123world")
    
    fr fr Test wildcard match
    vibez.assert_true(bytefit.WildcardMatch([]byte("hello*"), test_bytes), "Expected wildcard match")
    vibez.assert_true(bytefit.WildcardMatch([]byte("*world"), test_bytes), "Expected wildcard match")
    vibez.assert_false(bytefit.WildcardMatch([]byte("hello*universe"), test_bytes), "Expected no match")
    
    fr fr Test regex match
    vibez.assert_true(bytefit.RegexMatch("\\d+", test_bytes), "Expected regex match")
    vibez.assert_false(bytefit.RegexMatch("\\d{4,}", test_bytes), "Expected no match")
    
    fr fr Test regex find all
    results := bytefit.RegexFindAll("\\d+", test_bytes, -1)
    vibez.assert_equal(len(results), 1, "Expected 1 match")
    vibez.assert_equal(tea(results[0]), "123", "Expected to match '123'")
    
    fr fr Test regex replace
    replaced := bytefit.RegexReplace("\\d+", test_bytes, []byte("456"))
    vibez.assert_equal(tea(replaced), "hello456world", "Expected replacement")
}

fr fr Main test entry point
slay main() {
    vibez.spill("Running ByteFit package tests...")
    
    test_basic_operations()
    test_search_functions()
    test_transformation_functions()
    test_splitting_functions()
    test_trimming_functions()
    test_fit_buffer()
    test_binary_operations()
    test_pattern_matching()
    
    vibez.spill("All ByteFit tests passed!")
}