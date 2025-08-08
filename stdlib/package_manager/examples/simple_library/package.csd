// Simple library package example
// Demonstrates a minimal but complete package configuration

package "simple-math-lib"
{
    version = "1.0.0"
    description = "A simple mathematical operations library for CURSED"
    authors = ["Math Team <math@cursed.dev>"]
    license = "MIT"
    keywords = ["math", "library", "utilities"]
    repository = "https://github.com/cursed-lang/simple-math-lib"
    
    // Only essential dependencies
    dependencies = {
        "vibez" = "^1.0.0"    // For error output
    }
    
    // Development and testing dependencies
    dev_dependencies = {
        "testz" = "^1.0.0"    // Testing framework
        "benchz" = "^0.5.0"   // Performance benchmarks
    }
    
    // This package provides a library
    targets = {
        libs = [
            {
                name = "simple-math"
                main = "src/lib.csd"
                public = true
            }
        ]
    }
    
    // Simple feature flags
    features = {
        default = ["basic"]
        basic = []              // Basic math operations
        advanced = []           // Advanced functions (trigonometry, etc.)
        float = []              // Floating point operations
    }
    
    // Build configuration
    build = {
        cursed_version = ">=1.0.0"
        optimization = "release"
        
        // Export public API
        exports = [
            "add",
            "subtract", 
            "multiply",
            "divide",
            "power",
            "sqrt"
        ]
    }
    
    // Documentation
    docs = {
        include = ["src/"]
        main = "src/lib.csd"
    }
    
    // Publishing settings
    publish = {
        include = [
            "src/",
            "README.md",
            "LICENSE",
            "package.csd"
        ]
        exclude = [
            "tests/",
            "examples/private/",
            "*.log"
        ]
    }
}
