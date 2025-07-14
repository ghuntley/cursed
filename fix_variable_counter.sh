#!/bin/bash

# Fix variable_counter references in main.rs
sed -i 's/self\.variable_counter/self.register_tracker.get_current_counter()/g' src/codegen/llvm/main.rs
sed -i 's/self\.variable_counter +=/self.register_tracker.increment_counter(/g' src/codegen/llvm/main.rs
sed -i 's/self\.variable_counter =/self.register_tracker.set_counter(/g' src/codegen/llvm/main.rs
sed -i 's/self\.variable_counter - /self.register_tracker.get_current_counter() - /g' src/codegen/llvm/main.rs

echo "Fixed variable_counter references"
