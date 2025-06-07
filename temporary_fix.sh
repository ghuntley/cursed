#!/bin/bash
# Temporary workaround to bypass mold linking issues

# Create a temporary directory for our fake mold
TEMP_BIN_DIR=$(mktemp -d)

# Create a script that forwards to ld instead of mold
cat > "$TEMP_BIN_DIR/mold" << 'EOF'
#!/bin/bash
# Forward mold calls to ld with library paths
exec /nix/store/bwkb907myixfzzykp21m9iczkhrq5pfy-binutils-2.43.1/bin/ld \
  -L/nix/store/6pak77li0iw9x0b3yhmbjvp846w3p6bx-libffi-3.4.6/lib \
  -L/nix/store/l5g2v1jgfyf3j0jp9iv5b79fi8yrwzpp-zlib-1.3.1/lib \
  -L/nix/store/k3a7dzrqphj9ksbb43i24vy6inz8ys51-ncurses-6.4.20221231/lib \
  -L/nix/store/0z4hrksbdrwv9xb8ycjk3rq9ppmw0350-libxml2-2.13.5/lib \
  "$@"
EOF
chmod +x "$TEMP_BIN_DIR/mold"

# Override PATH to use our fake mold first
export PATH="$TEMP_BIN_DIR:$PATH"

# Override other environment variables
export CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_LINKER=gcc
unset RUSTFLAGS
unset RUSTDOCFLAGS

echo "Running cargo test with library path workaround..."
cargo test --no-run

# Clean up
rm -rf "$TEMP_BIN_DIR"
