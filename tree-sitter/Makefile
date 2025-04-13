# Makefile for the CURSED tree-sitter grammar

# Detect available tools
ifneq (,$(shell which tree-sitter 2>/dev/null))
	TS_CMD = tree-sitter
else ifneq (,$(shell which npx 2>/dev/null))
	TS_CMD = npx tree-sitter
else
	TS_CMD = echo "Error: tree-sitter command not found. Install with 'npm install -g tree-sitter-cli' or use npx"; exit 1
endif

.PHONY: all generate test parse clean

all: generate test

# Generate the parser from the grammar
generate:
	@echo "Generating parser from grammar..."
	@$(TS_CMD) generate

# Run tests against the corpus files
test: generate
	@echo "Running tests..."
	@$(TS_CMD) test

# Parse example files (defaults to hello_world.csd)
parse: generate
	@echo "Parsing example file..."
	@$(TS_CMD) parse examples/hello_world.csd

# Parse a specific file if FILE is provided
parse-file: generate
	@if [ -z "$(FILE)" ]; then \
		echo "Error: No file specified. Use 'make parse-file FILE=path/to/file.csd'"; \
		exit 1; \
	fi
	@echo "Parsing file: $(FILE)"
	@$(TS_CMD) parse "$(FILE)"

# Parse all example files
parse-all: generate
	@echo "Parsing all example files..."
	@for file in examples/*.csd; do \
		echo "\nParsing $$file:"; \
		$(TS_CMD) parse "$$file" || exit 1; \
	done

# Clean generated files
clean:
	@echo "Cleaning generated files..."
	@rm -rf src/ build/ binding.gyp bindings/ package-lock.json node_modules/

# Help target
help:
	@echo "CURSED Tree-Sitter Grammar Makefile"
	@echo ""
	@echo "Targets:"
	@echo "  generate   - Generate parser from grammar"
	@echo "  test       - Run test suite"
	@echo "  parse      - Parse hello_world.csd example file"
	@echo "  parse-file - Parse a specific file (usage: make parse-file FILE=path/to/file.csd)"
	@echo "  parse-all  - Parse all example files"
	@echo "  clean      - Remove generated files"
	@echo "  help       - Show this help message"
	@echo ""
	@echo "Example usage:"
	@echo "  make generate        # Generate the parser"
	@echo "  make test            # Run the tests"
	@echo "  make parse-file FILE=examples/complex.csd  # Parse a specific file"