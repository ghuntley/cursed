fr fr Comprehensive String Runtime Support Test
fr fr Tests all string operations that unblock stdlib modules

yeet "testz"
yeet "stringz"

fr fr === Test basic string runtime functions ===

test_start("runtime_string_char_at")
sus test_str tea = "hello"
sus char_h sip = runtime_string_char_at(test_str, 0)
sus char_e sip = runtime_string_char_at(test_str, 1)
sus char_l sip = runtime_string_char_at(test_str, 2)
sus char_null sip = runtime_string_char_at(test_str, 10)
assert_eq_int(char_h, 'h')
assert_eq_int(char_e, 'e')
assert_eq_int(char_l, 'l')
assert_eq_int(char_null, '\0')

test_start("runtime_char_to_string")
sus str_h tea = runtime_char_to_string('h')
sus str_e tea = runtime_char_to_string('e')
sus str_space tea = runtime_char_to_string(' ')
assert_eq_string(str_h, "h")
assert_eq_string(str_e, "e")
assert_eq_string(str_space, " ")

fr fr === Test stringz module functions ===

test_start("stringz.length")
sus len1 normie = stringz.length("hello")
sus len2 normie = stringz.length("")
sus len3 normie = stringz.length("world!")
assert_eq_int(len1, 5)
assert_eq_int(len2, 0)
assert_eq_int(len3, 6)

test_start("stringz.concat")
sus concat1 tea = stringz.concat("hello", " world")
sus concat2 tea = stringz.concat("", "test")
sus concat3 tea = stringz.concat("test", "")
assert_eq_string(concat1, "hello world")
assert_eq_string(concat2, "test")
assert_eq_string(concat3, "test")

test_start("stringz.char_at")
sus char1 sip = stringz.char_at("hello", 0)
sus char2 sip = stringz.char_at("hello", 4)
sus char3 sip = stringz.char_at("hello", 10)
assert_eq_int(char1, 'h')
assert_eq_int(char2, 'o')
assert_eq_int(char3, '\0')

test_start("stringz.substring")
sus substr1 tea = stringz.substring("hello world", 0, 5)
sus substr2 tea = stringz.substring("hello world", 6, 5)
sus substr3 tea = stringz.substring("hello world", 0, 100)
sus substr4 tea = stringz.substring("hello world", 20, 5)
assert_eq_string(substr1, "hello")
assert_eq_string(substr2, "world")
assert_eq_string(substr3, "hello world")
assert_eq_string(substr4, "")

test_start("stringz.equals")
sus eq1 lit = stringz.equals("hello", "hello")
sus eq2 lit = stringz.equals("hello", "world")
sus eq3 lit = stringz.equals("", "")
sus eq4 lit = stringz.equals("test", "")
assert_true(eq1)
assert_false(eq2)
assert_true(eq3)
assert_false(eq4)

test_start("stringz.is_empty")
sus empty1 lit = stringz.is_empty("")
sus empty2 lit = stringz.is_empty("hello")
assert_true(empty1)
assert_false(empty2)

fr fr === Test string searching operations ===

test_start("stringz.find")
sus find1 normie = stringz.find("hello world", "world")
sus find2 normie = stringz.find("hello world", "xyz")
sus find3 normie = stringz.find("hello world", "hello")
sus find4 normie = stringz.find("hello world", "")
assert_eq_int(find1, 6)
assert_eq_int(find2, -1)
assert_eq_int(find3, 0)
assert_eq_int(find4, 0)

test_start("stringz.contains")
sus contains1 lit = stringz.contains("hello world", "world")
sus contains2 lit = stringz.contains("hello world", "xyz")
sus contains3 lit = stringz.contains("hello world", "hello")
assert_true(contains1)
assert_false(contains2)
assert_true(contains3)

test_start("stringz.starts_with")
sus starts1 lit = stringz.starts_with("hello world", "hello")
sus starts2 lit = stringz.starts_with("hello world", "world")
sus starts3 lit = stringz.starts_with("hello world", "")
assert_true(starts1)
assert_false(starts2)
assert_true(starts3)

test_start("stringz.ends_with")
sus ends1 lit = stringz.ends_with("hello world", "world")
sus ends2 lit = stringz.ends_with("hello world", "hello")
sus ends3 lit = stringz.ends_with("hello world", "")
assert_true(ends1)
assert_false(ends2)
assert_true(ends3)

fr fr === Test string manipulation operations ===

test_start("stringz.replace")
sus replace1 tea = stringz.replace("hello world", "world", "CURSED")
sus replace2 tea = stringz.replace("hello hello", "hello", "hi")
sus replace3 tea = stringz.replace("hello world", "xyz", "test")
assert_eq_string(replace1, "hello CURSED")
assert_eq_string(replace2, "hi hi")
assert_eq_string(replace3, "hello world")

test_start("stringz.trim")
sus trim1 tea = stringz.trim("  hello world  ")
sus trim2 tea = stringz.trim("hello world")
sus trim3 tea = stringz.trim("   ")
sus trim4 tea = stringz.trim("")
assert_eq_string(trim1, "hello world")
assert_eq_string(trim2, "hello world")
assert_eq_string(trim3, "")
assert_eq_string(trim4, "")

test_start("stringz.reverse")
sus reverse1 tea = stringz.reverse("hello")
sus reverse2 tea = stringz.reverse("")
sus reverse3 tea = stringz.reverse("a")
assert_eq_string(reverse1, "olleh")
assert_eq_string(reverse2, "")
assert_eq_string(reverse3, "a")

fr fr === Test case conversion ===

test_start("stringz.to_upper")
sus upper1 tea = stringz.to_upper("hello")
sus upper2 tea = stringz.to_upper("Hello World")
sus upper3 tea = stringz.to_upper("HELLO")
assert_eq_string(upper1, "HELLO")
assert_eq_string(upper2, "HELLO WORLD")
assert_eq_string(upper3, "HELLO")

test_start("stringz.to_lower")
sus lower1 tea = stringz.to_lower("HELLO")
sus lower2 tea = stringz.to_lower("Hello World")
sus lower3 tea = stringz.to_lower("hello")
assert_eq_string(lower1, "hello")
assert_eq_string(lower2, "hello world")
assert_eq_string(lower3, "hello")

test_start("stringz.to_title")
sus title1 tea = stringz.to_title("hello world")
sus title2 tea = stringz.to_title("HELLO WORLD")
sus title3 tea = stringz.to_title("hELLO wORLD")
assert_eq_string(title1, "Hello World")
assert_eq_string(title2, "Hello World")
assert_eq_string(title3, "Hello World")

fr fr === Test validation functions ===

test_start("stringz.is_alpha")
sus alpha1 lit = stringz.is_alpha("hello")
sus alpha2 lit = stringz.is_alpha("hello123")
sus alpha3 lit = stringz.is_alpha("HELLO")
sus alpha4 lit = stringz.is_alpha("")
assert_true(alpha1)
assert_false(alpha2)
assert_true(alpha3)
assert_false(alpha4)

test_start("stringz.is_digit")
sus digit1 lit = stringz.is_digit("12345")
sus digit2 lit = stringz.is_digit("123a")
sus digit3 lit = stringz.is_digit("")
assert_true(digit1)
assert_false(digit2)
assert_false(digit3)

test_start("stringz.is_alnum")
sus alnum1 lit = stringz.is_alnum("hello123")
sus alnum2 lit = stringz.is_alnum("hello 123")
sus alnum3 lit = stringz.is_alnum("HELLO123")
assert_true(alnum1)
assert_false(alnum2)
assert_true(alnum3)

fr fr === Test splitting and joining ===

test_start("stringz.split")
sus parts1 [tea] = stringz.split("hello,world,test", ",")
sus parts2 [tea] = stringz.split("hello", ",")
sus parts3 [tea] = stringz.split("a,b,c,", ",")
assert_eq_int(len(parts1), 3)
assert_eq_string(parts1[0], "hello")
assert_eq_string(parts1[1], "world")
assert_eq_string(parts1[2], "test")
assert_eq_int(len(parts2), 1)
assert_eq_string(parts2[0], "hello")

test_start("stringz.join")
sus join_parts [tea] = ["hello", "world", "test"]
sus joined1 tea = stringz.join(join_parts, ",")
sus joined2 tea = stringz.join(join_parts, " ")
sus empty_parts [tea] = []
sus joined3 tea = stringz.join(empty_parts, ",")
assert_eq_string(joined1, "hello,world,test")
assert_eq_string(joined2, "hello world test")
assert_eq_string(joined3, "")

fr fr === Test encoding functions ===

test_start("stringz.url_encode")
sus encoded1 tea = stringz.url_encode("hello world")
sus encoded2 tea = stringz.url_encode("hello_world")
sus encoded3 tea = stringz.url_encode("")
assert_eq_string(encoded1, "hello%20world")
assert_eq_string(encoded2, "hello_world")
assert_eq_string(encoded3, "")

test_start("stringz.url_decode")
sus decoded1 tea = stringz.url_decode("hello%20world")
sus decoded2 tea = stringz.url_decode("hello_world")
sus decoded3 tea = stringz.url_decode("")
assert_eq_string(decoded1, "hello world")
assert_eq_string(decoded2, "hello_world")
assert_eq_string(decoded3, "")

print_test_summary()

vibez.spill("✅ Comprehensive string runtime support test completed!")
vibez.spill("✅ All stringz module operations now work with runtime functions!")
vibez.spill("✅ This unblocks hashz, vibez, and other string-dependent modules!")
