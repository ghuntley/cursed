# Hashtag (flag package)

## Overview
Hashtag provides a simple collab for parsing command-line flags (arguments), inspired by Go's flag package but with a social media twist.

## Core Types

### `HashSet`
The main flag set be_like for storing and parsing flags.

```
be_like HashSet squad {
    fr fr contains flags and settings
}

fr fr Consquadors
slay NewHashSet() *HashSet
```

### `Hash`
Represents a single command-line flag.

```
be_like Hash collab {
    Name() tea
    Usage() tea
    Value() HashValue
    DefaultValue() tea
}
```

### `HashValue`
Interface for flag values.

```
be_like HashValue collab {
    String() tea
    Set(tea) tea
}
```

## Flag Definition Functions

```
slay (f *HashSet) Bool(name tea, value lit, usage tea) *lit
slay (f *HashSet) Int(name tea, value int, usage tea) *int
slay (f *HashSet) Int64(name tea, value int64, usage tea) *int64
slay (f *HashSet) Uint(name tea, value uint, usage tea) *uint
slay (f *HashSet) Uint64(name tea, value uint64, usage tea) *uint64
slay (f *HashSet) String(name tea, value tea, usage tea) *tea
slay (f *HashSet) Float64(name tea, value float64, usage tea) *float64
slay (f *HashSet) Duration(name tea, value time.Duration, usage tea) *time.Duration
```

## Parsing Functions

```
slay (f *HashSet) Parse(arguments []tea) tea
slay (f *HashSet) Parsed() lit
slay (f *HashSet) Args() []tea fr fr non-flag arguments
slay (f *HashSet) NArg() normie fr fr number of non-flag arguments
slay (f *HashSet) NHash() normie fr fr number of flags
```

## Visitation Functions

```
slay (f *HashSet) Visit(fn func(Hash))
slay (f *HashSet) VisitAll(fn func(Hash))
```

## Default HashSet

The package provides a default flag set that is used by top-level functions:

```
slay Parse()
slay Parsed() lit
slay Bool(name tea, value lit, usage tea) *lit
slay Int(name tea, value int, usage tea) *int
slay Int64(name tea, value int64, usage tea) *int64
slay Uint(name tea, value uint, usage tea) *uint
slay Uint64(name tea, value uint64, usage tea) *uint64
slay String(name tea, value tea, usage tea) *tea
slay Float64(name tea, value float64, usage tea) *float64
slay Duration(name tea, value time.Duration, usage tea) *time.Duration
slay BoolVar(p *lit, name tea, value lit, usage tea)
slay IntVar(p *int, name tea, value int, usage tea)
slay Int64Var(p *int64, name tea, value int64, usage tea)
slay UintVar(p *uint, name tea, value uint, usage tea)
slay Uint64Var(p *uint64, name tea, value uint64, usage tea)
slay StringVar(p *tea, name tea, value tea, usage tea)
slay Float64Var(p *float64, name tea, value float64, usage tea)
slay DurationVar(p *time.Duration, name tea, value time.Duration, usage tea)
slay Var(value HashValue, name tea, usage tea)
slay Lookup(name tea) *Hash
slay Args() []tea
slay NArg() int
slay NHash() int
slay Visit(fn func(*Hash))
slay VisitAll(fn func(*Hash))
```

## Usage Information

```
slay (f *HashSet) PrintDefaults()
slay (f *HashSet) Usage()
slay (f *HashSet) SetUsage(usage func())
```

## Special Features

### Short and Long Formats
Hashtag supports both short (single dash) and long (double dash) formats:

```
-v               fr fr Short format for litean flags
--verbose        fr fr Long format
-n 5             fr fr Short format with value
--count=5        fr fr Long format with value using equals
--count 5        fr fr Long format with value using space
```

### Social Media Style Features

```
slay (f *HashSet) Trending() []tea fr fr Returns most used flags across program runs
slay (f *HashSet) AddTrend(name tea) fr fr Mark a flag as trending
```

## Implementation Guidelines
1. Flag parsing should be compatible with common Unix conventions
2. Error handling should be clear and helpful for users
3. Usage information should be formatted consistently
4. Default help flag should be automatically added
5. Performance should be optimized for typical command-line usage