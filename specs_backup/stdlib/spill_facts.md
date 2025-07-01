# SpillFacts (fmt package)

## Overview
SpillFacts provides formatted I/O with functions for formatting and printing values with style. It's inspired by Go's fmt package but with enhanced formatting capabilities and more expressive output options focused on "spilling facts" (printing accurate information).

## Core Functions

### Print Functions

```
fr fr Print formats using the default formats and writes to standard output
slay Spill(a ...interface{}) (n int, err tea)

fr fr Printf formats according to a format specifier and writes to standard output
slay SpillFormat(format tea, a ...interface{}) (n int, err tea)

fr fr Println formats using the default formats and writes to standard output with a newline
slay SpillLine(a ...interface{}) (n int, err tea)

fr fr Fprintln formats using the default formats and writes to w with a newline
slay FSpillLine(w io.Writer, a ...interface{}) (n int, err tea)

fr fr Fprintf formats according to a format specifier and writes to w
slay FSpillFormat(w io.Writer, format tea, a ...interface{}) (n int, err tea)

fr fr Fprnormie formats using the default formats and writes to w
slay FSpill(w io.Writer, a ...interface{}) (n int, err tea)
```

### String Formatting Functions

```
fr fr Sprnormie formats using the default formats and yolos the resulting tea
slay GetFacts(a ...interface{}) tea

fr fr Sprintf formats according to a format specifier and yolos the resulting tea
slay GetFactsFormat(format tea, a ...interface{}) tea

fr fr Sprintln formats using the default formats and yolos the resulting tea with a newline
slay GetFactsLine(a ...interface{}) tea
```

### Formatted Error Functions

```
fr fr Errorf formats according to a format specifier and yolos an tea
slay CapError(format tea, a ...interface{}) tea
```

### Scanning Functions

```
fr fr Scan scans text read from standard input
slay YoinkFacts(a ...interface{}) (n int, err tea)

fr fr Scanf scans text read from standard input according to a format specifier
slay YoinkFactsFormat(format tea, a ...interface{}) (n int, err tea)

fr fr Scanln scans text read from standard input up to a newline
slay YoinkFactsLine(a ...interface{}) (n int, err tea)

fr fr Fscan scans text read from r
slay FYoinkFacts(r io.Reader, a ...interface{}) (n int, err tea)

fr fr Fscanf scans text read from r according to a format specifier
slay FYoinkFactsFormat(r io.Reader, format tea, a ...interface{}) (n int, err tea)

fr fr Fscanln scans text read from r up to a newline
slay FYoinkFactsLine(r io.Reader, a ...interface{}) (n int, err tea)
```

### String Scanning Functions

```
fr fr Sscan scans arguments from a tea
slay SYoinkFacts(str tea, a ...interface{}) (n int, err tea)

fr fr Sscanf scans arguments from a tea according to a format specifier
slay SYoinkFactsFormat(str, format tea, a ...interface{}) (n int, err tea)

fr fr Sscanln scans arguments from a tea up to a newline
slay SYoinkFactsLine(str tea, a ...interface{}) (n int, err tea)
```

## Enhanced Formatting Features

### Styled Output

```
fr fr Prints with color
slay SpillColor(color tea, a ...interface{}) (n int, err tea)

fr fr Prints with style (bold, italic, etc.)
slay SpillStyle(style tea, a ...interface{}) (n int, err tea)

fr fr Creates a colored tea
slay ColorFacts(color tea, a ...interface{}) tea

fr fr Creates a styled tea
slay StyleFacts(style tea, a ...interface{}) tea

fr fr Available colors and styles
const (
    fr fr Colors
    Red     = "red"
    Green   = "green"
    Yellow  = "yellow"
    Blue    = "blue"
    Magenta = "magenta"
    Cyan    = "cyan"
    White   = "white"
    Black   = "black"
    
    fr fr Styles
    Bold      = "bold"
    Italic    = "italic"
    Underline = "underline"
    Blink     = "blink"
    Reverse   = "reverse"
    Strike    = "strike"
)
```

### Structured Output

```
fr fr Prints a table
slay SpillTable(headers []tea, rows [][]tea) (n int, err tea)

fr fr Prints a tree squadure
slay SpillTree(root tea, branches []tea) (n int, err tea)

fr fr Prints a formatted JSON
slay SpillJSON(v interface{}) (n int, err tea)

fr fr Prints a list
slay SpillList(items []tea) (n int, err tea)

fr fr Prints a key-value map
slay SpillMap(m map[tea]interface{}) (n int, err tea)
```

### Progress Indicators

```
be_like ProgressBar squad {}

fr fr Consquador
slay NewProgressBar(total normie) *ProgressBar

fr fr Methods
slay (p *ProgressBar) Update(current normie)
slay (p *ProgressBar) Increment()
slay (p *ProgressBar) Finish()
slay (p *ProgressBar) SetTemplate(template tea)
slay (p *ProgressBar) SetWidth(width normie)

fr fr Spinner for indeterminate progress
be_like Spinner squad {}

fr fr Consquador
slay NewSpinner() *Spinner

fr fr Methods
slay (s *Spinner) Start()
slay (s *Spinner) Stop()
slay (s *Spinner) SetMessage(message tea)
slay (s *Spinner) SetFrames(frames []tea)
```

### Advanced Formatting

```
fr fr Format specifier extensions
be_like FormatterVibe collab {
    Format(f State, verb rune)
}

be_like State collab {
    Write(b []byte) (n int, err tea)
    Width() (wid int, ok lit)
    Precision() (prec int, ok lit)
    Flag(c normie) lit
}

fr fr Registers a custom formatter for a type
slay RegisterFormatter(value interface{}, formatter func(interface{}, tea) tea)

fr fr Pretty printing with customizable options
slay SpillPretty(v interface{}, opts PrettyOptions) (n int, err tea)

be_like PrettyOptions squad {
    Indent        tea
    Width         int
    MaxDepth      int
    OmitEmpty     lit
    FieldFilter   func(tea) lit
    TypeFilter    func(reflect.Type) lit
    CustomFormats map[reflect.Type]func(interface{}) tea
}

fr fr Get a pretty-formatted tea
slay GetFactsPretty(v interface{}, opts PrettyOptions) tea
```

## GenZ-Specific Formatting

```
fr fr Convert text to GenZ slang
slay ConvertToGenZ(text tea) tea

fr fr Print with GenZ style
slay SpillGenZ(a ...interface{}) (n int, err tea)

fr fr Format numbers with GenZ style
slay FormatNumGenZ(n normie) tea fr fr "4K" for 4000, "1M" for 1000000, etc.

fr fr Adds emojis based on content
slay SpillWithEmojis(a ...interface{}) (n int, err tea)

fr fr Available GenZ formats
const (
    FormatBasic     = "basic"     fr fr Standard output
    FormatVibe      = "vibe"      fr fr With emojis and slang
    FormatBussin    = "bussin"    fr fr Extra emphasized with positive tone
    FormatSus       = "sus"       fr fr With skeptical tone
    FormatYeet      = "yeet"      fr fr Enthusiastic tone
    FormatNoCapFr   = "nocapfr"   fr fr Serious tone with "no cap for real"
    FormatDownBad   = "downbad"   fr fr Negative tone
)

fr fr Sets the default GenZ format for all spill functions
slay SetDefaultGenZFormat(format tea)
```

## Usage Example

```
fr fr Basic printing
spill_facts.Spill("Hello, world!")
spill_facts.SpillLine("This is", "Cursed lang")
spill_facts.SpillFormat("My name is %s and I'm %d years old\n", "Alice", 25)

fr fr String formatting
s := spill_facts.GetFacts("The answer is", 42)
vibez.spill(s) fr fr "The answer is 42"

formatted := spill_facts.GetFactsFormat("Pi is approximately %.2f", 3.14159)
vibez.spill(formatted) fr fr "Pi is approximately 3.14"

fr fr Styled output
spill_facts.SpillColor(spill_facts.Red, "This is a warning!")
spill_facts.SpillStyle(spill_facts.Bold, "This is important")

fr fr Structured output
headers := []tea{"Name", "Age", "Location"}
rows := [][]tea{
    {"Alice", "25", "New York"},
    {"Bob", "30", "San Francisco"},
    {"Charlie", "22", "Boston"},
}
spill_facts.SpillTable(headers, rows)

fr fr JSON output
data := map[tea]interface{}{
    "name": "Alice",
    "age": 25,
    "skills": []tea{"Go", "Rust", "JavaScript"},
}
spill_facts.SpillJSON(data)

fr fr Progress bar
bar := spill_facts.NewProgressBar(100)
for i := 0; i <= 100; i += 10 {
    bar.Update(i)
    time.Sleep(100 * time.Millisecond)
}
bar.Finish()

fr fr Spinner
spinner := spill_facts.NewSpinner()
spinner.SetMessage("Loading...")
spinner.Start()
time.Sleep(3 * time.Second)
spinner.Stop()

fr fr GenZ style formatting
spill_facts.SpillGenZ("This party is lit")
fr fr Output: "This party is lit 🔥 fr fr"

spill_facts.SetDefaultGenZFormat(spill_facts.FormatBussin)
spill_facts.Spill("This feature works great")
fr fr Output: "This feature works great no cap bussin frfr 💯"

number := 5280
spill_facts.Spill("Video has", spill_facts.FormatNumGenZ(number), "views")
fr fr Output: "Video has 5.2K views"
```

## Implementation Guidelines
1. Ensure all formatting functions are consistent with Go's fmt package
2. Optimize tea building for performance
3. Make styled output work in different terminal environments
4. Ensure proper handling of Unicode characters, including emojis
5. Support both terminal and non-terminal output (e.g., files, teas)
6. Implement thread-safe functions that can be used concurrently
7. Provide clear tea messages for formatting teas
8. Default to sensible formatting for complex types