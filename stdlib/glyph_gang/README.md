# GlyphGang Unicode Library

GlyphGang is a comprehensive Unicode character and string processing library for CURSED. It provides utilities for working with Unicode characters, strings, and text properties, including character classification, conversion, emoji support, bidirectional text handling, and script detection.

## Features

- **Character Classification**: Test for letters, digits, punctuation, symbols, and more
- **Character Conversion**: Convert between upper/lower/title case
- **String Operations**: Unicode-aware string processing and manipulation
- **Emoji Support**: Detect, extract, and manipulate emoji characters
- **Bidirectional Text**: Support for left-to-right and right-to-left text
- **Script Detection**: Identify writing scripts (Latin, Greek, Cyrillic, etc.)
- **Character Width**: Calculate display width for proper text alignment
- **Text Boundaries**: Find word and sentence boundaries
- **Case Folding**: Case-insensitive string comparison
- **Character Properties**: Get Unicode block, category, and code point information

## Installation

```bash
yeet "glyph_gang"
```

## Usage Examples

### Character Classification

```cursed
yeet "glyph_gang"

# Test if character is a letter
if glyph_gang.IsLetter('A') {
    vibez.spill("A is a letter")
}

# Test if character is uppercase
if glyph_gang.IsUpper('A') {
    vibez.spill("A is uppercase")
}

# Test if character is a digit
if glyph_gang.IsDigit('5') {
    vibez.spill("5 is a digit")
}

# Test if character is punctuation
if glyph_gang.IsPunct('!') {
    vibez.spill("! is punctuation")
}
```

### Character Conversion

```cursed
# Convert to uppercase
upper_char := glyph_gang.ToUpper('a')
vibez.spill(tea(upper_char)) # "A"

# Convert to lowercase
lower_char := glyph_gang.ToLower('Z')
vibez.spill(tea(lower_char)) # "z"

# Convert to title case
title_char := glyph_gang.ToTitle('a')
vibez.spill(tea(title_char)) # "A"
```

### String Operations

```cursed
# Convert string to uppercase
upper_string := glyph_gang.ToUpperString("hello world")
vibez.spill(upper_string) # "HELLO WORLD"

# Convert string to lowercase
lower_string := glyph_gang.ToLowerString("HELLO WORLD")
vibez.spill(lower_string) # "hello world"

# Convert string to title case
title_string := glyph_gang.ToTitleString("hello world")
vibez.spill(title_string) # "Hello World"

# Get string width
width := glyph_gang.StringWidth("Hello世界")
vibez.spill("Width: " + width) # "Width: 9"

# Truncate string
truncated := glyph_gang.TruncateString("Hello World", 5)
vibez.spill(truncated) # "Hello"

# Reverse string
reversed := glyph_gang.Reverse("hello")
vibez.spill(reversed) # "olleh"
```

### Emoji Support

```cursed
# Check if character is emoji
if glyph_gang.IsEmoji('😀') {
    vibez.spill("😀 is an emoji")
}

# Check if string contains emoji
if glyph_gang.ContainsEmoji("Hello 😀") {
    vibez.spill("String contains emoji")
}

# Extract emojis from string
emojis := glyph_gang.ExtractEmojis("I love 🍕 and 🍦!")
for _, emoji := range emojis {
    vibez.spill("Found emoji: " + emoji)
}

# Get emoji name
name := glyph_gang.GetEmojiName("😊")
vibez.spill("Emoji name: " + name) # "smiling face with smiling eyes"

# Find emoji by name
emoji := glyph_gang.FindEmojiByName("smile")
vibez.spill("Found emoji: " + emoji) # "😊"
```

### Bidirectional Text

```cursed
# Get text direction
direction := glyph_gang.GetStringDirection("Hello")
if direction == glyph_gang.LTR {
    vibez.spill("Text is left-to-right")
}

# Check if text is right-to-left
hebrew_text := "שלום"
if glyph_gang.IsRTL(hebrew_text) {
    vibez.spill("Hebrew text is right-to-left")
}
```

### Script Detection

```cursed
# Detect script
script := glyph_gang.DetectScript("Hello")
script_name := glyph_gang.GetScriptName(script)
vibez.spill("Script: " + script_name) # "Script: Latin"

# Get languages by script
languages := glyph_gang.GetLanguagesByScript(glyph_gang.ScriptLatin)
for _, lang := range languages {
    vibez.spill("Language: " + lang)
}
```

### Character Properties

```cursed
# Get character name
name := glyph_gang.CharacterName('A')
vibez.spill("Character name: " + name) # "LATIN CAPITAL LETTER A"

# Get Unicode block
block := glyph_gang.GetBlockName('A')
vibez.spill("Block: " + block) # "Basic Latin"

# Get character category
category := glyph_gang.GetCategory('A')
vibez.spill("Category: " + category) # "Letter"

# Get code point
code_point := glyph_gang.GetCodePoint('A')
vibez.spill("Code point: " + code_point) # "U+0041"
```

### Text Boundaries

```cursed
# Find word boundaries
text := "Hello, world! How are you?"
boundaries := glyph_gang.WordBoundaries(text)
vibez.spill("Word boundaries found: " + len(boundaries))

# Find sentence boundaries
sentences := glyph_gang.SentenceBoundaries(text)
vibez.spill("Sentence boundaries found: " + len(sentences))
```

### Case Folding

```cursed
# Case-insensitive comparison
if glyph_gang.EqualFold("Hello", "HELLO") {
    vibez.spill("Strings are equal (case-insensitive)")
}

# Fold string for comparison
folded := glyph_gang.FoldString("Hello World")
vibez.spill("Folded: " + folded) # "hello world"
```

## Constants

### Direction Constants
- `glyph_gang.LTR` - Left-to-right text direction
- `glyph_gang.RTL` - Right-to-left text direction
- `glyph_gang.Mixed` - Mixed text direction

### Script Constants
- `glyph_gang.ScriptLatin` - Latin script
- `glyph_gang.ScriptGreek` - Greek script
- `glyph_gang.ScriptCyrillic` - Cyrillic script
- `glyph_gang.ScriptHebrew` - Hebrew script
- `glyph_gang.ScriptArabic` - Arabic script
- `glyph_gang.ScriptHan` - Han (Chinese) script
- `glyph_gang.ScriptHiragana` - Hiragana script
- `glyph_gang.ScriptKatakana` - Katakana script
- `glyph_gang.ScriptHangul` - Hangul script

### Normalization Form Constants
- `glyph_gang.NFC` - Canonical Decomposition followed by Canonical Composition
- `glyph_gang.NFD` - Canonical Decomposition
- `glyph_gang.NFKC` - Compatibility Decomposition followed by Canonical Composition
- `glyph_gang.NFKD` - Compatibility Decomposition

## Character Classification Functions

- `IsLetter(r rune) lit` - Test if character is a letter
- `IsDigit(r rune) lit` - Test if character is a digit
- `IsNumber(r rune) lit` - Test if character is a number
- `IsSpace(r rune) lit` - Test if character is whitespace
- `IsPunct(r rune) lit` - Test if character is punctuation
- `IsSymbol(r rune) lit` - Test if character is a symbol
- `IsMark(r rune) lit` - Test if character is a combining mark
- `IsControl(r rune) lit` - Test if character is a control character
- `IsGraphic(r rune) lit` - Test if character is graphic (visible)
- `IsPrint(r rune) lit` - Test if character is printable
- `IsUpper(r rune) lit` - Test if character is uppercase
- `IsLower(r rune) lit` - Test if character is lowercase
- `IsTitle(r rune) lit` - Test if character is titlecase
- `IsEmoji(r rune) lit` - Test if character is an emoji
- `IsEmojiModifier(r rune) lit` - Test if character is an emoji modifier
- `IsEmojiComponent(r rune) lit` - Test if character is an emoji component
- `IsCurrency(r rune) lit` - Test if character is a currency symbol
- `IsMath(r rune) lit` - Test if character is a mathematical symbol
- `IsFormat(r rune) lit` - Test if character is a format character
- `IsPrivateUse(r rune) lit` - Test if character is in private use area
- `IsSurrogate(r rune) lit` - Test if character is a surrogate
- `IsASCII(r rune) lit` - Test if character is ASCII

## Character Conversion Functions

- `ToUpper(r rune) rune` - Convert character to uppercase
- `ToLower(r rune) rune` - Convert character to lowercase
- `ToTitle(r rune) rune` - Convert character to titlecase
- `ToASCII(r rune) rune` - Convert character to ASCII equivalent
- `SimpleFold(r rune) rune` - Simple case folding

## String Operations Functions

- `ToUpperString(s tea) tea` - Convert string to uppercase
- `ToLowerString(s tea) tea` - Convert string to lowercase
- `ToTitleString(s tea) tea` - Convert string to title case
- `NormalizeString(s tea, form NormalizationForm) tea` - Normalize string
- `RuneCount(s tea) normie` - Count runes in string
- `FirstRune(s tea) (rune, normie)` - Get first rune and its byte length
- `LastRune(s tea) (rune, normie)` - Get last rune and its byte length
- `RuneAt(s tea, index normie) rune` - Get rune at index
- `RuneIndices(s tea) [normie]` - Get indices of all runes
- `StringWidth(s tea) normie` - Calculate display width
- `TruncateString(s tea, width normie) tea` - Truncate to width
- `TruncateWithEllipsis(s tea, width normie) tea` - Truncate with ellipsis
- `Wrap(s tea, width normie) [tea]` - Wrap text to width
- `Reverse(s tea) tea` - Reverse string

## Emoji Support Functions

- `IsEmojiSequence(s tea) lit` - Check if string is emoji sequence
- `ContainsEmoji(s tea) lit` - Check if string contains emoji
- `ExtractEmojis(s tea) [tea]` - Extract all emojis from string
- `ReplaceEmojis(s tea, replacement tea) tea` - Replace emojis
- `GetEmojiName(emoji tea) tea` - Get emoji name
- `FindEmojiByName(name tea) tea` - Find emoji by name
- `EmojiCategories() [tea]` - Get emoji categories
- `EmojisInCategory(category tea) [tea]` - Get emojis in category

## Bidirectional Text Functions

- `GetDirection(r rune) Direction` - Get character direction
- `GetStringDirection(s tea) Direction` - Get string direction
- `IsRTL(s tea) lit` - Check if string is right-to-left
- `IsLTR(s tea) lit` - Check if string is left-to-right
- `IsMixed(s tea) lit` - Check if string has mixed direction

## Script Detection Functions

- `DetectScript(s tea) Script` - Detect script of string
- `GetScriptName(script Script) tea` - Get script name
- `GetLanguagesByScript(script Script) [tea]` - Get languages by script

## Character Width Functions

- `GetCharWidth(r rune) normie` - Get character display width
- `GetStringWidth(s tea) normie` - Get string display width

## Text Boundary Functions

- `WordBoundaries(s tea) [normie]` - Find word boundaries
- `SentenceBoundaries(s tea) [normie]` - Find sentence boundaries
- `LineBreakOpportunities(s tea) [normie]` - Find line break opportunities

## Case Folding Functions

- `FoldString(s tea) tea` - Fold string for comparison
- `EqualFold(s tea, t tea) lit` - Case-insensitive equality

## Character Properties Functions

- `CharacterName(r rune) tea` - Get Unicode character name
- `FindCharacterByName(name tea) (rune, lit)` - Find character by name
- `GetBlockName(r rune) tea` - Get Unicode block name
- `GetCategory(r rune) tea` - Get Unicode category
- `GetProperties(r rune) map[tea]tea` - Get character properties
- `GetCodePoint(r rune) tea` - Get Unicode code point
- `GetCanonicalEquivalent(r rune) [rune]` - Get canonical equivalents

## Testing

Run the test suite:

```bash
cargo run --bin cursed stdlib/glyph_gang/test_glyph_gang.💀
```

Compile and run tests:

```bash
cargo run --bin cursed -- compile stdlib/glyph_gang/test_glyph_gang.💀
./test_glyph_gang
```

## Implementation Notes

- This is a pure CURSED implementation without FFI dependencies
- Basic Unicode support with focus on common character ranges
- Emoji support includes basic ranges and common emojis
- Script detection covers major writing systems
- Character width calculation assumes ASCII = 1, non-ASCII = 2
- Bidirectional text support is simplified but functional
- Case folding and normalization are basic implementations

## Contributing

This module follows the CURSED stdlib patterns and conventions. All functions are implemented in pure CURSED without external dependencies.
