# CURSED Programming Language Compiler - Homebrew Formula
# Oracle Week 3 cross-platform packaging preparation for v1.0 launch
class Cursed < Formula
  desc "CURSED Programming Language: A production-ready systems programming language with ergonomic syntax"
  homepage "https://cursedlang.org"
  url "https://github.com/ghuntley/cursed/archive/refs/tags/v1.0.0.tar.gz"
  sha256 "0000000000000000000000000000000000000000000000000000000000000000" # Will be updated by release automation
  license "MIT"
  version "1.0.0"
  
  head "https://github.com/ghuntley/cursed.git", branch: "main"

  # Dependencies
  depends_on "zig" => :build
  depends_on "llvm@18" => :build
  depends_on "libllvm" => :build
  
  # macOS version requirements
  depends_on macos: :monterey if OS.mac?

  # Compiler requirements
  fails_with gcc: "10"

  def install
    # Detect architecture for universal binary support
    arch = Hardware::CPU.intel? ? "x86_64" : "arm64"
    target_spec = "#{arch}-apple-darwin"
    
    # Build configuration
    ENV["CC"] = "clang"
    ENV["CXX"] = "clang++"
    ENV["LLVM_SYS_180_PREFIX"] = Formula["llvm@18"].opt_prefix
    
    # Production build with full optimization
    system "zig", "build", 
           "-Doptimize=ReleaseFast",
           "-Dtarget=#{target_spec}",
           "--verbose"
    
    # Install main compiler binary
    bin.install "zig-out/bin/cursed"
    bin.install "zig-out/bin/cursed-zig" # Legacy compatibility
    
    # Install tooling suite
    bin.install "zig-out/bin/cursed-lsp"
    bin.install "zig-out/bin/cursed-fmt"
    bin.install "zig-out/bin/cursed-lint"
    bin.install "zig-out/bin/cursed-doc"
    bin.install "zig-out/bin/cursed-pkg"
    
    # Install standard library
    (pkgshare/"stdlib").install Dir["stdlib/*"]
    
    # Install language support files
    (pkgshare/"lang").install Dir["lang/*"] if Dir["lang/*"].any?
    
    # Install examples and documentation
    (pkgshare/"examples").install Dir["examples/*"]
    (pkgshare/"docs").install Dir["docs/*"]
    
    # Create wrapper script for environment setup
    (bin/"cursed-env").write <<~EOS
      #!/bin/bash
      export CURSED_HOME="#{pkgshare}"
      export CURSED_STDLIB_PATH="#{pkgshare}/stdlib"
      export PATH="#{bin}:$PATH"
      exec "$@"
    EOS
    (bin/"cursed-env").chmod 0755
    
    # Install man pages if they exist
    if Dir["docs/man/*"].any?
      man1.install Dir["docs/man/*.1"]
    end
    
    # Install shell completions if available
    if Dir["completions/*"].any?
      bash_completion.install Dir["completions/cursed.bash"]
      zsh_completion.install Dir["completions/_cursed"]
      fish_completion.install Dir["completions/cursed.fish"]
    end
  end

  # Comprehensive test suite
  test do
    # Test basic installation
    assert_match "CURSED", shell_output("#{bin}/cursed --version")
    assert_match "1.0.0", shell_output("#{bin}/cursed --version")
    
    # Test simple program compilation and execution
    testprog = <<~EOS
      yeet "vibez";
      sus greeting tea = "Hello, Homebrew!";
      vibez.spill(greeting);
    EOS
    
    (testpath/"test.💀").write testprog
    
    # Test interpreter mode
    output = shell_output("#{bin}/cursed test.💀")
    assert_match "Hello, Homebrew!", output
    
    # Test LSP server
    assert_predicate bin/"cursed-lsp", :exist?
    assert_match "CURSED Language Server", shell_output("#{bin}/cursed-lsp --version")
    
    # Test package manager
    assert_predicate bin/"cursed-pkg", :exist?
    assert_match "CURSED Package Manager", shell_output("#{bin}/cursed-pkg --version")
    
    # Test formatter
    assert_predicate bin/"cursed-fmt", :exist?
    assert_match "CURSED Formatter", shell_output("#{bin}/cursed-fmt --version")
    
    # Test linter
    assert_predicate bin/"cursed-lint", :exist?
    assert_match "CURSED Linter", shell_output("#{bin}/cursed-lint --version")
    
    # Test documentation generator
    assert_predicate bin/"cursed-doc", :exist?
    assert_match "CURSED Documentation Generator", shell_output("#{bin}/cursed-doc --version")
    
    # Test standard library accessibility
    assert_predicate pkgshare/"stdlib", :exist?
    assert_predicate pkgshare/"stdlib/vibez.💀", :exist?
    
    # Test advanced features
    advanced_testprog = <<~EOS
      yeet "vibez";
      yeet "mathz";
      yeet "stringz";
      
      slay fibonacci(n drip) drip {
          ready (n <= 1) {
              damn n;
          }
          damn fibonacci(n - 1) + fibonacci(n - 2);
      }
      
      sus result drip = fibonacci(10);
      vibez.spill("Fibonacci(10) =", result);
      
      sus text tea = "CURSED rocks!";
      sus upper_text tea = stringz.to_upper(text);
      vibez.spill("Uppercase:", upper_text);
    EOS
    
    (testpath/"advanced_test.💀").write advanced_testprog
    output = shell_output("#{bin}/cursed advanced_test.💀")
    assert_match "Fibonacci(10) = 55", output
    assert_match "CURSED rocks!", output.upcase
    
    # Test error handling
    error_testprog = <<~EOS
      yeet "vibez";
      
      slay divide(a drip, b drip) yikes<tea> {
          ready (b == 0) {
              yikes "Division by zero!";
          }
          damn a / b;
      }
      
      sus result drip = divide(10, 2) fam {
          when _ -> {
              vibez.spill("Caught an error!");
              damn 0;
          }
      };
      
      vibez.spill("Result:", result);
    EOS
    
    (testpath/"error_test.💀").write error_testprog
    output = shell_output("#{bin}/cursed error_test.💀")
    assert_match "Result: 5", output
    
    # Performance test
    perf_testprog = <<~EOS
      yeet "vibez";
      yeet "mathz";
      
      sus start_time = mathz.time_millis();
      
      sus sum drip = 0;
      bestie (sus i drip = 0; i < 100000; i = i + 1) {
          sum = sum + i;
      }
      
      sus end_time = mathz.time_millis();
      sus duration = end_time - start_time;
      
      vibez.spill("Sum:", sum);
      vibez.spill("Time:", duration, "ms");
    EOS
    
    (testpath/"perf_test.💀").write perf_testprog
    output = shell_output("#{bin}/cursed perf_test.💀")
    assert_match "Sum:", output
    assert_match "Time:", output
  end
  
  # Service management (for LSP server)
  service do
    run [opt_bin/"cursed-lsp", "--stdio"]
    keep_alive false
    log_path var/"log/cursed-lsp.log"
    error_log_path var/"log/cursed-lsp.error.log"
  end
  
  # Post-install message
  def caveats
    <<~EOS
      🎉 CURSED Programming Language is now installed!
      
      Getting Started:
        • Create a new CURSED program: echo 'yeet "vibez"; vibez.spill("Hello, World!");' > hello.💀
        • Run your program: cursed hello.💀
        • Format your code: cursed-fmt hello.💀
        • Start LSP server: cursed-lsp --stdio
      
      Environment Setup:
        • Standard library path: #{pkgshare}/stdlib
        • Examples directory: #{pkgshare}/examples
        • Documentation: #{pkgshare}/docs
        
      IDE Integration:
        • VS Code: Install "CURSED Language Support" extension
        • Vim/Neovim: LSP server runs on `cursed-lsp --stdio`
        • Other editors: Configure LSP client with cursed-lsp
      
      Resources:
        • Documentation: https://docs.cursedlang.org
        • Examples: #{pkgshare}/examples
        • Community: https://discord.gg/cursedlang
      
      Enterprise Support:
        • Commercial licenses: https://cursedlang.org/enterprise
        • Support: enterprise@cursedlang.org
        
      Happy coding with CURSED! 🚀
    EOS
  end
end
