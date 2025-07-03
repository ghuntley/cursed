{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  buildInputs = with pkgs; [
    # Core development dependencies
    rustc
    cargo
    rust-analyzer
    rustfmt
    clippy
    
    # System libraries required for linking
    libffi
    ncurses
    libxml2
    sqlite
    
    # Additional tools
    git
    pkg-config
    openssl
    
    # LLVM for the compiler backend
    llvm_17
  ];
  
  shellHook = ''
    echo "🚀 CURSED Development Environment Loaded"
    echo "📦 Dependencies: libffi, ncurses, libxml2, sqlite, LLVM 17"
    echo "🛠️  Tools: cargo, rust-analyzer, rustfmt, clippy"
    echo ""
    echo "To build: cargo build"
    echo "To test:  cargo test"
    echo "To run:   ./target/debug/cursed <file.csd>"
    echo ""
  '';
  
  # Set environment variables for consistent builds
  RUST_BACKTRACE = "1";
  LIBFFI_INCLUDE_DIR = "${pkgs.libffi.dev}/include";
  LIBFFI_LIB_DIR = "${pkgs.libffi.out}/lib";
}
