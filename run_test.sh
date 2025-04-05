#!/bin/bash
# Run standalone generics test
cargo test --test standalone_generics_test -- --nocapture

# Run generic parsing tests if they exist
cargo test test_parse_generics || true