# Cursed Programming Language - Memory

## Garbage Collector Implementation

### Weak Reference System
We've implemented a robust weak reference system with the following features:

- Global registry to maintain GC connections even after strong references are dropped
- Reference counting to track when objects can be fully removed from the registry
- Non-blocking operations to prevent deadlocks during parallel execution
- Proper integration with the garbage collector via non-blocking operations
- Clear API with upgrade/downgrade operations and liveness checking
- Support for tracking circular references
- Thread-safe implementation using timeouts to prevent deadlocks

### Key Files:
- `src/memory/weak.rs` - Main implementation of weak references
- `src/memory/mark_sweep.rs` - Improved mark-and-sweep garbage collection algorithm
- `src/memory/gc.rs` - Garbage collector API and internal state management

### Known Limitations:
1. The test environment has deadlock issues when running parallel tests
2. Finalization of objects could be improved to directly access Traceable objects
3. The current implementation uses a simplified approach for cycle detection

### Future Improvements:
1. Full reimplementation of object storage to allow direct access to Traceable objects
2. Better integration between GC and weak references to avoid registry lookups
3. More sophisticated finalization with finalization ordering
4. Improved deadlock prevention in the GC implementation

## Build/Lint/Test Commands
- Build: `devenv shell make build` or `devenv shell cargo build`
- Test: `devenv shell make test` or `devenv shell cargo test`
- Single test: `devenv shell cargo test test_name` or `devenv shell cargo test -- --test jit_integration_tests`
- Lint: `devenv shell devenv shell make lint` or `devenv shell cargo clippy -- -D warnings`
- Format check: `devenv shell make fmt` or `devenv shell cargo fmt -- --check`
- Format fix: `devenv shell make fmt-fix` or `cargo fmt`
- Run examples: `devenv shell make example EXAMPLE=fibonacci` or `devenv shell ./target/debug/cursed examples/fibonacci.csd`