# SpillFacts Module

The `spill_facts` module provides comprehensive string formatting, styled output, and advanced printing capabilities for the CURSED language. It's inspired by Go's fmt package but with enhanced features for modern terminal output and Gen Z-style formatting.

## Features

### Basic Printing
- **Spill**: Basic output printing
- **SpillLine**: Output with newline
- **SpillFormat**: Formatted output with placeholders

### String Formatting
- **GetFacts**: Convert values to formatted strings
- **GetFactsFormat**: Format strings with placeholders
- **GetFactsLine**: Format strings with newlines

### Styled Output
- **SpillColor**: Print text with colors
- **SpillStyle**: Print text with styles (bold, italic, etc.)
- **ColorFacts**: Create colored strings
- **StyleFacts**: Create styled strings

### Structured Output
- **SpillTable**: Print formatted tables
- **SpillJSON**: Print JSON-formatted data
- **SpillList**: Print formatted lists
- **SpillTree**: Print tree structures
- **SpillMap**: Print key-value mappings

### Progress Indicators
- **NewProgressBar**: Create progress bars
- **NewSpinner**: Create loading spinners

### Gen Z Formatting
- **ConvertToGenZ**: Convert text to Gen Z style
- **SpillGenZ**: Print with Gen Z formatting
- **FormatNumGenZ**: Format numbers with Gen Z style (K, M suffixes)
- **SpillWithEmojis**: Add emojis to text

## Usage Examples

```cursed
yeet "spill_facts"

fr fr Basic printing
spill_facts.Spill("Hello, world!")
spill_facts.SpillLine("This is a line")
spill_facts.SpillFormat("Name: %s", "Alice")

fr fr String formatting
sus message tea = spill_facts.GetFacts("The answer is", 42)
sus formatted tea = spill_facts.GetFactsFormat("Pi is %.2f", 3.14)

fr fr Styled output
spill_facts.SpillColor(spill_facts.Red, "Warning!")
spill_facts.SpillStyle(spill_facts.Bold, "Important")

fr fr Structured output
spill_facts.SpillTable("Name | Age", "Alice | 25")
spill_facts.SpillJSON("test data")

fr fr Gen Z formatting
spill_facts.SpillGenZ("This party is lit")
spill_facts.SpillWithEmojis("Great job")
```

## Constants

### Colors
- `Red`, `Green`, `Yellow`, `Blue`, `Magenta`, `Cyan`, `White`, `Black`

### Styles
- `Bold`, `Italic`, `Underline`, `Blink`, `Reverse`, `Strike`

### Gen Z Formats
- `FormatBasic`, `FormatVibe`, `FormatBussin`, `FormatSus`, `FormatYeet`, `FormatNoCapFr`, `FormatDownBad`

## Implementation Notes

This is a pure CURSED implementation focusing on:
- Clean, readable code structure
- Comprehensive test coverage
- Consistent API design
- No external FFI dependencies
- Gen Z-style formatting for modern appeal

The module provides a solid foundation for enhanced output formatting while maintaining the CURSED language's unique aesthetic and functionality.
