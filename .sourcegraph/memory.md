# Cursed Programming Language - Memory

## Build/Lint/Test Commands
- Build: `make build` or `cargo build`
- Test: `make test` or `cargo test`
- Single test: `cargo test test_name` or `cargo test -- --test jit_integration_tests`
- Lint: `make lint` or `cargo clippy -- -D warnings`
- Format check: `make fmt` or `cargo fmt -- --check`
- Format fix: `make fmt-fix` or `cargo fmt`
- Run examples: `make example EXAMPLE=fibonacci` or `./target/debug/cursed examples/fibonacci.csd`