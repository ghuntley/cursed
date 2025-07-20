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
    pkgs.llvmPackages_18.clang
    pkgs.llvmPackages_18.llvm
    pkgs.llvmPackages_18.libllvm
    pkgs.llvmPackages_18.libllvm.dev
    pkgs.llvmPackages_18.mlir
    pkgs.llvmPackages_18.stdenv
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
    pkgs.pkg-config
    pkgs.cacert
    # Cross-compilation toolchains with complete environments
    pkgs.pkgsCross.mingwW64.stdenv.cc     # Windows x86_64 cross-compilation
    pkgs.pkgsCross.mingwW64.stdenv        # Windows standard environment
    pkgs.pkgsCross.mingwW64.buildPackages.gcc
    pkgs.pkgsCross.mingwW64.windows.pthreads  # Windows pthreads library
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
    # Use system clang for native builds (don't set CC/CXX to avoid NIX pollution)
    # cc-rs will find the system compiler automatically
    
    # LLVM configuration
    export LLVM_SYS_181_PREFIX="${pkgs.llvmPackages_18.llvm.dev}"
    export LLVM_CONFIG_PATH="${pkgs.llvmPackages_18.llvm.dev}/bin/llvm-config"

    # Cross-compilation configuration - use target-specific compilers
    export CC_x86_64_unknown_linux_gnu="${pkgs.pkgsCross.gnu64.stdenv.cc}/bin/x86_64-unknown-linux-gnu-gcc"
    export CXX_x86_64_unknown_linux_gnu="${pkgs.pkgsCross.gnu64.stdenv.cc}/bin/x86_64-unknown-linux-gnu-g++"
    export AR_x86_64_unknown_linux_gnu="${pkgs.pkgsCross.gnu64.stdenv.cc}/bin/x86_64-unknown-linux-gnu-ar"



    export CC_aarch64_unknown_linux_gnu="${pkgs.pkgsCross.aarch64-multiplatform.stdenv.cc}/bin/aarch64-unknown-linux-gnu-gcc"
    export CXX_aarch64_unknown_linux_gnu="${pkgs.pkgsCross.aarch64-multiplatform.stdenv.cc}/bin/aarch64-unknown-linux-gnu-g++"
    export AR_aarch64_unknown_linux_gnu="${pkgs.pkgsCross.aarch64-multiplatform.stdenv.cc}/bin/aarch64-unknown-linux-gnu-ar"

    export CC_x86_64_pc_windows_gnu="${pkgs.pkgsCross.mingwW64.stdenv.cc}/bin/x86_64-w64-mingw32-gcc"
    export CXX_x86_64_pc_windows_gnu="${pkgs.pkgsCross.mingwW64.stdenv.cc}/bin/x86_64-w64-mingw32-g++"
    export AR_x86_64_pc_windows_gnu="${pkgs.pkgsCross.mingwW64.stdenv.cc}/bin/x86_64-w64-mingw32-ar"

    # macOS cross-compilation (use clang for x86_64-apple-darwin)
    export CC_x86_64_apple_darwin="${pkgs.clang}/bin/clang"
    export CXX_x86_64_apple_darwin="${pkgs.clang}/bin/clang++"
    export AR_x86_64_apple_darwin="${pkgs.llvmPackages_18.llvm}/bin/llvm-ar"



    # Library paths for runtime and compilation
    export LD_LIBRARY_PATH="${pkgs.libffi}/lib:${pkgs.zlib}/lib:${pkgs.ncurses}/lib:${pkgs.libxml2}/lib:${pkgs.sqlite}/lib:$LD_LIBRARY_PATH"
    export LIBRARY_PATH="${pkgs.libffi}/lib:${pkgs.zlib}/lib:${pkgs.ncurses}/lib:${pkgs.libxml2}/lib:${pkgs.sqlite}/lib:$LIBRARY_PATH"
    export PKG_CONFIG_PATH="${pkgs.libffi.dev}/lib/pkgconfig:${pkgs.zlib.dev}/lib/pkgconfig:${pkgs.ncurses.dev}/lib/pkgconfig:${pkgs.libxml2.dev}/lib/pkgconfig:${pkgs.sqlite.dev}/lib/pkgconfig"

    # Cross-compilation sysroots and linkers
    export CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_LINKER="${pkgs.pkgsCross.gnu64.stdenv.cc}/bin/x86_64-unknown-linux-gnu-gcc"
    export CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER="${pkgs.pkgsCross.aarch64-multiplatform.stdenv.cc}/bin/aarch64-unknown-linux-gnu-gcc"
    export CARGO_TARGET_X86_64_PC_WINDOWS_GNU_LINKER="${pkgs.pkgsCross.mingwW64.stdenv.cc}/bin/x86_64-w64-mingw32-gcc"
    export CARGO_TARGET_X86_64_APPLE_DARWIN_LINKER="${pkgs.clang}/bin/clang"

    # Clear any cross-compilation include paths that might pollute builds
    unset C_INCLUDE_PATH
    unset CPLUS_INCLUDE_PATH
    unset CPATH
    unset LIBRARY_PATH
    unset MACOSX_DEPLOYMENT_TARGET
    
    # CRITICAL: Clear host stdenv cc-wrapper variables that pollute cross-compilation
    # These contain Linux glibc paths that the Windows cross-compiler inherits
    unset NIX_CFLAGS_COMPILE
    unset NIX_CFLAGS_LINK  
    unset NIX_CXXSTDLIB_COMPILE
    unset NIX_CXXSTDLIB_LINK
    unset NIX_LDFLAGS

    # Make sure LLVM tools are available but don't override system compilers for native builds
    export PATH="${pkgs.llvmPackages_18.llvm}/bin:$PATH"
    
    # Add cross-compilation tools to PATH but at the end
    export PATH="$PATH:${pkgs.pkgsCross.gnu64.stdenv.cc}/bin:${pkgs.pkgsCross.aarch64-multiplatform.stdenv.cc}/bin:${pkgs.pkgsCross.mingwW64.stdenv.cc}/bin"
  '';


  # See full reference at https://devenv.sh/reference/options/
}
