# Token Vibe Module

Comprehensive tokenization system for CURSED lexical analysis, essential for the self-hosting compiler.

## Overview

The `token_vibe` module provides a complete lexical analysis foundation with:

- **102+ token types** covering all CURSED language constructs
- **State machine scanner** with proper error recovery
- **Position tracking** for debugging and error reporting
- **Token stream utilities** for parser integration
- **Character classification** optimized for CURSED syntax
- **Error recovery** mechanisms for robust parsing

## Token Types

### Core Tokens (0-9)
- `EOF_TOKEN` (0) - End of file
- `IDENT_TOKEN` (1) - Identifiers
- `INT_TOKEN` (2) - Integer literals
- `FLOAT_TOKEN` (3) - Floating-point literals
- `STRING_TOKEN` (4) - String literals
- `CHAR_TOKEN` (5) - Character literals
- `KEYWORD_TOKEN` (6) - Language keywords
- `COMMENT_TOKEN` (7) - Comments
- `WHITESPACE_TOKEN` (8) - Whitespace
- `NEWLINE_TOKEN` (9) - Line breaks

### Operators (10-39)
Comprehensive operator support including:
- Arithmetic: `+`, `-`, `*`, `/`, `%`
- Comparison: `==`, `!=`, `<`, `<=`, `>`, `>=`
- Logical: `&&`, `||`, `!`
- Assignment: `=`, `+=`, `-=`, `*=`, `/=`
- Increment/Decrement: `++`, `--`
- Bitwise: `&`, `|`, `^`, `~`, `<<`, `>>`
- Special: `->`, `<-`, `:=`

### Delimiters (40-59)
All CURSED delimiters:
- Parentheses: `(`, `)`
- Braces: `{`, `}`
- Brackets: `[`, `]`
- Punctuation: `;`, `,`, `.`, `:`, `?`
- Special: `...`, `..`, `|`, `@`, `#`, `$`, `%`, `^`, `~`

### CURSED Keywords (60-99)
Complete CURSED language keyword set:

#### Control Flow
- `sus` (60) - Variable declaration
- `damn` (61) - Return statement
- `slay` (62) - Function definition
- `lowkey` (68) - If conditional
- `bestie` (69) - For loop
- `yolo` (70) - Goroutine spawn
- `ready` (71) - Select statement
- `ghosted` (72) - Break statement
- `simp` (73) - Continue statement
- `defer` (74) - Defer statement

#### Types
- `lit` (78) - Boolean type
- `tea` (79) - String type
- `drip` (80) - Float type
- `normie` (81) - Integer type
- `thicc` (82) - i64 type
- `smol` (83) - i8 type
- `mid` (84) - i16 type
- `snack` (85) - f32 type
- `meal` (86) - f64 type
- `sip` (87) - Character type
- `byte` (88) - Byte type
- `rune` (89) - Unicode rune
- `extra` (90) - Complex type
- `chan` (91) - Channel type

#### Values
- `based` (65) - True literal
- `cap` (66) - False literal
- `cringe` (67) - Nil literal

#### Error Handling
- `yikes` (75) - Error declaration
- `shook` (76) - Error check
- `fam` (77) - Error propagation

#### Module System
- `yeet` (64) - Import statement
- `vibe` (63) - Package declaration
- `be_like` (92) - Type alias

## Core Functions

### Tokenization

```cursed
tokenize(source tea) normie
```
Main tokenization function that processes source code and returns token count.

**Parameters:**
- `source` - Source code string to tokenize

**Returns:**
- Number of tokens found

**Example:**
```cursed
sus source tea = "sus x normie = 42"
sus count normie = tokenize(source)
# count will be > 0 for valid CURSED syntax
```

### Position Tracking

```cursed
create_position(filename tea, line normie, column normie, offset normie) normie
```
Creates a position tracking object for debugging and error reporting.

```cursed
position_line(pos normie) normie
position_column(pos normie) normie  
position_offset(pos normie) normie
position_string(pos normie) tea
```
Extract position information and format as string.

### Token Information

```cursed
create_token_info(token_type normie, value tea, position normie, raw tea) normie
```
Creates a token information structure.

```cursed
token_type(token_info normie) normie
token_value(token_info normie) tea
```
Extract token type and value information.

### Character Classification

```cursed
is_letter(ch sip) lit
is_digit(ch sip) lit
is_alphanumeric(ch sip) lit
is_whitespace(ch sip) lit
is_newline(ch sip) lit
is_hex_digit(ch sip) lit
```
Character classification functions optimized for CURSED syntax.

### Token Classification

```cursed
is_identifier(token_info normie) lit
is_number(token_info normie) lit
is_string(token_info normie) lit
is_keyword(token_info normie) lit
is_operator(token_type normie) lit
is_delimiter(token_type normie) lit
is_eof(token_info normie) lit
is_error(token_info normie) lit
```
Classify tokens by category for parser integration.

### Keyword Recognition

```cursed
recognize_keyword(ident tea) normie
```
Recognizes CURSED keywords and returns appropriate token type.

**Example:**
```cursed
sus token_type normie = recognize_keyword("sus")
# Returns SUS_TOKEN (60)

sus token_type normie = recognize_keyword("variable") 
# Returns IDENT_TOKEN (1) for non-keywords
```

### Scanner State Machine

```cursed
create_scanner(source tea) normie
advance_scanner(scanner normie, ch sip) normie
scanner_position(scanner normie) normie
scanner_line(scanner normie) normie
scanner_column(scanner normie) normie
```
State machine for incremental tokenization with position tracking.

### Token Stream Utilities

```cursed
create_token_stream(source tea) normie
token_stream_has_next(stream normie) lit
token_stream_peek(stream normie) normie
token_stream_next(stream normie) normie
```
Token stream interface for parser consumption.

### Error Recovery

```cursed
create_error_token(message tea, position normie) normie
error_token_message(error_token normie) tea
recover_from_error(source tea, error_pos normie) normie
```
Error handling and recovery mechanisms for robust parsing.

## Usage Examples

### Basic Tokenization

```cursed
yeet "token_vibe"

# Tokenize a simple CURSED program
sus source tea = "slay hello() { vibez.spill(\"Hello, CURSED!\") }"
sus token_count normie = tokenize(source)
vibez.spill("Found " + string.from_int(token_count) + " tokens")
```

### Token Classification

```cursed
yeet "token_vibe"

# Create and classify tokens
sus pos normie = create_position("test.💀", 1, 5, 10)
sus ident_token normie = create_token_info(IDENT_TOKEN, "variable", pos, "variable")
sus keyword_token normie = create_token_info(SUS_TOKEN, "sus", pos, "sus")

lowkey is_identifier(ident_token) {
    vibez.spill("Found identifier token")
}

lowkey is_keyword(keyword_token) {
    vibez.spill("Found keyword token")
}
```

### Position Tracking

```cursed
yeet "token_vibe"

# Track token positions for error reporting
sus pos normie = create_position("main.💀", 15, 23, 342)
sus pos_str tea = position_string(pos)
vibez.spill("Token position: " + pos_str)
```

### Error Handling

```cursed
yeet "token_vibe"

# Handle tokenization errors
sus error_pos normie = create_position("bad.💀", 5, 10, 50)
sus error_token normie = create_error_token("Invalid character '@'", error_pos)

lowkey is_error(error_token) {
    sus message tea = error_token_message(error_token)
    vibez.spill("Tokenization error: " + message)
}
```

### Scanner Integration

```cursed
yeet "token_vibe"

# Use scanner for incremental processing
sus scanner normie = create_scanner("sus x normie = 42")
sus current_pos normie = scanner_position(scanner)
sus current_line normie = scanner_line(scanner)

# Advance scanner through characters
sus new_scanner normie = advance_scanner(scanner, 's')
```

## Self-Hosting Integration

The `token_vibe` module is designed specifically for the CURSED self-hosting compiler:

### Lexical Analysis Pipeline

1. **Source Input** → `tokenize()` → **Token Count**
2. **Token Stream** → `create_token_stream()` → **Parser Input**
3. **Error Recovery** → `recover_from_error()` → **Robust Parsing**
4. **Position Tracking** → Debug information and error reporting

### Compiler Integration Points

- **Lexer Frontend**: `tokenize()` and character classification functions
- **Parser Input**: Token stream utilities and token classification
- **Error Reporting**: Position tracking and error token creation
- **Debug Information**: Position strings and token type conversion

### Performance Characteristics

- **State Machine**: Efficient single-pass tokenization
- **Memory Usage**: Compact token encoding minimizes memory overhead
- **Error Recovery**: Graceful handling of invalid input without crashes
- **Position Tracking**: Minimal overhead for comprehensive debugging info

## Testing

Comprehensive test suite in `test_token_vibe.💀` covers:

- **102+ token type constants** validation
- **Character classification** edge cases
- **Position tracking** accuracy
- **Token stream** functionality
- **Error recovery** robustness
- **Keyword recognition** completeness
- **Large input** performance
- **Integration** with CURSED syntax

Run tests with:
```bash
cargo run --bin cursed stdlib/token_vibe/test_token_vibe.💀
```

## Architecture

### Token Encoding

Tokens use compact integer encoding:
- **Token Info**: `type(8) | value_hash(24) | position_ref(32)`
- **Position**: `filename_hash(16) | line(16) | column(16) | offset(16)`
- **Scanner State**: `position(16) | line(8) | column(8)`

### State Machine Design

The scanner implements a finite state machine with:
- **Character-by-character** processing
- **Lookahead** for multi-character operators
- **Error recovery** at character boundaries
- **Position advancement** with newline handling

### Memory Management

- **No dynamic allocation** - uses integer encoding
- **Constant memory usage** regardless of input size
- **Garbage collection friendly** - no heap allocations
- **Cache efficient** - compact data structures

## Contributing

When extending the tokenizer:

1. **Add token constants** in the appropriate range (0-99)
2. **Update token_string()** for string conversion
3. **Add keyword recognition** in `recognize_keyword()`
4. **Include classification** in appropriate `is_*()` functions
5. **Add comprehensive tests** for new functionality

## Future Enhancements

- **Incremental tokenization** for IDE integration
- **Token caching** for performance optimization
- **Multi-byte character** support for Unicode identifiers
- **Preprocessor integration** for macro expansion
- **Parallel tokenization** for large files

---

**Version**: 1.0  
**Status**: Production Ready  
**Dependencies**: Pure CURSED (no FFI)  
**Self-Hosting**: Essential for lexical analysis  
**Test Coverage**: 35+ comprehensive test functions
