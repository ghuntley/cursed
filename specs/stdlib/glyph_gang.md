# GlyphGang (unicode package)

## Overview
GlyphGang provides utilities for working with Unicode characters, strings, and properties. It's inspired by Go's unicode package but with enhanced features for modern text processing and a focus on international character support.

## Core Functions

### Character Classification

```go
// Tests for specific character properties
func IsLetter(r rune) bool
func IsDigit(r rune) bool
func IsNumber(r rune) bool
func IsSpace(r rune) bool
func IsPunct(r rune) bool
func IsSymbol(r rune) bool
func IsMark(r rune) bool
func IsControl(r rune) bool
func IsGraphic(r rune) bool
func IsPrint(r rune) bool

// Tests for specific character categories
func IsUpper(r rune) bool
func IsLower(r rune) bool
func IsTitle(r rune) bool

// Advanced classifications
func IsEmoji(r rune) bool
func IsEmojiModifier(r rune) bool
func IsEmojiComponent(r rune) bool
func IsCurrency(r rune) bool
func IsMath(r rune) bool
func IsFormat(r rune) bool
func IsPrivateUse(r rune) bool
func IsSurrogate(r rune) bool
func IsASCII(r rune) bool
```

### Character Conversion

```go
func ToUpper(r rune) rune
func ToLower(r rune) rune
func ToTitle(r rune) rune
func ToASCII(r rune) rune
func SimpleFold(r rune) rune
```

### Range and Character Set Functions

```go
func Is(rangeTab *RangeTable, r rune) bool
func In(r rune, ranges ...*RangeTable) bool
func IsOneOf(rangesTables []*RangeTable, r rune) bool
```

## Unicode Properties and Categories

### Range Tables

```go
type RangeTable struct {
    R16         []Range16
    R32         []Range32
    LatinOffset int
}

type Range16 struct {
    Lo     uint16
    Hi     uint16
    Stride uint16
}

type Range32 struct {
    Lo     uint32
    Hi     uint32
    Stride uint32
}
```

### Predefined Range Tables

```go
var (
    // Letter categories
    Letter = &RangeTable{...}
    UppercaseLetter = &RangeTable{...}
    LowercaseLetter = &RangeTable{...}
    TitlecaseLetter = &RangeTable{...}
    ModifierLetter = &RangeTable{...}
    OtherLetter = &RangeTable{...}
    
    // Number categories
    Number = &RangeTable{...}
    DecimalNumber = &RangeTable{...}
    LetterNumber = &RangeTable{...}
    OtherNumber = &RangeTable{...}
    
    // Punctuation categories
    Punct = &RangeTable{...}
    ConnectorPunctuation = &RangeTable{...}
    DashPunctuation = &RangeTable{...}
    OpenPunctuation = &RangeTable{...}
    ClosePunctuation = &RangeTable{...}
    InitialPunctuation = &RangeTable{...}
    FinalPunctuation = &RangeTable{...}
    OtherPunctuation = &RangeTable{...}
    
    // Symbol categories
    Symbol = &RangeTable{...}
    MathSymbol = &RangeTable{...}
    CurrencySymbol = &RangeTable{...}
    ModifierSymbol = &RangeTable{...}
    OtherSymbol = &RangeTable{...}
    
    // Mark categories
    Mark = &RangeTable{...}
    NonSpacingMark = &RangeTable{...}
    SpacingMark = &RangeTable{...}
    EnclosingMark = &RangeTable{...}
    
    // Other categories
    Space = &RangeTable{...}
    Control = &RangeTable{...}
    Format = &RangeTable{...}
    Surrogate = &RangeTable{...}
    Private = &RangeTable{...}
    Unassigned = &RangeTable{...}
    
    // Scripts
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
    
    // Special categories
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

```go
func ToUpperString(s string) string
func ToLowerString(s string) string
func ToTitleString(s string) string
func NormalizeString(s string, form NormalizationForm) string

type NormalizationForm int

const (
    NFC NormalizationForm = iota // Canonical Decomposition followed by Canonical Composition
    NFD                           // Canonical Decomposition
    NFKC                          // Compatibility Decomposition followed by Canonical Composition
    NFKD                          // Compatibility Decomposition
)
```

### String Analysis

```go
func RuneCount(s string) int
func FirstRune(s string) (rune, int)
func LastRune(s string) (rune, int)
func RuneAt(s string, index int) rune
func RuneIndices(s string) []int

func StringWidth(s string) int // Unicode character display width
func TruncateString(s string, width int) string
func Wrap(s string, width int) []string
func Reverse(s string) string
```

## Emoji Support

```go
func IsEmojiSequence(s string) bool
func ContainsEmoji(s string) bool
func ExtractEmojis(s string) []string
func ReplaceEmojis(s string, replacement string) string
func GetEmojiName(emoji string) string
func FindEmojiByName(name string) string
func EmojiCategories() []string
func EmojisInCategory(category string) []string
```

## Bidirectional Text Support

```go
func GetDirection(r rune) Direction
func GetStringDirection(s string) Direction
func IsRTL(s string) bool
func IsLTR(s string) bool
func IsMixed(s string) bool

type Direction int

const (
    LTR Direction = iota // Left-to-Right
    RTL                   // Right-to-Left
    Mixed                 // Mixed directionality
)
```

## Script Detection

```go
func DetectScript(s string) Script
func GetScriptName(script Script) string
func GetLanguagesByScript(script Script) []string

type Script int

const (
    ScriptUnknown Script = iota
    ScriptLatin
    ScriptGreek
    ScriptCyrillic
    // many more...
)
```

## International Text Support

### Character Width

```go
func GetCharWidth(r rune) int
func GetStringWidth(s string) int
func TruncateWithEllipsis(s string, width int) string
```

### Text Boundaries

```go
func WordBoundaries(s string) []int
func SentenceBoundaries(s string) []int
func LineBreakOpportunities(s string) []int
```

## Enhanced Utilities

### Case Folding

```go
func FoldString(s string) string // Case-insensitive comparison preparation
func EqualFold(s, t string) bool // Case-insensitive equality check
```

### Character Name Lookup

```go
func CharacterName(r rune) string
func FindCharacterByName(name string) (rune, bool)
```

### Character Properties

```go
func GetBlockName(r rune) string
func GetCategory(r rune) string
func GetProperties(r rune) map[string]string
func GetCodePoint(r rune) string
func GetCanonicalEquivalent(r rune) []rune
```

## Usage Example

```go
// Character classification
char := 'A'
if glyph_gang.IsLetter(char) {
    vibez.spill("'A' is a letter")
}

if glyph_gang.IsUpper(char) {
    vibez.spill("'A' is uppercase")
}

// Character conversion
lower := glyph_gang.ToLower(char)
vibez.spill(string(lower)) // "a"

// String operations
text := "Hello, World!"
upper := glyph_gang.ToUpperString(text)
vibez.spill(upper) // "HELLO, WORLD!"

// Emoji detection
emoji := "👨‍👩‍👧‍👦"
if glyph_gang.IsEmojiSequence(emoji) {
    vibez.spill("This is an emoji sequence")
}

emojis := glyph_gang.ExtractEmojis("I love 🍕 and 🍦!")
for _, e := range emojis {
    vibez.spill("Found emoji:", e, "named:", glyph_gang.GetEmojiName(e))
}

// Bidirectional text
hebrewText := "שלום"
direction := glyph_gang.GetStringDirection(hebrewText)
if direction == glyph_gang.RTL {
    vibez.spill("Hebrew text is right-to-left")
}

// Script detection
text = "こんにちは"
script := glyph_gang.DetectScript(text)
vibez.spill("Script:", glyph_gang.GetScriptName(script)) // "Hiragana"

// Character width
width := glyph_gang.GetStringWidth("Hello世界")
vibez.spill("String width:", width) // 9 (5 for ASCII, 4 for CJK)

// Character information
char = '漢'
vibez.spill("Name:", glyph_gang.CharacterName(char)) // "CJK UNIFIED IDEOGRAPH-6F22"
vibez.spill("Block:", glyph_gang.GetBlockName(char)) // "CJK Unified Ideographs"
vibez.spill("Code point:", glyph_gang.GetCodePoint(char)) // "U+6F22"

// Word boundaries
text = "Hello, world! How are you?"
boundaries := glyph_gang.WordBoundaries(text)
words := []string{}
for i := 0; i < len(boundaries)-1; i++ {
    start, end := boundaries[i], boundaries[i+1]
    words = append(words, text[start:end])
}
vibez.spill("Words:", words) // ["Hello", ", ", "world", "! ", "How", " ", "are", " ", "you", "?"]
```

## Implementation Guidelines
1. Ensure proper handling of all Unicode code points including astral planes
2. Optimize for common operations on ASCII text
3. Use efficient data structures for Unicode range tables
4. Properly handle combining characters and grapheme clusters
5. Support the latest Unicode standard version
6. Provide comprehensive test coverage for edge cases
7. Consider memory usage for large Unicode tables
8. Implement proper normalization forms for string comparison