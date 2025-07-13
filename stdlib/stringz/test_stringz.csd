yeet "testz"
yeet "stringz"

# Test string search and matching functions
test_start("Contains function")
assert_true(stringz.Contains("hello world", "world"))
assert_true(stringz.Contains("hello world", "hello"))
assert_false(stringz.Contains("hello world", "xyz"))
assert_false(stringz.Contains("hello", "hello world"))
assert_true(stringz.Contains("test", ""))

test_start("Count function")
assert_eq_int(stringz.Count("hello world", "l"), 3)
assert_eq_int(stringz.Count("hello world", "o"), 2)
assert_eq_int(stringz.Count("hello world", "xyz"), 0)
assert_eq_int(stringz.Count("aaa", "aa"), 1)
assert_eq_int(stringz.Count("", "a"), 0)

test_start("HasPrefix function")
assert_true(stringz.HasPrefix("hello world", "hello"))
assert_true(stringz.HasPrefix("hello world", ""))
assert_false(stringz.HasPrefix("hello world", "world"))
assert_false(stringz.HasPrefix("hello", "hello world"))

test_start("HasSuffix function")
assert_true(stringz.HasSuffix("hello world", "world"))
assert_true(stringz.HasSuffix("hello world", ""))
assert_false(stringz.HasSuffix("hello world", "hello"))
assert_false(stringz.HasSuffix("world", "hello world"))

# Test string manipulation and transformation functions
test_start("ToLower function")
assert_eq_string(stringz.ToLower("HELLO WORLD"), "hello world")
assert_eq_string(stringz.ToLower("Hello World"), "hello world")
assert_eq_string(stringz.ToLower("hello world"), "hello world")
assert_eq_string(stringz.ToLower("123ABC"), "123abc")

test_start("ToUpper function")
assert_eq_string(stringz.ToUpper("hello world"), "HELLO WORLD")
assert_eq_string(stringz.ToUpper("Hello World"), "HELLO WORLD")
assert_eq_string(stringz.ToUpper("HELLO WORLD"), "HELLO WORLD")
assert_eq_string(stringz.ToUpper("123abc"), "123ABC")

test_start("Trim function")
assert_eq_string(stringz.Trim("  hello world  "), "hello world")
assert_eq_string(stringz.Trim("\t\nhello\t\n"), "hello")
assert_eq_string(stringz.Trim("hello world"), "hello world")
assert_eq_string(stringz.Trim("   "), "")

test_start("TrimLeft function")
assert_eq_string(stringz.TrimLeft("  hello world  "), "hello world  ")
assert_eq_string(stringz.TrimLeft("\t\nhello\t\n"), "hello\t\n")
assert_eq_string(stringz.TrimLeft("hello world"), "hello world")

test_start("TrimRight function")
assert_eq_string(stringz.TrimRight("  hello world  "), "  hello world")
assert_eq_string(stringz.TrimRight("\t\nhello\t\n"), "\t\nhello")
assert_eq_string(stringz.TrimRight("hello world"), "hello world")

# Test string splitting and joining functions
test_start("Split function")
sus words [tea] = stringz.Split("hello,world,test", ",")
assert_eq_int(len(words), 3)
assert_eq_string(words[0], "hello")
assert_eq_string(words[1], "world")
assert_eq_string(words[2], "test")

sus single [tea] = stringz.Split("hello", ",")
assert_eq_int(len(single), 1)
assert_eq_string(single[0], "hello")

test_start("Join function")
sus parts [tea] = ["hello", "world", "test"]
assert_eq_string(stringz.Join(parts, ","), "hello,world,test")
assert_eq_string(stringz.Join(parts, " "), "hello world test")

sus empty [tea] = []
assert_eq_string(stringz.Join(empty, ","), "")

test_start("Repeat function")
assert_eq_string(stringz.Repeat("hello", 3), "hellohellohello")
assert_eq_string(stringz.Repeat("a", 5), "aaaaa")
assert_eq_string(stringz.Repeat("test", 0), "")

test_start("Replace function")
assert_eq_string(stringz.Replace("hello world", "world", "CURSED"), "hello CURSED")
assert_eq_string(stringz.Replace("hello world", "xyz", "CURSED"), "hello world")
assert_eq_string(stringz.Replace("hello hello", "hello", "hi"), "hi hello")

test_start("ReplaceAll function")
assert_eq_string(stringz.ReplaceAll("hello hello", "hello", "hi"), "hi hi")
assert_eq_string(stringz.ReplaceAll("hello world", "xyz", "CURSED"), "hello world")
assert_eq_string(stringz.ReplaceAll("aaa", "aa", "b"), "ba")

# Test helper functions
test_start("Length function")
assert_eq_int(stringz.Length("hello"), 5)
assert_eq_int(stringz.Length(""), 0)
assert_eq_int(stringz.Length("hello world"), 11)

test_start("Substring function")
assert_eq_string(stringz.Substring("hello world", 0, 5), "hello")
assert_eq_string(stringz.Substring("hello world", 6, 5), "world")
assert_eq_string(stringz.Substring("hello world", 0, 0), "")

test_start("IsWhitespace function")
assert_true(stringz.IsWhitespace(' '))
assert_true(stringz.IsWhitespace('\t'))
assert_true(stringz.IsWhitespace('\n'))
assert_true(stringz.IsWhitespace('\r'))
assert_false(stringz.IsWhitespace('a'))
assert_false(stringz.IsWhitespace('1'))

test_start("IndexOf function")
assert_eq_int(stringz.IndexOf("hello world", "world"), 6)
assert_eq_int(stringz.IndexOf("hello world", "hello"), 0)
assert_eq_int(stringz.IndexOf("hello world", "xyz"), -1)
assert_eq_int(stringz.IndexOf("hello", "hello world"), -1)

test_start("LastIndexOf function")
assert_eq_int(stringz.LastIndexOf("hello hello", "hello"), 6)
assert_eq_int(stringz.LastIndexOf("hello world", "o"), 7)
assert_eq_int(stringz.LastIndexOf("hello world", "xyz"), -1)

test_start("Reverse function")
assert_eq_string(stringz.Reverse("hello"), "olleh")
assert_eq_string(stringz.Reverse("world"), "dlrow")
assert_eq_string(stringz.Reverse(""), "")
assert_eq_string(stringz.Reverse("a"), "a")

test_start("PadLeft function")
assert_eq_string(stringz.PadLeft("hello", 8, ' '), "   hello")
assert_eq_string(stringz.PadLeft("hello", 5, ' '), "hello")
assert_eq_string(stringz.PadLeft("hello", 3, ' '), "hello")

test_start("PadRight function")
assert_eq_string(stringz.PadRight("hello", 8, ' '), "hello   ")
assert_eq_string(stringz.PadRight("hello", 5, ' '), "hello")
assert_eq_string(stringz.PadRight("hello", 3, ' '), "hello")

# Test edge cases and complex scenarios
test_start("Empty string handling")
assert_eq_string(stringz.ToLower(""), "")
assert_eq_string(stringz.ToUpper(""), "")
assert_eq_string(stringz.Trim(""), "")
assert_eq_string(stringz.Reverse(""), "")
assert_eq_int(stringz.Length(""), 0)

test_start("Single character strings")
assert_eq_string(stringz.ToLower("A"), "a")
assert_eq_string(stringz.ToUpper("a"), "A")
assert_eq_string(stringz.Trim(" "), "")
assert_eq_string(stringz.Reverse("a"), "a")

test_start("Unicode and special characters")
assert_eq_string(stringz.ToLower("Hello123!@#"), "hello123!@#")
assert_eq_string(stringz.ToUpper("Hello123!@#"), "HELLO123!@#")
assert_true(stringz.Contains("Hello123!@#", "123"))
assert_true(stringz.Contains("Hello123!@#", "!@#"))

test_start("Complex string operations")
sus complex_string tea = "  Hello, World! This is a test.  "
sus trimmed tea = stringz.Trim(complex_string)
sus lower tea = stringz.ToLower(trimmed)
sus words_split [tea] = stringz.Split(lower, " ")
sus joined tea = stringz.Join(words_split, "_")
assert_eq_string(joined, "hello,_world!_this_is_a_test.")

# Performance and stress tests
test_start("Large string operations")
sus large_string tea = stringz.Repeat("Hello World! ", 100)
assert_true(stringz.Contains(large_string, "Hello World!"))
assert_eq_int(stringz.Count(large_string, "Hello"), 100)
assert_true(stringz.HasPrefix(large_string, "Hello"))
assert_true(stringz.HasSuffix(large_string, "World! "))

test_start("String manipulation chain")
sus original tea = "HELLO WORLD"
sus processed tea = stringz.ToLower(original)
processed = stringz.Replace(processed, "world", "CURSED")
processed = stringz.Trim(" " + processed + " ")
processed = stringz.ToUpper(processed)
assert_eq_string(processed, "HELLO CURSED")

# Test string validation functions
test_start("IsEmpty function")
assert_true(stringz.IsEmpty(""))
assert_false(stringz.IsEmpty("hello"))
assert_false(stringz.IsEmpty(" "))
assert_false(stringz.IsEmpty("a"))

test_start("IsNumeric function")
assert_true(stringz.IsNumeric("123"))
assert_true(stringz.IsNumeric("0"))
assert_true(stringz.IsNumeric("987654321"))
assert_false(stringz.IsNumeric("123a"))
assert_false(stringz.IsNumeric("12.3"))
assert_false(stringz.IsNumeric("hello"))
assert_false(stringz.IsNumeric(""))
assert_false(stringz.IsNumeric("123 "))
assert_false(stringz.IsNumeric(" 123"))

test_start("IsAlpha function")
assert_true(stringz.IsAlpha("hello"))
assert_true(stringz.IsAlpha("HELLO"))
assert_true(stringz.IsAlpha("HelloWorld"))
assert_true(stringz.IsAlpha("a"))
assert_true(stringz.IsAlpha("Z"))
assert_false(stringz.IsAlpha("hello123"))
assert_false(stringz.IsAlpha("hello world"))
assert_false(stringz.IsAlpha("hello!"))
assert_false(stringz.IsAlpha(""))
assert_false(stringz.IsAlpha("123"))

test_start("IsAlphanumeric function")
assert_true(stringz.IsAlphanumeric("hello123"))
assert_true(stringz.IsAlphanumeric("HELLO123"))
assert_true(stringz.IsAlphanumeric("abc"))
assert_true(stringz.IsAlphanumeric("123"))
assert_true(stringz.IsAlphanumeric("a1"))
assert_true(stringz.IsAlphanumeric("Hello123World"))
assert_false(stringz.IsAlphanumeric("hello world"))
assert_false(stringz.IsAlphanumeric("hello!"))
assert_false(stringz.IsAlphanumeric("hello@123"))
assert_false(stringz.IsAlphanumeric(""))
assert_false(stringz.IsAlphanumeric("hello-world"))

test_start("StartsWith function (alias)")
assert_true(stringz.StartsWith("hello world", "hello"))
assert_true(stringz.StartsWith("hello world", ""))
assert_false(stringz.StartsWith("hello world", "world"))
assert_false(stringz.StartsWith("hello", "hello world"))

test_start("EndsWith function (alias)")
assert_true(stringz.EndsWith("hello world", "world"))
assert_true(stringz.EndsWith("hello world", ""))
assert_false(stringz.EndsWith("hello world", "hello"))
assert_false(stringz.EndsWith("world", "hello world"))

# Test validation with edge cases
test_start("Validation edge cases")
assert_false(stringz.IsNumeric("0123abc"))
assert_false(stringz.IsAlpha("hello\n"))
assert_false(stringz.IsAlphanumeric("test\t123"))
assert_true(stringz.IsNumeric("0000"))
assert_true(stringz.IsAlpha("abcDEF"))
assert_true(stringz.IsAlphanumeric("abc123DEF"))

print_test_summary()
