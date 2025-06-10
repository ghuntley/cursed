# ✨ CURSED Programming Language 🔥

OMG! CURSED is like, THE MOST iconic programming language that's giving major Go grammar vibes but with Gen Z slang for the keywords?! 💅 It's literally a self-compiling compiler written in Rust and I'm obsessed! 😍

## The Tea on CURSED ☕

CURSED is lowkey based on Go's practical design but like, replaces all those boring programming keywords with Gen Z slang that slaps so hard! It's such a vibe while still being fully functional, no cap! 🙌

### New Generic Programming Features! 🔥

CURSED now supports next-level generic programming with:

- **Type Parameters**: Write code that works with multiple types (`squad Container<T>`)
- **Constraint System**: Ensure type safety with powerful constraints (`where T: Clone + Display`)
- **Zero-Cost Abstractions**: Generics compile to optimized native code
- **Advanced Features**: Higher-kinded types, associated types, and variance support

Check out the [Generics Guide](docs/generics_guide.md) for the full tea! ☕

Example CURSED code (it ATE and left no crumbs):

```
vibe main

yeet "vibez"

slay main() {
    vibez.spill("Hello, World!")  fr fr This is a comment
    
    sus name tea = "bestie"
    vibez.spillf("Hey %s, what's good?", name)

    lowkey 1 < 2 {
        vibez.spill("This is based!")
    } highkey {
        vibez.spill("This is sus!")
    }
}
```

## Self-Compiling Compiler (so extra!) 💯

CURSED is literally built different with this bootstrapping compiler approach:

1. **Stage 0**: Bootstrap environment setup using Rust (so basic but necessary) 🙄
2. **Stage 1**: Minimal bootstrap compiler in Rust (starting to serve) 👀
3. **Stage 2**: Full compiler written in CURSED (the main character energy) ⭐
4. **Stage 3**: Self-compiled full compiler (iconic behavior) 💁‍♀️

## Project Structure (we stay organized, periodt!) 📁

- `/src`: Compiler source code (core behavior, pretty much carrying) 💅
- `/specs`: Language specifications and docs (the rulebook, very that) 📝
- `/examples`: Example CURSED programs (slay examples) ✨
- `/tests`: Test suite for the compiler (we don't flop, we test) ✅

## Getting Started (no gatekeeping here!) 🚀

### Prerequisites (the fit check before coding) 👗

- Rust toolchain (1.54.0 or later) ⚙️
- Cargo 📦

### Building (so you can build *that*) 🔨

```
make build
```

### Running Tests (gotta check the vibes) 🧪

```
make test
```

### Running the Compiler (it's giving main character energy) 💻

```
make run ARGS="path/to/your/file.csd"
```

### Formatting Your Code (keep it aesthetic!) ✨

CURSED comes with a built-in formatter that makes your code look absolutely iconic:

```bash
# Format a single file
cursed-fmt -w main.csd

# Format entire project
cursed-fmt -w src/

# Check if formatting is needed (perfect for CI)
cursed-fmt --check src/
```

The formatter supports customizable styling through `.cursed-fmt.toml` config files. It's giving consistency! 💅

## Developer Tools (the full toolkit, bestie!) 🧰

### Code Formatter (`cursed-fmt`)
- **Auto-formatting**: Makes your code look clean and consistent
- **Editor integration**: Works with VSCode, Vim, and more
- **CI/CD support**: Perfect for automated checks
- **Customizable**: Configure to match your team's style

Quick start:
```bash
# Install the formatter
make build-formatter

# Format your code
cursed-fmt -w .

# Set up pre-commit hooks
cp examples/formatter/integration/pre-commit .git/hooks/
```

📖 **[Complete Formatter Guide](docs/formatter.md)** | 🔧 **[Developer API](docs/formatter-dev.md)**

### Documentation Generator (`cursed-doc`)
- **Comprehensive docs**: Auto-generates beautiful HTML documentation
- **Live server**: Development server with auto-reload
- **Multiple formats**: HTML, Markdown, JSON output
- **Cross-references**: Automatic linking between code components
- **Search integration**: Full-text search with syntax highlighting

Quick start:
```bash
# Generate HTML documentation
make docs

# Serve docs locally with auto-reload
make docs-serve

# Generate Markdown documentation
make docs-markdown

# Check documentation completeness
make docs-check
```

📖 **[Documentation Guide](docs/cursed-doc.md)** | 🔧 **[Configuration Reference](docs/cursed-doc-config.md)**

## Language Documentation (real ones read the docs) 📚

The complete language specifications are available in the `/specs` directory (we ate with these specs):

- [Overview](specs/overview.md) 👁️
- [Lexical Structure](specs/lexical.md) 🔤
- [Types](specs/types.md) 🏷️
- [Grammar](specs/grammar.md) 📐
- [Compiler Stages](specs/compiler_stages.md) 🪜
- [Standard Library](specs/stdlib.md) 📚

## Contributing (join the CURSED group chat) 👯‍♀️

Contributions are welcome, bestie! Your pull requests can absolutely slay! Fork and go off! 💅

## License 📜

This project is licensed under the MIT License - see the LICENSE file for the deets. It's giving open source! 🤩