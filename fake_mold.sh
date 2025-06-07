#!/bin/bash
# Fake mold script that uses ld instead
exec /nix/store/bwkb907myixfzzykp21m9iczkhrq5pfy-binutils-2.43.1/bin/ld "$@"
