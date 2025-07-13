# tree-sitter-cursed

Tree-sitter grammar for the CURSED programming language.

## Overview

CURSED is a Go-inspired programming language that uses Gen Z slang keywords for a more expressive and fun coding experience. This tree-sitter grammar provides syntax highlighting, error recovery, and incremental parsing for CURSED code.

## Features

- **Complete syntax support** for all CURSED language constructs
- **Gen Z slang keywords** with proper highlighting:
  - `slay` - function declaration
  - `lowkey`/`highkey` - if/else statements
  - `vibe_check` - switch statements
  - `bestie` - for loops
  - `periodt` - while loops
  - `stan` - goroutines
  - `dm` - channels
  - `facts` - constants
  - `sus` - variables
  - `based`/`cap` - boolean literals
  - `cringe` - nil literal
- **Error recovery** for robust parsing
- **Incremental parsing** for fast IDE integration
- **VS Code extension** with syntax highlighting and snippets

## Language Keywords

### Control Flow
- `lowkey` - if statement
- `highkey` - else statement  
- `vibe_check` - switch statement
- `mood` - case clause
- `basic` - default clause
- `bestie` - for loop
- `flex` - range iteration
- `periodt` - while loop
- `ready` - select statement
- `yolo` - return statement
- `ghosted` - break statement
- `simp` - continue statement

### Declarations
- `vibe` - package declaration
- `yeet` - import declaration
- `facts` - constant declaration
- `sus` - variable declaration
- `be_like` - type declaration
- `slay` - function declaration
- `squad` - struct type
- `vibes` - interface type

### Types
- `normie` - i32 integer
- `smol` - i8 integer
- `mid` - i16 integer
- `thicc` - i64 integer
- `drip` - f32 float
- `snack` - f32 float
- `meal` - f64 float
- `byte` - u8 unsigned integer
- `rune` - i32 character
- `extra` - complex number
- `tea` - string
- `lit` - boolean
- `sip` - character

### Concurrency
- `stan` - spawn goroutine
- `dm<T>` - channel type
- `later` - defer statement
- `ready` - select statement

### Literals
- `based` - true
- `cap` - false
- `cringe` - nil

## Installation

### As Node.js Package

```bash
npm install tree-sitter-cursed
```

### Manual Installation

1. Clone the repository
2. Install tree-sitter CLI: `npm install -g tree-sitter-cli`
3. Generate parser: `tree-sitter generate`
4. Test the grammar: `tree-sitter test`

## Usage

### With Node.js

```javascript
const Parser = require('tree-sitter');
const Cursed = require('tree-sitter-cursed');

const parser = new Parser();
parser.setLanguage(Cursed);

const code = `
vibe main

slay main() {
    vibez.spill("Hello, CURSED!")
}
`;

const tree = parser.parse(code);
console.log(tree.rootNode.toString());
```

### With VS Code

1. Install the CURSED Language Support extension
2. Open a `.csd` file
3. Enjoy syntax highlighting and code snippets

## Grammar Structure

The grammar follows tree-sitter conventions and includes:

- **Lexical analysis** with proper token recognition
- **Syntactic analysis** with AST generation
- **Error recovery** for incomplete or malformed code
- **Incremental parsing** for efficient re-parsing
- **Highlighting queries** for syntax coloring
- **Locals queries** for scope analysis

## Test Cases

The grammar includes comprehensive test cases covering:

- Basic syntax (variables, functions, control flow)
- Advanced features (goroutines, channels, interfaces)
- Error cases and recovery
- Edge cases and corner cases

Run tests with:
```bash
tree-sitter test
```

## Development

### Building

```bash
# Generate parser
npx tree-sitter generate

# Build bindings  
npm run build

# Run tests
npx tree-sitter test

# Parse a file
npx tree-sitter parse examples/hello.csd
```

### Adding New Features

1. Update `grammar.js` with new rules
2. Add test cases in `test/corpus/`
3. Update highlighting queries in `queries/highlights.scm`
4. Update locals queries in `queries/locals.scm`
5. Run `npx tree-sitter generate` and `npx tree-sitter test`

## VS Code Extension

The included VS Code extension provides:

- **Syntax highlighting** using the tree-sitter grammar
- **Code snippets** for common CURSED constructs
- **Auto-completion** for keywords and types
- **Bracket matching** and **auto-closing**
- **Comment toggling** and **code folding**

### Installing the Extension

1. Navigate to `vscode-extension/`
2. Run `npm install`
3. Run `npm run compile`
4. Press F5 to launch extension development host
5. Open a `.csd` file to test

## Contributing

1. Fork the repository
2. Create a feature branch
3. Add tests for new features
4. Ensure all tests pass
5. Submit a pull request

## License

MIT License - see LICENSE file for details.

## Examples

### Hello World
```cursed
vibe main

slay main() {
    vibez.spill("Hello, CURSED!")
}
```

### Variables and Types
```cursed
vibe main

sus name tea = "World"
sus age normie = 25
sus height meal = 180.5
sus isActive lit = based
```

### Control Flow
```cursed
slay checkAge(age normie) tea {
    lowkey age >= 18 {
        yolo "Adult"
    } highkey lowkey age >= 13 {
        yolo "Teen"
    } highkey {
        yolo "Child"
    }
}
```

### Goroutines and Channels
```cursed
slay worker(ch dm<normie>) {
    bestie i := 0; i < 10; i++ {
        ch <- i
    }
    close(ch)
}

slay main() {
    sus ch dm<normie>
    stan worker(ch)
    
    bestie value := flex ch {
        vibez.spill(value)
    }
}
```

### Structs and Interfaces
```cursed
be_like Person squad {
    name tea
    age normie
}

be_like Speaker vibes {
    speak() tea
}

slay (p Person) speak() tea {
    yolo "Hello, I'm " + p.name
}
```

This tree-sitter grammar provides comprehensive support for the CURSED language, enabling rich IDE integration and tooling support.
