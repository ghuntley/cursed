# CURSED v1.0.0-rc2 Community Bug Bash 🐛⚡

Welcome to the CURSED programming language Bug Bash! Help us test the Release Candidate before the official v1.0.0 launch.

## Quick Start for Testers

### Installation
```bash
# Install CURSED v1.0.0-rc2
curl -sSf https://install.cursedlang.org/rc2 | sh
cd ~/cursed-testing && git clone https://github.com/ghuntley/cursed.git
cd cursed && zig build
```

### 5-Minute Smoke Test
```bash
# Test basic functionality
echo 'vibez.spill("Hello Bug Bash!")' > hello.csd
./zig-out/bin/cursed-zig hello.csd

# Test standard library
echo 'yeet "mathz"; vibez.spill("sqrt(16) =", sqrt(16))' > math.csd
./zig-out/bin/cursed-zig math.csd
```

## Priority Testing Areas

### 🔥 Critical Path Tests (P0)
1. **Compilation Pipeline**: Interpreter → Native Binary generation
2. **Memory Safety**: No crashes, leaks, or corruption
3. **Core Language**: Variables, functions, control flow
4. **Standard Library**: vibez, mathz, stringz, arrayz modules

### ⚡ High Impact Tests (P1)  
1. **Concurrency**: Goroutines, channels, select operations
2. **Error Handling**: yikes/fam/shook patterns
3. **Type System**: Generics, interfaces, pattern matching
4. **Cross-Platform**: Linux, macOS, Windows, WebAssembly

### 📚 Feature Tests (P2)
1. **Advanced Features**: Macros, reflection, async/await
2. **Developer Tools**: LSP, formatter, linter, debugger
3. **Package System**: Module imports, dependency management
4. **Documentation**: Code generation and examples

## Test Scenarios by Category

### Scenario 1: Basic Language Features
```cursed
# Test file: basic_features_test.csd
sus name tea = "Tester"
sus age drip = 25
sus active lit = based

slay greet(person tea) tea {
    damn "Hello, " + person + "!"
}

vibez.spill(greet(name))

ready (age > 18) {
    vibez.spill("Adult detected")
} otherwise {
    vibez.spill("Minor detected")  
}
```

### Scenario 2: Standard Library Validation
```cursed
# Test file: stdlib_validation.csd
yeet "mathz"
yeet "stringz" 
yeet "arrayz"

sus numbers []drip = [1, 2, 3, 4, 5]
sus doubled []drip = map(numbers, slay(x drip) drip { damn x * 2 })

vibez.spill("Original:", numbers)
vibez.spill("Doubled:", doubled)
vibez.spill("Max:", max(numbers))
```

### Scenario 3: Concurrency Testing
```cursed
# Test file: concurrency_test.csd
yeet "concurrenz"

sus ch chan<drip> = make_channel()

go {
    ch <- 42
}

sus value drip = <-ch
vibez.spill("Received:", value)
```

### Scenario 4: Error Handling
```cursed
# Test file: error_handling_test.csd
slay divide(a drip, b drip) yikes<drip> {
    ready (b == 0) {
        yikes "division by zero"
    }
    damn a / b
}

sus result drip = divide(10, 0) fam {
    when "division by zero" -> damn -1
    when _ -> damn 0
}

vibez.spill("Result:", result)
```

## Bug Reporting Guidelines

### Bug Report Template
```markdown
**CURSED Version**: v1.0.0-rc2
**Platform**: Linux/macOS/Windows + architecture  
**Severity**: Critical/High/Medium/Low
**Category**: Compiler/Runtime/Stdlib/Tools

**Issue**: One-line description
**Expected**: What should happen
**Actual**: What actually happens
**Reproduce**: Step-by-step instructions

**Code Sample**:
```cursed
# Minimal reproduction case
```

**Error Output**:
```
# Full error message and stack trace
```

**Additional Context**: Environment details, related issues
```

### Severity Guidelines
- **Critical**: Crashes, data corruption, security issues, compiler hangs
- **High**: Wrong results, major feature broken, performance regression
- **Medium**: Minor incorrect behavior, usability issues
- **Low**: Documentation, cosmetic issues, enhancement requests

## Testing Checklist

### Core Language ✅
- [ ] Variable declarations (sus, drip, tea, lit)
- [ ] Function definitions (slay) and calls
- [ ] Control flow (ready/otherwise, bestie loops)
- [ ] Array operations and bounds checking
- [ ] String manipulation and interpolation
- [ ] Arithmetic and logical operations

### Standard Library ✅
- [ ] vibez (I/O operations)
- [ ] mathz (mathematical functions)
- [ ] stringz (string processing)
- [ ] arrayz (array utilities)
- [ ] filez (file operations)
- [ ] timez (date/time handling)

### Advanced Features ✅
- [ ] Generic functions and types
- [ ] Pattern matching with sick
- [ ] Interface implementations (collab)
- [ ] Concurrency (go blocks, channels)
- [ ] Error handling (yikes/fam/shook)
- [ ] Module system (yeet imports)

### Developer Tools ✅
- [ ] cursed-zig compiler
- [ ] cursed-fmt formatter  
- [ ] cursed-lint linter
- [ ] cursed-lsp language server
- [ ] cursed-doc documentation generator

### Cross-Platform ✅
- [ ] Native compilation on Linux
- [ ] Native compilation on macOS
- [ ] Native compilation on Windows
- [ ] WebAssembly compilation
- [ ] Cross-compilation between platforms

## Performance Benchmarks

### Expected Performance Targets
- **Compilation**: <1s for typical projects
- **Memory Usage**: <100MB during compilation
- **Runtime Startup**: <50ms for applications
- **Goroutine Creation**: <1µs per goroutine

### Benchmark Scripts
```bash
# Compilation speed test
time zig build

# Memory usage test  
valgrind --leak-check=full ./zig-out/bin/cursed-zig test.csd

# Runtime performance
hyperfine './zig-out/bin/cursed-zig performance_test.csd'
```

## Community Testing Rewards

### Recognition Program
- **Bug Hunter**: First to find P0/P1 issues
- **Feature Validator**: Comprehensive testing of major features
- **Platform Champion**: Cross-platform testing coverage
- **Community Helper**: Assisting other testers

### Leaderboard Categories
1. Most bugs reported (quality over quantity)
2. Most comprehensive test coverage
3. Best bug reproduction cases
4. Most helpful community contributions

## Getting Help

### Support Channels
- **Discord**: #bug-bash channel for real-time help
- **GitHub Issues**: Formal bug reports and tracking
- **Forum**: Detailed discussions and Q&A
- **Office Hours**: Weekly community calls

### Testing Resources
- **Test Suite**: `/test_suite/` directory with examples
- **Documentation**: Complete language reference
- **Examples**: 269+ example files covering all features
- **Migration Guide**: From other languages to CURSED

## Submission Process

### 1. Test Execution
Run test scenarios and document results

### 2. Bug Discovery
Follow reproduction guidelines and capture full context

### 3. Report Submission
Use GitHub Issues with proper labels and template

### 4. Community Engagement
Share findings in Discord and help other testers

### 5. Follow-up
Respond to maintainer questions and verify fixes

## Timeline

- **Week 1**: Setup and basic feature testing
- **Week 2**: Advanced features and edge cases
- **Week 3**: Performance testing and platform coverage
- **Week 4**: Final validation and documentation

## Success Metrics

### Coverage Goals
- **Feature Coverage**: 95% of documented features tested
- **Platform Coverage**: All 4 major platforms validated
- **Edge Case Coverage**: Boundary conditions and error paths
- **Performance Coverage**: Benchmarks on all platforms

### Quality Goals
- **Zero P0 Issues**: No critical bugs in final release
- **<5 P1 Issues**: High-impact issues resolved or documented
- **Community Satisfaction**: Positive feedback from testers
- **Documentation Quality**: Clear, accurate, complete guides

Thank you for helping make CURSED v1.0.0 production-ready! 🚀
