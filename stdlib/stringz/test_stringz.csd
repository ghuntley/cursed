yeet "testz"
yeet "stringz"

fr fr ========================================
fr fr CURSED String Operations Test Suite v2.0
fr fr Comprehensive testing for all string functions
fr fr ========================================

fr fr ================================
fr fr String Search Function Tests
fr fr ================================

test_start("Contains function")
assert_true(Contains("hello world", "world"))
assert_true(Contains("hello world", "hello"))
assert_false(Contains("hello world", "xyz"))
assert_false(Contains("hello", "hello world"))
assert_true(Contains("test", ""))

test_start("ContainsAny function")
assert_true(ContainsAny("hello", "aeiou")) fr fr Contains vowels
assert_false(ContainsAny("bcdfg", "aeiou")) fr fr No vowels
assert_false(ContainsAny("hello", "")) fr fr Empty chars

test_start("Count function")
assert_eq_int(Count("hello world", "l"), 3)
assert_eq_int(Count("hello world", "o"), 2)
assert_eq_int(Count("hello world", "xyz"), 0)
assert_eq_int(Count("aaa", "aa"), 1) fr fr Non-overlapping

test_start("HasPrefix function")
assert_true(HasPrefix("hello world", "hello"))
assert_true(HasPrefix("hello world", ""))
assert_false(HasPrefix("hello world", "world"))
assert_false(HasPrefix("hello", "hello world"))

test_start("HasSuffix function")
assert_true(HasSuffix("hello world", "world"))
assert_true(HasSuffix("hello world", ""))
assert_false(HasSuffix("hello world", "hello"))
assert_false(HasSuffix("world", "hello world"))

test_start("IndexOf function")
assert_eq_int(IndexOf("hello world", "world"), 6)
assert_eq_int(IndexOf("hello world", "hello"), 0)
assert_eq_int(IndexOf("hello world", "xyz"), -1)

test_start("LastIndexOf function")
assert_eq_int(LastIndexOf("hello world hello", "hello"), 12)
assert_eq_int(LastIndexOf("hello world", "o"), 7)
assert_eq_int(LastIndexOf("hello world", "xyz"), -1)

fr fr ================================
fr fr String Manipulation Function Tests
fr fr ================================

test_start("ToLower function")
assert_eq_string(ToLower("HELLO WORLD"), "hello world")
assert_eq_string(ToLower("Hello World"), "hello world")
assert_eq_string(ToLower("hello world"), "hello world")
assert_eq_string(ToLower("123ABC"), "123abc")

test_start("ToUpper function")
assert_eq_string(ToUpper("hello world"), "HELLO WORLD")
assert_eq_string(ToUpper("Hello World"), "HELLO WORLD")
assert_eq_string(ToUpper("HELLO WORLD"), "HELLO WORLD")
assert_eq_string(ToUpper("123abc"), "123ABC")

test_start("Trim function")
assert_eq_string(Trim("  hello world  "), "hello world")
assert_eq_string(Trim("hello world"), "hello world")

test_start("TrimLeft function")
assert_eq_string(TrimLeft("  hello world  "), "hello world  ")

test_start("TrimRight function")
assert_eq_string(TrimRight("  hello world  "), "  hello world")

test_start("Replace function")
assert_eq_string(Replace("hello world", "world", "universe"), "hello universe")
assert_eq_string(Replace("hello hello", "hello", "hi"), "hi hello")
assert_eq_string(Replace("hello world", "xyz", "abc"), "hello world")

test_start("ReplaceAll function")
assert_eq_string(ReplaceAll("hello hello", "hello", "hi"), "hi hi")
assert_eq_string(ReplaceAll("abc abc abc", "abc", "xyz"), "xyz xyz xyz")
assert_eq_string(ReplaceAll("hello world", "xyz", "abc"), "hello world")

test_start("Repeat function")
assert_eq_string(Repeat("abc", 3), "abcabcabc")
assert_eq_string(Repeat("x", 5), "xxxxx")
assert_eq_string(Repeat("hello", 0), "")

fr fr ================================
fr fr String Splitting and Joining Tests
fr fr ================================

test_start("Split function")
sus parts [tea] = Split("a,b,c,d", ",")
assert_eq_int(len(parts), 4)
assert_eq_string(parts[0], "a")
assert_eq_string(parts[1], "b")
assert_eq_string(parts[2], "c")
assert_eq_string(parts[3], "d")

test_start("Join function")
sus words [tea] = ["hello", "beautiful", "world"]
assert_eq_string(Join(words, " "), "hello beautiful world")
assert_eq_string(Join(words, ","), "hello,beautiful,world")

fr fr ================================
fr fr String Utility Function Tests
fr fr ================================

test_start("Reverse function")
assert_eq_string(Reverse("hello"), "olleh")
assert_eq_string(Reverse("abc"), "cba")
assert_eq_string(Reverse(""), "")
assert_eq_string(Reverse("a"), "a")

test_start("PadLeft function")
assert_eq_string(PadLeft("hello", 10, '0'), "00000hello")

test_start("PadRight function")
assert_eq_string(PadRight("hello", 10, '0'), "hello00000")

test_start("Length function")
assert_eq_int(Length("hello"), 5)
assert_eq_int(Length(""), 0)
assert_eq_int(Length("test123"), 7)

test_start("Substring function")
assert_eq_string(Substring("hello world", 0, 5), "hello")
assert_eq_string(Substring("hello world", 6, 5), "world")
assert_eq_string(Substring("hello", 1, 3), "ell")

fr fr ================================
fr fr String Validation Function Tests
fr fr ================================

test_start("IsEmpty function")
assert_true(IsEmpty(""))
assert_false(IsEmpty(" "))
assert_false(IsEmpty("hello"))

test_start("IsNumeric function")
assert_true(IsNumeric("12345"))
assert_true(IsNumeric("0"))
assert_false(IsNumeric(""))
assert_false(IsNumeric("123a"))

test_start("IsAlpha function")
assert_true(IsAlpha("hello"))
assert_true(IsAlpha("ABC"))
assert_true(IsAlpha("AbCdEf"))
assert_false(IsAlpha(""))
assert_false(IsAlpha("hello123"))

test_start("IsAlphanumeric function")
assert_true(IsAlphanumeric("hello123"))
assert_true(IsAlphanumeric("ABC123"))
assert_true(IsAlphanumeric("test"))
assert_true(IsAlphanumeric("123"))
assert_false(IsAlphanumeric(""))
assert_false(IsAlphanumeric("hello world"))

fr fr ================================
fr fr Advanced String Function Tests
fr fr ================================

test_start("Before function")
assert_eq_string(Before("hello:world", ":"), "hello")
assert_eq_string(Before("no-separator", ":"), "no-separator")

test_start("After function")
assert_eq_string(After("hello:world", ":"), "world")
assert_eq_string(After("no-separator", ":"), "")

test_start("BeforeLast function")
assert_eq_string(BeforeLast("a:b:c", ":"), "a:b")
assert_eq_string(BeforeLast("no-separator", ":"), "no-separator")

test_start("AfterLast function")
assert_eq_string(AfterLast("a:b:c", ":"), "c")
assert_eq_string(AfterLast("no-separator", ":"), "")

test_start("Truncate function")
assert_eq_string(Truncate("hello world", 5), "hello")
assert_eq_string(Truncate("hello", 10), "hello")

test_start("TruncateWithEllipsis function")
assert_eq_string(TruncateWithEllipsis("hello world", 8), "hello...")
assert_eq_string(TruncateWithEllipsis("hello", 10), "hello")

fr fr ================================
fr fr Case Conversion Function Tests  
fr fr ================================

test_start("ToSnakeCase function")
assert_eq_string(ToSnakeCase("HelloWorld"), "hello_world")
assert_eq_string(ToSnakeCase("XMLHttpRequest"), "xml_http_request")

test_start("ToCamelCase function")
assert_eq_string(ToCamelCase("hello_world"), "helloWorld")
assert_eq_string(ToCamelCase("XML_HTTP_REQUEST"), "xmlHttpRequest")

test_start("ToPascalCase function")
assert_eq_string(ToPascalCase("hello_world"), "HelloWorld")
assert_eq_string(ToPascalCase("xml_http_request"), "XmlHttpRequest")

test_start("ToKebabCase function")
assert_eq_string(ToKebabCase("HelloWorld"), "hello-world")
assert_eq_string(ToKebabCase("XMLHttpRequest"), "xml-http-request")

fr fr ================================
fr fr Alias Function Tests
fr fr ================================

test_start("StartsWith alias")
assert_true(StartsWith("hello world", "hello"))
assert_false(StartsWith("hello world", "world"))

test_start("EndsWith alias")
assert_true(EndsWith("hello world", "world"))
assert_false(EndsWith("hello world", "hello"))

fr fr ================================
fr fr Comprehensive Integration Tests
fr fr ================================

test_start("Complex string processing")
fr fr Test a complete text processing pipeline
sus text tea = "  Hello, Beautiful World!  "
sus trimmed tea = Trim(text)
sus lower tea = ToLower(trimmed)
sus snake tea = ToSnakeCase(lower)
sus parts [tea] = Split(snake, "_")
sus joined tea = Join(parts, "-")

assert_eq_string(trimmed, "Hello, Beautiful World!")
assert_eq_string(lower, "hello, beautiful world!")
assert_eq_string(snake, "hello_beautiful_world")
assert_eq_int(len(parts), 3)
assert_eq_string(joined, "hello-beautiful-world")

test_start("Search and replace operations")
sus source tea = "The quick brown fox jumps over the lazy dog"
sus found lit = Contains(source, "fox")
sus index normie = IndexOf(source, "fox")
sus before tea = Before(source, "fox")
sus after tea = After(source, "fox")
sus replaced tea = Replace(source, "fox", "cat")

assert_true(found)
assert_eq_int(index, 16)
assert_eq_string(before, "The quick brown ")
assert_eq_string(after, " jumps over the lazy dog")
assert_eq_string(replaced, "The quick brown cat jumps over the lazy dog")

print_test_summary()
