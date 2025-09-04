yeet "testz"
yeet "string_energy"

slay test_basic_string_operations() {
    test_start("Basic String Operations")
    
    fr fr Test Contains
    assert_true(string_energy.Contains("hello world", "world"))
    assert_false(string_energy.Contains("hello world", "xyz"))
    
    fr fr Test HasPrefix/HasSuffix
    assert_true(string_energy.HasPrefix("hello world", "hello"))
    assert_true(string_energy.HasSuffix("hello world", "world"))
    assert_false(string_energy.HasPrefix("hello world", "world"))
    
    fr fr Test Index
    assert_eq_int(string_energy.Index("hello world", "world"), 6)
    assert_eq_int(string_energy.Index("hello world", "xyz"), -1)
    
    fr fr Test Count
    assert_eq_int(string_energy.Count("hello hello hello", "hello"), 3)
    assert_eq_int(string_energy.Count("hello world", "xyz"), 0)
    
    print_test_summary()
}

slay test_string_manipulation() {
    test_start("String Manipulation")
    
    fr fr Test Replace
    assert_eq_string(string_energy.Replace("hello world", "world", "universe", 1), "hello universe")
    assert_eq_string(string_energy.ReplaceAll("hello world world", "world", "universe"), "hello universe universe")
    
    fr fr Test Join/Split
    sus parts := tea[value]{"hello", "world", "test"}
    assert_eq_string(string_energy.Join(parts, " "), "hello world test")
    
    sus split := string_energy.Split("hello,world,test", ",")
    assert_eq_int(len(split), 3)
    assert_eq_string(split[0], "hello")
    assert_eq_string(split[1], "world")
    assert_eq_string(split[2], "test")
    
    fr fr Test Fields
    sus fields := string_energy.Fields("  hello   world  test  ")
    assert_eq_int(len(fields), 3)
    assert_eq_string(fields[0], "hello")
    assert_eq_string(fields[1], "world")
    assert_eq_string(fields[2], "test")
    
    print_test_summary()
}

slay test_string_transformation() {
    test_start("String Transformation")
    
    fr fr Test case conversion
    assert_eq_string(string_energy.ToUpper("hello"), "HELLO")
    assert_eq_string(string_energy.ToLower("HELLO"), "hello")
    assert_eq_string(string_energy.Title("hello world"), "Hello World")
    
    fr fr Test trimming
    assert_eq_string(string_energy.TrimSpace("  hello  "), "hello")
    assert_eq_string(string_energy.TrimPrefix("hello world", "hello "), "world")
    assert_eq_string(string_energy.TrimSuffix("hello world", " world"), "hello")
    
    fr fr Test repeat
    assert_eq_string(string_energy.Repeat("a", 3), "aaa")
    assert_eq_string(string_energy.Repeat("hello", 0), "")
    
    print_test_summary()
}

slay test_energy_builder() {
    test_start("Energy Builder")
    
    sus builder := string_energy.NewEnergyBuilder()
    
    fr fr Test basic building
    builder.WriteString("Hello")
    builder.WriteString(" ")
    builder.WriteString("World")
    
    assert_eq_string(builder.String(), "Hello World")
    assert_eq_int(builder.Len(), 11)
    
    fr fr Test reset
    builder.Reset()
    assert_eq_int(builder.Len(), 0)
    assert_eq_string(builder.String(), "")
    
    fr fr Test chaining
    builder.WriteString("Chain").WriteString(" ").WriteString("Test")
    assert_eq_string(builder.String(), "Chain Test")
    
    print_test_summary()
}

slay test_string_utilities() {
    test_start("String Utilities")
    
    fr fr Test Reverse
    assert_eq_string(string_energy.Reverse("hello"), "olleh")
    assert_eq_string(string_energy.Reverse(""), "")
    
    fr fr Test Before/After
    assert_eq_string(string_energy.Before("hello@world.com", "@"), "hello")
    assert_eq_string(string_energy.After("hello@world.com", "@"), "world.com")
    
    fr fr Test BeforeLast/AfterLast
    assert_eq_string(string_energy.BeforeLast("path/to/file.txt", "/"), "path/to")
    assert_eq_string(string_energy.AfterLast("path/to/file.txt", "/"), "file.txt")
    
    fr fr Test Chunk
    sus chunks := string_energy.Chunk("hello world", 3)
    assert_eq_int(len(chunks), 4)
    assert_eq_string(chunks[0], "hel")
    assert_eq_string(chunks[1], "lo ")
    assert_eq_string(chunks[2], "wor")
    assert_eq_string(chunks[3], "ld")
    
    print_test_summary()
}

slay test_text_padding() {
    test_start("Text Padding")
    
    fr fr Test PadLeft
    assert_eq_string(string_energy.PadLeft("hello", 8, "*"), "***hello")
    assert_eq_string(string_energy.PadLeft("hello", 3, "*"), "hello")
    
    fr fr Test PadRight
    assert_eq_string(string_energy.PadRight("hello", 8, "*"), "hello***")
    assert_eq_string(string_energy.PadRight("hello", 3, "*"), "hello")
    
    fr fr Test Center
    assert_eq_string(string_energy.Center("hello", 9, "*"), "**hello**")
    assert_eq_string(string_energy.Center("hello", 3, "*"), "hello")
    
    print_test_summary()
}

slay test_text_truncation() {
    test_start("Text Truncation")
    
    fr fr Test Truncate
    assert_eq_string(string_energy.Truncate("hello world", 5), "hello")
    assert_eq_string(string_energy.Truncate("hello", 10), "hello")
    
    fr fr Test TruncateWithEllipsis
    assert_eq_string(string_energy.TruncateWithEllipsis("hello world", 8), "hello...")
    assert_eq_string(string_energy.TruncateWithEllipsis("hello", 10), "hello")
    
    print_test_summary()
}

slay test_text_analysis() {
    test_start("Text Analysis")
    
    sus text := "hello world hello"
    
    fr fr Test CharCount
    sus charCounts := string_energy.CharCount(text)
    assert_eq_int(charCounts['h'], 2)
    assert_eq_int(charCounts['e'], 2)
    assert_eq_int(charCounts['l'], 6)
    assert_eq_int(charCounts['o'], 2)
    assert_eq_int(charCounts[' '], 2)
    
    fr fr Test WordCount
    sus wordCounts := string_energy.WordCount(text)
    assert_eq_int(wordCounts["hello"], 2)
    assert_eq_int(wordCounts["world"], 1)
    
    fr fr Test SentenceCount
    sus sentences := "Hello world. How are you? I'm fine!"
    assert_eq_int(string_energy.SentenceCount(sentences), 3)
    
    print_test_summary()
}

slay test_case_conversion() {
    test_start("Case Conversion")
    
    fr fr Test ToCamelCase
    assert_eq_string(string_energy.ToCamelCase("hello world test"), "helloWorldTest")
    assert_eq_string(string_energy.ToCamelCase("hello"), "hello")
    
    fr fr Test ToPascalCase
    assert_eq_string(string_energy.ToPascalCase("hello world test"), "HelloWorldTest")
    assert_eq_string(string_energy.ToPascalCase("hello"), "Hello")
    
    fr fr Test ToSnakeCase
    assert_eq_string(string_energy.ToSnakeCase("hello world test"), "hello_world_test")
    assert_eq_string(string_energy.ToSnakeCase("hello"), "hello")
    
    fr fr Test ToKebabCase
    assert_eq_string(string_energy.ToKebabCase("hello world test"), "hello-world-test")
    assert_eq_string(string_energy.ToKebabCase("hello"), "hello")
    
    print_test_summary()
}

slay test_html_processing() {
    test_start("HTML Processing")
    
    fr fr Test StripHTML
    assert_eq_string(string_energy.StripHTML("<h1>Hello</h1> <p>World</p>"), "Hello World")
    assert_eq_string(string_energy.StripHTML("No tags here"), "No tags here")
    
    fr fr Test EscapeHTML
    assert_eq_string(string_energy.EscapeHTML("<script>alert('test')</script>"), "&lt;script&gt;alert(&#39;test&#39;)&lt;/script&gt;")
    assert_eq_string(string_energy.EscapeHTML("Hello & World"), "Hello &amp; World")
    
    fr fr Test UnescapeHTML
    assert_eq_string(string_energy.UnescapeHTML("&lt;script&gt;"), "<script>")
    assert_eq_string(string_energy.UnescapeHTML("Hello &amp; World"), "Hello & World")
    
    print_test_summary()
}

slay test_interpolation() {
    test_start("String Interpolation")
    
    fr fr Test Interpolate
    sus vars := map[tea]tea{
        "name": "John",
        "age": "30",
    }
    sus template := "Hello ${name}, you are ${age} years old"
    sus result := string_energy.Interpolate(template, vars)
    assert_eq_string(result, "Hello John, you are 30 years old")
    
    fr fr Test ReplaceMultiple
    sus replacements := map[tea]tea{
        "hello": "hi",
        "world": "universe",
    }
    sus text := "hello world"
    sus replaced := string_energy.ReplaceMultiple(text, replacements)
    assert_eq_string(replaced, "hi universe")
    
    print_test_summary()
}

slay test_genz_transformations() {
    test_start("GenZ Transformations")
    
    fr fr Test ToGenZStyle
    sus result := string_energy.ToGenZStyle("this is really cool and awesome")
    assert_true(string_energy.Contains(result, "dis"))
    assert_true(string_energy.Contains(result, "rly"))
    assert_true(string_energy.Contains(result, "&"))
    
    fr fr Test ToGenZSlang
    sus slang := string_energy.ToGenZSlang("this is cool and awesome")
    assert_true(string_energy.Contains(slang, "bussin"))
    
    fr fr Test AddEmojis
    sus emoji := string_energy.AddEmojis("this is cool and amazing")
    assert_true(string_energy.Contains(emoji, "😎"))
    assert_true(string_energy.Contains(emoji, "🔥"))
    
    print_test_summary()
}

slay test_text_wrapping() {
    test_start("Text Wrapping")
    
    fr fr Test Wrap
    sus text := "This is a long line that needs to be wrapped"
    sus wrapped := string_energy.Wrap(text, 20)
    sus lines := string_energy.Split(wrapped, "\n")
    assert_true(len(lines) > 1)
    
    fr fr Each line should be <= 20 characters
    bestie _, line := range lines {
        assert_true(len(line) <= 20)
    }
    
    print_test_summary()
}

slay test_normalize_space() {
    test_start("Normalize Space")
    
    fr fr Test NormalizeSpace
    sus text := "  hello    world  \t\n  test  "
    sus normalized := string_energy.NormalizeSpace(text)
    assert_eq_string(normalized, "hello world test")
    
    fr fr Test with single spaces
    sus single := "hello world test"
    sus normalizedSingle := string_energy.NormalizeSpace(single)
    assert_eq_string(normalizedSingle, "hello world test")
    
    print_test_summary()
}

slay main_character() {
    test_basic_string_operations()
    test_string_manipulation()
    test_string_transformation()
    test_energy_builder()
    test_string_utilities()
    test_text_padding()
    test_text_truncation()
    test_text_analysis()
    test_case_conversion()
    test_html_processing()
    test_interpolation()
    test_genz_transformations()
    test_text_wrapping()
    test_normalize_space()
}
