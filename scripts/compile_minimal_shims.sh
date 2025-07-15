#!/bin/bash

echo "Compiling minimal C shims..."

# Compile minimal shims to static library
cd runtime
gcc -c -fPIC minimal_shims.c -o minimal_shims.o
ar rcs libcursed_minimal_shims.a minimal_shims.o

# Compile interface runtime to static library
gcc -c -fPIC interface_runtime.c -o interface_runtime.o
ar rcs libcursed_interface_runtime.a interface_runtime.o

# Compile type assertion runtime to static library
gcc -c -fPIC type_assertion_runtime.c -o type_assertion_runtime.o
ar rcs libcursed_type_assertion_runtime.a type_assertion_runtime.o

echo "Compiled minimal shims to runtime/libcursed_minimal_shims.a"
echo "Compiled interface runtime to runtime/libcursed_interface_runtime.a"
echo "Compiled type assertion runtime to runtime/libcursed_type_assertion_runtime.a"
