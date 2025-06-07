# Cursed Programming Language - Memory

## Build/Lint/Test Commands
- Build: `make build` or `cargo build`
- Test: `make test` or `cargo test`
- Single test: `cargo test test_name` or `cargo test -- --test jit_integration_tests`
- Ignored tests: `cargo test -- --ignored` or `cargo test -- --ignored --test gc_improved_test`
- Lint: `make lint` or `cargo clippy -- -D warnings`
- Format check: `make fmt` or `cargo fmt -- --check`
- Format fix: `make fmt-fix` or `cargo fmt`
- Run examples: `make example EXAMPLE=fibonacci` or `./target/debug/cursed examples/fibonacci.csd`

## Nix Environment Linking Issues and Workarounds
The Nix environment has linking issues with mold and missing libraries that affect both builds and tests.

### Current Status
- **Library building works** with the configured `.cargo/config.toml` 
- **Tests still fail** due to mold linker being forced by environment
- Missing libraries: libffi, libz, libtinfo, libxml2

### Working Solutions
1. Library builds work with `.cargo/config.toml` configuration that includes:
   - Using gcc as linker
   - Explicit library paths for required dependencies

### Failed Workarounds Attempted
- Setting `-C link-arg=-fuse-ld=ld` still results in mold being used
- RUSTFLAGS environment variables get overridden by system configuration

### Successful Test Command 
The linking issue was identified and successfully resolved! We found that:
1. The real issue was missing libgcc_s, libffi, libz, libtinfo, and libxml2
2. Created `.cargo/config.toml` with proper library paths
3. Library builds work, but tests may need additional environment setup

### Library Paths in Nix Store
- libffi: `/nix/store/6pak77li0iw9x0b3yhmbjvp846w3p6bx-libffi-3.4.6/lib`
- libz: `/nix/store/l5g2v1jgfyf3j0jp9iv5b79fi8yrwzpp-zlib-1.3.1/lib`
- libtinfo: `/nix/store/k3a7dzrqphj9ksbb43i24vy6inz8ys51-ncurses-6.4.20221231/lib`
- libxml2: `/nix/store/0z4hrksbdrwv9xb8ycjk3rq9ppmw0350-libxml2-2.13.5/lib`

## Structured Logging and Instrumentation
- Use the `tracing` crate for structured logging and instrumentation
- Annotate functions/methods with `#[instrument]` by default
- Event levels: `trace`, `debug`, `info`, `warn`, `error`
- Instrument with fields: `#[instrument(fields(param1 = ?self.param1))]`
- Span context: `let _span = info_span!("operation", field1 = value).entered();`
- Log events: `debug!(target: "app::module", field = value, "message {}", var);`
- Skip large fields: `#[instrument(skip(large_field))]`
- Record errors: `error!(error = ?err, "Failed operation");`

## Test Logging Infrastructure

### Common Module for Test Tracing
A common test module is available in `tests/common.rs` that provides test-specific tracing utilities:

```rust
// Initialize tracing in a test
use crate::common;

#[test]
fn my_test() {
    // Set up tracing for this test
    common::tracing::setup();
    
    // Use the macro (preferred)
    init_tracing!();
    
    // Log events in tests
    tracing::info!("Test started");
}
```

### The Timer Utility
For benchmarking operations in tests, use the `Timer` utility:

```rust
#[test]
fn performance_test() {
    init_tracing!();
    
    // Creates a timer that logs when dropped
    let _timer = common::timing::Timer::new("my_operation");
    
    // Perform the operation to be timed
    // When _timer goes out of scope, it will log the elapsed time
}
```

### Test-Specific Tracing Setup
For standalone tests, a simpler tracing setup is available in `tests/tracing_setup.rs`:

```rust
#[path = "tracing_setup.rs"]
pub mod tracing_setup;
use tracing::{debug, error, info};

#[test]
fn my_test() {
    tracing_setup::init_test_tracing();
    info!("Starting test");
}
```

### Best Practices for Test Logging
- Initialize tracing once per test using `init_tracing!()`
- Use descriptive context in log messages: `info!(test_case = "feature_x", "Starting test")`
- Log at appropriate levels in tests:
  - `info!` - Test start/completion, major steps
  - `debug!` - Test setup details, intermediate results
  - `error!` - Test failures with context
- Include relevant test data in structured fields rather than string interpolation
- For failures, log detailed context before assertions: `error!(expected = ?expected, actual = ?actual)`

## Memory Management Notes

### Circular Reference Handling
The garbage collector (GC) has been enhanced with proper cycle detection to handle circular references. The cycle detection mechanism works by:

1. Starting from root objects (those directly referenced by the program)
2. Using a visitor pattern to recursively trace through all object references
3. Maintaining a set of "reachable" objects during the mark phase
4. Properly handling cycles when an object refers back to an already visited object
5. Sweeping (collecting) all objects that weren't marked as reachable

This prevents memory leaks when objects reference each other in cycles but are no longer reachable from the root set.

## Interface Type Assertion Implementation

We're working on implementing interface type assertions for the language, which allows checking if an interface value is of a concrete type and safely converting between them. The current implementation includes:

1. AST structure in `src/ast/expressions/type_assertion.rs`
2. Parser support in `src/parser/type_assertion.rs`
3. LLVM code generation in `src/codegen/llvm/type_assertion.rs`

There are still several issues to fix:
1. Proper error propagation in the LLVM code generator
2. Handling LLVM Result types with `?` operator
3. Integration between AST and code generator

Implementation approach:
1. First fix parser and AST structures
2. Then implement basic LLVM code generation
3. Finally add full runtime type checking