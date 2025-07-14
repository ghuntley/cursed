# string_energy

Advanced string operations with vibrant energy.

## Overview

The `string_energy` module provides comprehensive string manipulation functions with enhanced capabilities for modern text processing, including GenZ transformations and social media formatting.

## Core Functions

### String Search
- `Contains(s, substr tea) lit` - Check if string contains substring
- `Index(s, substr tea) normie` - Find first occurrence of substring
- `Count(s, substr tea) normie` - Count non-overlapping occurrences

### String Manipulation
- `Replace(s, old, new tea, n normie) tea` - Replace occurrences
- `Split(s, sep tea) []tea` - Split string by separator
- `Join(a []tea, sep tea) tea` - Join strings with separator

### String Transformation
- `ToUpper(s tea) tea` - Convert to uppercase
- `ToLower(s tea) tea` - Convert to lowercase
- `TrimSpace(s tea) tea` - Remove leading/trailing whitespace

## Enhanced Features

### String Building
```cursed
builder := string_energy.NewEnergyBuilder()
builder.WriteString("Hello").WriteString(" ").WriteString("World")
result := builder.String()
```

### Case Conversion
```cursed
camelCase := string_energy.ToCamelCase("hello world test")     // "helloWorldTest"
pascalCase := string_energy.ToPascalCase("hello world test")   // "HelloWorldTest"
snakeCase := string_energy.ToSnakeCase("hello world test")     // "hello_world_test"
kebabCase := string_energy.ToKebabCase("hello world test")     // "hello-world-test"
```

### GenZ Transformations
```cursed
genZStyle := string_energy.ToGenZStyle("this is really cool")  // "dis is rly cool"
withEmojis := string_energy.AddEmojis("this is cool")          // "this is cool 😎"
socialText := string_energy.ToSocialText("awesome", based)     // "awesome ✨🚀 #Awesome"
```

### Text Analysis
- Word and character counting
- Frequency analysis
- Readability scoring
- Language detection
- Keyword extraction

## Utility Functions

- Text padding and centering
- Text wrapping and truncation
- HTML escaping/unescaping
- Pattern matching and interpolation
- Before/after string operations
