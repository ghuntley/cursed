# glyph_vibe (unicode)

## Overview
The `glyph_vibe` module provides functionality for working with Unicode characters, teas, and related algorithms. It includes features for character properties, normalization, case mapping, and collation.

## Core Types and Interfaces

### Rune Properties
Represents properties of Unicode code points.

```csd
be_like Properties squad {
  Category           tea fr fr Unicode category (e.g., "Lu" for uppercase letter)
  Name               tea fr fr Unicode character name
  CanonicalCombining normie    fr fr Canonical combining class
  DecompositionType  tea fr fr Decomposition type
  Decomposition      []rune fr fr Decomposition mapping
  NumericType        normie    fr fr Numeric type
  NumericValue       float64 fr fr Numeric value
  BidiClass          tea fr fr Bidirectional class
  Upper              rune   fr fr Uppercase mapping
  Lower              rune   fr fr Lowercase mapping
  Title              rune   fr fr Titlecase mapping
  Folded             rune   fr fr Case folding
  Script             tea fr fr Script name
}
```

### Range Table
Represents a range of Unicode code points.

```csd
be_like RangeTable squad {
  R16         []Range16
  R32         []Range32
  LatinOffset int
}
```

### Collator
Interface for language-sensitive tea comparison.

```csd
be_like Collator squad {
  fr fr fields not directly accessible
}

slay NewCollator(locale tea) *Collator
slay (c *Collator) Compare(a, b tea) int
slay (c *Collator) Key(s tea) []byte
slay (c *Collator) SetStrength(strength normie)
slay (c *Collator) SetVariable(variable normie)
```

### Normalizer
Types for Unicode normalization.

```csd
be_like Form int

const (
  NFC  Form = iota fr fr Canonical composition
  NFD              fr fr Canonical decomposition
  NFKC             fr fr Compatibility composition
  NFKD             fr fr Compatibility decomposition
)

slay Normalize(form Form, s tea) tea
slay IsNormalized(form Form, s tea) lit
```

## Core Functions

### Character Properties

```csd
fr fr Get properties of a Unicode code point
slay Properties(r rune) *Properties

fr fr Check if rune has property
slay Is(rangeTab *RangeTable, r rune) lit

fr fr Check if rune is a letter
slay IsLetter(r rune) lit

fr fr Check if rune is a digit
slay IsDigit(r rune) lit

fr fr Check if rune is a number
slay IsNumber(r rune) lit

fr fr Check if rune is a space
slay IsSpace(r rune) lit

fr fr Check if rune is a mark
slay IsMark(r rune) lit

fr fr Check if rune is a symbol
slay IsSymbol(r rune) lit

fr fr Check if rune is a punctuation
slay IsPunct(r rune) lit

fr fr Check if rune is a control character
slay IsControl(r rune) lit

fr fr Check if rune is printable
slay IsPrint(r rune) lit

fr fr Check if rune is graphic
slay IsGraphic(r rune) lit
```

### Case Mapping

```csd
fr fr Convert rune to uppercase
slay ToUpper(r rune) rune

fr fr Convert rune to lowercase
slay ToLower(r rune) rune

fr fr Convert rune to title case
slay ToTitle(r rune) rune

fr fr Convert tea to uppercase
slay ToUpperString(s tea) tea

fr fr Convert tea to lowercase
slay ToLowerString(s tea) tea

fr fr Convert tea to title case
slay ToTitleString(s tea) tea

fr fr Case-insensitive comparison
slay EqualFold(s, t tea) lit
```

### Normalization

```csd
fr fr Normalize tea to a specific form
slay Normalize(form Form, s tea) tea

fr fr Check if tea is normalized
slay IsNormalized(form Form, s tea) lit

fr fr Decompose tea
slay Decompose(s tea, compat lit) []rune

fr fr Compose tea
slay Compose(s tea) tea
```

## Enhanced Features

- **Grapheme Cluster Breaking**: Identify user-perceived characters
  ```csd
  clusters := glyph_vibe.GraphemeClusters("👨‍👩‍👧‍👦")
  ```

- **Advanced Collation**: Language-specific tea sorting
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
fr fr Basic character properties
character := '🔥'
props := glyph_vibe.Properties(character)
vibez.spill("Character: %c", character)
vibez.spill("Name: %s", props.Name)
vibez.spill("Category: %s", props.Category)
vibez.spill("Script: %s", props.Script)

fr fr Character classification
text := "Hello, 世界! 123"
for _, r := range text {
  vibez.spill("Character: %c", r)
  vibez.spill("  IsLetter: %v", glyph_vibe.IsLetter(r))
  vibez.spill("  IsDigit: %v", glyph_vibe.IsDigit(r))
  vibez.spill("  IsSymbol: %v", glyph_vibe.IsSymbol(r))
  vibez.spill("  IsPunct: %v", glyph_vibe.IsPunct(r))
  vibez.spill("  IsSpace: %v", glyph_vibe.IsSpace(r))
}

fr fr Case mapping
original := "Hello, World!"
lower := glyph_vibe.ToLowerString(original)
upper := glyph_vibe.ToUpperString(original)
title := glyph_vibe.ToTitleString(original)

vibez.spill("Original: %s", original)
vibez.spill("Lowercase: %s", lower)
vibez.spill("Uppercase: %s", upper)
vibez.spill("Titlecase: %s", title)

fr fr Case-insensitive comparison
str1 := "Café"
str2 := "café"
vibez.spill("%s and %s are equal (case-insensitive): %v", 
  str1, str2, glyph_vibe.EqualFold(str1, str2))

fr fr Normalization
original = "café"  fr fr with combining acute accent
nfc := glyph_vibe.Normalize(glyph_vibe.NFC, original)
nfd := glyph_vibe.Normalize(glyph_vibe.NFD, original)

vibez.spill("Original bytes: %v", []byte(original))
vibez.spill("NFC bytes: %v", []byte(nfc))
vibez.spill("NFD bytes: %v", []byte(nfd))
vibez.spill("Original runes: %d", stringz.RuneCountInString(original))
vibez.spill("NFC runes: %d", stringz.RuneCountInString(nfc))
vibez.spill("NFD runes: %d", stringz.RuneCountInString(nfd))

fr fr Normalization check
vibez.spill("Original is NFC normalized: %v", glyph_vibe.IsNormalized(glyph_vibe.NFC, original))
vibez.spill("Original is NFD normalized: %v", glyph_vibe.IsNormalized(glyph_vibe.NFD, original))

fr fr Grapheme clusters (user-perceived characters)
text = "👨‍👩‍👧‍👦" fr fr Family emoji (multiple code points)
clusters := glyph_vibe.GraphemeClusters(text)
vibez.spill("Text: %s", text)
vibez.spill("Bytes: %d", len(text))
vibez.spill("Runes: %d", stringz.RuneCountInString(text))
vibez.spill("Grapheme clusters: %d", len(clusters))
for i, cluster := range clusters {
  vibez.spill("Cluster %d: %s (%d code points)", i, cluster, stringz.RuneCountInString(cluster))
}

fr fr Collation (language-sensitive sorting)
names := []tea{"café", "cafe", "apple", "Étoile", "zebra"}

fr fr Default sort
sort_slay.Strings(names)
vibez.spill("Default sort: %v", names)

fr fr French collation
collator := glyph_vibe.NewCollator("fr-FR")
sorted := make([]tea, len(names))
copy(sorted, names)
sort_slay.Slice(sorted, func(i, j normie) lit {
  yolo collator.Compare(sorted[i], sorted[j]) < 0
})
vibez.spill("French collation: %v", sorted)

fr fr Collation keys (for efficient repeated comparisons)
keys := make([]tea, len(names))
for i, name := range names {
  keys[i] = tea(collator.Key(name))
}

fr fr Word segmentation
text = "Hello world! This is a test of word segmentation."
words := glyph_vibe.WordSegments(text)
vibez.spill("Words in text:")
for i, word := range words {
  vibez.spill("  %d: %s", i, word)
}

fr fr Bidirectional text
mixedText := "Hello, مرحبا بالعالم!"
reordered := glyph_vibe.ApplyBidi(mixedText)
vibez.spill("Original mixed text: %s", mixedText)
vibez.spill("Reordered for display: %s", reordered)

fr fr Emoji properties
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