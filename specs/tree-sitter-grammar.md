# CURSED Tree-Sitter Grammar Specification

This document describes the Tree-Sitter grammar for the CURSED programming language. Tree-Sitter is a parser generator tool and incremental parsing library that can build a concrete syntax tree for source files and efficiently update the syntax tree as the source file is edited.

## Overview

The CURSED Tree-Sitter grammar provides a formal syntax definition for parsing CURSED source code. It enables features like:

- Syntax highlighting
- Code folding
- Incremental parsing for editor integration
- Structural code navigation
- Query-based analysis and transformation of source code

## Grammar Structure

The Tree-Sitter grammar for CURSED is defined in JavaScript and consists of several components:

1. **Rules**: Defines the syntactic patterns of the language
2. **Extras**: Defines elements like whitespace and comments that can appear anywhere
3. **Precedence**: Defines operator precedence for resolving ambiguities
4. **Conflict Resolution**: Handles potential parsing conflicts

## Key Components

### Source File Structure

A CURSED source file follows this structure:

```javascript
source_file: $ => seq(
  $.package_clause,
  optional(';'),
  repeat(seq($.import_declaration, optional(';'))),
  repeat(seq($.top_level_declaration, optional(';'))),
)
```

This represents the overall file structure with a package declaration, followed by optional imports and top-level declarations.

### Comments

CURSED supports two types of comments:

```javascript
line_comment: $ => token(seq('fr fr', /.*/)),

block_comment: $ => token(seq(
  '/* cap',
  /([^c]|c[^a]|ca[^p]|cap[^\s]|cap\s[^*]|cap\s\*[^/])*/,
  'cap */'
))
```

Line comments start with `fr fr` and continue until the end of the line. Block comments start with `/* cap` and end with `cap */`.

### Declarations

Declarations include package, import, function, type, variable, and constant declarations:

```javascript
function_declaration: $ => seq(
  'slay',
  $.identifier,
  optional($.type_parameters),
  $.parameter_list,
  optional($.return_type),
  optional($.block)
)
```

The example above shows function declarations which start with the `slay` keyword, followed by a function name, optional type parameters, parameter list, optional return type, and function body.

### Types

Types include primitive types, arrays, slices, maps, structs, interfaces, pointers, and more:

```javascript
type: $ => choice(
  $.type_name,
  $.pointer_type,
  $.array_type,
  $.struct_type,
  $.interface_type,
  $.slice_type,
  $.map_type,
  $.channel_type,
  $.function_type,
  $.parametrized_type,
)
```

### Statements

Statements include declarations, assignments, control flow, and more:

```javascript
statement: $ => choice(
  $.declaration,
  $.simple_statement,
  $.return_statement,
  $.break_statement,
  $.continue_statement,
  $.block,
  $.if_statement,
  $.switch_statement,
  $.for_statement,
  $.while_statement,
  $.defer_statement,
  $.go_statement,
)
```

### Expressions

Expressions include literals, operators, function calls, and more:

```javascript
expression: $ => choice(
  $.unary_expression,
  $.binary_expression,
  $.primary_expression,
)
```

## Token Recognition

The grammar defines several token types for recognizing language elements:

### Literals

```javascript
int_literal: $ => token(choice(
  /[0-9]+/,            // decimal
  /0[oO][0-7]+/,       // octal
  /0[xX][0-9a-fA-F]+/, // hex
  /0[bB][01]+/         // binary
))

float_literal: $ => token(choice(
  /[0-9]+(\.[0-9]+)?([eE][\+\-]?[0-9]+)?/,
  /\.[0-9]+([eE][\+\-]?[0-9]+)?/
))

string_literal: $ => choice(
  seq('"', repeat(choice(/[^"\\\n]/, $.escape_sequence)), '"'),
  seq('`', /[^`]*/, '`')
)
```

### Identifiers

```javascript
identifier: $ => /[_a-zA-Z][_a-zA-Z0-9]*/,
```

## Query Files

The CURSED Tree-Sitter grammar includes several query files that enhance editor integration:

1. **highlights.scm**: Defines syntax highlighting rules for language elements
2. **folds.scm**: Specifies code folding behavior for blocks and structures
3. **indents.scm**: Controls automatic indentation rules
4. **locals.scm**: Handles variable scoping and reference tracking
5. **injections.scm**: Enables language injection for embedded languages (e.g., regex in strings)
6. **textobjects.scm**: Provides text objects for easy code navigation and selection

## Usage

The CURSED Tree-Sitter grammar can be used in several ways:

1. **Syntax Highlighting**: Editors like VS Code, Atom, and Neovim can use the grammar for semantic highlighting.
2. **Code Analysis**: Tools can traverse the syntax tree to analyze code structure.
3. **Refactoring Tools**: The grammar enables structural understanding for safe refactoring.
4. **Language Servers**: It can serve as a foundation for a Language Server Protocol implementation.
5. **Navigation**: The textobjects query file enables structural code navigation and selection.
6. **Intelligent Editing**: The indentation rules provide smart auto-indentation.

## Testing

The grammar includes test cases in the `corpus` directory that verify parsing behavior for various language constructs.

## Installation

To use the Tree-Sitter grammar:

```bash
npm install tree-sitter-cursed
```

Or clone the repository and build from source:

```bash
git clone https://github.com/cursed-lang/tree-sitter-cursed.git
cd tree-sitter-cursed
npm install
npm test
```

## Integration

### With Node.js

```javascript
const Parser = require('tree-sitter');
const Cursed = require('tree-sitter-cursed');

const parser = new Parser();
parser.setLanguage(Cursed);

const sourceCode = 'vibe main\n\nslay main() {\n  yolo 0\n}';
const tree = parser.parse(sourceCode);
console.log(tree.rootNode.toString());
```

### With VS Code

The grammar can be used in a VS Code extension to provide syntax highlighting and other features for CURSED files.

## Limitations and Future Work

- The current grammar doesn't implement semantic analysis - it only handles syntax.
- Support for upcoming CURSED language features will require grammar updates.
- Additional query files for more advanced highlighting and navigation could be added.

## Conclusion

The Tree-Sitter grammar provides a solid foundation for tooling around the CURSED programming language, enabling rich editor integration and code analysis capabilities.