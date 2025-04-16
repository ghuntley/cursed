# Cursed Programming Language - Memory

## Build/Lint/Test Commands
- Build: `make build` or `cargo build`
- Test: `make test` or `cargo test`
- Single test: `cargo test test_name` or `cargo test -- --test jit_integration_tests`
- Lint: `make lint` or `cargo clippy -- -D warnings`
- Format check: `make fmt` or `cargo fmt -- --check`
- Format fix: `make fmt-fix` or `cargo fmt`
- Run examples: `make example EXAMPLE=fibonacci` or `./target/debug/cursed examples/fibonacci.csd`

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