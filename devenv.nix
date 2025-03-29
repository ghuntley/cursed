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
    pkgs.libxml2
  ];

  # https://devenv.sh/languages/
  languages.rust.enable = true;

  env.LLVM_CONFIG_PATH = "${pkgs.llvmPackages_17.llvm}/bin/llvm-config";
  env.LLVM_SYS_170_PREFIX = "${pkgs.llvmPackages_17.libllvm.dev}";

  enterShell = ''
    export LLVM_SYS_170_PREFIX="${pkgs.llvmPackages_17.libllvm.dev}"
    export LLVM_CONFIG_PATH="${pkgs.llvmPackages_17.llvm}/bin/llvm-config"
  '';


  # See full reference at https://devenv.sh/reference/options/
}
