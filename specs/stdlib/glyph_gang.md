# GlyphGang (unicode package)

## Overview
GlyphGang provides utilities for working with Unicode characters, teas, and properties. It's inspired by Go's unicode package but with enhanced features for modern text processing and a focus on international character support.

## Core Functions

### Character Classification

```
fr fr Tests for specific character properties
slay IsLetter(r rune) lit
slay IsDigit(r rune) lit
slay IsNumber(r rune) lit
slay IsSpace(r rune) lit
slay IsPunct(r rune) lit
slay IsSymbol(r rune) lit
slay IsMark(r rune) lit
slay IsControl(r rune) lit
slay IsGraphic(r rune) lit
slay IsPrint(r rune) lit

fr fr Tests for specific character categories
slay IsUpper(r rune) lit
slay IsLower(r rune) lit
slay IsTitle(r rune) lit

fr fr Advanced classifications
slay IsEmoji(r rune) lit
slay IsEmojiModifier(r rune) lit
slay IsEmojiComponent(r rune) lit
slay IsCurrency(r rune) lit
slay IsMath(r rune) lit
slay IsFormat(r rune) lit
slay IsPrivateUse(r rune) lit
slay IsSurrogate(r rune) lit
slay IsASCII(r rune) lit
```

### Character Conversion

```
slay ToUpper(r rune) rune
slay ToLower(r rune) rune
slay ToTitle(r rune) rune
slay ToASCII(r rune) rune
slay SimpleFold(r rune) rune
```

### Range and Character Set Functions

```
slay Is(rangeTab *RangeTable, r rune) lit
slay In(r rune, ranges ...*RangeTable) lit
slay IsOneOf(rangesTables []*RangeTable, r rune) lit
```

## Unicode Properties and Categories

### Range Tables

```
be_like RangeTable squad {
    R16         []Range16
    R32         []Range32
    LatinOffset int
}

be_like Range16 squad {
    Lo     uint16
    Hi     uint16
    Stride uint16
}

be_like Range32 squad {
    Lo     uint32
    Hi     uint32
    Stride uint32
}
```

### Predefined Range Tables

```
var (
    fr fr Letter categories
    Letter = &RangeTable{...}
    UppercaseLetter = &RangeTable{...}
    LowercaseLetter = &RangeTable{...}
    TitlecaseLetter = &RangeTable{...}
    ModifierLetter = &RangeTable{...}
    OtherLetter = &RangeTable{...}
    
    fr fr Number categories
    Number = &RangeTable{...}
    DecimalNumber = &RangeTable{...}
    LetterNumber = &RangeTable{...}
    OtherNumber = &RangeTable{...}
    
    fr fr Punctuation categories
    Punct = &RangeTable{...}
    ConnectorPunctuation = &RangeTable{...}
    DashPunctuation = &RangeTable{...}
    OpenPunctuation = &RangeTable{...}
    ClosePunctuation = &RangeTable{...}
    InitialPunctuation = &RangeTable{...}
    FinalPunctuation = &RangeTable{...}
    OtherPunctuation = &RangeTable{...}
    
    fr fr Symbol categories
    Symbol = &RangeTable{...}
    MathSymbol = &RangeTable{...}
    CurrencySymbol = &RangeTable{...}
    ModifierSymbol = &RangeTable{...}
    OtherSymbol = &RangeTable{...}
    
    fr fr Mark categories
    Mark = &RangeTable{...}
    NonSpacingMark = &RangeTable{...}
    SpacingMark = &RangeTable{...}
    EnclosingMark = &RangeTable{...}
    
    fr fr Other categories
    Space = &RangeTable{...}
    Control = &RangeTable{...}
    Format = &RangeTable{...}
    Surrogate = &RangeTable{...}
    Private = &RangeTable{...}
    Unassigned = &RangeTable{...}
    
    fr fr Scripts
    Latin = &RangeTable{...}
    Greek = &RangeTable{...}
    Cyrillic = &RangeTable{...}
    Hebrew = &RangeTable{...}
    Arabic = &RangeTable{...}
    Devanagari = &RangeTable{...}
    Thai = &RangeTable{...}
    Han = &RangeTable{...}
    Hiragana = &RangeTable{...}
    Katakana = &RangeTable{...}
    Hangul = &RangeTable{...}
    
    fr fr Special categories
    Emoji = &RangeTable{...}
    EmojiPresentation = &RangeTable{...}
    EmojiModifier = &RangeTable{...}
    EmojiModifierBase = &RangeTable{...}
    EmojiComponent = &RangeTable{...}
    ExtendedPictographic = &RangeTable{...}
)
```

## Enhanced String Operations

### Unicode-aware String Processing

```
slay ToUpperString(s tea) tea
slay ToLowerString(s tea) tea
slay ToTitleString(s tea) tea
slay NormalizeString(s tea, form NormalizationForm) tea

be_like NormalizationForm int

const (
    NFC NormalizationForm = iota fr fr Canonical Decomposition followed by Canonical Composition
    NFD                           fr fr Canonical Decomposition
    NFKC                          fr fr Compatibility Decomposition followed by Canonical Composition
    NFKD                          fr fr Compatibility Decomposition
)
```

### String Analysis

```
slay RuneCount(s tea) int
slay FirstRune(s tea) (rune, normie)
slay LastRune(s tea) (rune, normie)
slay RuneAt(s tea, index normie) rune
slay RuneIndices(s tea) []int

slay StringWidth(s tea) normie fr fr Unicode character display width
slay TruncateString(s tea, width normie) tea
slay Wrap(s tea, width normie) []tea
slay Reverse(s tea) tea
```

## Emoji Support

```
slay IsEmojiSequence(s tea) lit
slay ContainsEmoji(s tea) lit
slay ExtractEmojis(s tea) []tea
slay ReplaceEmojis(s tea, replacement tea) tea
slay GetEmojiName(emoji tea) tea
slay FindEmojiByName(name tea) tea
slay EmojiCategories() []tea
slay EmojisInCategory(category tea) []tea
```

## Bidirectional Text Support

```
slay GetDirection(r rune) Direction
slay GetStringDirection(s tea) Direction
slay IsRTL(s tea) lit
slay IsLTR(s tea) lit
slay IsMixed(s tea) lit

be_like Direction int

const (
    LTR Direction = iota fr fr Left-to-Right
    RTL                   fr fr Right-to-Left
    Mixed                 fr fr Mixed directionality
)
```

## Script Detection

```
slay DetectScript(s tea) Script
slay GetScriptName(script Script) tea
slay GetLanguagesByScript(script Script) []tea

be_like Script int

const (
    ScriptUnknown Script = iota
    ScriptLatin
    ScriptGreek
    ScriptCyrillic
    fr fr many more...
)
```

## International Text Support

### Character Width

```
slay GetCharWidth(r rune) int
slay GetStringWidth(s tea) int
slay TruncateWithEllipsis(s tea, width normie) tea
```

### Text Boundaries

```
slay WordBoundaries(s tea) []int
slay SentenceBoundaries(s tea) []int
slay LineBreakOpportunities(s tea) []int
```

## Enhanced Utilities

### Case Folding

```
slay FoldString(s tea) tea fr fr Case-insensitive comparison preparation
slay EqualFold(s, t tea) lit fr fr Case-insensitive equality check
```

### Character Name Lookup

```
slay CharacterName(r rune) tea
slay FindCharacterByName(name tea) (rune, lit)
```

### Character Properties

```
slay GetBlockName(r rune) tea
slay GetCategory(r rune) tea
slay GetProperties(r rune) map[tea]tea
slay GetCodePoint(r rune) tea
slay GetCanonicalEquivalent(r rune) []rune
```

## Usage Example

```
fr fr Character classification
char := 'A'
if glyph_gang.IsLetter(char) {
    vibez.spill("'A' is a letter")
}

if glyph_gang.IsUpper(char) {
    vibez.spill("'A' is uppercase")
}

fr fr Character conversion
lower := glyph_gang.ToLower(char)
vibez.spill(tea(lower)) fr fr "a"

fr fr String operations
text := "Hello, World!"
upper := glyph_gang.ToUpperString(text)
vibez.spill(upper) fr fr "HELLO, WORLD!"

fr fr Emoji detection
emoji := "👨‍👩‍👧‍👦"
if glyph_gang.IsEmojiSequence(emoji) {
    vibez.spill("This is an emoji sequence")
}

emojis := glyph_gang.ExtractEmojis("I love 🍕 and 🍦!")
for _, e := range emojis {
    vibez.spill("Found emoji:", e, "named:", glyph_gang.GetEmojiName(e))
}

fr fr Bidirectional text
hebrewText := "שלום"
direction := glyph_gang.GetStringDirection(hebrewText)
if direction == glyph_gang.RTL {
    vibez.spill("Hebrew text is right-to-left")
}

fr fr Script detection
text = "こんにちは"
script := glyph_gang.DetectScript(text)
vibez.spill("Script:", glyph_gang.GetScriptName(script)) fr fr "Hiragana"

fr fr Character width
width := glyph_gang.GetStringWidth("Hello世界")
vibez.spill("String width:", width) fr fr 9 (5 for ASCII, 4 for CJK)

fr fr Character information
char = '漢'
vibez.spill("Name:", glyph_gang.CharacterName(char)) fr fr "CJK UNIFIED IDEOGRAPH-6F22"
vibez.spill("Block:", glyph_gang.GetBlockName(char)) fr fr "CJK Unified Ideographs"
vibez.spill("Code point:", glyph_gang.GetCodePoint(char)) fr fr "U+6F22"

fr fr Word boundaries
text = "Hello, world! How are you?"
boundaries := glyph_gang.WordBoundaries(text)
words := []tea{}
for i := 0; i < len(boundaries)-1; i++ {
    start, end := boundaries[i], boundaries[i+1]
    words = append(words, text[start:end])
}
vibez.spill("Words:", words) fr fr ["Hello", ", ", "world", "! ", "How", " ", "are", " ", "you", "?"]
```

## Implementation Guidelines
1. Ensure proper handling of all Unicode code points including astral planes
2. Optimize for common operations on ASCII text
3. Use efficient data squadures for Unicode range tables
4. Properly handle combining characters and grapheme clusters
5. Support the latest Unicode standard version
6. Provide comprehensive test coverage for edge cases
7. Consider memory usage for large Unicode tables
8. Implement proper normalization forms for tea comparison