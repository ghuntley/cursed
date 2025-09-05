# CURSED Development Guide for AI Agents

## Project Overview

CURSED is a programming language that uses Gen Z slang keywords and aims for pure self-hosting. The compiler is implemented in Zig and can run in both interpreter and compiled modes.

## CURSED Programming Language Specifications

The lexical structure, tokens, grammar and compiler specifications are in specs/* study them.

## Proper CURSED Program Structure

**CRITICAL**: Every CURSED program must follow this exact structure or it will fail to parse:

```cursed
vibe <package_name>         // Package clause (MANDATORY - must be first line)
yeet "<module_name>"        // Import declarations (optional, as needed)
yeet "<another_module>"     // Additional imports

// Your code here - function declarations, etc.
slay main_character() {
    // Function body
}
```

### Required Elements:

1. **Package Clause**: Every `.💀` file MUST start with `vibe <package_name>`
   - For main programs: `vibe main`
   - For modules: `vibe <module_name>` (e.g., `vibe mathz`, `vibe vibez`)

2. **Imports**: Use `yeet "<module_name>"` to import stdlib modules
   - `yeet "vibez"` - for output functions like `vibez.spill()`
   - `yeet "mathz"` - for mathematical operations
   - `yeet "stringz"` - for string manipulation
   - `yeet "collections"` - for array/collection operations

3. **Grammar Structure**: `PackageClause ";" { ImportDecl ";" } { TopLevelDecl ";" }`

### Common Mistakes to Avoid:

- **Missing package clause**: Parser will fail immediately if program doesn't start with `vibe`
- **Wrong import syntax**: Use `yeet` not `import` or other keywords
- **Missing required imports**: If code uses `vibez.spill()`, must have `yeet "vibez"`

### Example Valid Program:

```cursed
vibe main
yeet "vibez"
yeet "mathz"

slay main_character() {
    vibez.spill("Hello, CURSED!")
    sus result drip = mathz.abs_normie(-42)
    vibez.spill(result)
}
```

### Testing and Validation Insights:

**From Test Suite Experience:**
- **All existing test failures** in the original test suite were due to missing `vibe` package clauses
- **Stdlib modules must also follow proper structure**: Each stdlib module's `mod.💀` file must start with `vibe <module_name>`
- **Import dependencies**: Programs using stdlib functions MUST have corresponding `yeet` imports or compilation will fail
- **Parser error messages can be misleading**: Missing package clause often reports errors much later in the file

**Common Test Patterns:**
- Test files should be named descriptively: `validation_basic_syntax.💀`, `feature_stdlib_imports.💀`
- Use the test runner: `cd test_suite && ./run_tests.sh` to compare interpreter vs compiled modes
- Both modes should produce identical output for a test to pass

**Debugging Failed Programs:**
1. **First check**: Does the file start with `vibe <package_name>`?
2. **Second check**: Are all required imports present with `yeet` syntax?
3. **Third check**: Does the stdlib module being imported have proper `vibe <module_name>` structure?

## Build System

### Building the Compiler

```bash
# Build the compiler
zig build

# The compiler binary will be created at:
./zig-out/bin/cursed-compiler
```

### Running Programs

#### Interpreter Mode
```bash
./zig-out/bin/cursed-compiler --interpret <file.💀>
```

#### Compiled Mode
```bash
# Compile CURSED code to native binary
./zig-out/bin/cursed-compiler --compile <file.💀> -o <output_binary>

# Run the compiled binary
./<output_binary>
```

#### Additional Flags
```bash
# Emit LLVM IR for debugging
./zig-out/bin/cursed-compiler --compile <file.💀> -o <binary> --emit-ir

# Verbose output
./zig-out/bin/cursed-compiler --interpret <file.💀> --verbose
```

## Test Structure

### Important: Test Organization

- **DO NOT** store tests in the root directory
- Tests should be organized in structured directories:
  - `test_suite/` - Automated test suite comparing interpreter vs compiled modes
  - `tests/` - Individual test programs and validation suites
  - `examples/` - Example programs demonstrating language features

### Test File Naming Convention

- Use descriptive names: `test_parser_arithmetic.💀`, `test_stdlib_mathz.💀`
- Include purpose in filename: `validation_`, `regression_`, `feature_`
- Use `.💀` extension for CURSED source files

### Test Harness

#### Automated Parity Test Suite
```bash
# Run comprehensive test suite comparing interpreter vs compiled modes
cd test_suite
./run_tests.sh

# Results are generated in:
test_suite/results/parity_test_report_<timestamp>.md
```

## Development Workflow

### After Every Change

1. **Build the compiler**: `zig build`
2. **Test your specific change**: Create and run a targeted test
3. **Run the full test harness**: Execute the parity test suite
4. **Check health score**: Verify improvements or identify regressions

### Adding New Tests

1. **Write the test**: Create a `.💀` file with correct CURSED syntax
2. **Validate it works**: Test in both interpreter and compiled modes
3. **Verify identical behavior**: Ensure both modes produce the same output
4. **Add to test harness**: Only add tests that pass to the automated suite
5. **Document expected behavior**: Include comments about what the test validates


## Debugging

### Parser Issues
When parser fails, add debug logging:
```zig
std.debug.print("DEBUG: Current token = '{s}' ({any})\n", .{ token.lexeme, token.kind });
```


## Critical Files

### Core Implementation
- `src-zig/parser.zig` - Parser with Pratt expression parsing
- `src-zig/lexer.zig` - Tokenizer and keyword recognition
- `src-zig/interpreter.zig` - Interpreter execution engine
- `src-zig/llvm_ir_pipeline.zig` - LLVM compilation backend
- `src-zig/ast.zig` - Abstract syntax tree definitions

### Standard Library
- `stdlib/*/mod.💀` - CURSED standard library implementations

### Configuration
- `build.zig` - Zig build configuration
- `specs/` - Language specification documents

## IMPORTANT AND CRITICAL ITEMS ALWAYS FOLLOW

- IMPORTANT: Always test both interpreter and compiled modes after changes
- IMPORTANT: Focus on core functionality before advanced features
- IMPORTANT: Use the automated test suite to ensure no regressions
- IMPORTANT: always check that programs authored in CURSED (*.💀) are valid cursed programs as per the programming language specifications (specs/*.md) before making changes to the compiler by passing the specifications and the program over to the oracle for analysis.
- IMPORTANT: Resolve the root issue for test failures
- IMPORTANT: Full implementations, no minimal, no place holders, no "in a real implementation"
- IMPORTANT: Think extra extra hard when resolving root issues for test failures and consult the oracle.
- IMPORTANT: Any memory issues must be fixed immediately.
- CRITICAL: When troubleshooting Parser, Lexer, Interpreter, or LLVM/AST issues, then it is advisable to add logging to help you with your troubleshooting. Start with adding logging.
- CRITICAL: When assessing if a test has passed, you need to study the differences and identify if there are any between interpreted mode and compiled mode. A successful compilation does not mean it has worked. You need to study the output between interpreted and compiled.
- CRITICAL: When you've resolved an issue, use a sub-agent to do a git commit and push.
