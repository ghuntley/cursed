# CURSED Tree-sitter Integration

## Integration with Main Compiler

The tree-sitter grammar can be integrated with the main CURSED compiler for:

1. **IDE Language Server**: Provide syntax highlighting, code completion, and error detection
2. **Code Formatting**: Use the AST for consistent code formatting
3. **Syntax Validation**: Quick syntax checking during development
4. **Code Analysis**: Static analysis and linting tools

## Usage in Rust

Add to your `Cargo.toml`:

```toml
[dependencies]
tree-sitter = "0.20"
```

Create bindings:

```rust
use tree_sitter::{Language, Parser};

extern "C" { fn tree_sitter_cursed() -> Language; }

pub fn main() {
    let language = unsafe { tree_sitter_cursed() };
    let mut parser = Parser::new();
    parser.set_language(language).expect("Error loading CURSED grammar");
    
    let source_code = r#"
        vibe main
        
        slay main() {
            vibez.spill("Hello, CURSED!")
        }
    "#;
    
    let tree = parser.parse(source_code, None).unwrap();
    println!("{}", tree.root_node().to_sexp());
}
```

## Integration Commands

```bash
# Generate parser from grammar
cd tree-sitter
npx tree-sitter generate

# Test the grammar
npx tree-sitter test

# Build Node.js bindings
npm run build

# Build C/Rust bindings
tree-sitter build-wasm

# Parse a file
npx tree-sitter parse examples/hello.csd
```

## IDE Integration

### VS Code Extension

The included VS Code extension provides:
- Syntax highlighting using tree-sitter
- Code snippets for CURSED constructs
- Auto-completion for keywords
- Bracket matching and folding

Install the extension:
```bash
cd tree-sitter/vscode-extension
npm install
npm run compile
# Press F5 to launch extension development host
```

### Language Server Protocol

For full IDE integration, implement an LSP server using the tree-sitter parser:

```rust
use tower_lsp::{LspService, Server};
use tree_sitter::{Language, Parser, Tree};

extern "C" { fn tree_sitter_cursed() -> Language; }

struct CursedLanguageServer {
    parser: Parser,
}

impl CursedLanguageServer {
    fn new() -> Self {
        let language = unsafe { tree_sitter_cursed() };
        let mut parser = Parser::new();
        parser.set_language(language).unwrap();
        
        Self { parser }
    }
    
    fn parse_document(&mut self, content: &str) -> Option<Tree> {
        self.parser.parse(content, None)
    }
}
```

## Integration Benefits

1. **Fast Parsing**: Tree-sitter provides incremental parsing for fast updates
2. **Error Recovery**: Robust error recovery for incomplete code
3. **Language Agnostic**: Works with any editor that supports tree-sitter
4. **Extensible**: Easy to add new language features
5. **Performance**: Optimized for large files and real-time editing

## Maintenance

Keep the grammar synchronized with the main CURSED compiler:

1. Update `grammar.js` when adding new language features
2. Add test cases in `test/corpus/` for new syntax
3. Update highlighting queries in `queries/highlights.scm`
4. Test integration with `npx tree-sitter test`
5. Regenerate parser with `npx tree-sitter generate`

This ensures consistent parsing between the tree-sitter grammar and the main compiler.
