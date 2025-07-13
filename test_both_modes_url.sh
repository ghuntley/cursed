#!/bin/bash

echo "Testing URL parsing module in both modes..."

echo "=== Interpretation Mode ==="
cargo run --bin cursed stdlib/url_parsing/test_url_parsing.csd > interp_output.txt 2>&1

echo "=== Compilation Mode ==="
cargo run --bin cursed -- compile stdlib/url_parsing/test_url_parsing.csd > compile_output.txt 2>&1
./test_url_parsing > exec_output.txt 2>&1

echo "=== Comparing Outputs ==="
echo "Interpretation output:"
head -10 interp_output.txt

echo ""
echo "Compiled executable output:"
head -10 exec_output.txt

echo ""
echo "Files created:"
ls -la test_url_parsing* *.txt
