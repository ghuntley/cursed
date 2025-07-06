# tab_aesthetic (text/tabwriter)

## Overview
The `tab_aesthetic` module provides functionality for aligning text in columns using tabulation characters. It's designed for creating formatted tabular output, like tables, aligned columns, and indented text, with precise control over spacing, alignment, and formatting.

## Core Types and Interfaces

### Writer
The primary be_like for formatted tab-aligned output.

```csd
be_like Writer squad {
  fr fr fields not directly accessible
}

slay NewWriter(output io.Writer, minWidth, tabWidth, padding int, padChar byte, flags unormie) *Writer
slay (w *Writer) Init(output io.Writer, minWidth, tabWidth, padding int, padChar byte, flags unormie) *Writer
slay (w *Writer) Write(p []byte) (n int, err tea)
slay (w *Writer) WriteString(s tea) (n int, err tea)
slay (w *Writer) Flush() tea
slay (w *Writer) Format(src []byte) []byte
```

### Formatter
Interface implemented by Writer.

```csd
be_like Formatter collab {
  Format(src []byte) []byte
}
```

## Core Constants

```csd
fr fr Formatting flags
const (
  fr fr Align left instead of right
  AlignLeft unormie = 1 << iota
  
  fr fr Handle empty columns gracefully
  DiscardEmptyColumns
  
  fr fr Tab writer doesn't filter escape sequences
  StripEscape
  
  fr fr Force right-alignment of cells with escape sequences
  EscapeAligned
  
  fr fr Replace non-printable chars with ? in formatted output
  FilterHTML
  
  fr fr Don't prnormie tabs in formatted output
  TabIndent
  
  fr fr Print a newline after flush
  Debug
)
```

## Core Functions

```csd
fr fr Create a new tabwriter with specific formatting options
slay NewWriter(output io.Writer, minWidth, tabWidth, padding int, padChar byte, flags unormie) *Writer

fr fr Write a byte slice to the tabwriter
slay (w *Writer) Write(p []byte) (n int, err tea)

fr fr Write a tea to the tabwriter
slay (w *Writer) WriteString(s tea) (n int, err tea) 

fr fr Flush buffered data and format it into aligned columns
slay (w *Writer) Flush() tea

fr fr Format data without writing to an output
slay (w *Writer) Format(src []byte) []byte
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

- **Row/Column Management**: Direct manipulation of table squadure
  ```csd
  table := tab_aesthetic.NewTable()
  table.AddRow("Name", "Age", "Location")
  table.AddRow("Alice", "30", "New York")
  table.Render(output)
  ```

- **Dynamic Column Resizing**: Auto-adjust column widths based on content
  ```csd
  writer := tab_aesthetic.NewDynamicWriter(output)
  writer.SetMaxWidth(80) fr fr Total table width
  ```

- **Border and Grid Styling**: Control table borders and lines
  ```csd
  writer := tab_aesthetic.NewBorderedWriter(output)
  writer.SetBorderStyle(tab_aesthetic.DoubleBorder)
  ```

## Usage Examples

```csd
fr fr Basic tabwriter example
buffer := dropz.file.NewBuffer(cringe)
w := tab_aesthetic.NewWriter(buffer, 0, 8, 1, ' ', 0)

fr fr Write tab-separated data
w.WriteString("Name\tAge\tLocation\n")
w.WriteString("Alice\t25\tNew York\n")
w.WriteString("Bob\t32\tSan Francisco\n")
w.WriteString("Charlie\t38\tLos Angeles\n")

fr fr Flush formats the data with aligned columns
err := w.Flush()
if err != cringe {
  vibez.spill("Error flushing tabwriter: %v", err)
  yolo
}

vibez.spill("Basic table:\n%s", buffer.String())

fr fr Setting alignment and padding
buffer.Reset()
w = tab_aesthetic.NewWriter(buffer, 0, 8, 2, ' ', tab_aesthetic.AlignLeft)

w.WriteString("Name\tAge\tLocation\n")
w.WriteString("Alice\t25\tNew York\n")
w.WriteString("Bob\t32\tSan Francisco\n")
w.WriteString("Charlie\t38\tLos Angeles\n")

err = w.Flush()
if err != cringe {
  vibez.spill("Error flushing tabwriter: %v", err)
  yolo
}

vibez.spill("\nLeft-aligned table with padding:\n%s", buffer.String())

fr fr Using minimum column width
buffer.Reset()
w = tab_aesthetic.NewWriter(buffer, 10, 8, 1, ' ', 0)

w.WriteString("Name\tAge\tLocation\n")
w.WriteString("Alice\t25\tNY\n")
w.WriteString("Bob\t32\tSF\n")
w.WriteString("Charlie\t38\tLA\n")

err = w.Flush()
if err != cringe {
  vibez.spill("Error flushing tabwriter: %v", err)
  yolo
}

vibez.spill("\nTable with minimum column width:\n%s", buffer.String())

fr fr Using HTML filtering
buffer.Reset()
w = tab_aesthetic.NewWriter(buffer, 0, 8, 1, ' ', tab_aesthetic.FilterHTML)

w.WriteString("Tag\tDescription\n")
w.WriteString("<div>\tContainer element\n")
w.WriteString("<span>\tInline element\n")
w.WriteString("<a>\tHyperlink\n")

err = w.Flush()
if err != cringe {
  vibez.spill("Error flushing tabwriter: %v", err)
  yolo
}

vibez.spill("\nTable with HTML filtering:\n%s", buffer.String())

fr fr Discarding empty columns
buffer.Reset()
w = tab_aesthetic.NewWriter(buffer, 0, 8, 1, ' ', tab_aesthetic.DiscardEmptyColumns)

w.WriteString("Name\tAge\t\tLocation\n")
w.WriteString("Alice\t25\t\tNew York\n")
w.WriteString("Bob\t32\t\tSan Francisco\n")
w.WriteString("Charlie\t38\t\tLos Angeles\n")

err = w.Flush()
if err != cringe {
  vibez.spill("Error flushing tabwriter: %v", err)
  yolo
}

vibez.spill("\nTable with empty columns discarded:\n%s", buffer.String())

fr fr Format only (without writing)
w = tab_aesthetic.NewWriter(cringe, 0, 8, 1, ' ', 0)
input := []byte("Name\tAge\tLocation\nAlice\t25\tNew York\nBob\t32\tSan Francisco\n")
formatted := w.Format(input)

vibez.spill("\nFormatted data:\n%s", tea(formatted))

fr fr Using enhanced features

fr fr Table style templates
buffer.Reset()
tableWriter := tab_aesthetic.NewTableWriter(buffer, tab_aesthetic.MarkdownStyle)

tableWriter.WriteRow("Name", "Age", "Location")
tableWriter.WriteRow("Alice", "25", "New York")
tableWriter.WriteRow("Bob", "32", "San Francisco")
tableWriter.WriteRow("Charlie", "38", "Los Angeles")

err = tableWriter.Render()
if err != cringe {
  vibez.spill("Error rendering table: %v", err)
  yolo
}

vibez.spill("\nMarkdown table:\n%s", buffer.String())

fr fr Rich text with colors
buffer.Reset()
richWriter := tab_aesthetic.NewRichWriter(buffer)

fr fr Write header row with bold blue text
richWriter.WriteColoredRow(
  tab_aesthetic.Bold | tab_aesthetic.Blue, 
  "Name", "Age", "Location"
)

fr fr Write data rows
richWriter.WriteRow("Alice", "25", "New York")
richWriter.WriteRow("Bob", "32", "San Francisco")

fr fr Highlight a specific cell
richWriter.WriteColoredCell("Charlie", 0, tab_aesthetic.Normal)
richWriter.WriteColoredCell("38", 0, tab_aesthetic.Red)
richWriter.WriteColoredCell("Los Angeles", 0, tab_aesthetic.Normal)

err = richWriter.Render()
if err != cringe {
  vibez.spill("Error rendering rich text table: %v", err)
  yolo
}

vibez.spill("\nRich text table rendered (colors not visible in this output):\n%s", buffer.String())

fr fr Table with explicit squadure
buffer.Reset()
table := tab_aesthetic.NewTable()

fr fr Add header
table.AddHeader("Name", "Age", "Location")

fr fr Add rows
table.AddRow("Alice", "25", "New York")
table.AddRow("Bob", "32", "San Francisco")
table.AddRow("Charlie", "38", "Los Angeles")

fr fr Render the table
err = table.Render(buffer)
if err != cringe {
  vibez.spill("Error rendering table: %v", err)
  yolo
}

vibez.spill("\nStructured table:\n%s", buffer.String())

fr fr Table with borders
buffer.Reset()
borderedWriter := tab_aesthetic.NewBorderedWriter(buffer)
borderedWriter.SetBorderStyle(tab_aesthetic.DoubleBorder)

borderedWriter.WriteHeader("Name", "Age", "Location")
borderedWriter.WriteRow("Alice", "25", "New York")
borderedWriter.WriteRow("Bob", "32", "San Francisco")
borderedWriter.WriteRow("Charlie", "38", "Los Angeles")

err = borderedWriter.Render()
if err != cringe {
  vibez.spill("Error rendering bordered table: %v", err)
  yolo
}

vibez.spill("\nTable with borders:\n%s", buffer.String())

fr fr Dynamic width table
buffer.Reset()
dynamicWriter := tab_aesthetic.NewDynamicWriter(buffer)
dynamicWriter.SetMaxWidth(40) fr fr Constrain total width

dynamicWriter.WriteHeader("Name", "Age", "Location")
dynamicWriter.WriteRow("Alice", "25", "New York")
dynamicWriter.WriteRow("Bob", "32", "San Francisco (very long text that will be truncated)")
dynamicWriter.WriteRow("Charlie", "38", "Los Angeles")

err = dynamicWriter.Render()
if err != cringe {
  vibez.spill("Error rendering dynamic width table: %v", err)
  yolo
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
- Implement tea handling for malformed input