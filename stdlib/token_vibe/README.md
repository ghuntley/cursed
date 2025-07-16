# token_vibe Module

Advanced lexical scanning and tokenization module for CURSED language. Provides comprehensive tokenization functionality essential for compiler self-hosting capabilities.

## Overview

The `token_vibe` module implements a complete lexical analyzer that breaks source code into tokens, handles different token types, tracks position information, and provides advanced scanning features. This module is critical for the CURSED compiler's self-hosting capability.

## Key Features

- **Complete Token Types**: Supports all CURSED language tokens (identifiers, numbers, strings, operators, keywords)
- **Position Tracking**: Accurate line and column position tracking for error reporting
- **Flexible Scanning Modes**: Configurable scanning modes for different use cases
- **Error Handling**: Robust error reporting with custom error handlers
- **Stream Processing**: Advanced token stream filtering and mapping capabilities
- **Pure CURSED Implementation**: No FFI dependencies, fully self-hosting compatible

## Core Types

### Token Types
```cursed
facts Token {
    EOF = 0, IDENT = 1, INT = 2, FLOAT = 3, CHAR = 4, STRING = 5, COMMENT = 6
    ADD = 10, SUB = 11, MUL = 12, DIV = 13, MOD = 14  # Operators
    EQL = 20, NEQ = 21, LSS = 22, LEQ = 23, GTR = 24, GEQ = 25  # Comparisons
    ASSIGN = 30, NOT = 31, LPAREN = 32, RPAREN = 33  # Other tokens
    # ... and more
}
```

### Position Tracking
```cursed
vibe Position {
    filename tea    # Source filename
    offset normie   # Byte offset
    line normie     # Line number (1-based)
    column normie   # Column number (1-based)
}
```

### Token Information
```cursed
vibe TokenInfo {
    token normie      # Token type
    text tea          # Token text
    pos Position      # Position in source
    value tea         # Token value
}
```

### Scanner
```cursed
vibe Scanner {
    source tea             # Source code text
    pos normie            # Current position
    line normie           # Current line
    column normie         # Current column
    current_char sip      # Current character
    mode normie           # Scanning mode
    error_count normie    # Number of errors
    # ... internal state
}
```

## Basic Usage

### Simple Tokenization
```cursed
yeet "token_vibe"

# Tokenize source code
sus source tea = "sus x normie = 42"
sus tokens [token_vibe.TokenInfo] = token_vibe.tokenize(source)

bestie i := 0; i < collections.length(tokens); i++ {
    sus token token_vibe.TokenInfo = tokens[i]
    vibez.spill("Token: %s, Text: %s", 
        token_vibe.token_string(token_vibe.token_type(token)),
        token_vibe.token_value(token))
}
```

### Scanner Usage
```cursed
# Create and configure scanner
sus scanner token_vibe.Scanner = token_vibe.create_scanner(source)
scanner = token_vibe.set_scanner_mode(scanner, token_vibe.ScanModePresets.ScanAll)

# Scan tokens one by one
bestie based {
    sus token normie = token_vibe.scan_token(scanner)
    lowkey token == token_vibe.Token.EOF { ghosted }
    
    sus text tea = token_vibe.token_text(scanner)
    sus pos token_vibe.Position = token_vibe.current_position(scanner)
    vibez.spill("Token: %s at %s", text, token_vibe.position_string(pos))
}
```

## Advanced Features

### Scanning Modes
```cursed
# Different scanning modes
facts ScanMode {
    ScanIdents = 1      # Scan identifiers
    ScanInts = 2        # Scan integers
    ScanFloats = 4      # Scan floats
    ScanStrings = 16    # Scan strings
    ScanComments = 32   # Scan comments
    SkipComments = 128  # Skip comments
}

# Preset combinations
sus all_tokens = token_vibe.ScanModePresets.ScanAll
sus skip_comments = token_vibe.ScanModePresets.ScanAll | token_vibe.ScanMode.SkipComments
```

### Error Handling
```cursed
# Set custom error handler
scanner = token_vibe.set_error_handler(scanner, slay(pos token_vibe.Position, msg tea) {
    vibez.spill("ERROR at %s: %s", token_vibe.position_string(pos), msg)
})

# Check error count
sus error_count normie = token_vibe.get_error_count(scanner)
```

### Token Stream Processing
```cursed
# Create token stream
sus stream token_vibe.TokenStream = token_vibe.create_token_stream(scanner)

# Filter tokens (remove whitespace)
sus filtered = token_vibe.filter_token_stream(stream, slay(info token_vibe.TokenInfo) lit {
    damn token_vibe.token_type(info) != token_vibe.Token.WHITESPACE
})

# Map tokens (add prefix to operators)
sus mapped = token_vibe.map_token_stream(stream, slay(info token_vibe.TokenInfo) token_vibe.TokenInfo {
    lowkey token_vibe.is_operator(token_vibe.token_type(info)) {
        damn token_vibe.create_token_info(
            token_vibe.token_type(info),
            string.concat("OP:", token_vibe.token_value(info)),
            token_vibe.token_position(info),
            token_vibe.token_value(info)
        )
    }
    damn info
})
```

## Core Functions

### Token Type Functions
- `token_string(tok normie) tea` - Convert token type to string
- `is_operator(tok normie) lit` - Check if token is an operator
- `token_type(info TokenInfo) normie` - Get token type from TokenInfo
- `token_value(info TokenInfo) tea` - Get token value from TokenInfo

### Position Functions
- `create_position(filename tea, offset normie, line normie, column normie) Position`
- `position_is_valid(pos Position) lit` - Check if position is valid
- `position_string(pos Position) tea` - Convert position to string

### Scanner Functions
- `create_scanner(source tea) Scanner` - Create new scanner
- `init_scanner(scanner Scanner, source tea) Scanner` - Initialize scanner
- `set_scanner_mode(scanner Scanner, mode normie) Scanner` - Set scanning mode
- `scan_token(scanner Scanner) normie` - Scan next token
- `token_text(scanner Scanner) tea` - Get current token text
- `current_position(scanner Scanner) Position` - Get current position

### Character Functions
- `peek_char(scanner Scanner) sip` - Peek at current character
- `next_char(scanner Scanner) sip` - Advance to next character
- `peek_next_char(scanner Scanner) sip` - Peek at next character

### Convenience Functions
- `tokenize(source tea) [TokenInfo]` - Tokenize entire source
- `scan_with_mode(source tea, mode normie) [TokenInfo]` - Scan with specific mode
- `is_eof(info TokenInfo) lit` - Check if token is EOF
- `is_identifier(info TokenInfo) lit` - Check if token is identifier
- `is_number(info TokenInfo) lit` - Check if token is number
- `is_string(info TokenInfo) lit` - Check if token is string
- `is_comment(info TokenInfo) lit` - Check if token is comment

## CURSED Language Support

The tokenizer recognizes all CURSED language tokens:

### Keywords (as identifiers)
- `sus`, `slay`, `damn`, `yeet`, `vibe`, `facts`
- `lowkey`, `highkey`, `bestie`, `ghosted`, `simp`
- `based`, `cap`, `cringe`, `lit`, `yolo`, `ready`

### Types (as identifiers)
- `normie`, `drip`, `tea`, `thicc`, `smol`, `meal`
- `snack`, `mid`, `sip`, `byte`, `rune`, `extra`

### Operators
- Arithmetic: `+`, `-`, `*`, `/`, `%`
- Comparison: `==`, `!=`, `<`, `<=`, `>`, `>=`
- Logical: `&&`, `||`, `!`
- Bitwise: `&`, `|`, `^`, `<<`, `>>`
- Assignment: `=`, `:=`

### Delimiters
- Parentheses: `(`, `)`
- Brackets: `[`, `]`
- Braces: `{`, `}`
- Others: `,`, `.`, `:`, `;`

## Integration with Compiler Core

The token_vibe module integrates seamlessly with the compiler_core module:

```cursed
yeet "token_vibe"
yeet "compiler_core"

# Tokenize with token_vibe
sus tokens [token_vibe.TokenInfo] = token_vibe.tokenize(source)

# Convert to compiler_core format
sus core_tokens [compiler_core.Token] = []
bestie i := 0; i < collections.length(tokens); i++ {
    sus token_info token_vibe.TokenInfo = tokens[i]
    sus pos token_vibe.Position = token_vibe.token_position(token_info)
    
    sus core_token compiler_core.Token = compiler_core.create_token(
        map_token_type(token_vibe.token_type(token_info)),
        token_vibe.token_value(token_info),
        pos.line,
        pos.column,
        pos.offset
    )
    
    core_tokens = collections.append(core_tokens, core_token)
}
```

## Testing

Comprehensive test suite covers:
- Basic token type recognition
- Position tracking accuracy
- Scanner state management
- Error handling and reporting
- Stream processing functionality
- Integration with compiler_core
- Complex expression tokenization
- CURSED language specific features

Run tests:
```bash
cargo run --bin cursed stdlib/token_vibe/test_token_vibe.csd
```

## Performance Characteristics

- **Efficient**: Single-pass character-by-character scanning
- **Memory Efficient**: Minimal memory allocation during scanning
- **Accurate**: Precise position tracking for error reporting
- **Extensible**: Easy to add new token types and scanning modes
- **Self-Hosting**: Pure CURSED implementation without FFI dependencies

## Self-Hosting Benefits

- **No External Dependencies**: Pure CURSED implementation
- **Complete Feature Set**: All tokenization features needed for compiler
- **Integration Ready**: Designed to work with compiler_core module
- **Error Reporting**: Comprehensive error handling for debugging
- **Performance**: Optimized for compiler workloads

## Module Status

The token_vibe module is production-ready and essential for CURSED compiler self-hosting. It provides all necessary tokenization functionality with comprehensive error handling and performance optimization.

```cursed
vibez.spill(token_vibe.token_vibe_status())
# Output: "token_vibe module loaded - advanced tokenization ready for self-hosting"
```
