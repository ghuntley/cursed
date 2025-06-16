# LaTeX Documentation Generator

The LaTeX documentation generator for CURSED produces professional, academic-style documentation suitable for technical manuals, research papers, and presentations. This generator provides comprehensive LaTeX output with advanced features including syntax highlighting, mathematical notation, cross-references, and support for multiple document classes.

## Features

### Document Classes

The LaTeX generator supports four document classes, each optimized for different use cases:

#### Article (`DocumentClass::Article`)
- **Use Case**: Smaller documentation, API references, single-topic guides
- **Features**: Simple structure with sections and subsections
- **Default Options**: `11pt`, `a4paper`
- **Generated Files**: Main document, bibliography, Makefile, compilation script

#### Report (`DocumentClass::Report`)
- **Use Case**: Comprehensive documentation with multiple modules
- **Features**: Chapter-based structure, separate module files
- **Default Options**: `11pt`, `a4paper`, `twoside`
- **Generated Files**: Main document + individual module files, bibliography, Makefile, compilation script

#### Book (`DocumentClass::Book`)
- **Use Case**: Complete language documentation, extensive manuals
- **Features**: Full book structure with chapters, appendices, detailed TOC
- **Default Options**: `11pt`, `a4paper`, `twoside`, `openright`
- **Generated Files**: Main document + individual module files, bibliography, Makefile, compilation script

#### Beamer (`DocumentClass::Beamer`)
- **Use Case**: Presentation slides, talks, lectures
- **Features**: Slide-based presentation format, outline frames
- **Default Options**: `11pt`, `aspectratio=169`
- **Generated Files**: Presentation slides, Makefile, compilation script

### Syntax Highlighting

The generator provides two syntax highlighting options:

#### Listings Package (Default)
- **Compatibility**: High - works with standard LaTeX installations
- **Features**: Custom CURSED language definition, configurable colors
- **Requirements**: Only standard LaTeX packages
- **Keywords Supported**: All CURSED keywords (`slay`, `yolo`, `sus`, etc.)

#### Minted Package (Advanced)
- **Compatibility**: Requires Python pygments
- **Features**: Advanced syntax highlighting, better color schemes
- **Requirements**: Python, pygments, `--shell-escape` flag
- **Performance**: Higher quality output, more language features

### Mathematical Notation

When math support is enabled (`math_support = true`), the generator includes:

- **Packages**: `amsmath`, `amsfonts`, `amssymb`, `mathtools`
- **Function Signatures**: Rendered in mathematical notation
- **Type Relationships**: Mathematical arrows and symbols
- **Generic Types**: Proper mathematical formatting

Example function signature rendering:
```latex
\begin{equation*}
\text{calculate}(\text{x} : \text{i32}, \text{y} : \text{i32}) \rightarrow \text{i32}
\end{equation*}
```

### Cross-References and Hyperlinks

The generator automatically creates:

- **Labels**: For all sections, functions, types, and constants
- **Hyperlinks**: Clickable references in PDF output
- **Index Entries**: Comprehensive indexing of all documented items
- **Bibliography**: Standard references to CURSED language and documentation

### Special Character Handling

The LaTeX generator properly escapes all special characters:

| Character | LaTeX Output |
|-----------|--------------|
| `\` | `\textbackslash{}` |
| `{` | `\{` |
| `}` | `\}` |
| `$` | `\$` |
| `&` | `\&` |
| `%` | `\%` |
| `#` | `\#` |
| `^` | `\textasciicircum{}` |
| `_` | `\_` |
| `~` | `\textasciitilde{}` |

## Configuration

### Basic Configuration

```toml
[latex]
document_class = "article"  # or "report", "book", "beamer"
paper_size = "a4paper"
font_size = "11pt"
margins = "margin=1in"
generate_toc = true
generate_index = true
include_code_listings = true
```

### Advanced Configuration

```toml
[latex.syntax_highlighting]
use_minted = false
show_line_numbers = true
tab_size = 4
break_lines = true

[latex.syntax_highlighting.color_scheme]
background = "backcolour"
comment = "codegreen"
keyword = "magenta"
string = "codepurple"
```

### Package Configuration

```toml
[latex.packages]
standard_packages = ["inputenc", "fontenc", "geometry"]

[latex.packages.additional_packages]
"hyperref" = ["colorlinks=true", "linkcolor=blue"]

custom_definitions = [
    "\\newcommand{\\cursed}[1]{\\texttt{#1}}",
    "\\definecolor{cursedblue}{RGB}{0,102,204}"
]
```

## Usage

### Basic Usage

```rust
use cursed::documentation::generators::{LaTeXGenerator, LaTeXConfig, DocumentClass};

let config = LaTeXConfig {
    document_class: DocumentClass::Article,
    ..LaTeXConfig::default()
};

let mut generator = LaTeXGenerator::new(config);
let output_files = generator.generate_documentation(&docs, &output_dir)?;
```

### Custom Configuration

```rust
let mut config = LaTeXConfig::default();
config.document_class = DocumentClass::Book;
config.syntax_highlighting.use_minted = true;
config.generate_bibliography = true;
config.math_support = true;

let mut generator = LaTeXGenerator::new(config);
```

### Command Line Usage

```bash
# Generate LaTeX documentation
cursed-doc --format latex --output docs/

# With custom configuration
cursed-doc --format latex --config latex_config.toml --output docs/

# Compile the generated LaTeX
cd docs/
make
# or
./compile.sh
```

## Generated Files

### Main Document (`documentation.tex`)

The main LaTeX document containing:
- Document preamble with all necessary packages
- Title page and table of contents
- Complete documentation content
- Bibliography and index sections

### Individual Module Files (Report/Book classes)

For larger document classes, separate `.tex` files for each module:
- `module_name.tex` - Individual module documentation
- Can be compiled separately or included in main document

### Bibliography (`references.bib`)

BibTeX file containing:
- Standard CURSED language references
- Documentation generation metadata
- Custom bibliography entries

### Build Files

- `Makefile` - Comprehensive build system with multiple targets
- `compile.sh` - Cross-platform compilation script
- Proper handling of bibliography, index, and cross-references

## Compilation

### Using Make

```bash
make all          # Build complete documentation
make clean        # Remove auxiliary files
make cleanall     # Remove all generated files
make view         # Open PDF in default viewer
make help         # Show available targets
```

### Using Compilation Script

```bash
./compile.sh      # Automatic compilation with dependency checking
```

### Manual Compilation

```bash
# Standard compilation
pdflatex documentation.tex
bibtex documentation
makeindex documentation.idx
pdflatex documentation.tex
pdflatex documentation.tex

# With minted (requires --shell-escape)
pdflatex --shell-escape documentation.tex
bibtex documentation
makeindex documentation.idx
pdflatex --shell-escape documentation.tex
pdflatex --shell-escape documentation.tex
```

## Requirements

### Standard Requirements

- LaTeX distribution (TeX Live, MiKTeX, etc.)
- `pdflatex` command
- `bibtex` for bibliography
- `makeindex` for index generation

### Additional Requirements (Minted)

- Python with pygments package
- LaTeX compiler with `--shell-escape` support

### Package Dependencies

The generator automatically includes necessary LaTeX packages:
- Core: `inputenc`, `fontenc`, `geometry`
- Typography: `lmodern`, `fancyhdr`
- Code: `listings` or `minted`
- Graphics: `xcolor`, `graphicx`
- Math: `amsmath`, `amsfonts`, `amssymb`
- Tables: `booktabs`, `longtable`
- References: `hyperref`, `makeidx`

## Best Practices

### Document Class Selection

- **Article**: For API references, tutorials, small documentation sets
- **Report**: For comprehensive module documentation (5-20 modules)
- **Book**: For complete language documentation (20+ modules)
- **Beamer**: For presentations and talks

### Syntax Highlighting

- Use **listings** for maximum compatibility and ease of compilation
- Use **minted** for highest quality output when Python is available

### Mathematical Notation

- Enable math support for languages with complex type systems
- Use mathematical notation for function signatures and type relationships

### Cross-References

- Always enable cross-references for professional documentation
- Provides clickable links in PDF output

### Bibliography

- Include bibliography for formal documentation
- Add custom entries for related work and standards

## Troubleshooting

### Common Issues

1. **Missing Packages**: Install required LaTeX packages through your distribution
2. **Compilation Errors**: Check LaTeX log files for specific error messages
3. **Minted Issues**: Ensure Python and pygments are installed, use `--shell-escape`
4. **Encoding Issues**: Verify UTF-8 encoding in source files

### Performance Tips

- Use listings instead of minted for faster compilation
- Disable unnecessary features (LOF, LOT) for smaller documents
- Use article class for simple documentation

### Quality Improvements

- Enable minted for better syntax highlighting
- Use book class for extensive documentation
- Include mathematical notation for technical accuracy
- Generate comprehensive index and bibliography

## Academic and Professional Use

The LaTeX generator is specifically designed for academic and professional documentation needs:

### Academic Benefits

- **IEEE/ACM Standards**: Follows academic formatting conventions
- **Mathematical Notation**: Proper rendering of technical concepts
- **Bibliography**: Standard academic citation format
- **Cross-References**: Professional document navigation
- **Index**: Comprehensive term lookup

### Professional Benefits

- **Corporate Documentation**: Clean, professional appearance
- **Technical Manuals**: Comprehensive coverage with proper structure
- **API Documentation**: Clear, searchable reference material
- **Training Materials**: Presentation slides and detailed guides

### Publishing Ready

- **Print Quality**: High-resolution PDF output
- **Standard Formatting**: Compatible with academic and professional standards
- **Accessibility**: Proper document structure for screen readers
- **Archive Quality**: Long-term document preservation format

The LaTeX documentation generator transforms CURSED language documentation into publication-ready academic and professional documents suitable for research papers, technical manuals, corporate documentation, and educational materials.
