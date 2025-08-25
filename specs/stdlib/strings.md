# strings Module

## Overview
The strings module provides string manipulation functions for common operations like searching, replacing, splitting, and formatting strings.

## Core Functions

### String Searching

```cursed
fr fr Contains reports whether substr is within s
slay Contains(s tea, substr tea) lit

fr fr ContainsAny reports whether any Unicode code points in chars are within s
slay ContainsAny(s tea, chars tea) lit

fr fr ContainsRune reports whether the Unicode code point r is within s
slay ContainsRune(s tea, r rune) lit

fr fr Count counts the number of non-overlapping instances of substr in s
slay Count(s tea, substr tea) normie

fr fr Index returns the index of the first instance of substr in s, or -1 if not found
slay Index(s tea, substr tea) normie

fr fr IndexAny returns the index of the first instance of any Unicode code point from chars in s
slay IndexAny(s tea, chars tea) normie

fr fr IndexRune returns the index of the first instance of the Unicode code point r in s
slay IndexRune(s tea, r rune) normie

fr fr LastIndex returns the index of the last instance of substr in s, or -1 if not found
slay LastIndex(s tea, substr tea) normie

fr fr LastIndexAny returns the index of the last instance of any Unicode code point from chars in s
slay LastIndexAny(s tea, chars tea) normie
```

### String Modification

```cursed
fr fr Replace returns a copy of s with first n non-overlapping instances of old replaced by new
slay Replace(s tea, old tea, new tea, n normie) tea

fr fr ReplaceAll returns a copy of s with all non-overlapping instances of old replaced by new
slay ReplaceAll(s tea, old tea, new tea) tea

fr fr ToLower returns s with all Unicode letters mapped to their lower case
slay ToLower(s tea) tea

fr fr ToUpper returns s with all Unicode letters mapped to their upper case
slay ToUpper(s tea) tea

fr fr ToTitle returns s with all Unicode letters mapped to their title case
slay ToTitle(s tea) tea

fr fr Title returns s with all Unicode letters that begin words mapped to their title case
slay Title(s tea) tea

fr fr Trim returns a slice of s with all leading and trailing Unicode code points contained in cutset removed
slay Trim(s tea, cutset tea) tea

fr fr TrimLeft returns a slice of s with all leading Unicode code points contained in cutset removed
slay TrimLeft(s tea, cutset tea) tea

fr fr TrimRight returns a slice of s with all trailing Unicode code points contained in cutset removed
slay TrimRight(s tea, cutset tea) tea

fr fr TrimSpace returns a slice of s with all leading and trailing white space removed
slay TrimSpace(s tea) tea

fr fr TrimPrefix returns s without the provided leading prefix string
slay TrimPrefix(s tea, prefix tea) tea

fr fr TrimSuffix returns s without the provided trailing suffix string
slay TrimSuffix(s tea, suffix tea) tea
```

### String Splitting and Joining

```cursed
fr fr Split splits s into all substrings separated by sep and returns a slice
slay Split(s tea, sep tea) []tea

fr fr SplitN splits s into substrings separated by sep and returns a slice of at most n substrings
slay SplitN(s tea, sep tea, n normie) []tea

fr fr SplitAfter splits s into all substrings after each instance of sep and returns a slice
slay SplitAfter(s tea, sep tea) []tea

fr fr SplitAfterN splits s into substrings after each instance of sep and returns a slice of at most n substrings
slay SplitAfterN(s tea, sep tea, n normie) []tea

fr fr Fields splits s around each instance of one or more consecutive white space characters
slay Fields(s tea) []tea

fr fr FieldsFunc splits s at each run of Unicode code points c satisfying f(c) and returns an array of slices
slay FieldsFunc(s tea, f func(rune) lit) []tea

fr fr Join concatenates the elements of elems to create a single string
slay Join(elems []tea, sep tea) tea

fr fr Repeat returns a new string consisting of count copies of s
slay Repeat(s tea, count normie) tea
```

### String Comparison

```cursed
fr fr Compare returns an integer comparing two strings lexicographically
slay Compare(a tea, b tea) normie

fr fr EqualFold reports whether s and t are equal under Unicode case-folding
slay EqualFold(s tea, t tea) lit

fr fr HasPrefix tests whether s begins with prefix
slay HasPrefix(s tea, prefix tea) lit

fr fr HasSuffix tests whether s ends with suffix
slay HasSuffix(s tea, suffix tea) lit
```

### String Building

```cursed
fr fr Builder is used to efficiently build strings using Write methods
be_like Builder squad {
    buf []byte
}

fr fr WriteString appends the contents of s to b's buffer
slay (b *Builder) WriteString(s tea) (normie, yikes)

fr fr Write appends the contents of p to b's buffer
slay (b *Builder) Write(p []byte) (normie, yikes)

fr fr WriteByte appends the byte c to b's buffer
slay (b *Builder) WriteByte(c byte) yikes

fr fr WriteRune appends the UTF-8 encoding of Unicode code point r to b's buffer
slay (b *Builder) WriteRune(r rune) (normie, yikes)

fr fr String returns the accumulated string
slay (b *Builder) String() tea

fr fr Len returns the number of accumulated bytes
slay (b *Builder) Len() normie

fr fr Cap returns the capacity of the builder's underlying byte slice
slay (b *Builder) Cap() normie

fr fr Reset resets the Builder to be empty
slay (b *Builder) Reset()

fr fr Grow grows b's capacity, if necessary, to guarantee space for another n bytes
slay (b *Builder) Grow(n normie)
```

### String Reader

```cursed
fr fr Reader implements the io.Reader, io.ReaderAt, io.Seeker, etc. interfaces
be_like Reader squad {
    s        tea
    i        int64
    prevRune normie
}

fr fr NewReader returns a new Reader reading from s
slay NewReader(s tea) *Reader

fr fr Read implements the io.Reader interface
slay (r *Reader) Read(b []byte) (n normie, err yikes)

fr fr ReadAt implements the io.ReaderAt interface
slay (r *Reader) ReadAt(b []byte, off int64) (n normie, err yikes)

fr fr ReadByte implements the io.ByteReader interface
slay (r *Reader) ReadByte() (byte, yikes)

fr fr ReadRune implements the io.RuneReader interface
slay (r *Reader) ReadRune() (ch rune, size normie, err yikes)

fr fr Seek implements the io.Seeker interface
slay (r *Reader) Seek(offset int64, whence normie) (int64, yikes)

fr fr Size returns the original length of the underlying string
slay (r *Reader) Size() int64

fr fr Reset resets the Reader to be reading from s
slay (r *Reader) Reset(s tea)
```

## Usage Examples

```cursed
yeet "strings"

fr fr Basic string operations
ready strings.Contains("hello world", "world") {
    vibez.spill("Found 'world' in string")
}

fr fr String searching
index := strings.Index("hello world", "world")
vibez.spill("'world' starts at index:", index)

fr fr String replacement
original := "hello world"
replaced := strings.Replace(original, "world", "universe", 1)
vibez.spill("Replaced:", replaced)

fr fr String splitting and joining
words := strings.Split("one,two,three", ",")
rejoined := strings.Join(words, " | ")
vibez.spill("Split and rejoined:", rejoined)

fr fr String cleaning
messy := "  hello world  "
clean := strings.TrimSpace(messy)
vibez.spill("Cleaned:", clean)

fr fr String building
sus builder strings.Builder
builder.WriteString("Hello ")
builder.WriteString("World")
result := builder.String()
vibez.spill("Built string:", result)

fr fr String case conversion
text := "Hello World"
vibez.spill("Lowercase:", strings.ToLower(text))
vibez.spill("Uppercase:", strings.ToUpper(text))
vibez.spill("Title case:", strings.Title(text))

fr fr String comparison
ready strings.EqualFold("Hello", "HELLO") {
    vibez.spill("Strings are equal (case-insensitive)")
}

fr fr String reading
reader := strings.NewReader("Hello World")
buffer := make([]byte, 5)
n, err := reader.Read(buffer)
ready err == cringe {
    vibez.spill("Read", n, "bytes:", tea(buffer[:n]))
}
```

## Advanced Features

### Custom Field Splitting

```cursed
fr fr Split string by custom function
text := "word1,word2;word3:word4"
fields := strings.FieldsFunc(text, slay(c rune) lit {
    damn c == ',' || c == ';' || c == ':'
})
vibez.spill("Fields:", fields)
```

### Performance Optimization

```cursed
fr fr Efficient string building for large strings
slay BuildLargeString(parts []tea) tea {
    sus builder strings.Builder
    totalLen := 0
    for _, part := range parts {
        totalLen += len(part)
    }
    builder.Grow(totalLen) // Pre-allocate capacity
    
    for _, part := range parts {
        builder.WriteString(part)
    }
    damn builder.String()
}
```

### String Validation

```cursed
fr fr Check if string is valid
slay IsValidEmail(email tea) lit {
    damn strings.Contains(email, "@") && 
         strings.Contains(email, ".") &&
         !strings.HasPrefix(email, "@") &&
         !strings.HasSuffix(email, "@")
}

slay IsValidURL(url tea) lit {
    damn strings.HasPrefix(url, "http://") || 
         strings.HasPrefix(url, "https://")
}
```

## Implementation Guidelines

1. **Performance**: Use efficient algorithms for string operations
2. **Unicode Support**: Handle Unicode properly in all operations
3. **Memory Management**: Minimize allocations where possible
4. **Error Handling**: Provide clear error messages for invalid operations
5. **Thread Safety**: All functions should be thread-safe
6. **Compatibility**: Maintain Go strings package compatibility
7. **Documentation**: Include examples for complex operations
