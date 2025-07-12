yeet "testz"
yeet "glyph_gang"

# Test character classification functions
test_start("Character Classification - IsLetter")
assert_true(IsLetter('A'))
assert_true(IsLetter('z'))
assert_false(IsLetter('1'))
assert_false(IsLetter(' '))
assert_false(IsLetter('!'))

test_start("Character Classification - IsDigit")
assert_true(IsDigit('0'))
assert_true(IsDigit('9'))
assert_false(IsDigit('A'))
assert_false(IsDigit(' '))

test_start("Character Classification - IsSpace")
assert_true(IsSpace(' '))
assert_true(IsSpace('\t'))
assert_true(IsSpace('\n'))
assert_false(IsSpace('A'))
assert_false(IsSpace('1'))

test_start("Character Classification - IsPunct")
assert_true(IsPunct('.'))
assert_true(IsPunct('!'))
assert_true(IsPunct('?'))
assert_false(IsPunct('A'))
assert_false(IsPunct('1'))

test_start("Character Classification - IsSymbol")
assert_true(IsSymbol('+'))
assert_true(IsSymbol('*'))
assert_true(IsSymbol('<'))
assert_false(IsSymbol('A'))
assert_false(IsSymbol('1'))

test_start("Character Classification - IsUpper")
assert_true(IsUpper('A'))
assert_true(IsUpper('Z'))
assert_false(IsUpper('a'))
assert_false(IsUpper('1'))

test_start("Character Classification - IsLower")
assert_true(IsLower('a'))
assert_true(IsLower('z'))
assert_false(IsLower('A'))
assert_false(IsLower('1'))

test_start("Character Classification - IsControl")
assert_true(IsControl('\x00'))
assert_true(IsControl('\x1F'))
assert_false(IsControl('A'))
assert_false(IsControl(' '))

test_start("Character Classification - IsASCII")
assert_true(IsASCII('A'))
assert_true(IsASCII('1'))
assert_true(IsASCII(' '))

test_start("Character Classification - IsCurrency")
assert_true(IsCurrency('$'))
assert_false(IsCurrency('A'))
assert_false(IsCurrency('1'))

test_start("Character Classification - IsMath")
assert_true(IsMath('+'))
assert_true(IsMath('-'))
assert_true(IsMath('*'))
assert_true(IsMath('='))
assert_false(IsMath('A'))
assert_false(IsMath('1'))

# Test character conversion functions
test_start("Character Conversion - ToUpper")
assert_eq_int(normie(ToUpper('a')), normie('A'))
assert_eq_int(normie(ToUpper('z')), normie('Z'))
assert_eq_int(normie(ToUpper('A')), normie('A'))
assert_eq_int(normie(ToUpper('1')), normie('1'))

test_start("Character Conversion - ToLower")
assert_eq_int(normie(ToLower('A')), normie('a'))
assert_eq_int(normie(ToLower('Z')), normie('z'))
assert_eq_int(normie(ToLower('a')), normie('a'))
assert_eq_int(normie(ToLower('1')), normie('1'))

test_start("Character Conversion - ToTitle")
assert_eq_int(normie(ToTitle('a')), normie('A'))
assert_eq_int(normie(ToTitle('z')), normie('Z'))

test_start("Character Conversion - ToASCII")
assert_eq_int(normie(ToASCII('A')), normie('A'))
assert_eq_int(normie(ToASCII('1')), normie('1'))

test_start("Character Conversion - SimpleFold")
assert_eq_int(normie(SimpleFold('A')), normie('a'))
assert_eq_int(normie(SimpleFold('Z')), normie('z'))

# Test string operations
test_start("String Operations - ToUpperString")
assert_eq_string(ToUpperString("hello"), "HELLO")
assert_eq_string(ToUpperString("Hello World"), "HELLO WORLD")
assert_eq_string(ToUpperString("ABC"), "ABC")
assert_eq_string(ToUpperString("123"), "123")

test_start("String Operations - ToLowerString")
assert_eq_string(ToLowerString("HELLO"), "hello")
assert_eq_string(ToLowerString("Hello World"), "hello world")
assert_eq_string(ToLowerString("abc"), "abc")
assert_eq_string(ToLowerString("123"), "123")

test_start("String Operations - ToTitleString")
assert_eq_string(ToTitleString("hello world"), "Hello World")
assert_eq_string(ToTitleString("the quick brown fox"), "The Quick Brown Fox")
assert_eq_string(ToTitleString("HELLO WORLD"), "Hello World")

test_start("String Operations - NormalizeString")
assert_eq_string(NormalizeString("hello", NFC), "hello")
assert_eq_string(NormalizeString("world", NFD), "world")

# Test string analysis functions
test_start("String Analysis - RuneCount")
assert_eq_int(RuneCount("hello"), 5)
assert_eq_int(RuneCount(""), 0)
assert_eq_int(RuneCount("a"), 1)
assert_eq_int(RuneCount("hello world"), 11)

test_start("String Analysis - FirstRune")
first_rune, first_len := FirstRune("hello")
assert_eq_int(normie(first_rune), normie('h'))
assert_eq_int(first_len, 1)

empty_rune, empty_len := FirstRune("")
assert_eq_int(normie(empty_rune), 0)
assert_eq_int(empty_len, 0)

test_start("String Analysis - LastRune")
last_rune, last_len := LastRune("hello")
assert_eq_int(normie(last_rune), normie('o'))
assert_eq_int(last_len, 1)

test_start("String Analysis - RuneAt")
assert_eq_int(normie(RuneAt("hello", 0)), normie('h'))
assert_eq_int(normie(RuneAt("hello", 4)), normie('o'))
assert_eq_int(normie(RuneAt("hello", 5)), 0)

test_start("String Analysis - StringWidth")
assert_eq_int(StringWidth("hello"), 5)
assert_eq_int(StringWidth(""), 0)
assert_eq_int(StringWidth("a"), 1)

test_start("String Analysis - TruncateString")
assert_eq_string(TruncateString("hello world", 5), "hello")
assert_eq_string(TruncateString("hello", 10), "hello")
assert_eq_string(TruncateString("hello", 3), "hel")

test_start("String Analysis - Reverse")
assert_eq_string(Reverse("hello"), "olleh")
assert_eq_string(Reverse(""), "")
assert_eq_string(Reverse("a"), "a")
assert_eq_string(Reverse("abc"), "cba")

# Test emoji support
test_start("Emoji Support - ContainsEmoji")
assert_false(ContainsEmoji("Hello World"))
assert_false(ContainsEmoji("123"))

test_start("Emoji Support - ReplaceEmojis")
assert_eq_string(ReplaceEmojis("Hello World", "[emoji]"), "Hello World")

test_start("Emoji Support - GetEmojiName")
assert_eq_string(GetEmojiName("unknown"), "unknown emoji")

test_start("Emoji Support - FindEmojiByName")
assert_eq_string(FindEmojiByName("smile"), "😊")
assert_eq_string(FindEmojiByName("joy"), "😂")
assert_eq_string(FindEmojiByName("unknown"), "")

# Test bidirectional text support
test_start("Bidirectional Text - GetDirection")
assert_eq_int(GetDirection('A'), LTR)

test_start("Bidirectional Text - GetStringDirection")
assert_eq_int(GetStringDirection("Hello"), LTR)

test_start("Bidirectional Text - IsLTR")
assert_true(IsLTR("Hello"))

test_start("Bidirectional Text - IsRTL")
assert_false(IsRTL("Hello"))

# Test script detection
test_start("Script Detection - DetectScript")
assert_eq_int(DetectScript("Hello"), ScriptLatin)

test_start("Script Detection - GetScriptName")
assert_eq_string(GetScriptName(ScriptLatin), "Latin")
assert_eq_string(GetScriptName(ScriptGreek), "Greek")
assert_eq_string(GetScriptName(ScriptCyrillic), "Cyrillic")
assert_eq_string(GetScriptName(ScriptHebrew), "Hebrew")

# Test character width functions
test_start("Character Width - GetCharWidth")
assert_eq_int(GetCharWidth('A'), 1)
assert_eq_int(GetCharWidth('1'), 1)

test_start("Character Width - GetStringWidth")
assert_eq_int(GetStringWidth("Hello"), 5)
assert_eq_int(GetStringWidth(""), 0)
assert_eq_int(GetStringWidth("A"), 1)

test_start("Character Width - TruncateWithEllipsis")
assert_eq_string(TruncateWithEllipsis("Hello World", 8), "Hello...")
assert_eq_string(TruncateWithEllipsis("Hello", 10), "Hello")
assert_eq_string(TruncateWithEllipsis("Hello", 3), "...")

# Test case folding functions
test_start("Case Folding - FoldString")
assert_eq_string(FoldString("Hello"), "hello")
assert_eq_string(FoldString("WORLD"), "world")
assert_eq_string(FoldString("MiXeD"), "mixed")

test_start("Case Folding - EqualFold")
assert_true(EqualFold("Hello", "hello"))
assert_true(EqualFold("WORLD", "world"))
assert_true(EqualFold("MiXeD", "mixed"))
assert_false(EqualFold("Hello", "world"))

# Test character name functions
test_start("Character Names - CharacterName")
assert_eq_string(CharacterName('A'), "LATIN CAPITAL LETTER A")
assert_eq_string(CharacterName('a'), "LATIN SMALL LETTER a")
assert_eq_string(CharacterName('0'), "DIGIT 0")

test_start("Character Names - FindCharacterByName")
char_a, found_a := FindCharacterByName("LATIN CAPITAL LETTER A")
assert_eq_int(normie(char_a), normie('A'))
assert_true(found_a)

char_unknown, found_unknown := FindCharacterByName("UNKNOWN CHARACTER")
assert_eq_int(normie(char_unknown), 0)
assert_false(found_unknown)

# Test character properties
test_start("Character Properties - GetBlockName")
assert_eq_string(GetBlockName('A'), "Basic Latin")

test_start("Character Properties - GetCategory")
assert_eq_string(GetCategory('A'), "Letter")
assert_eq_string(GetCategory('1'), "Number")
assert_eq_string(GetCategory('.'), "Punctuation")
assert_eq_string(GetCategory('+'), "Symbol")
assert_eq_string(GetCategory(' '), "Space")

test_start("Character Properties - GetCodePoint")
assert_eq_string(GetCodePoint('A'), "U+0041")
assert_eq_string(GetCodePoint('0'), "U+0030")

# Test helper functions
test_start("Helper Functions - FormatHex")
assert_eq_string(FormatHex(0), "0000")
assert_eq_string(FormatHex(10), "000A")
assert_eq_string(FormatHex(255), "00FF")

print_test_summary()
