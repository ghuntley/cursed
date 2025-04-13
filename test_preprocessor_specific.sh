#!/bin/sh

# Run the specific test we need
cd tests
devenv shell rustc -L ../target/debug/deps --test preprocessor_integration_test.rs -o preprocessor_test
cd ..
./tests/preprocessor_test