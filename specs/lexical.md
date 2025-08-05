# CURSED Lexical Structure

This document defines the lexical elements of the CURSED programming language, including tokens, keywords, and syntactic structure.

## Characters and Encoding

CURSED source code is Unicode text encoded in UTF-8. Source files are assumed to be in UTF-8, with no byte order mark (BOM).

## Tokens

CURSED code consists of the following tokens:

- Identifiers
- Keywords
- Operators and punctuation
- Literals (numeric, string, etc.)
- Comments

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

### Floating-Point Literals

Floating-point literals consist of decimal digits, a decimal point, and optionally an exponent part:
- `3.14159`
- `1.0e10`
- `.5`
- `1.`

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

### Boolean Literals

- `based` (true) - CANONICAL
- `cringe` (false) - CANONICAL

### Nil Literal

- `nah` (nil) - CANONICAL

### Deprecated Literals (LEGACY COMPATIBILITY)

The following literals are deprecated and SHOULD NOT be used in new code:
- `cap` (formerly false) - Use `cringe` instead
- `yolo` (formerly return) - Use `damn` instead

**PARSER REQUIREMENT**: Parsers MAY emit deprecation warnings for these tokens.