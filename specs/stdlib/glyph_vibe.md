# glyph_vibe (unicode)

## Overview
The `glyph_vibe` module provides functionality for working with Unicode characters, strings, and related algorithms. It includes features for character properties, normalization, case mapping, and collation.

## Core Types and Interfaces

### Rune Properties
Represents properties of Unicode code points.

```csd
type Properties struct {
  Category           string // Unicode category (e.g., "Lu" for uppercase letter)
  Name               string // Unicode character name
  CanonicalCombining int    // Canonical combining class
  DecompositionType  string // Decomposition type
  Decomposition      []rune // Decomposition mapping
  NumericType        int    // Numeric type
  NumericValue       float64 // Numeric value
  BidiClass          string // Bidirectional class
  Upper              rune   // Uppercase mapping
  Lower              rune   // Lowercase mapping
  Title              rune   // Titlecase mapping
  Folded             rune   // Case folding
  Script             string // Script name
}
```

### Range Table
Represents a range of Unicode code points.

```csd
type RangeTable struct {
  R16         []Range16
  R32         []Range32
  LatinOffset int
}
```

### Collator
Interface for language-sensitive string comparison.

```csd
type Collator struct {
  // fields not directly accessible
}

func NewCollator(locale string) *Collator
func (c *Collator) Compare(a, b string) int
func (c *Collator) Key(s string) []byte
func (c *Collator) SetStrength(strength int)
func (c *Collator) SetVariable(variable int)
```

### Normalizer
Types for Unicode normalization.

```csd
type Form int

const (
  NFC  Form = iota // Canonical composition
  NFD              // Canonical decomposition
  NFKC             // Compatibility composition
  NFKD             // Compatibility decomposition
)

func Normalize(form Form, s string) string
func IsNormalized(form Form, s string) bool
```

## Core Functions

### Character Properties

```csd
// Get properties of a Unicode code point
func Properties(r rune) *Properties

// Check if rune has property
func Is(rangeTab *RangeTable, r rune) bool

// Check if rune is a letter
func IsLetter(r rune) bool

// Check if rune is a digit
func IsDigit(r rune) bool

// Check if rune is a number
func IsNumber(r rune) bool

// Check if rune is a space
func IsSpace(r rune) bool

// Check if rune is a mark
func IsMark(r rune) bool

// Check if rune is a symbol
func IsSymbol(r rune) bool

// Check if rune is a punctuation
func IsPunct(r rune) bool

// Check if rune is a control character
func IsControl(r rune) bool

// Check if rune is printable
func IsPrint(r rune) bool

// Check if rune is graphic
func IsGraphic(r rune) bool
```

### Case Mapping

```csd
// Convert rune to uppercase
func ToUpper(r rune) rune

// Convert rune to lowercase
func ToLower(r rune) rune

// Convert rune to title case
func ToTitle(r rune) rune

// Convert string to uppercase
func ToUpperString(s string) string

// Convert string to lowercase
func ToLowerString(s string) string

// Convert string to title case
func ToTitleString(s string) string

// Case-insensitive comparison
func EqualFold(s, t string) bool
```

### Normalization

```csd
// Normalize string to a specific form
func Normalize(form Form, s string) string

// Check if string is normalized
func IsNormalized(form Form, s string) bool

// Decompose string
func Decompose(s string, compat bool) []rune

// Compose string
func Compose(s string) string
```

## Enhanced Features

- **Grapheme Cluster Breaking**: Identify user-perceived characters
  ```csd
  clusters := glyph_vibe.GraphemeClusters("👨‍👩‍👧‍👦")
  ```

- **Advanced Collation**: Language-specific string sorting
  ```csd
  collator := glyph_vibe.NewCollator("fr-FR")
  sorted := collator.SortStrings(names)
  ```

- **Unicode Segmentation**: Word and sentence breaking
  ```csd
  words := glyph_vibe.WordSegments(text)
  sentences := glyph_vibe.SentenceSegments(text)
  ```

- **Bidirectional Algorithm**: Support for right-to-left text
  ```csd
  reordered := glyph_vibe.ApplyBidi(mixedText)
  ```

- **Emoji Support**: Emoji detection and properties
  ```csd
  isEmoji := glyph_vibe.IsEmoji("🔥")
  emojiData := glyph_vibe.EmojiProperties("👨‍👩‍👧‍👦")
  ```

- **Unicode Regular Expressions**: Enhanced regex with Unicode properties
  ```csd
  matcher := glyph_vibe.NewMatcher(`\p{Script=Han}+`)
  chinese := matcher.FindAll(text)
  ```

## Usage Examples

```csd
// Basic character properties
character := '🔥'
props := glyph_vibe.Properties(character)
vibez.spill("Character: %c", character)
vibez.spill("Name: %s", props.Name)
vibez.spill("Category: %s", props.Category)
vibez.spill("Script: %s", props.Script)

// Character classification
text := "Hello, 世界! 123"
for _, r := range text {
  vibez.spill("Character: %c", r)
  vibez.spill("  IsLetter: %v", glyph_vibe.IsLetter(r))
  vibez.spill("  IsDigit: %v", glyph_vibe.IsDigit(r))
  vibez.spill("  IsSymbol: %v", glyph_vibe.IsSymbol(r))
  vibez.spill("  IsPunct: %v", glyph_vibe.IsPunct(r))
  vibez.spill("  IsSpace: %v", glyph_vibe.IsSpace(r))
}

// Case mapping
original := "Hello, World!"
lower := glyph_vibe.ToLowerString(original)
upper := glyph_vibe.ToUpperString(original)
title := glyph_vibe.ToTitleString(original)

vibez.spill("Original: %s", original)
vibez.spill("Lowercase: %s", lower)
vibez.spill("Uppercase: %s", upper)
vibez.spill("Titlecase: %s", title)

// Case-insensitive comparison
str1 := "Café"
str2 := "café"
vibez.spill("%s and %s are equal (case-insensitive): %v", 
  str1, str2, glyph_vibe.EqualFold(str1, str2))

// Normalization
original = "café"  // with combining acute accent
nfc := glyph_vibe.Normalize(glyph_vibe.NFC, original)
nfd := glyph_vibe.Normalize(glyph_vibe.NFD, original)

vibez.spill("Original bytes: %v", []byte(original))
vibez.spill("NFC bytes: %v", []byte(nfc))
vibez.spill("NFD bytes: %v", []byte(nfd))
vibez.spill("Original runes: %d", stringz.RuneCountInString(original))
vibez.spill("NFC runes: %d", stringz.RuneCountInString(nfc))
vibez.spill("NFD runes: %d", stringz.RuneCountInString(nfd))

// Normalization check
vibez.spill("Original is NFC normalized: %v", glyph_vibe.IsNormalized(glyph_vibe.NFC, original))
vibez.spill("Original is NFD normalized: %v", glyph_vibe.IsNormalized(glyph_vibe.NFD, original))

// Grapheme clusters (user-perceived characters)
text = "👨‍👩‍👧‍👦" // Family emoji (multiple code points)
clusters := glyph_vibe.GraphemeClusters(text)
vibez.spill("Text: %s", text)
vibez.spill("Bytes: %d", len(text))
vibez.spill("Runes: %d", stringz.RuneCountInString(text))
vibez.spill("Grapheme clusters: %d", len(clusters))
for i, cluster := range clusters {
  vibez.spill("Cluster %d: %s (%d code points)", i, cluster, stringz.RuneCountInString(cluster))
}

// Collation (language-sensitive sorting)
names := []string{"café", "cafe", "apple", "Étoile", "zebra"}

// Default sort
sort_slay.Strings(names)
vibez.spill("Default sort: %v", names)

// French collation
collator := glyph_vibe.NewCollator("fr-FR")
sorted := make([]string, len(names))
copy(sorted, names)
sort_slay.Slice(sorted, func(i, j int) bool {
  return collator.Compare(sorted[i], sorted[j]) < 0
})
vibez.spill("French collation: %v", sorted)

// Collation keys (for efficient repeated comparisons)
keys := make([]string, len(names))
for i, name := range names {
  keys[i] = string(collator.Key(name))
}

// Word segmentation
text = "Hello world! This is a test of word segmentation."
words := glyph_vibe.WordSegments(text)
vibez.spill("Words in text:")
for i, word := range words {
  vibez.spill("  %d: %s", i, word)
}

// Bidirectional text
mixedText := "Hello, مرحبا بالعالم!"
reordered := glyph_vibe.ApplyBidi(mixedText)
vibez.spill("Original mixed text: %s", mixedText)
vibez.spill("Reordered for display: %s", reordered)

// Emoji properties
emoji := "🔥"
isEmoji := glyph_vibe.IsEmoji(emoji)
vibez.spill("%s is emoji: %v", emoji, isEmoji)

emojiProps := glyph_vibe.EmojiProperties(emoji)
vibez.spill("Emoji name: %s", emojiProps.Name)
vibez.spill("Emoji version: %s", emojiProps.Version)
vibez.spill("Emoji group: %s", emojiProps.Group)
vibez.spill("Emoji subgroup: %s", emojiProps.Subgroup)
```

## Implementation Guidelines

- Support the full range of Unicode Standard properties and algorithms
- Optimize for both memory usage and performance
- Maintain accurate implementations of Unicode normalization forms
- Support the latest Unicode Standard version
- Implement efficient case mapping and folding
- Provide comprehensive support for Unicode categories and properties
- Ensure proper handling of grapheme clusters and other user-perceived characters
- Support major world scripts and languages
- Maintain thread safety for all operations