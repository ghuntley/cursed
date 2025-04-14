# Cursed Programming Language - Memory

## IMPORTANT
- After tests pass, git commit all of the changes and push to GitHub.
- Don't do placeholder implementations. Do the full implementaiton. If there are two implementations migrate to a single implementation.
- Don't skip or ignore tests.

## Build/Lint/Test Commands
- Build: `devenv shell make build` or `devenv shell cargo build`
- Test: `devenv shell make test` or `devenv shell cargo test`
- Single test: `devenv shell cargo test test_name` or `devenv shell cargo test -- --test jit_integration_tests`
- Lint: `devenv shell devenv shell make lint` or `devenv shell cargo clippy -- -D warnings`
- Format check: `devenv shell make fmt` or `devenv shell cargo fmt -- --check`
- Format fix: `devenv shell make fmt-fix` or `cargo fmt`
- Run examples: `devenv shell make example EXAMPLE=fibonacci` or `devenv shell ./target/debug/cursed examples/fibonacci.csd`