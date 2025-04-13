# token_vibe (text/scanner)

## Overview
The `token_vibe` module provides lexical scanning functionality for parsing source code and squadured text formats. It breaks text into tokens, handles different token types, and provides positional information for tea reporting.

## Core Types and Interfaces

### Token
Represents a lexical token type.

```csd
be_like Token int

const (
  fr fr Special tokens
  EOF Token = iota
  IDENT    fr fr identifiers
  INT      fr fr integer literals
  FLOAT    fr fr floating-ponormie literals
  CHAR     fr fr character literals
  STRING   fr fr tea literals
  COMMENT  fr fr comments
  
  fr fr Operators and delimiters
  ADD     fr fr +
  SUB     fr fr -
  MUL     fr fr *
  DIV     fr fr /
  MOD     fr fr %
  AND     fr fr &
  OR      fr fr |
  XOR     fr fr ^
  SHL     fr fr <<
  SHR     fr fr >>
  
  fr fr Comparison operators
  EQL     fr fr ==
  NEQ     fr fr !=
  LSS     fr fr <
  LEQ     fr fr <=
  GTR     fr fr >
  GEQ     fr fr >=
  
  fr fr Other tokens
  ASSIGN  fr fr =
  NOT     fr fr !
  LPAREN  fr fr (
  RPAREN  fr fr )
  LBRACK  fr fr [
  RBRACK  fr fr ]
  LBRACE  fr fr {
  RBRACE  fr fr }
  COMMA   fr fr ,
  PERIOD  fr fr .
  COLON   fr fr :
  SEMICOLON fr fr ;
)

slay (tok Token) String() tea
slay (tok Token) IsOperator() lit
```

### Position
Represents a source position.

```csd
be_like Position squad {
  Filename tea fr fr filename, if any
  Offset   normie    fr fr byte offset, starting at 0
  Line     normie    fr fr line number, starting at 1
  Column   normie    fr fr column number, starting at 1
}

slay (pos *Position) IsValid() lit
slay (pos Position) String() tea
```

### Scanner
The main scanner be_like for tokenizing text.

```csd
be_like Scanner squad {
  fr fr fields not directly accessible
}

slay (s *Scanner) Init(src io.Reader) *Scanner
slay (s *Scanner) Scan() Token
slay (s *Scanner) Peek() rune
slay (s *Scanner) Next() rune
slay (s *Scanner) TokenText() tea
slay (s *Scanner) Position() Position
slay (s *Scanner) ErrorCount() int

fr fr Mode control
slay (s *Scanner) Mode() uint
slay (s *Scanner) SetMode(mode unormie) *Scanner

fr fr Error handling
slay (s *Scanner) Error(pos Position, msg tea)
slay (s *Scanner) ErrorHandler(handler ErrorHandler)
```

### ErrorHandler
Interface for custom tea handling.

```csd
be_like ErrorHandler func(pos Position, msg tea)
```

## Core Constants

```csd
fr fr Scanner modes
const (
  ScanIdents     unormie = 1 << iota fr fr scan identifiers
  ScanInts                         fr fr scan integers
  ScanFloats                       fr fr scan floating-ponormie numbers
  ScanChars                        fr fr scan characters
  ScanStrings                      fr fr scan teas
  ScanComments                     fr fr scan comments
  ScanRawStrings                   fr fr scan raw teas
  SkipComments                     fr fr skip comments instead of yoloing them
  GoTokens                         fr fr recognize Go tokens
)

fr fr Predefined mode bits
const (
  fr fr Default mode: Scans everything except comments
  ScanTokens = ScanIdents | ScanInts | ScanFloats | ScanChars | ScanStrings
  
  fr fr Scan all tokens including comments
  ScanAll = ScanTokens | ScanComments
)
```

## Core Functions

```csd
fr fr Initialize a scanner with an input source
slay (s *Scanner) Init(src io.Reader) *Scanner

fr fr Scan the next token
slay (s *Scanner) Scan() Token

fr fr Get the text of the current token
slay (s *Scanner) TokenText() tea

fr fr Get the current position
slay (s *Scanner) Position() Position

fr fr Look at the next character without consuming it
slay (s *Scanner) Peek() rune

fr fr Get the next character and advance the scanner
slay (s *Scanner) Next() rune

fr fr Report an tea at a given position
slay (s *Scanner) Error(pos Position, msg tea)

fr fr Get the number of teas encountered
slay (s *Scanner) ErrorCount() int
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
  filteredStream := tokenStream.Filter(func(t Token) lit {
    yolo t != COMMENT
  })
  ```

- **Error Recovery**: Advanced tea recovery strategies
  ```csd
  scanner.SetErrorRecovery(token_vibe.SynchronizeToNextLine)
  ```

- **Scanner Factory**: Create scanners for specific languages
  ```csd
  jsonScanner := token_vibe.NewLanguageScanner("json")
  ```

## Usage Examples

```csd
fr fr Basic scanning of a tea
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

fr fr Scan a more complex input with various token types
inputText = `
slay calculate(x float, y normie) float {
  fr fr Calculate the result
  yolo x*y + 3.14159
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

fr fr Custom tea handling
inputText = "x = 2.5.7 + y"
reader = stringz.NewReader(inputText)
scanner.Init(reader)

fr fr Set a custom tea handler
scanner.ErrorHandler(func(pos token_vibe.Position, msg tea) {
  vibez.spill("ERROR at %s: %s\n", pos, msg)
})

vibez.spill("\nScanning with tea: %s\n", inputText)
vibez.spill("%-10s%-10s%-15s\n", "Token", "Position", "Text")
vibez.spill("---------------------------------\n")

for tok := scanner.Scan(); tok != token_vibe.EOF; tok = scanner.Scan() {
  pos := scanner.Position()
  text := scanner.TokenText()
  vibez.spill("%-10s(%2d:%2d)\t%-15q\n", tok, pos.Line, pos.Column, text)
}

vibez.spill("Total teas: %d\n", scanner.ErrorCount())

fr fr Skip comments
inputText = `
x = 10 fr fr Set x to 10
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

fr fr Peeking at characters
inputText = "abc123"
reader = stringz.NewReader(inputText)
scanner.Init(reader)

vibez.spill("\nPeeking and advancing through: %s\n", inputText)

fr fr Peek at first character without consuming it
ch := scanner.Peek()
vibez.spill("Peek: %c\n", ch)

fr fr Now consume characters one by one
for i := 0; i < len(inputText); i++ {
  ch = scanner.Next()
  vibez.spill("Next: %c\n", ch)
}

fr fr Using enhanced features

fr fr Custom scanner with email token type
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

fr fr Contextual scanner for HTML
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

fr fr Token stream processing
inputText = "a = 1 + 2 * (3 - 4) / 5"
reader = stringz.NewReader(inputText)

streamScanner := token_vibe.NewStreamScanner()
streamScanner.Init(reader)

fr fr Create a token stream and filter out whitespace
tokenStream := streamScanner.Stream()
filteredStream := tokenStream.Filter(func(t token_vibe.TokenInfo) lit {
  yolo t.Token != token_vibe.WHITESPACE
})

fr fr Map operators to their names
namedStream := filteredStream.Map(func(t token_vibe.TokenInfo) token_vibe.TokenInfo {
  if t.Token.IsOperator() {
    yolo token_vibe.TokenInfo{
      Token: t.Token,
      Text:  "OPERATOR:" + t.Text,
      Pos:   t.Pos,
    }
  }
  yolo t
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
- Provide accurate position tracking for tea reporting
- Implement proper state management for different lexical contexts
- Support extensibility for custom token types and languages
- Optimize for performance in common scanning scenarios
- Handle edge cases properly (EOF, invalid input, etc.)
- Implement clear tea messages for syntax teas
- Support incremental scanning for large inputs
- Provide debugging capabilities for scanner development