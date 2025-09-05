#!/bin/bash

echo "=== Testing CURSED Interpreter with Fixed Stdlib ==="
echo

cd test_suite

# Test specific stdlib functions that were fixed
echo "1. Testing stringz module:"
../zig-out/bin/cursed-compiler --interpret test_programs/stdlib/02_stringz_basic.💀
echo

echo "2. Testing mathz module:"  
../zig-out/bin/cursed-compiler --interpret test_programs/stdlib/03_mathz_advanced.💀
echo

echo "3. Testing stdlib integration:"
../zig-out/bin/cursed-compiler --interpret test_programs/stdlib/01_stdlib_integration_basic.💀
echo

echo "4. Testing comprehensive stdlib:"
../zig-out/bin/cursed-compiler --interpret test_programs/validation/validation_stdlib_complete.💀
echo

echo "=== Interpreter Test Summary Complete ==="
