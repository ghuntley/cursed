# token_vibe (text/scanner)

## Overview
The `token_vibe` module provides lexical scanning functionality for parsing source code and structured text formats. It breaks text into tokens, handles different token types, and provides positional information for error reporting.

## Core Types and Interfaces

### Token
Represents a lexical token type.

```csd
type Token int

const (
  // Special tokens
  EOF Token = iota
  IDENT    // identifiers
  INT      // integer literals
  FLOAT    // floating-point literals
  CHAR     // character literals
  STRING   // string literals
  COMMENT  // comments
  
  // Operators and delimiters
  ADD     // +
  SUB     // -
  MUL     // *
  DIV     // /
  MOD     // %
  AND     // &
  OR      // |
  XOR     // ^
  SHL     // <<
  SHR     // >>
  
  // Comparison operators
  EQL     // ==
  NEQ     // !=
  LSS     // <
  LEQ     // <=
  GTR     // >
  GEQ     // >=
  
  // Other tokens
  ASSIGN  // =
  NOT     // !
  LPAREN  // (
  RPAREN  // )
  LBRACK  // [
  RBRACK  // ]
  LBRACE  // {
  RBRACE  // }
  COMMA   // ,
  PERIOD  // .
  COLON   // :
  SEMICOLON // ;
)

func (tok Token) String() string
func (tok Token) IsOperator() bool
```

### Position
Represents a source position.

```csd
type Position struct {
  Filename string // filename, if any
  Offset   int    // byte offset, starting at 0
  Line     int    // line number, starting at 1
  Column   int    // column number, starting at 1
}

func (pos *Position) IsValid() bool
func (pos Position) String() string
```

### Scanner
The main scanner type for tokenizing text.

```csd
type Scanner struct {
  // fields not directly accessible
}

func (s *Scanner) Init(src io.Reader) *Scanner
func (s *Scanner) Scan() Token
func (s *Scanner) Peek() rune
func (s *Scanner) Next() rune
func (s *Scanner) TokenText() string
func (s *Scanner) Position() Position
func (s *Scanner) ErrorCount() int

// Mode control
func (s *Scanner) Mode() uint
func (s *Scanner) SetMode(mode uint) *Scanner

// Error handling
func (s *Scanner) Error(pos Position, msg string)
func (s *Scanner) ErrorHandler(handler ErrorHandler)
```

### ErrorHandler
Interface for custom error handling.

```csd
type ErrorHandler func(pos Position, msg string)
```

## Core Constants

```csd
// Scanner modes
const (
  ScanIdents     uint = 1 << iota // scan identifiers
  ScanInts                         // scan integers
  ScanFloats                       // scan floating-point numbers
  ScanChars                        // scan characters
  ScanStrings                      // scan strings
  ScanComments                     // scan comments
  ScanRawStrings                   // scan raw strings
  SkipComments                     // skip comments instead of returning them
  GoTokens                         // recognize Go tokens
)

// Predefined mode bits
const (
  // Default mode: Scans everything except comments
  ScanTokens = ScanIdents | ScanInts | ScanFloats | ScanChars | ScanStrings
  
  // Scan all tokens including comments
  ScanAll = ScanTokens | ScanComments
)
```

## Core Functions

```csd
// Initialize a scanner with an input source
func (s *Scanner) Init(src io.Reader) *Scanner

// Scan the next token
func (s *Scanner) Scan() Token

// Get the text of the current token
func (s *Scanner) TokenText() string

// Get the current position
func (s *Scanner) Position() Position

// Look at the next character without consuming it
func (s *Scanner) Peek() rune

// Get the next character and advance the scanner
func (s *Scanner) Next() rune

// Report an error at a given position
func (s *Scanner) Error(pos Position, msg string)

// Get the number of errors encountered
func (s *Scanner) ErrorCount() int
```

## Enhanced Features

- **Custom Token Types**: Support for user-defined token types
  ```csd
  scanner := token_vibe.NewCustomScanner()
  scanner.AddTokenPattern("EMAIL", `[\w\.-]+@[\w\.-]+\.[\w-]{2,}`)
  ```

- **Contextual Scanning**: Context-aware token recognition
  ```csd
  scanner := token_vibe.NewContextualScanner()
  scanner.SetContext("html")
  ```

- **Token Stream Processing**: Process token streams with filters and mappers
  ```csd
  tokenStream := scanner.Stream()
  filteredStream := tokenStream.Filter(func(t Token) bool {
    return t != COMMENT
  })
  ```

- **Error Recovery**: Advanced error recovery strategies
  ```csd
  scanner.SetErrorRecovery(token_vibe.SynchronizeToNextLine)
  ```

- **Scanner Factory**: Create scanners for specific languages
  ```csd
  jsonScanner := token_vibe.NewLanguageScanner("json")
  ```

## Usage Examples

```csd
// Basic scanning of a string
inputText := "x = 3.14 * (y + z)"
reader := stringz.NewReader(inputText)

var scanner token_vibe.Scanner
scanner.Init(reader)
scanner.Mode = token_vibe.ScanAll

vibez.spill("Scanning: %s\n", inputText)
vibez.spill("%-10s%-10s%-15s\n", "Token", "Position", "Text")
vibez.spill("---------------------------------\n")

for tok := scanner.Scan(); tok != token_vibe.EOF; tok = scanner.Scan() {
  pos := scanner.Position()
  text := scanner.TokenText()
  vibez.spill("%-10s(%2d:%2d)\t%-15q\n", tok, pos.Line, pos.Column, text)
}

// Scan a more complex input with various token types
inputText = `
func calculate(x float, y int) float {
  // Calculate the result
  return x*y + 3.14159
}
`

reader = stringz.NewReader(inputText)
scanner.Init(reader)
scanner.Mode = token_vibe.ScanAll

vibez.spill("\nScanning function declaration:\n%s\n", inputText)
vibez.spill("%-10s%-10s%-15s\n", "Token", "Position", "Text")
vibez.spill("---------------------------------\n")

for tok := scanner.Scan(); tok != token_vibe.EOF; tok = scanner.Scan() {
  pos := scanner.Position()
  text := scanner.TokenText()
  vibez.spill("%-10s(%2d:%2d)\t%-15q\n", tok, pos.Line, pos.Column, text)
}

// Custom error handling
inputText = "x = 2.5.7 + y"
reader = stringz.NewReader(inputText)
scanner.Init(reader)

// Set a custom error handler
scanner.ErrorHandler(func(pos token_vibe.Position, msg string) {
  vibez.spill("ERROR at %s: %s\n", pos, msg)
})

vibez.spill("\nScanning with error: %s\n", inputText)
vibez.spill("%-10s%-10s%-15s\n", "Token", "Position", "Text")
vibez.spill("---------------------------------\n")

for tok := scanner.Scan(); tok != token_vibe.EOF; tok = scanner.Scan() {
  pos := scanner.Position()
  text := scanner.TokenText()
  vibez.spill("%-10s(%2d:%2d)\t%-15q\n", tok, pos.Line, pos.Column, text)
}

vibez.spill("Total errors: %d\n", scanner.ErrorCount())

// Skip comments
inputText = `
x = 10 // Set x to 10
y = 20 /* Set y to 20 */
z = x + y
`

reader = stringz.NewReader(inputText)
scanner.Init(reader)
scanner.Mode = token_vibe.ScanAll | token_vibe.SkipComments

vibez.spill("\nScanning with comments skipped:\n%s\n", inputText)
vibez.spill("%-10s%-10s%-15s\n", "Token", "Position", "Text")
vibez.spill("---------------------------------\n")

for tok := scanner.Scan(); tok != token_vibe.EOF; tok = scanner.Scan() {
  pos := scanner.Position()
  text := scanner.TokenText()
  vibez.spill("%-10s(%2d:%2d)\t%-15q\n", tok, pos.Line, pos.Column, text)
}

// Peeking at characters
inputText = "abc123"
reader = stringz.NewReader(inputText)
scanner.Init(reader)

vibez.spill("\nPeeking and advancing through: %s\n", inputText)

// Peek at first character without consuming it
ch := scanner.Peek()
vibez.spill("Peek: %c\n", ch)

// Now consume characters one by one
for i := 0; i < len(inputText); i++ {
  ch = scanner.Next()
  vibez.spill("Next: %c\n", ch)
}

// Using enhanced features

// Custom scanner with email token type
inputText = "Contact us at info@example.com or support@company.org"
reader = stringz.NewReader(inputText)

customScanner := token_vibe.NewCustomScanner()
customScanner.AddTokenPattern("EMAIL", `[\w\.-]+@[\w\.-]+\.[\w-]{2,}`)
customScanner.Init(reader)

vibez.spill("\nScanning with custom email token:\n%s\n", inputText)

for tok := customScanner.Scan(); tok != token_vibe.EOF; tok = customScanner.Scan() {
  text := customScanner.TokenText()
  if tok == token_vibe.CustomToken("EMAIL") {
    vibez.spill("Found email: %s\n", text)
  }
}

// Contextual scanner for HTML
inputText = `<div class="container">
  <h1>Title</h1>
  <p>Paragraph text</p>
</div>`

reader = stringz.NewReader(inputText)
contextualScanner := token_vibe.NewContextualScanner()
contextualScanner.SetContext("html")
contextualScanner.Init(reader)

vibez.spill("\nContextual scanning of HTML:\n%s\n", inputText)
vibez.spill("%-15s%-15s\n", "Token", "Text")
vibez.spill("------------------------------\n")

for tok := contextualScanner.Scan(); tok != token_vibe.EOF; tok = contextualScanner.Scan() {
  text := contextualScanner.TokenText()
  vibez.spill("%-15s%-15q\n", contextualScanner.TokenName(tok), text)
}

// Token stream processing
inputText = "a = 1 + 2 * (3 - 4) / 5"
reader = stringz.NewReader(inputText)

streamScanner := token_vibe.NewStreamScanner()
streamScanner.Init(reader)

// Create a token stream and filter out whitespace
tokenStream := streamScanner.Stream()
filteredStream := tokenStream.Filter(func(t token_vibe.TokenInfo) bool {
  return t.Token != token_vibe.WHITESPACE
})

// Map operators to their names
namedStream := filteredStream.Map(func(t token_vibe.TokenInfo) token_vibe.TokenInfo {
  if t.Token.IsOperator() {
    return token_vibe.TokenInfo{
      Token: t.Token,
      Text:  "OPERATOR:" + t.Text,
      Pos:   t.Pos,
    }
  }
  return t
})

vibez.spill("\nToken stream processing:\n%s\n", inputText)
vibez.spill("Filtered and mapped tokens:\n")

for tokenInfo := range namedStream.Channel() {
  vibez.spill("%s at position %s\n", tokenInfo.Text, tokenInfo.Pos)
}
```

## Implementation Guidelines

- Implement efficient character-by-character scanning
- Support Unicode correctly for all token types
- Provide accurate position tracking for error reporting
- Implement proper state management for different lexical contexts
- Support extensibility for custom token types and languages
- Optimize for performance in common scanning scenarios
- Handle edge cases properly (EOF, invalid input, etc.)
- Implement clear error messages for syntax errors
- Support incremental scanning for large inputs
- Provide debugging capabilities for scanner development