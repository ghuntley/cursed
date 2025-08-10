{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  buildInputs = with pkgs; [
    # Core development dependencies (Zig-based)
    zig
    
    # System libraries required for linking
    libffi
    ncurses
    libxml2
    sqlite
    
    # Additional tools
    git
    pkg-config
    openssl
    valgrind  # For memory safety validation
    
    # LLVM for the compiler backend
    llvm_17
    
    # Development tools
    gdb        # Debugging
    strace     # System call tracing
  ];
  
  shellHook = ''
    echo "🚀 CURSED Development Environment Loaded (Zig Implementation)"
    echo "📦 Dependencies: libffi, ncurses, libxml2, sqlite, LLVM 17"
    echo "🛠️  Tools: zig, valgrind, gdb"
    echo ""
    echo "⚡ Fast builds with Zig:"
    echo "  Build:    zig build                    (0.1-0.2s)"
    echo "  Test:     zig build test              (includes memory checks)"
    echo "  Run:      ./zig-out/bin/cursed-zig file.csd"
    echo "  Stable:   ./zig-out/bin/cursed-stable file.csd"
    echo "  LSP:      ./zig-out/bin/cursed-lsp"
    echo ""
    echo "🔍 Memory validation:"
    echo "  valgrind ./zig-out/bin/cursed-zig file.csd"
    echo ""
    echo "📦 Rust implementation archived in: archive/rust-implementation/"
    echo ""
  '';
  
  # Set environment variables for consistent builds
  LLVM_SYS_170_PREFIX = "${pkgs.llvm_17}";
}
