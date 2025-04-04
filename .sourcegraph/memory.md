# Cursed Programming Language - Memory

## Build/Lint/Test Commands
- Build: `devenv shell make build` or `devenv shell cargo build`
- Test: `devenv shell make test` or `devenv shell cargo test`
- Single test: `devenv shell cargo test test_name` or `devenv shell cargo test -- --test jit_integration_tests`
- Lint: `devenv shell devenv shell make lint` or `devenv shell cargo clippy -- -D warnings`
- Format check: `devenv shell make fmt` or `devenv shell cargo fmt -- --check`
- Format fix: `devenv shell make fmt-fix` or `cargo fmt`
- Run examples: `devenv shell make example EXAMPLE=fibonacci` or `devenv shell ./target/debug/cursed examples/fibonacci.csd`

## Bug Fixes
- In `parse_break_statement` don't advance tokens after seeing a semicolon, as that's handled by the calling function
- Fixed the parse_while_statement method to properly handle parentheses around conditions

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