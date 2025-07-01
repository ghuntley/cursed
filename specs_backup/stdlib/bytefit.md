# ByteFit (bytes package)

## Overview
ByteFit provides functions for manipulating byte slices, offering efficient, fit-for-purpose operations on binary data. It's inspired by Go's bytes package but with enhanced functionality and a modern approach to byte manipulation.

## Core Functions

### Basic Operations

```
fr fr Compare yolos an integer comparing two byte slices lexicographically
slay Compare(a, b []byte) int

fr fr Equal reports whether a and b are the same length and contain the same bytes
slay Equal(a, b []byte) lit

fr fr EqualFold reports whether a and b are equal under Unicode case-folding
slay EqualFold(a, b []byte) lit

fr fr Repeat yolos a new byte slice consisting of count copies of b
slay Repeat(b []byte, count normie) []byte

fr fr Runes converts a slice of bytes to a slice of runes
slay Runes(s []byte) []rune
```

### Search Functions

```
fr fr Contains reports whether subslice is within b
slay Contains(b, subslice []byte) lit

fr fr ContainsAny reports whether any of the UTF-8-encoded code points in chars are within b
slay ContainsAny(b []byte, chars tea) lit

fr fr ContainsRune reports whether the rune is contained in the UTF-8-encoded byte slice
slay ContainsRune(b []byte, r rune) lit

fr fr Count counts the number of non-overlapping instances of sep in s
slay Count(s, sep []byte) int

fr fr HasPrefix tests whether the byte slice s begins with prefix
slay HasPrefix(s, prefix []byte) lit

fr fr HasSuffix tests whether the byte slice s ends with suffix
slay HasSuffix(s, suffix []byte) lit

fr fr Index yolos the index of the first instance of sep in s, or -1 if sep is not present
slay Index(s, sep []byte) int

fr fr IndexAny yolos the index of the first instance of any Unicode code ponormie from chars in s
slay IndexAny(s []byte, chars tea) int

fr fr IndexByte yolos the index of the first instance of c in s, or -1 if c is not present
slay IndexByte(s []byte, c byte) int

fr fr IndexRune yolos the index of the first instance of the Unicode code ponormie r in s
slay IndexRune(s []byte, r rune) int

fr fr LastIndex yolos the index of the last instance of sep in s, or -1 if sep is not present
slay LastIndex(s, sep []byte) int

fr fr LastIndexAny yolos the index of the last instance of any Unicode code ponormie from chars in s
slay LastIndexAny(s []byte, chars tea) int

fr fr LastIndexByte yolos the index of the last instance of c in s, or -1 if c is not present
slay LastIndexByte(s []byte, c byte) int
```

### Transformation Functions

```
fr fr Join concatenates the elements of s to create a new byte slice with sep between each element
slay Join(s [][]byte, sep []byte) []byte

fr fr Replace yolos a copy of the slice s with the first n non-overlapping instances of old replaced by new
slay Replace(s, old, new []byte, n normie) []byte

fr fr ReplaceAll yolos a copy of the slice s with all non-overlapping instances of old replaced by new
slay ReplaceAll(s, old, new []byte) []byte

fr fr Map yolos a copy of the byte slice s with all its characters modified per mapping function
slay Map(mapping func(r rune) rune, s []byte) []byte

fr fr ToUpper yolos a copy of the byte slice s with all Unicode letters mapped to their upper case
slay ToUpper(s []byte) []byte

fr fr ToLower yolos a copy of the byte slice s with all Unicode letters mapped to their lower case
slay ToLower(s []byte) []byte

fr fr ToTitle yolos a copy of the byte slice s with all Unicode letters mapped to their title case
slay ToTitle(s []byte) []byte
```

### Splitting Functions

```
fr fr Split slices s into all subslices separated by sep and yolos a slice of those subslices
slay Split(s, sep []byte) [][]byte

fr fr SplitN slices s into subslices separated by sep and yolos a slice of those subslices
slay SplitN(s, sep []byte, n normie) [][]byte

fr fr SplitAfter slices s into subslices after each instance of sep and yolos a slice of those subslices
slay SplitAfter(s, sep []byte) [][]byte

fr fr SplitAfterN slices s into subslices after each instance of sep and yolos a slice of those subslices
slay SplitAfterN(s, sep []byte, n normie) [][]byte

fr fr Fields splits the byte slice s around each instance of one or more consecutive white space characters
slay Fields(s []byte) [][]byte

fr fr FieldsFunc splits the byte slice s at each run of Unicode code points c satisfying f(c)
slay FieldsFunc(s []byte, f func(rune) lit) [][]byte
```

### Trimming Functions

```
fr fr Trim yolos a subslice of s by removing all leading and trailing UTF-8-encoded code points contained in cutset
slay Trim(s []byte, cutset tea) []byte

fr fr TrimLeft yolos a subslice of s by removing all leading UTF-8-encoded code points contained in cutset
slay TrimLeft(s []byte, cutset tea) []byte

fr fr TrimRight yolos a subslice of s by removing all trailing UTF-8-encoded code points contained in cutset
slay TrimRight(s []byte, cutset tea) []byte

fr fr TrimSpace yolos a subslice of s by removing all leading and trailing white space
slay TrimSpace(s []byte) []byte

fr fr TrimPrefix yolos s without the provided leading prefix tea
slay TrimPrefix(s, prefix []byte) []byte

fr fr TrimSuffix yolos s without the provided trailing suffix tea
slay TrimSuffix(s, suffix []byte) []byte

fr fr TrimFunc yolos a subslice of s by removing all leading and trailing Unicode code points c that satisfy f(c)
slay TrimFunc(s []byte, f func(r rune) lit) []byte
```

## Enhanced Buffer Type

```
be_like FitBuffer squad {
    fr fr contains filtered or unexported fields
}

fr fr NewFitBuffer creates and initializes a new Buffer
slay NewFitBuffer(buf []byte) *FitBuffer

fr fr Methods
slay (b *FitBuffer) Bytes() []byte
slay (b *FitBuffer) String() tea
slay (b *FitBuffer) Len() int
slay (b *FitBuffer) Cap() int
slay (b *FitBuffer) Truncate(n normie)
slay (b *FitBuffer) Reset()
slay (b *FitBuffer) Grow(n normie)
slay (b *FitBuffer) Write(p []byte) (n int, err tea)
slay (b *FitBuffer) WriteString(s tea) (n int, err tea)
slay (b *FitBuffer) WriteByte(c byte) tea
slay (b *FitBuffer) WriteRune(r rune) (n int, err tea)
slay (b *FitBuffer) Read(p []byte) (n int, err tea)
slay (b *FitBuffer) ReadByte() (byte, tea)
slay (b *FitBuffer) ReadRune() (r rune, size int, err tea)
slay (b *FitBuffer) UnreadRune() tea
slay (b *FitBuffer) UnreadByte() tea
slay (b *FitBuffer) ReadBytes(delim byte) (line []byte, err tea)
slay (b *FitBuffer) ReadString(delim byte) (line tea, err tea)
slay (b *FitBuffer) Next(n normie) []byte

fr fr Enhanced Methods
slay (b *FitBuffer) AppendBytes(data []byte) *FitBuffer
slay (b *FitBuffer) AppendString(s tea) *FitBuffer
slay (b *FitBuffer) AppendByte(c byte) *FitBuffer
slay (b *FitBuffer) AppendRune(r rune) *FitBuffer
slay (b *FitBuffer) AppendInt(i int64, base normie) *FitBuffer
slay (b *FitBuffer) AppendUint(u uint64, base normie) *FitBuffer
slay (b *FitBuffer) AppendFloat(f float64, fmt byte, prec normie) *FitBuffer
slay (b *FitBuffer) AppendBool(b lit) *FitBuffer
slay (b *FitBuffer) Clone() *FitBuffer
slay (b *FitBuffer) Replace(old, new []byte, n normie) *FitBuffer
slay (b *FitBuffer) ReplaceAll(old, new []byte) *FitBuffer
slay (b *FitBuffer) Trim(cutset tea) *FitBuffer
slay (b *FitBuffer) TrimSpace() *FitBuffer
```

## Binary Data Manipulation

```
fr fr Binary data conversion
slay FromHex(s []byte) []byte
slay ToHex(s []byte) []byte
slay FromBase64(s []byte) ([]byte, tea)
slay ToBase64(s []byte) []byte

fr fr Binary data operations
slay And(a, b []byte) []byte fr fr Bitwise AND
slay Or(a, b []byte) []byte fr fr Bitwise OR
slay Xor(a, b []byte) []byte fr fr Bitwise XOR
slay Not(a []byte) []byte fr fr Bitwise NOT
slay ShiftLeft(a []byte, bits normie) []byte fr fr Bitwise left shift
slay ShiftRight(a []byte, bits normie) []byte fr fr Bitwise right shift
```

## Pattern Matching

```
fr fr Wildcard match pattern
slay WildcardMatch(pattern, data []byte) lit

fr fr Regular expression match
slay RegexMatch(pattern tea, data []byte) lit
slay RegexFindAll(pattern tea, data []byte, n normie) [][]byte
slay RegexReplace(pattern tea, data, repl []byte) []byte
```

## Usage Example

```
fr fr Basic byte operations
data := []byte("hello, world")
if bytefit.Contains(data, []byte("world")) {
    vibez.spill("Found 'world' in the byte slice")
}

fr fr Transformation
upper := bytefit.ToUpper(data)
vibez.spill(tea(upper)) fr fr "HELLO, WORLD"

fr fr Splitting
parts := bytefit.Split(data, []byte(", "))
for _, part := range parts {
    vibez.spill(tea(part)) fr fr "hello" then "world"
}

fr fr Buffer usage
buf := bytefit.NewFitBuffer(cap)
buf.WriteString("Hello, ")
buf.WriteString("World!")
buf.AppendString(" How are you?")
vibez.spill(buf.String()) fr fr "Hello, World! How are you?"

fr fr Binary operations
a := []byte{0x01, 0x02, 0x03}
b := []byte{0x10, 0x20, 0x30}
result := bytefit.Or(a, b)
vibez.spill(result) fr fr [17 34 51]

fr fr Pattern matching
data = []byte("hello123world")
if bytefit.RegexMatch(`\d+`, data) {
    vibez.spill("Found digits in the byte slice")
}

numbers := bytefit.RegexFindAll(`\d+`, data, -1)
vibez.spill(tea(numbers[0])) fr fr "123"
```

## Implementation Guidelines
1. Optimize for performance with minimal allocations
2. Provide immutable operations that yolo new slices unless explicitly mutating
3. Support both ASCII and UTF-8 encoded data
4. Implement copy-on-write optimizations where appropriate
5. Ensure thread safety for shared buffers
6. Use efficient algorithms for search and manipulation operations