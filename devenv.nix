{ pkgs, lib, config, inputs, ... }:

{
  # https://devenv.sh/basics/
  env.GREET = "devenv";

  # https://devenv.sh/packages/
  packages = [
    pkgs.git
    pkgs.ninja
    pkgs.cmake
    pkgs.llvmPackages_17.clang
    pkgs.llvmPackages_17.llvm
    pkgs.llvmPackages_17.libllvm
    pkgs.llvmPackages_17.libllvm.dev
    pkgs.llvmPackages_17.mlir
    pkgs.llvmPackages_17.stdenv
    pkgs.libffi
    pkgs.libbfd
    pkgs.libffi.dev
    pkgs.libxml2
    pkgs.libxml2.dev
    pkgs.zlib
    pkgs.zlib.dev
    pkgs.ncurses
    pkgs.ncurses.dev
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
  languages.rust.enable = true;
  languages.rust.mold.enable = false;  # Disable mold linker
  languages.swift.enable = true;
  languages.zig.enable = true;


  enterShell = ''
    export LLVM_SYS_170_PREFIX="${pkgs.llvmPackages_17.llvm.dev}"
    export LLVM_CONFIG_PATH="${pkgs.llvmPackages_17.llvm.dev}/bin/llvm-config"
    export LD_LIBRARY_PATH="${pkgs.libffi}/lib:${pkgs.zlib}/lib:${pkgs.ncurses}/lib:${pkgs.libxml2}/lib:$LD_LIBRARY_PATH"
    export PKG_CONFIG_PATH="${pkgs.libffi.dev}/lib/pkgconfig:${pkgs.zlib.dev}/lib/pkgconfig:${pkgs.ncurses.dev}/lib/pkgconfig:${pkgs.libxml2.dev}/lib/pkgconfig:$PKG_CONFIG_PATH"
    export LIBRARY_PATH="${pkgs.libffi}/lib:${pkgs.zlib}/lib:${pkgs.ncurses}/lib:${pkgs.libxml2}/lib:$LIBRARY_PATH"
    # Completely disable mold and force GNU linker
    export CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_LINKER="${pkgs.gcc}/bin/gcc"
    # Create proper mold override that includes the libffi path
    mkdir -p ~/.local/bin
    cat > ~/.local/bin/mold << 'EOF'
#!/bin/bash
# Override mold to use standard ld with libffi path
exec ${pkgs.binutils}/bin/ld -L${pkgs.libffi}/lib -L${pkgs.zlib}/lib -L${pkgs.ncurses}/lib -L${pkgs.libxml2}/lib "$@"
EOF
    chmod +x ~/.local/bin/mold
    export PATH="$HOME/.local/bin:${pkgs.binutils}/bin:${pkgs.gcc}/bin:${pkgs.llvmPackages_17.clang}/bin:$PATH"
    # More aggressive mold override with correct library paths
    export RUSTFLAGS="-C linker=${pkgs.gcc}/bin/gcc -C link-arg=-fuse-ld=${pkgs.binutils}/bin/ld -L ${pkgs.libffi}/lib -L ${pkgs.zlib}/lib -L ${pkgs.ncurses}/lib -L ${pkgs.libxml2}/lib"
    export RUSTDOCFLAGS="-C linker=${pkgs.gcc}/bin/gcc -C link-arg=-fuse-ld=${pkgs.binutils}/bin/ld -L ${pkgs.libffi}/lib -L ${pkgs.zlib}/lib -L ${pkgs.ncurses}/lib -L ${pkgs.libxml2}/lib"
    # Ensure we find the libraries by name
    export C_INCLUDE_PATH="${pkgs.libffi.dev}/include:${pkgs.zlib.dev}/include:${pkgs.ncurses.dev}/include:${pkgs.libxml2.dev}/include/libxml2:$C_INCLUDE_PATH"
    export CPLUS_INCLUDE_PATH="${pkgs.libffi.dev}/include:${pkgs.zlib.dev}/include:${pkgs.ncurses.dev}/include:${pkgs.libxml2.dev}/include/libxml2:$CPLUS_INCLUDE_PATH"
    # Also try to force the linker through environment variables that override system defaults
    export LINKER="${pkgs.binutils}/bin/ld"
    export LD="${pkgs.binutils}/bin/ld"
    # Additional mold override attempts with explicit library references
    export CARGO_BUILD_RUSTFLAGS="$RUSTFLAGS"
    # Remove mold from environment variables
    unset CC_x86_64_unknown_linux_gnu
    unset CXX_x86_64_unknown_linux_gnu
    # Override any mold configuration
    unset MOLD_PATH
    unset USE_MOLD
    # Force environment to use our overridden binaries
    export CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_RUSTFLAGS="$RUSTFLAGS"
  '';


  # See full reference at https://devenv.sh/reference/options/
}
