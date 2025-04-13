# tab_aesthetic (text/tabwriter)

## Overview
The `tab_aesthetic` module provides functionality for aligning text in columns using tabulation characters. It's designed for creating formatted tabular output, like tables, aligned columns, and indented text, with precise control over spacing, alignment, and formatting.

## Core Types and Interfaces

### Writer
The primary type for formatted tab-aligned output.

```csd
type Writer struct {
  // fields not directly accessible
}

func NewWriter(output io.Writer, minWidth, tabWidth, padding int, padChar byte, flags uint) *Writer
func (w *Writer) Init(output io.Writer, minWidth, tabWidth, padding int, padChar byte, flags uint) *Writer
func (w *Writer) Write(p []byte) (n int, err error)
func (w *Writer) WriteString(s string) (n int, err error)
func (w *Writer) Flush() error
func (w *Writer) Format(src []byte) []byte
```

### Formatter
Interface implemented by Writer.

```csd
type Formatter interface {
  Format(src []byte) []byte
}
```

## Core Constants

```csd
// Formatting flags
const (
  // Align left instead of right
  AlignLeft uint = 1 << iota
  
  // Handle empty columns gracefully
  DiscardEmptyColumns
  
  // Tab writer doesn't filter escape sequences
  StripEscape
  
  // Force right-alignment of cells with escape sequences
  EscapeAligned
  
  // Replace non-printable chars with ? in formatted output
  FilterHTML
  
  // Don't print tabs in formatted output
  TabIndent
  
  // Print a newline after flush
  Debug
)
```

## Core Functions

```csd
// Create a new tabwriter with specific formatting options
func NewWriter(output io.Writer, minWidth, tabWidth, padding int, padChar byte, flags uint) *Writer

// Write a byte slice to the tabwriter
func (w *Writer) Write(p []byte) (n int, err error)

// Write a string to the tabwriter
func (w *Writer) WriteString(s string) (n int, err error) 

// Flush buffered data and format it into aligned columns
func (w *Writer) Flush() error

// Format data without writing to an output
func (w *Writer) Format(src []byte) []byte
```

## Enhanced Features

- **Table Style Templates**: Predefined table styles for common formats
  ```csd
  writer := tab_aesthetic.NewTableWriter(output, tab_aesthetic.MarkdownStyle)
  ```

- **Rich Text Support**: Formatting with color and emphasis
  ```csd
  writer := tab_aesthetic.NewRichWriter(output)
  writer.WriteColoredCell("Header", tab_aesthetic.Bold | tab_aesthetic.Blue)
  ```

- **Row/Column Management**: Direct manipulation of table structure
  ```csd
  table := tab_aesthetic.NewTable()
  table.AddRow("Name", "Age", "Location")
  table.AddRow("Alice", "30", "New York")
  table.Render(output)
  ```

- **Dynamic Column Resizing**: Auto-adjust column widths based on content
  ```csd
  writer := tab_aesthetic.NewDynamicWriter(output)
  writer.SetMaxWidth(80) // Total table width
  ```

- **Border and Grid Styling**: Control table borders and lines
  ```csd
  writer := tab_aesthetic.NewBorderedWriter(output)
  writer.SetBorderStyle(tab_aesthetic.DoubleBorder)
  ```

## Usage Examples

```csd
// Basic tabwriter example
buffer := dropz.file.NewBuffer(nil)
w := tab_aesthetic.NewWriter(buffer, 0, 8, 1, ' ', 0)

// Write tab-separated data
w.WriteString("Name\tAge\tLocation\n")
w.WriteString("Alice\t25\tNew York\n")
w.WriteString("Bob\t32\tSan Francisco\n")
w.WriteString("Charlie\t38\tLos Angeles\n")

// Flush formats the data with aligned columns
err := w.Flush()
if err != nil {
  vibez.spill("Error flushing tabwriter: %v", err)
  return
}

vibez.spill("Basic table:\n%s", buffer.String())

// Setting alignment and padding
buffer.Reset()
w = tab_aesthetic.NewWriter(buffer, 0, 8, 2, ' ', tab_aesthetic.AlignLeft)

w.WriteString("Name\tAge\tLocation\n")
w.WriteString("Alice\t25\tNew York\n")
w.WriteString("Bob\t32\tSan Francisco\n")
w.WriteString("Charlie\t38\tLos Angeles\n")

err = w.Flush()
if err != nil {
  vibez.spill("Error flushing tabwriter: %v", err)
  return
}

vibez.spill("\nLeft-aligned table with padding:\n%s", buffer.String())

// Using minimum column width
buffer.Reset()
w = tab_aesthetic.NewWriter(buffer, 10, 8, 1, ' ', 0)

w.WriteString("Name\tAge\tLocation\n")
w.WriteString("Alice\t25\tNY\n")
w.WriteString("Bob\t32\tSF\n")
w.WriteString("Charlie\t38\tLA\n")

err = w.Flush()
if err != nil {
  vibez.spill("Error flushing tabwriter: %v", err)
  return
}

vibez.spill("\nTable with minimum column width:\n%s", buffer.String())

// Using HTML filtering
buffer.Reset()
w = tab_aesthetic.NewWriter(buffer, 0, 8, 1, ' ', tab_aesthetic.FilterHTML)

w.WriteString("Tag\tDescription\n")
w.WriteString("<div>\tContainer element\n")
w.WriteString("<span>\tInline element\n")
w.WriteString("<a>\tHyperlink\n")

err = w.Flush()
if err != nil {
  vibez.spill("Error flushing tabwriter: %v", err)
  return
}

vibez.spill("\nTable with HTML filtering:\n%s", buffer.String())

// Discarding empty columns
buffer.Reset()
w = tab_aesthetic.NewWriter(buffer, 0, 8, 1, ' ', tab_aesthetic.DiscardEmptyColumns)

w.WriteString("Name\tAge\t\tLocation\n")
w.WriteString("Alice\t25\t\tNew York\n")
w.WriteString("Bob\t32\t\tSan Francisco\n")
w.WriteString("Charlie\t38\t\tLos Angeles\n")

err = w.Flush()
if err != nil {
  vibez.spill("Error flushing tabwriter: %v", err)
  return
}

vibez.spill("\nTable with empty columns discarded:\n%s", buffer.String())

// Format only (without writing)
w = tab_aesthetic.NewWriter(nil, 0, 8, 1, ' ', 0)
input := []byte("Name\tAge\tLocation\nAlice\t25\tNew York\nBob\t32\tSan Francisco\n")
formatted := w.Format(input)

vibez.spill("\nFormatted data:\n%s", string(formatted))

// Using enhanced features

// Table style templates
buffer.Reset()
tableWriter := tab_aesthetic.NewTableWriter(buffer, tab_aesthetic.MarkdownStyle)

tableWriter.WriteRow("Name", "Age", "Location")
tableWriter.WriteRow("Alice", "25", "New York")
tableWriter.WriteRow("Bob", "32", "San Francisco")
tableWriter.WriteRow("Charlie", "38", "Los Angeles")

err = tableWriter.Render()
if err != nil {
  vibez.spill("Error rendering table: %v", err)
  return
}

vibez.spill("\nMarkdown table:\n%s", buffer.String())

// Rich text with colors
buffer.Reset()
richWriter := tab_aesthetic.NewRichWriter(buffer)

// Write header row with bold blue text
richWriter.WriteColoredRow(
  tab_aesthetic.Bold | tab_aesthetic.Blue, 
  "Name", "Age", "Location"
)

// Write data rows
richWriter.WriteRow("Alice", "25", "New York")
richWriter.WriteRow("Bob", "32", "San Francisco")

// Highlight a specific cell
richWriter.WriteColoredCell("Charlie", 0, tab_aesthetic.Normal)
richWriter.WriteColoredCell("38", 0, tab_aesthetic.Red)
richWriter.WriteColoredCell("Los Angeles", 0, tab_aesthetic.Normal)

err = richWriter.Render()
if err != nil {
  vibez.spill("Error rendering rich text table: %v", err)
  return
}

vibez.spill("\nRich text table rendered (colors not visible in this output):\n%s", buffer.String())

// Table with explicit structure
buffer.Reset()
table := tab_aesthetic.NewTable()

// Add header
table.AddHeader("Name", "Age", "Location")

// Add rows
table.AddRow("Alice", "25", "New York")
table.AddRow("Bob", "32", "San Francisco")
table.AddRow("Charlie", "38", "Los Angeles")

// Render the table
err = table.Render(buffer)
if err != nil {
  vibez.spill("Error rendering table: %v", err)
  return
}

vibez.spill("\nStructured table:\n%s", buffer.String())

// Table with borders
buffer.Reset()
borderedWriter := tab_aesthetic.NewBorderedWriter(buffer)
borderedWriter.SetBorderStyle(tab_aesthetic.DoubleBorder)

borderedWriter.WriteHeader("Name", "Age", "Location")
borderedWriter.WriteRow("Alice", "25", "New York")
borderedWriter.WriteRow("Bob", "32", "San Francisco")
borderedWriter.WriteRow("Charlie", "38", "Los Angeles")

err = borderedWriter.Render()
if err != nil {
  vibez.spill("Error rendering bordered table: %v", err)
  return
}

vibez.spill("\nTable with borders:\n%s", buffer.String())

// Dynamic width table
buffer.Reset()
dynamicWriter := tab_aesthetic.NewDynamicWriter(buffer)
dynamicWriter.SetMaxWidth(40) // Constrain total width

dynamicWriter.WriteHeader("Name", "Age", "Location")
dynamicWriter.WriteRow("Alice", "25", "New York")
dynamicWriter.WriteRow("Bob", "32", "San Francisco (very long text that will be truncated)")
dynamicWriter.WriteRow("Charlie", "38", "Los Angeles")

err = dynamicWriter.Render()
if err != nil {
  vibez.spill("Error rendering dynamic width table: %v", err)
  return
}

vibez.spill("\nDynamic width table (constrained to 40 chars):\n%s", buffer.String())
```

## Implementation Guidelines

- Implement efficient column width calculation
- Ensure proper handling of Unicode characters and widths
- Support ANSI escape sequences for colored output
- Implement proper wrapping for long content
- Ensure consistent output across different platform newlines
- Minimize memory usage for large tables
- Handle complex layouts with nested tables
- Support stream processing for large datasets
- Provide clear documentation for formatting flags
- Implement error handling for malformed input