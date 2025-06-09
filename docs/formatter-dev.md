# CURSED Formatter Developer Guide 🧑‍💻

This guide covers the internal architecture of the CURSED formatter and how to extend it for new features.

## Architecture Overview

The CURSED formatter is built with a modular architecture that separates concerns:

```
cursed-fmt
├── Parser Integration    (Reuses existing CURSED parser)
├── AST Processing       (Walks and analyzes syntax tree)
├── Formatting Engine    (Applies formatting rules)
├── Configuration        (Manages user preferences)
├── Output Generation    (Produces formatted code)
└── CLI Interface        (Command-line tool)
```

## Core Components

### 1. Formatter Engine (`src/formatter/engine.rs`)

```rust
pub struct FormattingEngine {
    config: FormattingConfig,
    context: FormattingContext,
}

impl FormattingEngine {
    pub fn new(config: FormattingConfig) -> Self;
    pub fn format_file(&mut self, source: &str) -> Result<String, FormattingError>;
    pub fn format_ast(&mut self, ast: &AstNode) -> Result<String, FormattingError>;
}
```

### 2. Configuration System (`src/formatter/config.rs`)

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormattingConfig {
    pub formatting: FormattingOptions,
    pub comments: CommentOptions,
    pub newlines: NewlineOptions,
    pub strings: StringOptions,
}

impl FormattingConfig {
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self, ConfigError>;
    pub fn load_from_dir<P: AsRef<Path>>(dir: P) -> Result<Self, ConfigError>;
    pub fn merge(self, other: Self) -> Self;
}
```

### 3. AST Visitor Pattern (`src/formatter/visitor.rs`)

```rust
pub trait FormattingVisitor {
    fn visit_function(&mut self, func: &FunctionDeclaration) -> Result<(), FormattingError>;
    fn visit_expression(&mut self, expr: &Expression) -> Result<(), FormattingError>;
    fn visit_statement(&mut self, stmt: &Statement) -> Result<(), FormattingError>;
    fn visit_type(&mut self, ty: &Type) -> Result<(), FormattingError>;
}

pub struct DefaultFormattingVisitor {
    output: String,
    config: FormattingConfig,
    context: FormattingContext,
}
```

### 4. Context Tracking (`src/formatter/context.rs`)

```rust
#[derive(Debug, Clone)]
pub struct FormattingContext {
    pub indentation_level: usize,
    pub current_line_length: usize,
    pub in_function_signature: bool,
    pub in_struct_literal: bool,
    pub in_comment_block: bool,
    pub pending_newlines: usize,
}

impl FormattingContext {
    pub fn increase_indent(&mut self);
    pub fn decrease_indent(&mut self);
    pub fn reset_line_length(&mut self);
    pub fn should_break_line(&self, additional_length: usize) -> bool;
}
```

## Adding New Formatting Rules

### Step 1: Define Configuration Options

Add new options to `FormattingConfig`:

```rust
// In src/formatter/config.rs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormattingOptions {
    // ... existing fields ...
    
    /// Whether to align map keys vertically
    #[serde(default = "default_align_map_keys")]
    pub align_map_keys: bool,
}

fn default_align_map_keys() -> bool { false }
```

### Step 2: Implement the Formatting Logic

```rust
// In src/formatter/rules/map_formatting.rs
use crate::formatter::{FormattingVisitor, FormattingContext, FormattingConfig};

pub struct MapFormatter<'a> {
    config: &'a FormattingConfig,
    context: &'a mut FormattingContext,
    output: &'a mut String,
}

impl<'a> MapFormatter<'a> {
    pub fn format_map_literal(&mut self, map: &MapLiteral) -> Result<(), FormattingError> {
        if self.config.formatting.align_map_keys {
            self.format_aligned_map(map)
        } else {
            self.format_compact_map(map)
        }
    }
    
    fn format_aligned_map(&mut self, map: &MapLiteral) -> Result<(), FormattingError> {
        // Find the longest key for alignment
        let max_key_length = map.entries.iter()
            .map(|entry| self.calculate_key_length(&entry.key))
            .max()
            .unwrap_or(0);
            
        for entry in &map.entries {
            self.format_map_entry(entry, max_key_length)?;
        }
        
        Ok(())
    }
}
```

### Step 3: Integrate with Visitor

```rust
// In src/formatter/visitor.rs
impl FormattingVisitor for DefaultFormattingVisitor {
    fn visit_expression(&mut self, expr: &Expression) -> Result<(), FormattingError> {
        match expr {
            Expression::MapLiteral(map) => {
                let mut map_formatter = MapFormatter::new(
                    &self.config,
                    &mut self.context,
                    &mut self.output
                );
                map_formatter.format_map_literal(map)
            }
            // ... other expressions
        }
    }
}
```

### Step 4: Add Tests

```rust
// In tests/formatter/map_formatting_test.rs
#[cfg(test)]
mod tests {
    use super::*;
    use crate::formatter::FormattingConfig;
    
    #[test]
    fn test_aligned_map_formatting() {
        let source = r#"
sus person = map[string]string{
    "name": "Alice",
    "occupation": "Developer",
    "city": "NYC",
}
        "#;
        
        let expected = r#"
sus person = map[string]string{
    "name"       : "Alice",
    "occupation" : "Developer", 
    "city"       : "NYC",
}
        "#;
        
        let mut config = FormattingConfig::default();
        config.formatting.align_map_keys = true;
        
        let result = format_source(source, config).unwrap();
        assert_eq!(result.trim(), expected.trim());
    }
}
```

## Extension Points

### Custom Formatting Plugins

The formatter supports plugin-based extensions:

```rust
// src/formatter/plugins.rs
pub trait FormattingPlugin {
    fn name(&self) -> &str;
    fn handles(&self, node: &AstNode) -> bool;
    fn format(&self, node: &AstNode, context: &mut FormattingContext) -> Result<String, FormattingError>;
}

pub struct PluginRegistry {
    plugins: Vec<Box<dyn FormattingPlugin>>,
}

impl PluginRegistry {
    pub fn register<P: FormattingPlugin + 'static>(&mut self, plugin: P) {
        self.plugins.push(Box::new(plugin));
    }
    
    pub fn find_handler(&self, node: &AstNode) -> Option<&dyn FormattingPlugin> {
        self.plugins.iter()
            .find(|p| p.handles(node))
            .map(|p| p.as_ref())
    }
}
```

### Example Plugin Implementation

```rust
// Custom plugin for formatting function chains
pub struct FunctionChainPlugin;

impl FormattingPlugin for FunctionChainPlugin {
    fn name(&self) -> &str { "function_chain" }
    
    fn handles(&self, node: &AstNode) -> bool {
        matches!(node, AstNode::MethodCall(_)) && self.is_chain(node)
    }
    
    fn format(&self, node: &AstNode, context: &mut FormattingContext) -> Result<String, FormattingError> {
        // Custom formatting logic for method chains
        // e.g., break each call onto a new line
        Ok(formatted_chain)
    }
}
```

## Performance Considerations

### Memory Usage

The formatter aims to be memory-efficient:

```rust
// Use string builders to avoid excessive allocations
pub struct OutputBuilder {
    buffer: String,
    capacity_hint: usize,
}

impl OutputBuilder {
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            buffer: String::with_capacity(capacity),
            capacity_hint: capacity,
        }
    }
    
    pub fn write_str(&mut self, s: &str) {
        self.buffer.push_str(s);
    }
    
    pub fn finish(self) -> String {
        self.buffer
    }
}
```

### Parallel Processing

For large codebases, consider parallel processing:

```rust
// src/formatter/parallel.rs
use rayon::prelude::*;

pub struct ParallelFormatter {
    config: FormattingConfig,
}

impl ParallelFormatter {
    pub fn format_files(&self, files: Vec<PathBuf>) -> Result<Vec<FormattingResult>, FormattingError> {
        files.par_iter()
            .map(|file| self.format_file(file))
            .collect()
    }
}
```

## Testing Infrastructure

### Unit Testing

```rust
// tests/formatter/test_helpers.rs
pub fn format_source(source: &str, config: FormattingConfig) -> Result<String, FormattingError> {
    let mut engine = FormattingEngine::new(config);
    engine.format_file(source)
}

pub fn assert_formats_to(source: &str, expected: &str) {
    let result = format_source(source, FormattingConfig::default()).unwrap();
    assert_eq!(result.trim(), expected.trim());
}

pub fn assert_no_change(source: &str) {
    assert_formats_to(source, source);
}
```

### Integration Testing

```rust
// tests/formatter/integration_test.rs
#[test]
fn test_real_world_file() {
    let source = std::fs::read_to_string("examples/complex_example.csd").unwrap();
    let result = format_source(&source, FormattingConfig::default());
    
    assert!(result.is_ok());
    
    // Verify the formatted code still parses correctly
    let ast = parse_source(&result.unwrap());
    assert!(ast.is_ok());
}
```

### Property-Based Testing

```rust
// tests/formatter/property_test.rs
use proptest::prelude::*;

proptest! {
    #[test]
    fn formatting_preserves_semantics(source in generate_valid_cursed_source()) {
        let formatted = format_source(&source, FormattingConfig::default())?;
        
        // Both original and formatted should parse to equivalent ASTs
        let original_ast = parse_source(&source)?;
        let formatted_ast = parse_source(&formatted)?;
        
        prop_assert!(semantically_equivalent(&original_ast, &formatted_ast));
    }
    
    #[test]
    fn formatting_is_idempotent(source in generate_valid_cursed_source()) {
        let first_format = format_source(&source, FormattingConfig::default())?;
        let second_format = format_source(&first_format, FormattingConfig::default())?;
        
        prop_assert_eq!(first_format, second_format);
    }
}
```

## Error Handling

### Custom Error Types

```rust
// src/formatter/error.rs
#[derive(Debug, thiserror::Error)]
pub enum FormattingError {
    #[error("Parse error: {0}")]
    ParseError(#[from] ParseError),
    
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    
    #[error("Configuration error: {0}")]
    ConfigError(#[from] ConfigError),
    
    #[error("Formatting error at line {line}, column {column}: {message}")]
    FormattingError {
        line: usize,
        column: usize,
        message: String,
    },
}

impl FormattingError {
    pub fn at_position(line: usize, column: usize, message: impl Into<String>) -> Self {
        Self::FormattingError {
            line,
            column,
            message: message.into(),
        }
    }
}
```

## CLI Tool Implementation

### Command Structure

```rust
// src/bin/cursed-fmt.rs
use clap::{App, Arg, SubCommand};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = App::new("cursed-fmt")
        .version(env!("CARGO_PKG_VERSION"))
        .about("CURSED code formatter")
        .arg(Arg::with_name("write")
            .short("w")
            .long("write")
            .help("Write result to file instead of stdout"))
        .arg(Arg::with_name("check")
            .short("c")
            .long("check")
            .help("Check if input is formatted"))
        .arg(Arg::with_name("diff")
            .short("d")
            .long("diff")
            .help("Show formatting differences"))
        .arg(Arg::with_name("files")
            .multiple(true)
            .help("Files to format"))
        .get_matches();

    let formatter = CursedFormatter::new()?;
    
    match matches.subcommand() {
        ("check", Some(check_matches)) => {
            formatter.check_files(get_files(check_matches)?)
        }
        _ => {
            formatter.format_files(get_files(&matches)?)
        }
    }
}
```

## Contributing Guidelines

### Code Style

- Follow Rust standard formatting (use `cargo fmt`)
- Add comprehensive tests for new features
- Document public APIs with examples
- Use descriptive error messages

### Pull Request Process

1. Fork the repository
2. Create a feature branch: `git checkout -b feature/new-formatting-rule`
3. Add tests for your changes
4. Ensure all tests pass: `cargo test`
5. Add documentation for new features
6. Submit a pull request with clear description

### Testing Your Changes

```bash
# Run all formatter tests
cargo test formatter

# Run integration tests
cargo test --test formatter_integration

# Test on real files
cargo run --bin cursed-fmt -- --check examples/

# Performance testing
cargo run --release --bin cursed-fmt -- examples/large_file.csd
```

This developer guide should help you understand and extend the CURSED formatter. Happy coding! 🚀
