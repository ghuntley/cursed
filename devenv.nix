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
    # LLVM configuration
    export LLVM_SYS_170_PREFIX="${pkgs.llvmPackages_17.llvm.dev}"
    export LLVM_CONFIG_PATH="${pkgs.llvmPackages_17.llvm.dev}/bin/llvm-config"
    
    # Library paths for runtime and compilation
    export LD_LIBRARY_PATH="${pkgs.libffi}/lib:${pkgs.zlib}/lib:${pkgs.ncurses}/lib:${pkgs.libxml2}/lib:$LD_LIBRARY_PATH"
    export LIBRARY_PATH="${pkgs.libffi}/lib:${pkgs.zlib}/lib:${pkgs.ncurses}/lib:${pkgs.libxml2}/lib:$LIBRARY_PATH"
    export PKG_CONFIG_PATH="${pkgs.libffi.dev}/lib/pkgconfig:${pkgs.zlib.dev}/lib/pkgconfig:${pkgs.ncurses.dev}/lib/pkgconfig:${pkgs.libxml2.dev}/lib/pkgconfig:$PKG_CONFIG_PATH"
    
    # Include paths for C/C++
    export C_INCLUDE_PATH="${pkgs.libffi.dev}/include:${pkgs.zlib.dev}/include:${pkgs.ncurses.dev}/include:${pkgs.libxml2.dev}/include/libxml2:$C_INCLUDE_PATH"
    export CPLUS_INCLUDE_PATH="${pkgs.libffi.dev}/include:${pkgs.zlib.dev}/include:${pkgs.ncurses.dev}/include:${pkgs.libxml2.dev}/include/libxml2:$CPLUS_INCLUDE_PATH"
    
    # BFD linker configuration is handled by .cargo/config.toml
    # Just ensure gcc and binutils are available in PATH
    export PATH="${pkgs.gcc}/bin:${pkgs.binutils}/bin:$PATH"
  '';


  # See full reference at https://devenv.sh/reference/options/
}
