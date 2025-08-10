# Homebrew Formula for CURSED Compiler
# Enterprise-ready package management for macOS

class Cursed < Formula
  desc "CURSED Programming Language Compiler"
  homepage "https://cursed.dev"
  version "1.0.0"
  license "MIT"

  # Platform-specific URLs and checksums
  on_macos do
    if Hardware::CPU.arm?
      url "https://github.com/ghuntley/cursed/releases/download/v#{version}/cursed-#{version}-aarch64-apple-darwin.tar.gz"
      sha256 "0000000000000000000000000000000000000000000000000000000000000000"  # Update with actual checksum
    else
      url "https://github.com/ghuntley/cursed/releases/download/v#{version}/cursed-#{version}-x86_64-apple-darwin.tar.gz"
      sha256 "0000000000000000000000000000000000000000000000000000000000000000"  # Update with actual checksum
    end
  end

  # Dependencies
  depends_on "llvm@18"
  depends_on "zig" => :build

  # Runtime requirements
  depends_on :macos => :monterey

  def install
    # Install binaries
    bin.install Dir["bin/*"]
    
    # Install standard library
    (share/"cursed").install "stdlib"
    
    # Install documentation
    if Dir.exist?("docs")
      (share/"cursed/docs").install Dir["docs/*"]
    end
    
    # Install man pages if available
    if Dir.exist?("man")
      man1.install Dir["man/*.1"]
    end
    
    # Create cache directory
    (var/"cache/cursed").mkpath
    
    # Set up environment
    (buildpath/"cursed-env.sh").write <<~EOS
      export CURSED_CACHE_DIR="#{var}/cache/cursed"
      export CURSED_STDLIB_PATH="#{share}/cursed/stdlib"
      export PATH="#{bin}:$PATH"
    EOS
    
    (etc/"cursed").install "cursed-env.sh"
  end

  def post_install
    # Create cache directory with proper permissions
    (var/"cache/cursed").mkpath
    
    # Inform user about environment setup
    ohai "CURSED Compiler installed successfully!"
    ohai "Standard library path: #{share}/cursed/stdlib"
    ohai "Cache directory: #{var}/cache/cursed"
    ohai ""
    ohai "To load the CURSED environment automatically, add to your shell profile:"
    ohai "  source #{etc}/cursed/cursed-env.sh"
  end

  def caveats
    <<~EOS
      CURSED Compiler has been installed!
      
      Environment Setup:
        The compiler will automatically find the standard library at:
        #{share}/cursed/stdlib
        
        Cache directory is located at:
        #{var}/cache/cursed
      
      Getting Started:
        cursed-zig --help           # Show help information
        cursed-zig --version        # Show version information
        cursed-zig file.csd         # Compile and run a CURSED program
        cursed-lsp                  # Start the Language Server Protocol server
      
      Documentation:
        Local docs: #{share}/cursed/docs (if installed)
        Online docs: https://docs.cursed.dev
      
      Examples:
        Create a new project:
          mkdir my-project && cd my-project
          echo 'vibez.spill("Hello, CURSED!")' > hello.csd
          cursed-zig hello.csd
    EOS
  end

  test do
    # Basic functionality tests
    system "#{bin}/cursed-zig", "--version"
    system "#{bin}/cursed-zig", "--help"
    
    # Test compilation of a simple program
    (testpath/"hello.csd").write <<~EOS
      vibez.spill("Hello from CURSED!")
    EOS
    
    # Test that the compiler can parse and execute the file
    output = shell_output("#{bin}/cursed-zig #{testpath}/hello.csd 2>&1")
    assert_match "Hello from CURSED!", output
    
    # Test LSP server can start and show version
    system "timeout", "5s", "#{bin}/cursed-lsp", "--version"
    
    # Test standard library access
    (testpath/"stdlib_test.csd").write <<~EOS
      yeet "mathz"
      vibez.spill(abs_normie(-42))
    EOS
    
    output = shell_output("#{bin}/cursed-zig #{testpath}/stdlib_test.csd 2>&1")
    assert_match "42", output
  end
end
