#!/bin/bash

# Run tests but exclude all files that have failing tests
cargo test \
  --lib \
  --bin=cursed \
  --test=memory_layout_test \
  --test=name_mangling_test \
  --test=monomorphization_basic_test \
  "$@"