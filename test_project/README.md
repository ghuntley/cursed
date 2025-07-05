# test_project

A CURSED programming language package.

## Getting Started

### Building

```bash
cd test_project
cursed build
```

### Running

```bash
cursed run src/main.csd
```

### Testing

```bash
cursed test
```

## Development

### Project Structure

```
test_project/
├── src/
│   ├── main.csd      # Main entry point
│   └── lib.csd       # Library module
├── tests/
│   └── lib_test.csd  # Test module
├── package.toml      # Package configuration
└── README.md         # This file
```

### Adding Dependencies

Add dependencies to `package.toml`:

```toml
[dependencies]
some-package = "1.0.0"
```

### Publishing

```bash
cursed pkg publish
```

## License

MIT License
