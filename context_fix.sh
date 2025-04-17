#!/bin/bash
# Extract the LlvmCodeGenerator struct and implementation
grep -v '// Import of other module implementations is done in respective files' src/codegen/llvm/context.rs > fixed_context.rs
# Move the fixed file over the original
mv fixed_context.rs src/codegen/llvm/context.rs