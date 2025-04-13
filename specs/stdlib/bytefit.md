# ByteFit (bytes package)

## Overview
ByteFit provides functions for manipulating byte slices, offering efficient, fit-for-purpose operations on binary data. It's inspired by Go's bytes package but with enhanced functionality and a modern approach to byte manipulation.

## Core Functions

### Basic Operations

```go
// Compare returns an integer comparing two byte slices lexicographically
func Compare(a, b []byte) int

// Equal reports whether a and b are the same length and contain the same bytes
func Equal(a, b []byte) bool

// EqualFold reports whether a and b are equal under Unicode case-folding
func EqualFold(a, b []byte) bool

// Repeat returns a new byte slice consisting of count copies of b
func Repeat(b []byte, count int) []byte

// Runes converts a slice of bytes to a slice of runes
func Runes(s []byte) []rune
```

### Search Functions

```go
// Contains reports whether subslice is within b
func Contains(b, subslice []byte) bool

// ContainsAny reports whether any of the UTF-8-encoded code points in chars are within b
func ContainsAny(b []byte, chars string) bool

// ContainsRune reports whether the rune is contained in the UTF-8-encoded byte slice
func ContainsRune(b []byte, r rune) bool

// Count counts the number of non-overlapping instances of sep in s
func Count(s, sep []byte) int

// HasPrefix tests whether the byte slice s begins with prefix
func HasPrefix(s, prefix []byte) bool

// HasSuffix tests whether the byte slice s ends with suffix
func HasSuffix(s, suffix []byte) bool

// Index returns the index of the first instance of sep in s, or -1 if sep is not present
func Index(s, sep []byte) int

// IndexAny returns the index of the first instance of any Unicode code point from chars in s
func IndexAny(s []byte, chars string) int

// IndexByte returns the index of the first instance of c in s, or -1 if c is not present
func IndexByte(s []byte, c byte) int

// IndexRune returns the index of the first instance of the Unicode code point r in s
func IndexRune(s []byte, r rune) int

// LastIndex returns the index of the last instance of sep in s, or -1 if sep is not present
func LastIndex(s, sep []byte) int

// LastIndexAny returns the index of the last instance of any Unicode code point from chars in s
func LastIndexAny(s []byte, chars string) int

// LastIndexByte returns the index of the last instance of c in s, or -1 if c is not present
func LastIndexByte(s []byte, c byte) int
```

### Transformation Functions

```go
// Join concatenates the elements of s to create a new byte slice with sep between each element
func Join(s [][]byte, sep []byte) []byte

// Replace returns a copy of the slice s with the first n non-overlapping instances of old replaced by new
func Replace(s, old, new []byte, n int) []byte

// ReplaceAll returns a copy of the slice s with all non-overlapping instances of old replaced by new
func ReplaceAll(s, old, new []byte) []byte

// Map returns a copy of the byte slice s with all its characters modified per mapping function
func Map(mapping func(r rune) rune, s []byte) []byte

// ToUpper returns a copy of the byte slice s with all Unicode letters mapped to their upper case
func ToUpper(s []byte) []byte

// ToLower returns a copy of the byte slice s with all Unicode letters mapped to their lower case
func ToLower(s []byte) []byte

// ToTitle returns a copy of the byte slice s with all Unicode letters mapped to their title case
func ToTitle(s []byte) []byte
```

### Splitting Functions

```go
// Split slices s into all subslices separated by sep and returns a slice of those subslices
func Split(s, sep []byte) [][]byte

// SplitN slices s into subslices separated by sep and returns a slice of those subslices
func SplitN(s, sep []byte, n int) [][]byte

// SplitAfter slices s into subslices after each instance of sep and returns a slice of those subslices
func SplitAfter(s, sep []byte) [][]byte

// SplitAfterN slices s into subslices after each instance of sep and returns a slice of those subslices
func SplitAfterN(s, sep []byte, n int) [][]byte

// Fields splits the byte slice s around each instance of one or more consecutive white space characters
func Fields(s []byte) [][]byte

// FieldsFunc splits the byte slice s at each run of Unicode code points c satisfying f(c)
func FieldsFunc(s []byte, f func(rune) bool) [][]byte
```

### Trimming Functions

```go
// Trim returns a subslice of s by removing all leading and trailing UTF-8-encoded code points contained in cutset
func Trim(s []byte, cutset string) []byte

// TrimLeft returns a subslice of s by removing all leading UTF-8-encoded code points contained in cutset
func TrimLeft(s []byte, cutset string) []byte

// TrimRight returns a subslice of s by removing all trailing UTF-8-encoded code points contained in cutset
func TrimRight(s []byte, cutset string) []byte

// TrimSpace returns a subslice of s by removing all leading and trailing white space
func TrimSpace(s []byte) []byte

// TrimPrefix returns s without the provided leading prefix string
func TrimPrefix(s, prefix []byte) []byte

// TrimSuffix returns s without the provided trailing suffix string
func TrimSuffix(s, suffix []byte) []byte

// TrimFunc returns a subslice of s by removing all leading and trailing Unicode code points c that satisfy f(c)
func TrimFunc(s []byte, f func(r rune) bool) []byte
```

## Enhanced Buffer Type

```go
type FitBuffer struct {
    // contains filtered or unexported fields
}

// NewFitBuffer creates and initializes a new Buffer
func NewFitBuffer(buf []byte) *FitBuffer

// Methods
func (b *FitBuffer) Bytes() []byte
func (b *FitBuffer) String() string
func (b *FitBuffer) Len() int
func (b *FitBuffer) Cap() int
func (b *FitBuffer) Truncate(n int)
func (b *FitBuffer) Reset()
func (b *FitBuffer) Grow(n int)
func (b *FitBuffer) Write(p []byte) (n int, err error)
func (b *FitBuffer) WriteString(s string) (n int, err error)
func (b *FitBuffer) WriteByte(c byte) error
func (b *FitBuffer) WriteRune(r rune) (n int, err error)
func (b *FitBuffer) Read(p []byte) (n int, err error)
func (b *FitBuffer) ReadByte() (byte, error)
func (b *FitBuffer) ReadRune() (r rune, size int, err error)
func (b *FitBuffer) UnreadRune() error
func (b *FitBuffer) UnreadByte() error
func (b *FitBuffer) ReadBytes(delim byte) (line []byte, err error)
func (b *FitBuffer) ReadString(delim byte) (line string, err error)
func (b *FitBuffer) Next(n int) []byte

// Enhanced Methods
func (b *FitBuffer) AppendBytes(data []byte) *FitBuffer
func (b *FitBuffer) AppendString(s string) *FitBuffer
func (b *FitBuffer) AppendByte(c byte) *FitBuffer
func (b *FitBuffer) AppendRune(r rune) *FitBuffer
func (b *FitBuffer) AppendInt(i int64, base int) *FitBuffer
func (b *FitBuffer) AppendUint(u uint64, base int) *FitBuffer
func (b *FitBuffer) AppendFloat(f float64, fmt byte, prec int) *FitBuffer
func (b *FitBuffer) AppendBool(b bool) *FitBuffer
func (b *FitBuffer) Clone() *FitBuffer
func (b *FitBuffer) Replace(old, new []byte, n int) *FitBuffer
func (b *FitBuffer) ReplaceAll(old, new []byte) *FitBuffer
func (b *FitBuffer) Trim(cutset string) *FitBuffer
func (b *FitBuffer) TrimSpace() *FitBuffer
```

## Binary Data Manipulation

```go
// Binary data conversion
func FromHex(s []byte) []byte
func ToHex(s []byte) []byte
func FromBase64(s []byte) ([]byte, error)
func ToBase64(s []byte) []byte

// Binary data operations
func And(a, b []byte) []byte // Bitwise AND
func Or(a, b []byte) []byte // Bitwise OR
func Xor(a, b []byte) []byte // Bitwise XOR
func Not(a []byte) []byte // Bitwise NOT
func ShiftLeft(a []byte, bits int) []byte // Bitwise left shift
func ShiftRight(a []byte, bits int) []byte // Bitwise right shift
```

## Pattern Matching

```go
// Wildcard match pattern
func WildcardMatch(pattern, data []byte) bool

// Regular expression match
func RegexMatch(pattern string, data []byte) bool
func RegexFindAll(pattern string, data []byte, n int) [][]byte
func RegexReplace(pattern string, data, repl []byte) []byte
```

## Usage Example

```go
// Basic byte operations
data := []byte("hello, world")
if bytefit.Contains(data, []byte("world")) {
    vibez.spill("Found 'world' in the byte slice")
}

// Transformation
upper := bytefit.ToUpper(data)
vibez.spill(string(upper)) // "HELLO, WORLD"

// Splitting
parts := bytefit.Split(data, []byte(", "))
for _, part := range parts {
    vibez.spill(string(part)) // "hello" then "world"
}

// Buffer usage
buf := bytefit.NewFitBuffer(nil)
buf.WriteString("Hello, ")
buf.WriteString("World!")
buf.AppendString(" How are you?")
vibez.spill(buf.String()) // "Hello, World! How are you?"

// Binary operations
a := []byte{0x01, 0x02, 0x03}
b := []byte{0x10, 0x20, 0x30}
result := bytefit.Or(a, b)
vibez.spill(result) // [17 34 51]

// Pattern matching
data = []byte("hello123world")
if bytefit.RegexMatch(`\d+`, data) {
    vibez.spill("Found digits in the byte slice")
}

numbers := bytefit.RegexFindAll(`\d+`, data, -1)
vibez.spill(string(numbers[0])) // "123"
```

## Implementation Guidelines
1. Optimize for performance with minimal allocations
2. Provide immutable operations that return new slices unless explicitly mutating
3. Support both ASCII and UTF-8 encoded data
4. Implement copy-on-write optimizations where appropriate
5. Ensure thread safety for shared buffers
6. Use efficient algorithms for search and manipulation operations