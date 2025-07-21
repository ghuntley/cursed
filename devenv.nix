{ pkgs, lib, config, inputs, ... }:

let
  fenix = inputs.fenix.packages.${pkgs.system};
  rustToolchain = fenix.combine [
    fenix.stable.cargo
    fenix.stable.rustc
    fenix.stable.clippy
    fenix.stable.rustfmt
    fenix.stable.rust-src      # for rust-analyzer
    fenix.targets.aarch64-apple-darwin.stable.rust-std
    fenix.targets.x86_64-apple-darwin.stable.rust-std
    fenix.targets.aarch64-unknown-linux-gnu.stable.rust-std
    fenix.targets.x86_64-unknown-linux-gnu.stable.rust-std
    fenix.targets.x86_64-pc-windows-gnu.stable.rust-std
    fenix.targets.wasm32-unknown-unknown.stable.rust-std
  ];
in

{
  # https://devenv.sh/basics/
  env.GREET = "devenv";

  # https://devenv.sh/packages/
  packages = [
    # Fenix Rust toolchain with cross-compilation support
    rustToolchain
    pkgs.git
    pkgs.ninja
    pkgs.cmake
    # Use LLVM 18 (closest stable to Rust's LLVM 20.1.5)
    pkgs.llvmPackages_18.clang
    pkgs.llvmPackages_18.llvm
    pkgs.llvmPackages_18.libllvm
    pkgs.llvmPackages_18.libllvm.dev
    pkgs.llvmPackages_18.stdenv
    # Zig as universal fallback linker
    pkgs.zig
    # C compiler and build tools for cc-rs
    pkgs.gcc
    pkgs.binutils
    pkgs.libffi
    pkgs.libffi.dev
    pkgs.libxml2
    pkgs.libxml2.dev
    pkgs.zlib
    pkgs.zlib.dev
    pkgs.ncurses
    pkgs.ncurses.dev
    # SQLite dependencies
    pkgs.sqlite
    pkgs.sqlite.dev
    # Package manager additional dependencies
    pkgs.curl
    pkgs.openssl
    pkgs.openssl.dev
    pkgs.pkg-config
    pkgs.cacert
    # libiconv for build scripts
    pkgs.libiconv
    # Cross-compilation toolchains with complete environments
    pkgs.pkgsCross.mingwW64.stdenv.cc     # Windows x86_64 cross-compilation
    pkgs.pkgsCross.mingwW64.stdenv        # Windows standard environment
    pkgs.pkgsCross.mingwW64.buildPackages.gcc
    # Windows threading libraries - complete pthread stack
    pkgs.pkgsCross.mingwW64.windows.mcfgthreads        # Modern Windows threading library
    pkgs.pkgsCross.mingwW64.windows.mingw_w64_pthreads # Primary Windows pthreads library
    pkgs.pkgsCross.mingwW64.windows.pthreads          # Fallback pthreads implementation
    # Essential Windows runtime libraries for complete toolchain
    pkgs.pkgsCross.mingwW64.windows.mingw_w64         # Core MinGW-w64 runtime
    pkgs.wasm-pack                        # WebAssembly toolchain
    # Linux cross-compilation with complete toolchains
    pkgs.pkgsCross.gnu64.stdenv.cc        # Linux x86_64 cross-compiler
    pkgs.pkgsCross.gnu64.glibc            # Linux x86_64 system libraries
    pkgs.pkgsCross.gnu64.glibc.dev        # Linux x86_64 headers
    pkgs.pkgsCross.gnu64.buildPackages.gcc
    pkgs.pkgsCross.aarch64-multiplatform.stdenv.cc  # Linux ARM64 cross-compiler
    pkgs.pkgsCross.aarch64-multiplatform.glibc      # Linux ARM64 system libraries
    pkgs.pkgsCross.aarch64-multiplatform.glibc.dev  # Linux ARM64 headers
    pkgs.pkgsCross.aarch64-multiplatform.buildPackages.gcc
    # Additional cross-compilation dependencies
    pkgs.pkgsCross.gnu64.zlib
    pkgs.pkgsCross.gnu64.openssl
  ] ++ lib.optionals pkgs.stdenv.isLinux [
    # Linux-specific packages
    pkgs.libbfd
    pkgs.glibc
    pkgs.glibc.dev
  ] ++ lib.optionals pkgs.stdenv.isDarwin [
    # macOS-specific packages
    pkgs.darwin.apple_sdk.frameworks.CoreFoundation
    pkgs.darwin.apple_sdk.frameworks.Security
    pkgs.darwin.apple_sdk.frameworks.SystemConfiguration

  ];

  # https://devenv.sh/languages/
  languages.c.enable = true;
  languages.clojure.enable = true;
  languages.cplusplus.enable = true;
  languages.dotnet.enable = true;
  languages.erlang.enable = true;
  languages.fortran.enable = true;
  languages.go.enable = true;
  languages.haskell.enable = true;
  languages.java.enable = true;
  languages.javascript.enable = true;
  languages.kotlin.enable = true;
  languages.ocaml.enable = true;
  languages.pascal.enable = true;
  languages.perl.enable = true;
  languages.php.enable = true;
  languages.python.enable = true;
  languages.ruby.enable = true;
  # Disable built-in Rust (we use fenix directly)
  languages.rust.enable = false;
  languages.swift.enable = true;
  languages.zig.enable = true;


  enterShell = ''
    echo "🔧 Setting up CURSED development environment..."

    # LLVM configuration - using LLVM 18 for stability
    export LLVM_SYS_181_PREFIX="${pkgs.llvmPackages_18.llvm.dev}"
    export LLVM_CONFIG_PATH="${pkgs.llvmPackages_18.llvm.dev}/bin/llvm-config"

    # Debug LLVM setup
    echo "📋 LLVM Debug Information:"
    echo "  LLVM_SYS_181_PREFIX: $LLVM_SYS_181_PREFIX"
    echo "  LLVM_CONFIG_PATH: $LLVM_CONFIG_PATH"
    echo "  LLVM version: $(${pkgs.llvmPackages_18.llvm.dev}/bin/llvm-config --version 2>/dev/null || echo 'N/A')"
    echo "  Rust LLVM: $(rustc -vV | grep 'LLVM version' || echo 'N/A')"
    echo "  llvm-ar path: ${pkgs.llvmPackages_18.llvm}/bin/llvm-ar"
    echo "  llvm-ar exists: $(test -f ${pkgs.llvmPackages_18.llvm}/bin/llvm-ar && echo 'YES' || echo 'NO')"

    # Cross-compilation configuration - use target-specific compilers
    # Skip native x86_64 Linux cross-compilation (we're already on x86_64 Linux)
    # export CC_x86_64_unknown_linux_gnu="${pkgs.pkgsCross.gnu64.stdenv.cc}/bin/x86_64-unknown-linux-gnu-gcc"
    # export CXX_x86_64_unknown_linux_gnu="${pkgs.pkgsCross.gnu64.stdenv.cc}/bin/x86_64-unknown-linux-gnu-g++"
    # export AR_x86_64_unknown_linux_gnu="${pkgs.pkgsCross.gnu64.stdenv.cc}/bin/x86_64-unknown-linux-gnu-ar"

    export CC_aarch64_unknown_linux_gnu="${pkgs.pkgsCross.aarch64-multiplatform.stdenv.cc}/bin/aarch64-unknown-linux-gnu-gcc"
    export CXX_aarch64_unknown_linux_gnu="${pkgs.pkgsCross.aarch64-multiplatform.stdenv.cc}/bin/aarch64-unknown-linux-gnu-g++"
    export AR_aarch64_unknown_linux_gnu="${pkgs.pkgsCross.aarch64-multiplatform.stdenv.cc}/bin/aarch64-unknown-linux-gnu-ar"

    export CC_x86_64_pc_windows_gnu="${pkgs.pkgsCross.mingwW64.stdenv.cc}/bin/x86_64-w64-mingw32-gcc"
    export CXX_x86_64_pc_windows_gnu="${pkgs.pkgsCross.mingwW64.stdenv.cc}/bin/x86_64-w64-mingw32-g++"
    export AR_x86_64_pc_windows_gnu="${pkgs.pkgsCross.mingwW64.stdenv.cc}/bin/x86_64-w64-mingw32-ar"

    # Windows pthread library paths - ensure all pthread libraries are in search path
    export CARGO_TARGET_X86_64_PC_WINDOWS_GNU_RUSTFLAGS="-L ${pkgs.pkgsCross.mingwW64.windows.mingw_w64_pthreads}/lib -L ${pkgs.pkgsCross.mingwW64.windows.mcfgthreads}/lib -L ${pkgs.pkgsCross.mingwW64.windows.pthreads}/lib -C link-arg=-Wl,-L${pkgs.pkgsCross.mingwW64.windows.mingw_w64_pthreads}/lib -C link-arg=-Wl,-L${pkgs.pkgsCross.mingwW64.windows.mcfgthreads}/lib"

    # MinGW linker library search paths - critical for finding libpthread.a
    export LIBRARY_PATH_x86_64_pc_windows_gnu="${pkgs.pkgsCross.mingwW64.windows.mingw_w64_pthreads}/lib:${pkgs.pkgsCross.mingwW64.windows.mcfgthreads}/lib:${pkgs.pkgsCross.mingwW64.windows.pthreads}/lib"

    # Windows linker flags - ensure libpthread.a can be found
    export CARGO_TARGET_X86_64_PC_WINDOWS_GNU_LDFLAGS="-L${pkgs.pkgsCross.mingwW64.windows.mingw_w64_pthreads}/lib -L${pkgs.pkgsCross.mingwW64.windows.mcfgthreads}/lib -L${pkgs.pkgsCross.mingwW64.windows.pthreads}/lib"

    # Critical: Set include and library search paths for MinGW cross-compilation
    export CPATH_x86_64_pc_windows_gnu="${pkgs.pkgsCross.mingwW64.windows.mingw_w64_pthreads}/include:${pkgs.pkgsCross.mingwW64.windows.mcfgthreads}/include:${pkgs.pkgsCross.mingwW64.windows.pthreads}/include"
    export C_INCLUDE_PATH_x86_64_pc_windows_gnu="${pkgs.pkgsCross.mingwW64.windows.mingw_w64_pthreads}/include:${pkgs.pkgsCross.mingwW64.windows.mcfgthreads}/include:${pkgs.pkgsCross.mingwW64.windows.pthreads}/include"
    export CPLUS_INCLUDE_PATH_x86_64_pc_windows_gnu="${pkgs.pkgsCross.mingwW64.windows.mingw_w64_pthreads}/include:${pkgs.pkgsCross.mingwW64.windows.mcfgthreads}/include:${pkgs.pkgsCross.mingwW64.windows.pthreads}/include"
    export LIBRARY_PATH="${pkgs.pkgsCross.mingwW64.windows.mingw_w64_pthreads}/lib:${pkgs.pkgsCross.mingwW64.windows.mcfgthreads}/lib:${pkgs.pkgsCross.mingwW64.windows.pthreads}/lib:$LIBRARY_PATH"

    # macOS cross-compilation (use clang for x86_64-apple-darwin)
    export CC_x86_64_apple_darwin="${pkgs.clang}/bin/clang"
    export CXX_x86_64_apple_darwin="${pkgs.clang}/bin/clang++"
    export AR_x86_64_apple_darwin="${pkgs.llvmPackages_18.llvm}/bin/llvm-ar"

    # Cross-compilation linkers - only for actual cross-compilation
    # For native x86_64 Linux builds, use the native linker (don't override)
    # export CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_LINKER="${pkgs.pkgsCross.gnu64.stdenv.cc}/bin/x86_64-unknown-linux-gnu-gcc"
    export CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER="${pkgs.pkgsCross.aarch64-multiplatform.stdenv.cc}/bin/aarch64-unknown-linux-gnu-gcc"
    export CARGO_TARGET_X86_64_PC_WINDOWS_GNU_LINKER="${pkgs.pkgsCross.mingwW64.stdenv.cc}/bin/x86_64-w64-mingw32-gcc"
    export CARGO_TARGET_X86_64_APPLE_DARWIN_LINKER="${pkgs.clang}/bin/clang"

    # Zig universal linker as fallback for complex cross-compilation
    export ZIG_CC="${pkgs.zig}/bin/zig cc"
    export ZIG_CXX="${pkgs.zig}/bin/zig c++"

    # Force LLVM ar/ranlib for cross-compilation targets only
    export CARGO_TARGET_AARCH64_APPLE_DARWIN_AR="${pkgs.llvmPackages_18.llvm}/bin/llvm-ar"
    export CARGO_TARGET_X86_64_APPLE_DARWIN_AR="${pkgs.llvmPackages_18.llvm}/bin/llvm-ar"

    # Clear problematic variables that could interfere (set CC/CXX later)
    unset STRIP
    unset OBJCOPY
    unset C_INCLUDE_PATH
    unset CPLUS_INCLUDE_PATH
    unset CPATH

    # CRITICAL: Clear host stdenv cc-wrapper variables that pollute cross-compilation
    unset NIX_CFLAGS_COMPILE
    unset NIX_CFLAGS_LINK
    unset NIX_CXXSTDLIB_COMPILE
    unset NIX_CXXSTDLIB_LINK
    unset NIX_LDFLAGS

    # CRITICAL FIX: Set linker library search path for MinGW to find libpthread.a
    export RUSTFLAGS_x86_64_pc_windows_gnu="-L ${pkgs.pkgsCross.mingwW64.windows.mingw_w64_pthreads}/lib -L ${pkgs.pkgsCross.mingwW64.windows.mcfgthreads}/lib -L ${pkgs.pkgsCross.mingwW64.windows.pthreads}/lib"

    # Set library paths for native builds AFTER clearing cross-compilation pollution
    # Note: Use the main sqlite package (not sqlite.bin) for libraries
    # Get sqlite library path from pkg-config to ensure correct Nix store path
    SQLITE_LIB_PATH=$(pkg-config --variable=libdir sqlite3 2>/dev/null || echo "${pkgs.sqlite}/lib")
    export SQLITE3_LIB_DIR="$SQLITE_LIB_PATH"
    export SQLITE3_INCLUDE_DIR="${pkgs.sqlite.dev}/include"
    export LD_LIBRARY_PATH="$SQLITE_LIB_PATH:${pkgs.libffi}/lib:${pkgs.zlib}/lib:${pkgs.ncurses}/lib:${pkgs.libxml2}/lib:${pkgs.openssl}/lib:${pkgs.libiconv}/lib:$LD_LIBRARY_PATH"
    export LIBRARY_PATH="$SQLITE_LIB_PATH:${pkgs.libffi}/lib:${pkgs.zlib}/lib:${pkgs.ncurses}/lib:${pkgs.libxml2}/lib:${pkgs.openssl}/lib:${pkgs.libiconv}/lib"
    export PKG_CONFIG_PATH="${pkgs.libffi.dev}/lib/pkgconfig:${pkgs.zlib.dev}/lib/pkgconfig:${pkgs.ncurses.dev}/lib/pkgconfig:${pkgs.sqlite.dev}/lib/pkgconfig:${pkgs.openssl.dev}/lib/pkgconfig"
    
    # SQLite3 specific configuration - use direct paths for reliability
    export SQLITE3_STATIC=0  # Use dynamic linking
    
    # Ensure SQLite3 can be found by the linker - add to RUSTFLAGS for cargo
    export RUSTFLAGS="$RUSTFLAGS -L $SQLITE_LIB_PATH -C link-arg=-Wl,-rpath,$SQLITE_LIB_PATH"
    export LDFLAGS="-L$SQLITE_LIB_PATH -Wl,-rpath,$SQLITE_LIB_PATH $LDFLAGS"
    
    # Additional library search paths for linker
    export CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_RUSTFLAGS="-L $SQLITE_LIB_PATH -C link-arg=-Wl,-rpath,$SQLITE_LIB_PATH"

    # CRITICAL: Set native compiler environment for cc-rs AFTER clearing
    export CC="${pkgs.gcc}/bin/gcc"
    export CXX="${pkgs.gcc}/bin/g++"
    export AR="${pkgs.binutils}/bin/ar"
    export RANLIB="${pkgs.binutils}/bin/ranlib"

    # Debug compiler setup
    echo "🔧 Final Compiler Configuration:"
    echo "  CC: $CC"
    echo "  CXX: $CXX"
    echo "  AR: $AR"
    echo "  RANLIB: $RANLIB"
    
    # Debug SQLite3 setup
    echo "📊 SQLite3 Configuration:"
    echo "  SQLITE3_LIB_DIR: $SQLITE3_LIB_DIR"
    echo "  SQLITE3_INCLUDE_DIR: $SQLITE3_INCLUDE_DIR"
    echo "  SQLite3 library exists: $(test -f ${pkgs.sqlite}/lib/libsqlite3.so && echo 'YES' || echo 'NO')"
    echo "  pkg-config sqlite3: $(pkg-config --exists sqlite3 && echo 'YES' || echo 'NO')"
    echo "  LIBRARY_PATH contains sqlite: $(echo $LIBRARY_PATH | grep -q sqlite && echo 'YES' || echo 'NO')"
    echo "  RUSTFLAGS contains sqlite: $(echo $RUSTFLAGS | grep -q sqlite && echo 'YES' || echo 'NO')"

    # Critical: macOS-specific fixes for cc-rs build scripts finding libiconv
    ${lib.optionalString pkgs.stdenv.isDarwin ''
      # Ensure cc-rs can find system clang (not cross-compilers)
      export PATH="/usr/bin:$PATH"

      # Set linker and compiler flags so cc-rs can find libiconv
      export LDFLAGS="-L${pkgs.libiconv}/lib $LDFLAGS"
      export CPPFLAGS="-I${pkgs.libiconv}/include $CPPFLAGS"
      export CFLAGS="-I${pkgs.libiconv}/include $CFLAGS"
      export CXXFLAGS="-I${pkgs.libiconv}/include $CXXFLAGS"

      # Ensure pkg-config can find libiconv if packages support it
      export PKG_CONFIG_PATH="${pkgs.libiconv}/lib/pkgconfig:$PKG_CONFIG_PATH"

      # Try using GNU ar for native builds on macOS to avoid LLVM version conflicts
      export AR="${pkgs.binutils}/bin/ar"
      export RANLIB="${pkgs.binutils}/bin/ranlib"
    ''}

    # Make sure LLVM tools are available (updated to LLVM 20)
    export PATH="${pkgs.llvmPackages_18.llvm}/bin:$PATH"

    # Add Zig to PATH for universal cross-compilation fallback
    export PATH="${pkgs.zig}/bin:$PATH"

    # For macOS native builds, add GNU binutils BEFORE system tools to avoid LLVM version conflicts
    ${lib.optionalString pkgs.stdenv.isDarwin ''
      export PATH="${pkgs.binutils}/bin:$PATH"
    ''}

    # Add cross-compilation tools to PATH but at the end (after system tools)
    export PATH="$PATH:${pkgs.pkgsCross.gnu64.stdenv.cc}/bin:${pkgs.pkgsCross.aarch64-multiplatform.stdenv.cc}/bin:${pkgs.pkgsCross.mingwW64.stdenv.cc}/bin"

    # Debug final environment
    echo "🔍 Final Environment Check:"
    echo "  which llvm-ar: $(which llvm-ar 2>/dev/null || echo 'NOT FOUND')"
    echo "  which llvm-config: $(which llvm-config 2>/dev/null || echo 'NOT FOUND')"
    echo "  PATH includes LLVM: $(echo $PATH | grep -q llvm && echo 'YES' || echo 'NO')"
    echo "  Native CC: $(which cc 2>/dev/null || echo 'NOT FOUND')"
    echo "  Native AR: $(which ar 2>/dev/null || echo 'NOT FOUND')"
    echo "✅ Environment setup complete!"
  '';


  # See full reference at https://devenv.sh/reference/options/
}
