# tree-sitter-cursed

Tree-sitter grammar for the CURSED programming language.

## About CURSED

CURSED is an esoteric programming language that combines the structure and efficiency of Go with the contemporary linguistic flair of Generation Z slang. It replaces traditional programming keywords with Gen Z slang terms while maintaining a familiar syntax structure.

## Features

This grammar provides:

- Complete syntax highlighting for CURSED files (`.csd` extension)
- Code folding for blocks, functions, and other structures
- Intelligent indentation rules
- Local variable tracking and scope analysis
- Support for text objects (functions, classes, parameters, etc.)
- Language injections for regex, SQL, and JSON in strings
- Support for all CURSED language constructs including:
  - Gen Z slang keywords (`vibe`, `slay`, `yeet`, etc.)
  - Specialized comments (`fr fr` for line comments, `no cap`/`on god` for block comments)
  - All control structures and expression types
  - Support for generic types and functions

## Installation

### As a dependency

```bash
npm install tree-sitter-cursed
```

### Building from source

```bash
git clone https://github.com/cursed-lang/tree-sitter-cursed.git
cd tree-sitter-cursed
npm install
npm run build
npm test
```

## Usage

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

### With Neovim

Add to your Neovim configuration:

```lua
local parser_config = require "nvim-treesitter.parsers".get_parser_configs()
parser_config.cursed = {
  install_info = {
    url = "https://github.com/cursed-lang/tree-sitter-cursed",
    files = {"src/parser.c"},
    branch = "main",
  },
  filetype = "csd",
}
```

### With other editors

Most editors that support Tree-sitter can be configured to use this grammar. Refer to your editor's documentation for specifics.

## Examples

Examples of CURSED code can be found in the `examples/` directory.

## Development

### Generating the parser

```bash
npx tree-sitter generate
```

### Testing

```bash
npx tree-sitter test
```

### Playground (visual parsing)

```bash
npx tree-sitter playground
```

## License

MIT