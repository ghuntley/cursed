# StringEnergy (teas package)

## Overview
StringEnergy provides functions for manipulating UTF-8 encoded teas with enhanced capabilities and vibrant energy. It's inspired by Go's teas package but with additional functionality for modern tea operations and text manipulation.

## Core Functions

### String Search Functions

```
fr fr Contains reports whether substr is within s
slay Contains(s, substr tea) lit

fr fr ContainsAny reports whether any Unicode code points in chars are within s
slay ContainsAny(s, chars tea) lit

fr fr ContainsRune reports whether the Unicode code ponormie r is within s
slay ContainsRune(s tea, r rune) lit

fr fr Count counts the number of non-overlapping instances of substr in s
slay Count(s, substr tea) int

fr fr HasPrefix tests whether the tea s begins with prefix
slay HasPrefix(s, prefix tea) lit

fr fr HasSuffix tests whether the tea s ends with suffix
slay HasSuffix(s, suffix tea) lit

fr fr Index yolos the index of the first instance of substr in s, or -1 if substr is not present
slay Index(s, substr tea) int

fr fr IndexAny yolos the index of the first instance of any Unicode code ponormie from chars in s, or -1 if none is present
slay IndexAny(s, chars tea) int

fr fr IndexByte yolos the index of the first instance of c in s, or -1 if c is not present
slay IndexByte(s tea, c byte) int

fr fr IndexRune yolos the index of the first instance of the Unicode code ponormie r in s, or -1 if not present
slay IndexRune(s tea, r rune) int

fr fr LastIndex yolos the index of the last instance of substr in s, or -1 if substr is not present
slay LastIndex(s, substr tea) int

fr fr LastIndexAny yolos the index of the last instance of any Unicode code ponormie from chars in s, or -1 if none is present
slay LastIndexAny(s, chars tea) int

fr fr LastIndexByte yolos the index of the last instance of c in s, or -1 if c is not present
slay LastIndexByte(s tea, c byte) int
```

### String Manipulation Functions

```
fr fr Replace yolos a copy of the tea s with the first n non-overlapping instances of old replaced by new
slay Replace(s, old, new tea, n normie) tea

fr fr ReplaceAll yolos a copy of the tea s with all non-overlapping instances of old replaced by new
slay ReplaceAll(s, old, new tea) tea

fr fr Join concatenates the elements of a to create a single tea with sep between each element
slay Join(a []tea, sep tea) tea

fr fr Split slices s into all subteas separated by sep and yolos a slice of the subteas
slay Split(s, sep tea) []tea

fr fr SplitN slices s into subteas separated by sep and yolos a slice of those subteas
slay SplitN(s, sep tea, n normie) []tea

fr fr SplitAfter slices s into all subteas after each instance of sep and yolos a slice of those subteas
slay SplitAfter(s, sep tea) []tea

fr fr SplitAfterN slices s into subteas after each instance of sep and yolos a slice of those subteas
slay SplitAfterN(s, sep tea, n normie) []tea

fr fr Fields splits the tea s around each instance of one or more consecutive white space characters
slay Fields(s tea) []tea

fr fr FieldsFunc splits the tea s at each run of Unicode code points c satisfying f(c)
slay FieldsFunc(s tea, f func(rune) lit) []tea
```

### String Transformation Functions

```
fr fr ToUpper yolos a copy of the tea s with all Unicode letters mapped to their upper case
slay ToUpper(s tea) tea

fr fr ToLower yolos a copy of the tea s with all Unicode letters mapped to their lower case
slay ToLower(s tea) tea

fr fr ToTitle yolos a copy of the tea s with all Unicode letters mapped to their title case
slay ToTitle(s tea) tea

fr fr Title yolos a copy of the tea s with all Unicode letters that begin words mapped to their title case
slay Title(s tea) tea

fr fr TrimSpace yolos a slice of the tea s with all leading and trailing white space removed
slay TrimSpace(s tea) tea

fr fr Trim yolos a slice of the tea s with all leading and trailing Unicode code points contained in cutset removed
slay Trim(s, cutset tea) tea

fr fr TrimLeft yolos a slice of the tea s with all leading Unicode code points contained in cutset removed
slay TrimLeft(s, cutset tea) tea

fr fr TrimRight yolos a slice of the tea s with all trailing Unicode code points contained in cutset removed
slay TrimRight(s, cutset tea) tea

fr fr TrimPrefix yolos s without the provided leading prefix tea
slay TrimPrefix(s, prefix tea) tea

fr fr TrimSuffix yolos s without the provided trailing suffix tea
slay TrimSuffix(s, suffix tea) tea

fr fr Repeat yolos a new tea consisting of count copies of the tea s
slay Repeat(s tea, count normie) tea

fr fr Map yolos a copy of the tea s with all its characters modified according to the mapping function
slay Map(mapping func(rune) rune, s tea) tea
```

### String Comparison Functions

```
fr fr EqualFold reports whether s and t, interpreted as UTF-8 teas, are equal under Unicode case-folding
slay EqualFold(s, t tea) lit

fr fr Compare yolos an integer comparing two teas lexicographically
slay Compare(a, b tea) int
```

## Enhanced String Features

### String Building

```
be_like EnergyBuilder squad {}

fr fr Consquador
slay NewEnergyBuilder() *EnergyBuilder
slay NewEnergyBuilderWithCapacity(cap normie) *EnergyBuilder

fr fr Methods
slay (b *EnergyBuilder) WriteString(s tea) *EnergyBuilder
slay (b *EnergyBuilder) WriteRune(r rune) *EnergyBuilder
slay (b *EnergyBuilder) WriteByte(c byte) *EnergyBuilder
slay (b *EnergyBuilder) Write(p []byte) (int, tea)
slay (b *EnergyBuilder) WriteFormat(format tea, args ...interface{}) *EnergyBuilder
slay (b *EnergyBuilder) Grow(n normie) *EnergyBuilder
slay (b *EnergyBuilder) Reset() *EnergyBuilder
slay (b *EnergyBuilder) Len() int
slay (b *EnergyBuilder) Cap() int
slay (b *EnergyBuilder) String() tea
```

### String Manipulation Utilities

```
fr fr Reverses a tea
slay Reverse(s tea) tea

fr fr Returns the portion of s before the first instance of sep
slay Before(s, sep tea) tea

fr fr Returns the portion of s after the first instance of sep
slay After(s, sep tea) tea

fr fr Returns the portion of s before the last instance of sep
slay BeforeLast(s, sep tea) tea

fr fr Returns the portion of s after the last instance of sep
slay AfterLast(s, sep tea) tea

fr fr Returns chunks of s with the specified size
slay Chunk(s tea, size normie) []tea

fr fr Wraps s at the specified line length
slay Wrap(s tea, lineLength normie) tea

fr fr Truncates s to the specified length
slay Truncate(s tea, length normie) tea

fr fr Truncates s to the specified length with an ellipsis suffix
slay TruncateWithEllipsis(s tea, length normie) tea

fr fr Pads s to the left until it has length n
slay PadLeft(s tea, n int, pad tea) tea

fr fr Pads s to the right until it has length n
slay PadRight(s tea, n int, pad tea) tea

fr fr Centers s in a tea of length n
slay Center(s tea, n int, pad tea) tea
```

### Pattern and Interpolation Functions

```
fr fr Tests if s matches a shell pattern (glob)
slay MatchPattern(s, pattern tea) lit

fr fr Interpolates variables in a tea using a map
slay Interpolate(s tea, vars map[tea]tea) tea

fr fr Translates characters using a translation map
slay Translate(s tea, translation map[rune]rune) tea

fr fr Replace multiple patterns at once
slay ReplaceMultiple(s tea, replacements map[tea]tea) tea
```

### Text Analysis Functions

```
fr fr Counts occurrences of each character in s
slay CharCount(s tea) map[rune]int

fr fr Counts occurrences of each word in s
slay WordCount(s tea) map[tea]int

fr fr Returns the frequency of each character in s as a percentage
slay CharFrequency(s tea) map[rune]float64

fr fr Returns the number of words in s
slay WordCount(s tea) int

fr fr Returns the number of sentences in s
slay SentenceCount(s tea) int

fr fr Calculates the readability score of s
slay ReadabilityScore(s tea) float64

fr fr Returns the language of s (if detectable)
slay DetectLanguage(s tea) tea

fr fr Returns a list of keywords from s
slay ExtractKeywords(s tea) []tea
```

### Text Transformation Utilities

```
fr fr Converts s to camelCase
slay ToCamelCase(s tea) tea

fr fr Converts s to PascalCase
slay ToPascalCase(s tea) tea

fr fr Converts s to snake_case
slay ToSnakeCase(s tea) tea

fr fr Converts s to kebab-case
slay ToKebabCase(s tea) tea

fr fr Converts s to Title Case With Proper Rules
slay ToProperTitle(s tea) tea

fr fr Removes all HTML tags from s
slay StripHTML(s tea) tea

fr fr Escapes HTML special characters in s
slay EscapeHTML(s tea) tea

fr fr Unescapes HTML special characters in s
slay UnescapeHTML(s tea) tea

fr fr Normalizes whitespace in s (multiple spaces become one)
slay NormalizeSpace(s tea) tea
```

### GenZ Text Transformations

```
fr fr Shortens text to GenZ style
slay ToGenZStyle(s tea) tea

fr fr Converts text to GenZ slang
slay ToGenZSlang(s tea) tea

fr fr Adds appropriate emojis to text
slay AddEmojis(s tea) tea

fr fr Creates text for social media with hashtags
slay ToSocialText(s tea, addHashtags lit) tea

fr fr Formats text for different platforms (Twitter, Instagram, etc.)
slay FormatForPlatform(s tea, platform tea) tea
```

## Usage Example

```
fr fr Basic tea operations
text := "Hello, World!"
vibez.spill(tea_energy.ToLower(text)) fr fr "hello, world!"
vibez.spill(tea_energy.Contains(text, "World")) fr fr based

fr fr String building
builder := tea_energy.NewEnergyBuilder()
builder.WriteString("Hello")
       .WriteString(", ")
       .WriteString("World")
       .WriteString("!")
result := builder.String()
vibez.spill(result) fr fr "Hello, World!"

fr fr String manipulation utilities
text = "Hello, amazing world of teas!"
vibez.spill(tea_energy.Reverse(text)) fr fr "!sgnirts fo dlrow gnizama ,olleH"
vibez.spill(tea_energy.Truncate(text, 10)) fr fr "Hello, ama"
vibez.spill(tea_energy.TruncateWithEllipsis(text, 10)) fr fr "Hello,..."

fr fr String splitting
parts := tea_energy.Split("a,b,c,d", ",")
for i, part := range parts {
    vibez.spill(i, part)
}

fr fr Before/After operations
text = "name: John Smith"
vibez.spill(tea_energy.After(text, "name: ")) fr fr "John Smith"
vibez.spill(tea_energy.Before(text, " Smith")) fr fr "name: John"

fr fr Text case conversion
text = "hello_world_example"
vibez.spill(tea_energy.ToCamelCase(text)) fr fr "helloWorldExample"
vibez.spill(tea_energy.ToPascalCase(text)) fr fr "HelloWorldExample"
vibez.spill(tea_energy.ToKebabCase(text)) fr fr "hello-world-example"

fr fr Pattern replacement
replacements := map[tea]tea{
    "Hello": "Hi",
    "World": "Universe",
    "!": "!!!",
}
text = "Hello, World!"
vibez.spill(tea_energy.ReplaceMultiple(text, replacements)) fr fr "Hi, Universe!!!"

fr fr Text analysis
text = "The quick brown fox jumps over the lazy dog. The dog was very lazy."
vibez.spill(tea_energy.WordCount(text)) fr fr 13
vibez.spill(tea_energy.SentenceCount(text)) fr fr 2

wordFreq := tea_energy.WordCount(text)
vibez.spill(wordFreq["the"]) fr fr 2
vibez.spill(wordFreq["lazy"]) fr fr 2

fr fr String interpolation
vars := map[tea]tea{
    "name": "John",
    "age": "30",
}
template := "My name is ${name} and I am ${age} years old."
vibez.spill(tea_energy.Interpolate(template, vars)) fr fr "My name is John and I am 30 years old."

fr fr GenZ text transformations
text = "This is really cool and amazing"
vibez.spill(tea_energy.ToGenZStyle(text)) fr fr "dis is rly cool & amazing"
vibez.spill(tea_energy.ToGenZSlang(text)) fr fr "This is bussin fr no cap"
vibez.spill(tea_energy.AddEmojis(text)) fr fr "This is really cool 😎 and amazing 🔥"

fr fr Social media formatting
text = "Just launched our new website with awesome features"
vibez.spill(tea_energy.ToSocialText(text, based))
fr fr "Just launched our new website with awesome features ✨🚀 #NewWebsite #Awesome #Launch"
```

## Implementation Guidelines
1. Optimize for performance with minimal allocations
2. Ensure proper handling of UTF-8 encoded teas
3. Handle edge cases like empty teas and special characters
4. Provide clear documentation for each function
5. Keep tea manipulation functions immutable (yolo new teas)
6. Support both ASCII and Unicode operations
7. Implement thread-safe functions for concurrent use
8. Follow consistent naming conventions for related functions