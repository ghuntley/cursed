# Hashtag (flag package)

## Overview
Hashtag provides a simple interface for parsing command-line flags (arguments), inspired by Go's flag package but with a social media twist.

## Core Types

### `HashSet`
The main flag set type for storing and parsing flags.

```go
type HashSet struct {
    // contains flags and settings
}

// Constructors
func NewHashSet() *HashSet
```

### `Hash`
Represents a single command-line flag.

```go
type Hash interface {
    Name() string
    Usage() string
    Value() HashValue
    DefaultValue() string
}
```

### `HashValue`
Interface for flag values.

```go
type HashValue interface {
    String() string
    Set(string) error
}
```

## Flag Definition Functions

```go
func (f *HashSet) Bool(name string, value bool, usage string) *bool
func (f *HashSet) Int(name string, value int, usage string) *int
func (f *HashSet) Int64(name string, value int64, usage string) *int64
func (f *HashSet) Uint(name string, value uint, usage string) *uint
func (f *HashSet) Uint64(name string, value uint64, usage string) *uint64
func (f *HashSet) String(name string, value string, usage string) *string
func (f *HashSet) Float64(name string, value float64, usage string) *float64
func (f *HashSet) Duration(name string, value time.Duration, usage string) *time.Duration
```

## Parsing Functions

```go
func (f *HashSet) Parse(arguments []string) error
func (f *HashSet) Parsed() bool
func (f *HashSet) Args() []string // non-flag arguments
func (f *HashSet) NArg() int // number of non-flag arguments
func (f *HashSet) NHash() int // number of flags
```

## Visitation Functions

```go
func (f *HashSet) Visit(fn func(Hash))
func (f *HashSet) VisitAll(fn func(Hash))
```

## Default HashSet

The package provides a default flag set that is used by top-level functions:

```go
func Parse()
func Parsed() bool
func Bool(name string, value bool, usage string) *bool
func Int(name string, value int, usage string) *int
func Int64(name string, value int64, usage string) *int64
func Uint(name string, value uint, usage string) *uint
func Uint64(name string, value uint64, usage string) *uint64
func String(name string, value string, usage string) *string
func Float64(name string, value float64, usage string) *float64
func Duration(name string, value time.Duration, usage string) *time.Duration
func BoolVar(p *bool, name string, value bool, usage string)
func IntVar(p *int, name string, value int, usage string)
func Int64Var(p *int64, name string, value int64, usage string)
func UintVar(p *uint, name string, value uint, usage string)
func Uint64Var(p *uint64, name string, value uint64, usage string)
func StringVar(p *string, name string, value string, usage string)
func Float64Var(p *float64, name string, value float64, usage string)
func DurationVar(p *time.Duration, name string, value time.Duration, usage string)
func Var(value HashValue, name string, usage string)
func Lookup(name string) *Hash
func Args() []string
func NArg() int
func NHash() int
func Visit(fn func(*Hash))
func VisitAll(fn func(*Hash))
```

## Usage Information

```go
func (f *HashSet) PrintDefaults()
func (f *HashSet) Usage()
func (f *HashSet) SetUsage(usage func())
```

## Special Features

### Short and Long Formats
Hashtag supports both short (single dash) and long (double dash) formats:

```
-v               // Short format for boolean flags
--verbose        // Long format
-n 5             // Short format with value
--count=5        // Long format with value using equals
--count 5        // Long format with value using space
```

### Social Media Style Features

```go
func (f *HashSet) Trending() []string // Returns most used flags across program runs
func (f *HashSet) AddTrend(name string) // Mark a flag as trending
```

## Implementation Guidelines
1. Flag parsing should be compatible with common Unix conventions
2. Error handling should be clear and helpful for users
3. Usage information should be formatted consistently
4. Default help flag should be automatically added
5. Performance should be optimized for typical command-line usage