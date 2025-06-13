# CURSED Documentation Generation System

The CURSED programming language includes a comprehensive documentation generation system that automatically extracts and formats documentation from source code comments. This system supports multiple output formats and provides rich features for creating professional API documentation.

## Features

### 🚀 **Multiple Output Formats**
- **HTML**: Interactive documentation with search, navigation, and syntax highlighting
- **Markdown**: GitHub/GitLab compatible documentation for repositories
- **JSON**: Machine-readable format for tooling integration

### 📝 **Rich Documentation Comments**
- **JSDoc-style tags**: `@param`, `@return`, `@example`, `@since`, `@deprecated`
- **Code examples**: Syntax-highlighted code blocks with output examples
- **Cross-references**: Automatic linking between related items
- **Type information**: Parameter and return type documentation

### 🔍 **Advanced Features**
- **Search functionality**: Full-text search with keyword indexing
- **Cross-references**: Automatic linking between documented items
- **Navigation**: Generated table of contents and module indexes
- **Responsive design**: Mobile-friendly documentation layout
- **Syntax highlighting**: CURSED-specific syntax highlighting

### ⚙️ **Flexible Configuration**
- **Customizable themes**: Custom CSS and template support
- **Output control**: Include/exclude private items, examples, cross-references
- **Project metadata**: Title, description, version, author information
- **Server mode**: Built-in documentation server for local development

## Quick Start

### Basic Usage

Generate HTML documentation for your project:

```bash
cursed doc
```

Generate Markdown documentation:

```bash
cursed doc --format markdown
```

### Command Line Options

```bash
cursed doc [OPTIONS] [INPUT]

OPTIONS:
    -o, --output <DIR>           Output directory [default: docs]
    -f, --format <FORMAT>        Output format: html, markdown, json [default: html]
        --title <TITLE>          Documentation title
        --description <DESC>     Project description
        --version <VERSION>      Project version
        --author <AUTHOR>        Project authors (repeatable)
        --include-private        Include private items
        --no-examples           Exclude code examples
        --no-cross-refs         Disable cross-references
        --custom-css <FILE>     Custom CSS file
        --template-dir <DIR>    Custom template directory
        --base-url <URL>        Base URL for linking
        --serve                 Start documentation server
        --port <PORT>           Server port [default: 8080]
    -w, --watch                 Watch for changes and regenerate
    -c, --config <FILE>         Configuration file (TOML/JSON/YAML)
    -v, --verbose               Verbose output
```

### Configuration File

Create a `docs.toml` configuration file:

```toml
title = "My CURSED Project"
description = "A comprehensive CURSED application"
version = "1.0.0"
authors = ["Your Name"]

output_dir = "documentation"
format = "html"
include_examples = true
include_private = false
generate_cross_refs = true
base_url = "https://myproject.github.io/"

[output]
format = "html"
include_examples = true
include_private = false
```

## Documentation Comment Syntax

### Function Documentation

```cursed
/// Calculate the factorial of a number
/// 
/// This function recursively computes the factorial of the given number.
/// It includes proper error handling for negative inputs.
/// 
/// @param n The number to calculate factorial for (must be non-negative)
/// @return The factorial of n, or 1 if n is 0
/// @throws Error if n is negative
/// @since 1.0.0
/// @example Basic usage
/// ```cursed
/// let result = factorial(5)
/// println(result)  // Output: 120
/// ```
/// @example Error case
/// ```cursed
/// let result = factorial(-1)?  // Will throw error
/// ```
slay factorial(n i32) i32 {
    lowkey (n < 0) {
        panic("Factorial undefined for negative numbers")
    }
    lowkey (n <= 1) {
        yolo 1
    }
    yolo n * factorial(n - 1)
}
```

### Struct Documentation

```cursed
/// Represents a user in the system
/// 
/// Contains all necessary information for user management
/// including authentication and profile data.
/// 
/// @since 1.0.0
/// @example Creating a user
/// ```cursed
/// let user = User{
///     id: 123,
///     name: "Alice",
///     email: "alice@example.com",
///     active: true
/// }
/// ```
squad User {
    /// Unique user identifier
    /// @required Must be positive
    id i32
    
    /// User's display name
    /// @length 1-100 characters
    name string
    
    /// User's email address
    /// @format Valid email required
    email string
    
    /// Whether the user account is active
    /// @default true
    active bool
}
```

### Interface Documentation

```cursed
/// Interface for drawable objects
/// 
/// All objects that can be rendered to screen should implement
/// this interface to ensure consistent drawing behavior.
/// 
/// @since 1.0.0
/// @see Canvas for the drawing surface
collab Drawable {
    /// Draw the object to the given canvas
    /// @param canvas The canvas to draw on
    /// @param x X coordinate
    /// @param y Y coordinate
    slay draw(canvas Canvas, x i32, y i32)
    
    /// Get the object's bounding box
    /// @return Rectangle representing the bounds
    slay get_bounds() Rectangle
}
```

### Constant Documentation

```cursed
/// Maximum number of concurrent connections
/// 
/// This limit prevents server overload and ensures
/// stable performance under high load conditions.
/// 
/// @value 1000
/// @environment Can be overridden with MAX_CONNECTIONS env var
/// @category Configuration
facts MAX_CONNECTIONS = 1000
```

## Supported Tags

| Tag | Description | Example |
|-----|-------------|---------|
| `@param` | Parameter description | `@param name The user's name` |
| `@return` | Return value description | `@return The computed result` |
| `@throws` | Exception conditions | `@throws Error if input invalid` |
| `@example` | Code example | `@example Basic usage` |
| `@since` | Version introduced | `@since 1.0.0` |
| `@deprecated` | Deprecation notice | `@deprecated Use newFunction instead` |
| `@see` | Cross-references | `@see relatedFunction` |
| `@author` | Author information | `@author John Doe` |
| `@version` | Version information | `@version 1.2.0` |
| `@category` | Categorization | `@category Utilities` |
| `@internal` | Internal use only | `@internal Not part of public API` |

## HTML Output Features

### Interactive Search

The HTML output includes a powerful search feature:

- **Real-time search**: Results appear as you type
- **Keyword matching**: Searches names, descriptions, and keywords
- **Category filtering**: Filter by item type (function, struct, etc.)
- **Keyboard navigation**: Navigate results with arrow keys

### Navigation

- **Sidebar navigation**: Hierarchical module and item listing
- **Breadcrumb navigation**: Shows current location in documentation
- **Cross-reference links**: Click to jump between related items
- **Table of contents**: Generated automatically for each module

### Responsive Design

- **Mobile-friendly**: Responsive layout works on all device sizes
- **Touch-friendly**: Large tap targets for mobile devices
- **Progressive enhancement**: Works without JavaScript

### Syntax Highlighting

- **CURSED syntax**: Full syntax highlighting for CURSED code
- **Multiple languages**: Support for other languages in examples
- **Copy buttons**: One-click copying of code examples

## Markdown Output Features

### GitHub Integration

- **README generation**: Creates comprehensive README.md files
- **Module documentation**: Individual .md files for each module
- **Badge support**: Generates status badges for version, build, etc.
- **Table formatting**: Clean tables for parameters and options

### Cross-Platform

- **GitHub Pages**: Ready for GitHub Pages deployment
- **GitLab**: Compatible with GitLab's Markdown renderer
- **Generic Markdown**: Works with any Markdown processor

## JSON Output Features

### Machine-Readable

The JSON output provides structured data for:

- **API documentation tools**: Integration with documentation platforms
- **IDE plugins**: Rich tooltips and autocomplete information
- **Static analysis**: Code quality and documentation coverage tools
- **Custom tooling**: Build your own documentation processors

### Comprehensive Data

- **Complete AST information**: Full structural data about code
- **Rich metadata**: Timestamps, file information, statistics
- **Search index**: Pre-built search index for fast lookups
- **Cross-references**: Complete linking information

## Advanced Configuration

### Custom Templates

Create custom HTML templates for branded documentation:

```bash
cursed doc --template-dir ./my-templates
```

Template structure:
```
my-templates/
├── layout.html      # Main page layout
├── module.html      # Module page template
├── item.html        # Documentation item template
├── styles.css       # Custom CSS
└── scripts.js       # Custom JavaScript
```

### Custom CSS

Apply custom styling:

```bash
cursed doc --custom-css ./styles/brand.css
```

### Watch Mode

Automatically regenerate documentation when source files change:

```bash
cursed doc --watch
```

### Server Mode

Start a local development server:

```bash
cursed doc --serve --port 3000
```

## Integration Examples

### GitHub Actions

```yaml
name: Generate Documentation
on:
  push:
    branches: [main]

jobs:
  docs:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      - name: Generate Documentation
        run: cursed doc --format html --output docs
      - name: Deploy to GitHub Pages
        uses: peaceiris/actions-gh-pages@v3
        with:
          github_token: ${{ secrets.GITHUB_TOKEN }}
          publish_dir: ./docs
```

### GitLab CI

```yaml
pages:
  script:
    - cursed doc --format html --output public
  artifacts:
    paths:
      - public
  only:
    - main
```

### Pre-commit Hook

```bash
#!/bin/sh
# .git/hooks/pre-commit
cursed doc --format markdown --no-serve
git add docs/
```

## API Reference

### Programmatic Usage

```rust
use cursed::docs::{DocumentationGenerator, DocGeneratorConfig, DocFormat};

let mut config = DocGeneratorConfig::default();
config.format = DocFormat::Html;
config.title = "My Project".to_string();
config.output_dir = std::path::PathBuf::from("output");

let mut generator = DocumentationGenerator::new(config);
generator.generate_from_directory("src")?;
```

### Configuration Options

```rust
pub struct DocGeneratorConfig {
    pub output_dir: PathBuf,
    pub format: DocFormat,
    pub include_examples: bool,
    pub include_private: bool,
    pub generate_cross_refs: bool,
    pub custom_css: Option<String>,
    pub template_dir: Option<PathBuf>,
    pub title: String,
    pub description: Option<String>,
    pub version: Option<String>,
    pub authors: Vec<String>,
    pub base_url: Option<String>,
}
```

## Best Practices

### Documentation Guidelines

1. **Write clear summaries**: First line should be a clear, concise summary
2. **Document all parameters**: Include type information and constraints
3. **Provide examples**: Show typical usage patterns
4. **Include error conditions**: Document when functions might fail
5. **Use consistent formatting**: Follow team documentation standards

### Organization

1. **Group related functions**: Keep related functionality together
2. **Use modules effectively**: Organize code into logical modules
3. **Cross-reference liberally**: Link to related functions and types
4. **Version your APIs**: Use @since tags for version tracking

### Maintenance

1. **Keep docs current**: Update documentation when code changes
2. **Review regularly**: Regular documentation reviews catch issues
3. **Use automation**: Integrate documentation generation into CI/CD
4. **Monitor coverage**: Track documentation coverage metrics

## Troubleshooting

### Common Issues

**Documentation not generated**
- Check file paths and permissions
- Verify CURSED syntax is valid
- Run with `--verbose` for detailed output

**Missing cross-references**
- Ensure functions are properly documented
- Check that referenced items exist
- Verify naming matches exactly

**Styling issues**
- Check custom CSS syntax
- Verify template file paths
- Use browser developer tools to debug

**Search not working**
- Ensure JavaScript is enabled
- Check browser console for errors
- Verify search index was generated

### Performance Tips

1. **Large projects**: Use `--no-cross-refs` for faster generation
2. **Frequent changes**: Use `--watch` mode for development
3. **CI/CD**: Cache documentation dependencies
4. **Network**: Use `--base-url` for absolute links

## Contributing

The documentation system is extensible and welcomes contributions:

- **Templates**: Create new themes and layouts
- **Generators**: Add support for new output formats
- **Features**: Enhance search, navigation, and formatting
- **Integrations**: Build plugins for editors and tools

See the main CURSED repository for contribution guidelines.
