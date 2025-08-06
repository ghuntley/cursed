# CURSED Lexical Structure

This document defines the lexical elements of the CURSED programming language, including tokens, keywords, and syntactic structure.

## Characters and Encoding

CURSED source code is Unicode text encoded in UTF-8. Source files are assumed to be in UTF-8, with no byte order mark (BOM).

## Tokens

CURSED code consists of the following token categories:

- `IDENTIFIER` - Identifiers for variables, functions, types
- `KEYWORD` - Reserved language keywords
- `OPERATOR` - Operators and punctuation symbols
- `int_lit` - Integer literals
- `float_lit` - Floating-point literals
- `string_lit` - String literals  
- `bool_lit` - Boolean literals (`based`, `cringe`)
- `nil_lit` - Nil literal (`nah`)
- `char_lit` - Character literals
- `@` - At symbol for pointer types
- `COMMENT` - Comments (ignored by parser)

Whitespace characters (spaces, tabs, newlines) separate tokens but are otherwise ignored.

## Comments

CURSED supports multiple comment styles:

- Line comments start with `fr fr` and continue until the end of the line
- Line comments can also use `#` (for compatibility)
- Block comments start with `no cap` and end with `on god`

```
fr fr This is a line comment
# This is also a line comment

no cap
This is a block comment
that spans multiple lines
on god
```

## Identifiers

Identifiers name program entities such as variables and types. An identifier is a sequence of one or more letters, digits, and underscores, with the first character not being a digit.

```
validIdentifier
_also_valid
invalid2Identifier  fr fr Valid in CURSED
```

## Keywords

Traditional programming keywords are replaced with Gen Z slang. Here's the mapping:

| Go Keyword | CURSED Keyword |
|------------|---------------|
| package    | vibe          |
| import     | yeet          |
| func       | slay          |
| return     | damn / yolo   |
| var        | sus           |
| const      | facts         |
| if         | lowkey        |
| else       | highkey       |
| for        | bestie        |
| while      | periodt       |
| switch     | vibe_check    |
| case       | mood          |
| default    | basic         |
| break      | ghosted       |
| continue   | simp          |
| type       | be_like       |
| struct     | squad         |
| interface  | collab        |
| map        | map         |
| chan       | dm            |
| go         | stan          |
| range      | flex          |
| defer      | later         |
| select     | ready         |
| true       | based         |
| false      | cringe        |
| nil        | nah           |
| panic      | shook         |
| recover    | fam           |

## Operators and Punctuation

| Operator/Punctuation | Description                      |
|----------------------|----------------------------------|
| +                    | Addition                         |
| -                    | Subtraction                      |
| *                    | Multiplication                   |
| /                    | Division                         |
| %                    | Remainder                        |
| &                    | Bitwise AND                      |
| \|                   | Bitwise OR                       |
| ^                    | Bitwise XOR                      |
| <<                   | Left shift                       |
| >>                   | Right shift                      |
| ==                   | Equal                            |
| !=                   | Not equal                        |
| <                    | Less than                        |
| <=                   | Less than or equal               |
| >                    | Greater than                     |
| >=                   | Greater than or equal            |
| =                    | Assignment                       |
| :=                   | Short variable declaration       |
| ...                  | Ellipsis (variadic parameters)   |
| +=, -=, etc.         | Assignment operators             |
| &&                   | Logical AND                      |
| \|\|                 | Logical OR                       |
| !                    | Logical NOT                      |
| <-                   | Channel send/receive             |
| ()                   | Function call, grouping          |
| []                   | Array/slice indexing             |
| {}                   | Block statement, struct literal  |
| ,                    | Separator                        |
| ;                    | Statement terminator             |
| .                    | Member selection                 |
| :                    | Type declaration                 |

## Literals

### Integer Literals

Integer literals can be written as:
- Decimal: `123`
- Octal: `0o173`
- Hexadecimal: `0xAB`
- Binary: `0b1010`

**Formal Definition**:
```regex
int_lit     = decimal_lit | octal_lit | hex_lit | binary_lit
decimal_lit = [1-9][0-9]*
octal_lit   = "0o"[0-7]+
hex_lit     = "0x"[0-9a-fA-F]+
binary_lit  = "0b"[01]+
```

### Floating-Point Literals

Floating-point literals consist of decimal digits, a decimal point, and optionally an exponent part:
- `3.14159`
- `1.0e10`
- `.5`
- `1.`

**Formal Definition**:
```regex
float_lit = decimal_float | hex_float
decimal_float = [0-9]*"."[0-9]*([eE][+-]?[0-9]+)?
              | [0-9]+[eE][+-]?[0-9]+
hex_float = "0x"[0-9a-fA-F]*"."[0-9a-fA-F]*([pP][+-]?[0-9]+)?
          | "0x"[0-9a-fA-F]+[pP][+-]?[0-9]+
```

### String Literals

String literals are created using double quotes `"` or backticks `` ` ``:
- `"hello world"`
- `` `multiline
   string` ``

Escape sequences in quoted strings:
- `\n` - newline
- `\t` - tab
- `\\` - backslash
- `\'` - single quote
- `\"` - double quote

**Formal Definition**:
```regex
string_lit     = raw_string_lit | interpreted_string_lit
raw_string_lit = "`" { unicode_char | newline } "`"
interpreted_string_lit = "\"" { unicode_value | byte_value } "\""
unicode_value  = unicode_char | little_u_value | big_u_value | escaped_char
escaped_char   = "\\" ( "a" | "b" | "f" | "n" | "r" | "t" | "v" | "\\" | "'" | "\"" )
```

### Character Literals

Character literals represent single Unicode characters enclosed in single quotes:
- `'a'`
- `'\\n'`
- `'\\u0041'`

**Formal Definition**:
```regex
char_lit = "'" ( unicode_value | byte_value ) "'"
```

### Boolean Literals

- `based` (true) - CANONICAL
- `cringe` (false) - CANONICAL

**Formal Definition**:
```regex
bool_lit = "based" | "cringe"
```

### Nil Literal

- `nah` (nil) - CANONICAL

**Formal Definition**:
```regex
nil_lit = "nah"
```

### Deprecated Literals (LEGACY COMPATIBILITY)

The following literals are deprecated and SHOULD NOT be used in new code:
- `cap` (formerly false) - Use `cringe` instead
- `yolo` (formerly return) - Use `damn` instead

**PARSER REQUIREMENT**: Parsers MAY emit deprecation warnings for these tokens.

## Semicolon Insertion

CURSED follows automatic semicolon insertion rules similar to Go:

1. When the input is broken into tokens, a semicolon is automatically inserted into the token stream immediately after a line's final token if that token is:
   - An identifier
   - An integer, floating-point, character, or string literal  
   - One of the keywords: `damn`, `ghosted`, `simp`, `based`, `cringe`, `nah`
   - One of the operators and punctuation: `++`, `--`, `)`, `]`, `}`

2. To allow complex statements to occupy a single line, a semicolon may be omitted before a closing `)` or `}`.

3. A semicolon is never inserted automatically if it would separate two expression statements that could be interpreted as a single statement.

**Examples**:
```cursed
sus x normie = 42     // Semicolon inserted after 42
damn x + y           // Semicolon inserted after y

sus arr = []normie{
    1, 2, 3          // No semicolon inserted before }
}

lowkey x > 0 {       // No semicolon inserted before {
    vibez.spill(x)   // Semicolon inserted after )
}                    // Semicolon inserted after }
```