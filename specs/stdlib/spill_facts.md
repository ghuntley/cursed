# SpillFacts (fmt package)

## Overview
SpillFacts provides formatted I/O with functions for formatting and printing values with style. It's inspired by Go's fmt package but with enhanced formatting capabilities and more expressive output options focused on "spilling facts" (printing accurate information).

## Core Functions

### Print Functions

```go
// Print formats using the default formats and writes to standard output
func Spill(a ...interface{}) (n int, err error)

// Printf formats according to a format specifier and writes to standard output
func SpillFormat(format string, a ...interface{}) (n int, err error)

// Println formats using the default formats and writes to standard output with a newline
func SpillLine(a ...interface{}) (n int, err error)

// Fprintln formats using the default formats and writes to w with a newline
func FSpillLine(w io.Writer, a ...interface{}) (n int, err error)

// Fprintf formats according to a format specifier and writes to w
func FSpillFormat(w io.Writer, format string, a ...interface{}) (n int, err error)

// Fprint formats using the default formats and writes to w
func FSpill(w io.Writer, a ...interface{}) (n int, err error)
```

### String Formatting Functions

```go
// Sprint formats using the default formats and returns the resulting string
func GetFacts(a ...interface{}) string

// Sprintf formats according to a format specifier and returns the resulting string
func GetFactsFormat(format string, a ...interface{}) string

// Sprintln formats using the default formats and returns the resulting string with a newline
func GetFactsLine(a ...interface{}) string
```

### Formatted Error Functions

```go
// Errorf formats according to a format specifier and returns an error
func CapError(format string, a ...interface{}) error
```

### Scanning Functions

```go
// Scan scans text read from standard input
func YoinkFacts(a ...interface{}) (n int, err error)

// Scanf scans text read from standard input according to a format specifier
func YoinkFactsFormat(format string, a ...interface{}) (n int, err error)

// Scanln scans text read from standard input up to a newline
func YoinkFactsLine(a ...interface{}) (n int, err error)

// Fscan scans text read from r
func FYoinkFacts(r io.Reader, a ...interface{}) (n int, err error)

// Fscanf scans text read from r according to a format specifier
func FYoinkFactsFormat(r io.Reader, format string, a ...interface{}) (n int, err error)

// Fscanln scans text read from r up to a newline
func FYoinkFactsLine(r io.Reader, a ...interface{}) (n int, err error)
```

### String Scanning Functions

```go
// Sscan scans arguments from a string
func SYoinkFacts(str string, a ...interface{}) (n int, err error)

// Sscanf scans arguments from a string according to a format specifier
func SYoinkFactsFormat(str, format string, a ...interface{}) (n int, err error)

// Sscanln scans arguments from a string up to a newline
func SYoinkFactsLine(str string, a ...interface{}) (n int, err error)
```

## Enhanced Formatting Features

### Styled Output

```go
// Prints with color
func SpillColor(color string, a ...interface{}) (n int, err error)

// Prints with style (bold, italic, etc.)
func SpillStyle(style string, a ...interface{}) (n int, err error)

// Creates a colored string
func ColorFacts(color string, a ...interface{}) string

// Creates a styled string
func StyleFacts(style string, a ...interface{}) string

// Available colors and styles
const (
    // Colors
    Red     = "red"
    Green   = "green"
    Yellow  = "yellow"
    Blue    = "blue"
    Magenta = "magenta"
    Cyan    = "cyan"
    White   = "white"
    Black   = "black"
    
    // Styles
    Bold      = "bold"
    Italic    = "italic"
    Underline = "underline"
    Blink     = "blink"
    Reverse   = "reverse"
    Strike    = "strike"
)
```

### Structured Output

```go
// Prints a table
func SpillTable(headers []string, rows [][]string) (n int, err error)

// Prints a tree structure
func SpillTree(root string, branches []string) (n int, err error)

// Prints a formatted JSON
func SpillJSON(v interface{}) (n int, err error)

// Prints a list
func SpillList(items []string) (n int, err error)

// Prints a key-value map
func SpillMap(m map[string]interface{}) (n int, err error)
```

### Progress Indicators

```go
type ProgressBar struct {}

// Constructor
func NewProgressBar(total int) *ProgressBar

// Methods
func (p *ProgressBar) Update(current int)
func (p *ProgressBar) Increment()
func (p *ProgressBar) Finish()
func (p *ProgressBar) SetTemplate(template string)
func (p *ProgressBar) SetWidth(width int)

// Spinner for indeterminate progress
type Spinner struct {}

// Constructor
func NewSpinner() *Spinner

// Methods
func (s *Spinner) Start()
func (s *Spinner) Stop()
func (s *Spinner) SetMessage(message string)
func (s *Spinner) SetFrames(frames []string)
```

### Advanced Formatting

```go
// Format specifier extensions
type FormatterVibe interface {
    Format(f State, verb rune)
}

type State interface {
    Write(b []byte) (n int, err error)
    Width() (wid int, ok bool)
    Precision() (prec int, ok bool)
    Flag(c int) bool
}

// Registers a custom formatter for a type
func RegisterFormatter(value interface{}, formatter func(interface{}, string) string)

// Pretty printing with customizable options
func SpillPretty(v interface{}, opts PrettyOptions) (n int, err error)

type PrettyOptions struct {
    Indent        string
    Width         int
    MaxDepth      int
    OmitEmpty     bool
    FieldFilter   func(string) bool
    TypeFilter    func(reflect.Type) bool
    CustomFormats map[reflect.Type]func(interface{}) string
}

// Get a pretty-formatted string
func GetFactsPretty(v interface{}, opts PrettyOptions) string
```

## GenZ-Specific Formatting

```go
// Convert text to GenZ slang
func ConvertToGenZ(text string) string

// Print with GenZ style
func SpillGenZ(a ...interface{}) (n int, err error)

// Format numbers with GenZ style
func FormatNumGenZ(n int) string // "4K" for 4000, "1M" for 1000000, etc.

// Adds emojis based on content
func SpillWithEmojis(a ...interface{}) (n int, err error)

// Available GenZ formats
const (
    FormatBasic     = "basic"     // Standard output
    FormatVibe      = "vibe"      // With emojis and slang
    FormatBussin    = "bussin"    // Extra emphasized with positive tone
    FormatSus       = "sus"       // With skeptical tone
    FormatYeet      = "yeet"      // Enthusiastic tone
    FormatNoCapFr   = "nocapfr"   // Serious tone with "no cap for real"
    FormatDownBad   = "downbad"   // Negative tone
)

// Sets the default GenZ format for all spill functions
func SetDefaultGenZFormat(format string)
```

## Usage Example

```go
// Basic printing
spill_facts.Spill("Hello, world!")
spill_facts.SpillLine("This is", "Cursed lang")
spill_facts.SpillFormat("My name is %s and I'm %d years old\n", "Alice", 25)

// String formatting
s := spill_facts.GetFacts("The answer is", 42)
vibez.spill(s) // "The answer is 42"

formatted := spill_facts.GetFactsFormat("Pi is approximately %.2f", 3.14159)
vibez.spill(formatted) // "Pi is approximately 3.14"

// Styled output
spill_facts.SpillColor(spill_facts.Red, "This is a warning!")
spill_facts.SpillStyle(spill_facts.Bold, "This is important")

// Structured output
headers := []string{"Name", "Age", "Location"}
rows := [][]string{
    {"Alice", "25", "New York"},
    {"Bob", "30", "San Francisco"},
    {"Charlie", "22", "Boston"},
}
spill_facts.SpillTable(headers, rows)

// JSON output
data := map[string]interface{}{
    "name": "Alice",
    "age": 25,
    "skills": []string{"Go", "Rust", "JavaScript"},
}
spill_facts.SpillJSON(data)

// Progress bar
bar := spill_facts.NewProgressBar(100)
for i := 0; i <= 100; i += 10 {
    bar.Update(i)
    time.Sleep(100 * time.Millisecond)
}
bar.Finish()

// Spinner
spinner := spill_facts.NewSpinner()
spinner.SetMessage("Loading...")
spinner.Start()
time.Sleep(3 * time.Second)
spinner.Stop()

// GenZ style formatting
spill_facts.SpillGenZ("This party is lit")
// Output: "This party is lit 🔥 fr fr"

spill_facts.SetDefaultGenZFormat(spill_facts.FormatBussin)
spill_facts.Spill("This feature works great")
// Output: "This feature works great no cap bussin frfr 💯"

number := 5280
spill_facts.Spill("Video has", spill_facts.FormatNumGenZ(number), "views")
// Output: "Video has 5.2K views"
```

## Implementation Guidelines
1. Ensure all formatting functions are consistent with Go's fmt package
2. Optimize string building for performance
3. Make styled output work in different terminal environments
4. Ensure proper handling of Unicode characters, including emojis
5. Support both terminal and non-terminal output (e.g., files, strings)
6. Implement thread-safe functions that can be used concurrently
7. Provide clear error messages for formatting errors
8. Default to sensible formatting for complex types