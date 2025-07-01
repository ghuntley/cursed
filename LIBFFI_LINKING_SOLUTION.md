# LibFFI Linking Solution for CURSED Compiler on NixOS

## Problem Summary
The CURSED compiler was failing to build binaries due to libffi linking issues in the NixOS environment, despite the library check passing with `cargo check --lib`.

## Root Cause Analysis
1. **Environment Variable Override**: The `RUSTFLAGS` environment variable was set to `-C link-arg=-fuse-ld=mold`, forcing the use of the mold linker.
2. **Incompatible GCC Option**: The `.cargo/config.toml` contained `-C link-arg=-fuse-ld=ld` which is not supported by GNU gcc (only clang/LLVM).
3. **Mold Linker Issue**: The mold linker couldn't find the libffi library despite correct library paths.

## Solution Applied

### 1. Fixed .cargo/config.toml
- Removed the unsupported `-C link-arg=-fuse-ld=ld` flag
- Kept library paths: `-L /nix/store/6pak77li0iw9x0b3yhmbjvp846w3p6bx-libffi-3.4.6/lib`
- Kept runtime paths: `-C link-arg=-Wl,-rpath,/nix/store/6pak77li0iw9x0b3yhmbjvp846w3p6bx-libffi-3.4.6/lib`

### 2. Environment Variable Management
- Ensured `RUSTFLAGS` environment variable is unset during builds to prevent mold linker override
- Used `unset RUSTFLAGS && cargo build` for compilation

### 3. Verification Steps
- Confirmed libffi library exists at expected path
- Created simple FFI test to verify linking works with direct rustc
- Built multiple binaries successfully
- Verified runtime linking with `ldd` command

## Current Working Configuration

**File: .cargo/config.toml**
```toml
[target.x86_64-unknown-linux-gnu]
linker = "gcc"
rustflags = [
    "-L", "/nix/store/6pak77li0iw9x0b3yhmbjvp846w3p6bx-libffi-3.4.6/lib",
    "-L", "/nix/store/k3a7dzrqphj9ksbb43i24vy6inz8ys51-ncurses-6.4.20221231/lib", 
    "-L", "/nix/store/0z4hrksbdrwv9xb8ycjk3rq9ppmw0350-libxml2-2.13.5/lib",
    "-C", "link-arg=-Wl,-rpath,/nix/store/6pak77li0iw9x0b3yhmbjvp846w3p6bx-libffi-3.4.6/lib",
    "-C", "link-arg=-Wl,-rpath,/nix/store/k3a7dzrqphj9ksbb43i24vy6inz8ys51-ncurses-6.4.20221231/lib",
    "-C", "link-arg=-Wl,-rpath,/nix/store/0z4hrksbdrwv9xb8ycjk3rq9ppmw0350-libxml2-2.13.5/lib",
    "-C", "link-arg=-Wl,-Bdynamic"
]

[build]
target = "x86_64-unknown-linux-gnu"

[env]
CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_LINKER = "gcc"
```

## Build Commands
```bash
# Ensure RUSTFLAGS is not set to avoid mold linker override
unset RUSTFLAGS

# Build the main binary
cargo build --bin cursed

# Build other binaries
cargo build --bin cursed-repl
cargo build --bin cursed-test

# Verify linking
ldd ./target/x86_64-unknown-linux-gnu/debug/cursed | grep ffi
```

## Verification Results
- ✅ Library check passes: `cargo check --lib`
- ✅ Binary builds successfully: `cargo build --bin cursed`  
- ✅ LibFFI properly linked: `libffi.so.8 => /nix/store/.../libffi-3.4.6/lib/libffi.so.8`
- ✅ Binary executes correctly: `./target/.../cursed --help`
- ✅ Simple CURSED program runs: `./target/.../cursed simple_test.csd`

## Key Takeaways
1. Environment variables take precedence over .cargo/config.toml settings
2. GNU gcc doesn't support `-fuse-ld=ld` (clang-specific option)
3. Proper library and runtime paths are essential for NixOS linking
4. The mold linker can cause issues with certain library configurations
5. Testing both direct rustc compilation and cargo builds helps isolate issues

## Future Considerations
- Consider creating a build script that automatically unsets RUSTFLAGS
- Monitor for any NixOS store path changes that might affect library locations
- Document this solution for other developers working in similar environments
