#!/bin/bash
# Wrapper script for CURSED compiler
exec cargo run --bin cursed -- "$@"
