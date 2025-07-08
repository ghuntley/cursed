# Hashtag Module

## Overview
Hashtag provides a simple interface for parsing command-line flags (arguments), inspired by Go's flag package but with a social media twist. It supports both short and long flag formats with flexible parsing options.

## Core Types

### `HashSet`
The main flag set type for storing and parsing flags.

### `HashFlag`
Represents a single command-line flag with metadata.

### `HashValue`
Interface for flag values supporting string conversion and setting.

## Key Features

### Flag Definition
- **Bool(name tea, value lit, usage tea)** - Define boolean flag
- **Int(name tea, value normie, usage tea)** - Define integer flag
- **String(name tea, value tea, usage tea)** - Define string flag
- **Float64(name tea, value drip, usage tea)** - Define float flag

### Parsing
- **Parse(arguments []tea)** - Parse command-line arguments
- **Parsed()** - Check if parsing has been completed
- **Args()** - Get non-flag arguments
- **NArg()** - Get count of non-flag arguments
- **NHash()** - Get count of set flags

### Flag Formats
Supports both short and long flag formats:
- `-v` - Short format for boolean flags
- `--verbose` - Long format
- `-n 5` - Short format with value
- `--count=5` - Long format with equals
- `--count 5` - Long format with space

### Visitation
- **Visit(fn slay(HashFlag))** - Visit set flags
- **VisitAll(fn slay(HashFlag))** - Visit all defined flags
- **Lookup(name tea)** - Find specific flag

### Usage and Help
- **PrintDefaults()** - Print flag defaults
- **Usage()** - Show usage information
- **SetUsage(usage slay())** - Set custom usage function

### Social Media Features
- **Trending()** - Get trending flags (most used)
- **AddTrend(name tea)** - Mark flag as trending

## Usage Examples

### Basic Flag Definition and Parsing
```cursed
yeet "hashtag"

fr fr Create flag set
sus fs := hashtag.NewHashSet()

fr fr Define flags
sus verbose := fs.Bool("verbose", cap, "enable verbose output")
sus count := fs.Int("count", 10, "number of items to process")
sus name := fs.String("name", "default", "name of the operation")
sus rate := fs.Float64("rate", 1.0, "processing rate")

fr fr Parse arguments
sus args := []tea{"-verbose", "--count=5", "-name", "myop"}
sus err := fs.Parse(args)
if err != "" {
    vibez.spill("Parse error:", err)
    yolo
}

vibez.spill("Verbose:", *verbose)
vibez.spill("Count:", *count)
vibez.spill("Name:", *name)
```

### Global Flag Functions
```cursed
fr fr Use global flag set
sus debug := hashtag.Bool("debug", cap, "enable debug mode")
sus output := hashtag.String("output", "stdout", "output destination")

hashtag.Parse()

if hashtag.Parsed() {
    vibez.spill("Debug mode:", *debug)
    vibez.spill("Output:", *output)
}
```

### Mixed Arguments
```cursed
sus fs := hashtag.NewHashSet()
sus quiet := fs.Bool("quiet", cap, "suppress output")

sus args := []tea{"-quiet", "file1.txt", "file2.txt"}
fs.Parse(args)

vibez.spill("Set flags:", fs.NHash())  fr fr 1
vibez.spill("Non-flag args:", fs.NArg())  fr fr 2

sus files := fs.Args()
for i := 0; i < len(files); i++ {
    vibez.spill("File:", files[i])
}
```

### Flag Visitation
```cursed
fs.Visit(slay(flag hashtag.HashFlag) {
    vibez.spill("Set flag:", flag.name, "=", flag.value)
})

fs.VisitAll(slay(flag hashtag.HashFlag) {
    vibez.spill("All flag:", flag.name, "default:", flag.defaultValue)
})
```

### Usage and Help
```cursed
fs.SetUsage(slay() {
    vibez.spill("MyApp - A sample application")
    vibez.spill("Usage: myapp [flags] [files...]")
})

fs.Usage()  fr fr Shows custom usage + flag defaults
```

### Social Media Features
```cursed
fr fr Track trending flags
fs.AddTrend("verbose")
fs.AddTrend("verbose")
fs.AddTrend("debug")
fs.AddTrend("verbose")

sus trending := fs.Trending()
for i := 0; i < len(trending); i++ {
    vibez.spill("Trending flag:", trending[i])
}
```

## Implementation Features

1. **Pure CURSED Implementation** - No FFI dependencies
2. **Flexible Parsing** - Support for multiple flag formats
3. **Type Safety** - Strong typing for different value types
4. **Error Handling** - Clear error messages for invalid flags
5. **Social Features** - Trending flag tracking
6. **Usage Generation** - Automatic help text generation
7. **Global and Local** - Support for both global and local flag sets

## Error Handling

The Parse method returns error messages as strings:
- Empty string ("") indicates successful parsing
- Non-empty string contains the error description

## Flag Value Types

### Supported Types
- **Boolean** (`lit`) - true/false values
- **Integer** (`normie`) - numeric values  
- **String** (`tea`) - text values
- **Float** (`drip`) - floating-point values

### Value Parsing
- Boolean flags can be set without values (defaults to true)
- Numeric values support common formats
- String values preserve exact input

## Implementation Notes

This is a pure CURSED implementation that provides essential command-line flag parsing functionality. The implementation focuses on usability and includes social media-inspired features like trending flag tracking.
