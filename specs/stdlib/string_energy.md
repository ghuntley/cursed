# StringEnergy (strings package)

## Overview
StringEnergy provides functions for manipulating UTF-8 encoded strings with enhanced capabilities and vibrant energy. It's inspired by Go's strings package but with additional functionality for modern string operations and text manipulation.

## Core Functions

### String Search Functions

```go
// Contains reports whether substr is within s
func Contains(s, substr string) bool

// ContainsAny reports whether any Unicode code points in chars are within s
func ContainsAny(s, chars string) bool

// ContainsRune reports whether the Unicode code point r is within s
func ContainsRune(s string, r rune) bool

// Count counts the number of non-overlapping instances of substr in s
func Count(s, substr string) int

// HasPrefix tests whether the string s begins with prefix
func HasPrefix(s, prefix string) bool

// HasSuffix tests whether the string s ends with suffix
func HasSuffix(s, suffix string) bool

// Index returns the index of the first instance of substr in s, or -1 if substr is not present
func Index(s, substr string) int

// IndexAny returns the index of the first instance of any Unicode code point from chars in s, or -1 if none is present
func IndexAny(s, chars string) int

// IndexByte returns the index of the first instance of c in s, or -1 if c is not present
func IndexByte(s string, c byte) int

// IndexRune returns the index of the first instance of the Unicode code point r in s, or -1 if not present
func IndexRune(s string, r rune) int

// LastIndex returns the index of the last instance of substr in s, or -1 if substr is not present
func LastIndex(s, substr string) int

// LastIndexAny returns the index of the last instance of any Unicode code point from chars in s, or -1 if none is present
func LastIndexAny(s, chars string) int

// LastIndexByte returns the index of the last instance of c in s, or -1 if c is not present
func LastIndexByte(s string, c byte) int
```

### String Manipulation Functions

```go
// Replace returns a copy of the string s with the first n non-overlapping instances of old replaced by new
func Replace(s, old, new string, n int) string

// ReplaceAll returns a copy of the string s with all non-overlapping instances of old replaced by new
func ReplaceAll(s, old, new string) string

// Join concatenates the elements of a to create a single string with sep between each element
func Join(a []string, sep string) string

// Split slices s into all substrings separated by sep and returns a slice of the substrings
func Split(s, sep string) []string

// SplitN slices s into substrings separated by sep and returns a slice of those substrings
func SplitN(s, sep string, n int) []string

// SplitAfter slices s into all substrings after each instance of sep and returns a slice of those substrings
func SplitAfter(s, sep string) []string

// SplitAfterN slices s into substrings after each instance of sep and returns a slice of those substrings
func SplitAfterN(s, sep string, n int) []string

// Fields splits the string s around each instance of one or more consecutive white space characters
func Fields(s string) []string

// FieldsFunc splits the string s at each run of Unicode code points c satisfying f(c)
func FieldsFunc(s string, f func(rune) bool) []string
```

### String Transformation Functions

```go
// ToUpper returns a copy of the string s with all Unicode letters mapped to their upper case
func ToUpper(s string) string

// ToLower returns a copy of the string s with all Unicode letters mapped to their lower case
func ToLower(s string) string

// ToTitle returns a copy of the string s with all Unicode letters mapped to their title case
func ToTitle(s string) string

// Title returns a copy of the string s with all Unicode letters that begin words mapped to their title case
func Title(s string) string

// TrimSpace returns a slice of the string s with all leading and trailing white space removed
func TrimSpace(s string) string

// Trim returns a slice of the string s with all leading and trailing Unicode code points contained in cutset removed
func Trim(s, cutset string) string

// TrimLeft returns a slice of the string s with all leading Unicode code points contained in cutset removed
func TrimLeft(s, cutset string) string

// TrimRight returns a slice of the string s with all trailing Unicode code points contained in cutset removed
func TrimRight(s, cutset string) string

// TrimPrefix returns s without the provided leading prefix string
func TrimPrefix(s, prefix string) string

// TrimSuffix returns s without the provided trailing suffix string
func TrimSuffix(s, suffix string) string

// Repeat returns a new string consisting of count copies of the string s
func Repeat(s string, count int) string

// Map returns a copy of the string s with all its characters modified according to the mapping function
func Map(mapping func(rune) rune, s string) string
```

### String Comparison Functions

```go
// EqualFold reports whether s and t, interpreted as UTF-8 strings, are equal under Unicode case-folding
func EqualFold(s, t string) bool

// Compare returns an integer comparing two strings lexicographically
func Compare(a, b string) int
```

## Enhanced String Features

### String Building

```go
type EnergyBuilder struct {}

// Constructor
func NewEnergyBuilder() *EnergyBuilder
func NewEnergyBuilderWithCapacity(cap int) *EnergyBuilder

// Methods
func (b *EnergyBuilder) WriteString(s string) *EnergyBuilder
func (b *EnergyBuilder) WriteRune(r rune) *EnergyBuilder
func (b *EnergyBuilder) WriteByte(c byte) *EnergyBuilder
func (b *EnergyBuilder) Write(p []byte) (int, error)
func (b *EnergyBuilder) WriteFormat(format string, args ...interface{}) *EnergyBuilder
func (b *EnergyBuilder) Grow(n int) *EnergyBuilder
func (b *EnergyBuilder) Reset() *EnergyBuilder
func (b *EnergyBuilder) Len() int
func (b *EnergyBuilder) Cap() int
func (b *EnergyBuilder) String() string
```

### String Manipulation Utilities

```go
// Reverses a string
func Reverse(s string) string

// Returns the portion of s before the first instance of sep
func Before(s, sep string) string

// Returns the portion of s after the first instance of sep
func After(s, sep string) string

// Returns the portion of s before the last instance of sep
func BeforeLast(s, sep string) string

// Returns the portion of s after the last instance of sep
func AfterLast(s, sep string) string

// Returns chunks of s with the specified size
func Chunk(s string, size int) []string

// Wraps s at the specified line length
func Wrap(s string, lineLength int) string

// Truncates s to the specified length
func Truncate(s string, length int) string

// Truncates s to the specified length with an ellipsis suffix
func TruncateWithEllipsis(s string, length int) string

// Pads s to the left until it has length n
func PadLeft(s string, n int, pad string) string

// Pads s to the right until it has length n
func PadRight(s string, n int, pad string) string

// Centers s in a string of length n
func Center(s string, n int, pad string) string
```

### Pattern and Interpolation Functions

```go
// Tests if s matches a shell pattern (glob)
func MatchPattern(s, pattern string) bool

// Interpolates variables in a string using a map
func Interpolate(s string, vars map[string]string) string

// Translates characters using a translation map
func Translate(s string, translation map[rune]rune) string

// Replace multiple patterns at once
func ReplaceMultiple(s string, replacements map[string]string) string
```

### Text Analysis Functions

```go
// Counts occurrences of each character in s
func CharCount(s string) map[rune]int

// Counts occurrences of each word in s
func WordCount(s string) map[string]int

// Returns the frequency of each character in s as a percentage
func CharFrequency(s string) map[rune]float64

// Returns the number of words in s
func WordCount(s string) int

// Returns the number of sentences in s
func SentenceCount(s string) int

// Calculates the readability score of s
func ReadabilityScore(s string) float64

// Returns the language of s (if detectable)
func DetectLanguage(s string) string

// Returns a list of keywords from s
func ExtractKeywords(s string) []string
```

### Text Transformation Utilities

```go
// Converts s to camelCase
func ToCamelCase(s string) string

// Converts s to PascalCase
func ToPascalCase(s string) string

// Converts s to snake_case
func ToSnakeCase(s string) string

// Converts s to kebab-case
func ToKebabCase(s string) string

// Converts s to Title Case With Proper Rules
func ToProperTitle(s string) string

// Removes all HTML tags from s
func StripHTML(s string) string

// Escapes HTML special characters in s
func EscapeHTML(s string) string

// Unescapes HTML special characters in s
func UnescapeHTML(s string) string

// Normalizes whitespace in s (multiple spaces become one)
func NormalizeSpace(s string) string
```

### GenZ Text Transformations

```go
// Shortens text to GenZ style
func ToGenZStyle(s string) string

// Converts text to GenZ slang
func ToGenZSlang(s string) string

// Adds appropriate emojis to text
func AddEmojis(s string) string

// Creates text for social media with hashtags
func ToSocialText(s string, addHashtags bool) string

// Formats text for different platforms (Twitter, Instagram, etc.)
func FormatForPlatform(s string, platform string) string
```

## Usage Example

```go
// Basic string operations
text := "Hello, World!"
vibez.spill(string_energy.ToLower(text)) // "hello, world!"
vibez.spill(string_energy.Contains(text, "World")) // true

// String building
builder := string_energy.NewEnergyBuilder()
builder.WriteString("Hello")
       .WriteString(", ")
       .WriteString("World")
       .WriteString("!")
result := builder.String()
vibez.spill(result) // "Hello, World!"

// String manipulation utilities
text = "Hello, amazing world of strings!"
vibez.spill(string_energy.Reverse(text)) // "!sgnirts fo dlrow gnizama ,olleH"
vibez.spill(string_energy.Truncate(text, 10)) // "Hello, ama"
vibez.spill(string_energy.TruncateWithEllipsis(text, 10)) // "Hello,..."

// String splitting
parts := string_energy.Split("a,b,c,d", ",")
for i, part := range parts {
    vibez.spill(i, part)
}

// Before/After operations
text = "name: John Smith"
vibez.spill(string_energy.After(text, "name: ")) // "John Smith"
vibez.spill(string_energy.Before(text, " Smith")) // "name: John"

// Text case conversion
text = "hello_world_example"
vibez.spill(string_energy.ToCamelCase(text)) // "helloWorldExample"
vibez.spill(string_energy.ToPascalCase(text)) // "HelloWorldExample"
vibez.spill(string_energy.ToKebabCase(text)) // "hello-world-example"

// Pattern replacement
replacements := map[string]string{
    "Hello": "Hi",
    "World": "Universe",
    "!": "!!!",
}
text = "Hello, World!"
vibez.spill(string_energy.ReplaceMultiple(text, replacements)) // "Hi, Universe!!!"

// Text analysis
text = "The quick brown fox jumps over the lazy dog. The dog was very lazy."
vibez.spill(string_energy.WordCount(text)) // 13
vibez.spill(string_energy.SentenceCount(text)) // 2

wordFreq := string_energy.WordCount(text)
vibez.spill(wordFreq["the"]) // 2
vibez.spill(wordFreq["lazy"]) // 2

// String interpolation
vars := map[string]string{
    "name": "John",
    "age": "30",
}
template := "My name is ${name} and I am ${age} years old."
vibez.spill(string_energy.Interpolate(template, vars)) // "My name is John and I am 30 years old."

// GenZ text transformations
text = "This is really cool and amazing"
vibez.spill(string_energy.ToGenZStyle(text)) // "dis is rly cool & amazing"
vibez.spill(string_energy.ToGenZSlang(text)) // "This is bussin fr no cap"
vibez.spill(string_energy.AddEmojis(text)) // "This is really cool 😎 and amazing 🔥"

// Social media formatting
text = "Just launched our new website with awesome features"
vibez.spill(string_energy.ToSocialText(text, true))
// "Just launched our new website with awesome features ✨🚀 #NewWebsite #Awesome #Launch"
```

## Implementation Guidelines
1. Optimize for performance with minimal allocations
2. Ensure proper handling of UTF-8 encoded strings
3. Handle edge cases like empty strings and special characters
4. Provide clear documentation for each function
5. Keep string manipulation functions immutable (return new strings)
6. Support both ASCII and Unicode operations
7. Implement thread-safe functions for concurrent use
8. Follow consistent naming conventions for related functions