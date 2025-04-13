# Cursed Programming Language - Memory

## Build/Lint/Test Commands
- Build: `devenv shell make build` or `devenv shell cargo build`
- Test: `devenv shell make test` or `devenv shell cargo test`
- Single test: `devenv shell cargo test test_name` or `devenv shell cargo test -- --test jit_integration_tests`
- Lint: `devenv shell devenv shell make lint` or `devenv shell cargo clippy -- -D warnings`
- Format check: `devenv shell make fmt` or `devenv shell cargo fmt -- --check`
- Format fix: `devenv shell make fmt-fix` or `cargo fmt`
- Run examples: `devenv shell make example EXAMPLE=fibonacci` or `devenv shell ./target/debug/cursed examples/fibonacci.csd`

## Garbage Collector
- Based on mark-and-sweep algorithm with incremental collection support
- Memory manager API: `allocate<T>`, `collect_garbage()`, `stats()`, `debug_info()`
- Types must implement `Traceable` and `Clone` traits to be garbage-collected
- Advanced features:
  - Incremental collection to reduce pause times
  - Weak references to handle cyclic dependencies
  - Memory layout optimizations for better cache locality
  - Detailed memory statistics and debugging tools
  - Generational collection for more efficient collection
- Core types are in `src/memory/gc.rs` and `src/memory/weak.rs`
- Detailed documentation in `.sourcegraph/gc_implementation.md`
- Example usage in `examples/concurrency_example.csd` and `examples/web_server_example.csd`

## Dot Expression Support
- Supports dot expressions like `vibez.spill("Hello")` for package functions
- Currently supports multiple package functions:
  - `vibez.spill(string)` - Print a string to console
  - `htmlrizzler.escape_html(string)` - Escape HTML special characters
  - `timez.Now()` - Get current time as timestamp
- Registry-based implementation with central dot expression management
- Implementation uses both compiler integration and a runtime fast path
- Enhanced support for user-defined type methods
- JSON-based support for non-string arguments
- Key files:
  - `src/stdlib/dot_registry.rs` - Central registry for dot expression handlers
  - `src/codegen/llvm/dot_expressions.rs` - LLVM IR generation for dot expressions
  - `src/codegen/llvm/hook_dot_expressions.rs` - Runtime patching mechanism
  - `src/main_patch.rs` - Generalized regex-based parser for dot expressions
- Test and example files:
  - `examples/all_dot_calls.csd` - Demonstrates multiple dot expression types
  - `tests/dot_registry_test.rs` - Test suite for the registry
  - `tests/vibez_spill_test.csd` - Test suite for specific function
  - `examples/htmlrizzler_test.csd` - Original use case
- Detailed documentation in `.sourcegraph/dot_expression_implementation.md`

## Code Style Guidelines
- Follow TDD (Test-Driven Development) for all new features: DO NOT IGNORE OR COMMENT OUT TESTS. RESOLVE THE UNDERLYING REASON WHY THE TESTS FAIL.
- Use descriptive comments for complex algorithms/unsafe code
- Contain unsafe code in the smallest possible scope
- Document safety requirements extensively
- Use Result types consistently for error handling with meaningful error messages
- Use snake_case for variables and functions, CamelCase for types
- Follow Rust module structure with clear module boundaries
- Keep the compiler pipeline stages modular and testable
- Always add proper tests for new language features
- Use Make as the primary build interface rather than direct cargo commands
- 