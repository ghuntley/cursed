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


## Code Style Guidelines
- Follow TDD (Test-Driven Development) for all new features
- Use descriptive comments for complex algorithms/unsafe code
- Contain unsafe code in the smallest possible scope
- Document safety requirements extensively
- Use Result types consistently for error handling with meaningful error messages
- Use snake_case for variables and functions, CamelCase for types
- Follow Rust module structure with clear module boundaries
- Keep the compiler pipeline stages modular and testable
- Always add proper tests for new language features
- Use Make as the primary build interface rather than direct cargo commands