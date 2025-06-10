# CURSED Compiler Feature Detection System

## Overview

The CURSED compiler feature detection system provides comprehensive capability management for the bootstrap compiler. It enables programs to detect available compiler features, ensure compatibility across bootstrap stages, and gracefully degrade when advanced features aren't available.

## Key Components

### 1. Feature Detection System (`feature_detection.rs`)

The core system that identifies and manages compiler capabilities across different bootstrap stages.

#### Bootstrap Stages

- **Stage 0**: Initial Rust-based bootstrap compiler with basic features
- **Stage 1**: First self-hosted compiler with most features available
- **Stage 2**: Stability-tested compiler with all stable features
- **Development**: Development builds with experimental features

#### Feature Categories

- **Core Language**: Basic types, advanced types, generics with constraints, interfaces, type assertion, error handling
- **Memory Management**: Garbage collection, memory profiler, leak detection
- **Concurrency**: Goroutines, channels, channel buffering, select statement, mutex support
- **Code Generation**: LLVM codegen, JIT compilation, optimized codegen, bitstream output, static linking
- **Advanced Features**: Reflection, meta programming, compiler plugins, cross compilation
- **Standard Library**: Core stdlib, extended stdlib, experimental stdlib
- **Debugging & Diagnostics**: Debug info, profiling, trace generation, error recovery
- **Development Tools**: Language server, syntax highlighting, auto complete, refactoring

#### Support Levels

- **Stable**: Feature is fully supported and production-ready
- **Limited**: Feature is supported but may have limitations
- **Experimental**: Feature is available but may not work reliably
- **Unsupported**: Feature is not available in this stage

### 2. Conditional Compilation (`conditional_compilation.rs`)

Enables feature-conditional compilation directives that allow CURSED programs to adapt based on available compiler capabilities.

#### Directive Types

```cursed
#if_feature goroutines
spn some_goroutine()
#else
some_function()
#endif

#if_not_feature jit_compilation
compile_error!("JIT compilation required")
#endif

#ifdef debug
vibez.spill("Debug mode enabled")
#endif
```

#### Fallback Strategies

- **Remove**: Remove unsupported code entirely
- **NoOp**: Replace with no-operation implementation
- **Simplify**: Replace with simpler implementation
- **RuntimeError**: Replace with runtime error
- **CompileError**: Replace with compile-time error

### 3. Version Negotiation (`version_negotiation.rs`)

Handles version detection and feature negotiation between different compiler stages.

#### Protocol Versions

- **V1.0**: Basic negotiation support
- **V1.1**: Enhanced feature detection
- **V2.0**: Advanced capabilities

#### Negotiation Process

1. **Capability Advertisement**: Each compiler advertises its supported features
2. **Request Creation**: Requester specifies required and preferred features
3. **Feature Analysis**: Compare capabilities and identify gaps
4. **Fallback Options**: Generate alternatives for unsupported features
5. **Response Generation**: Provide negotiation result and agreed features

### 4. Diagnostic Tools (`diagnostic_tools.rs`)

Comprehensive diagnostic and reporting tools for analyzing compiler capabilities.

#### Report Formats

- **JSON**: Machine-readable structured data
- **YAML**: Human-readable structured data
- **Text**: Simple text format for console output
- **HTML**: Rich formatted report for web viewing

#### Diagnostic Categories

- **Compiler Information**: Version, stage, build info, supported targets
- **Feature Matrix**: Current features, cross-stage comparison, dependencies
- **Compatibility Analysis**: Backward/forward compatibility, breaking changes
- **Performance Metrics**: Compilation speed, runtime performance, memory usage
- **Environment Information**: OS, architecture, LLVM installation
- **Recommendations**: Actionable suggestions for improvement

## Usage Examples

### Basic Feature Detection

```rust
use cursed::bootstrap::feature_detection::*;

// Initialize feature detection
let version = CompilerVersion {
    major: 0, minor: 1, patch: 0,
    stage: BootstrapStage::Stage1,
    commit_hash: None, build_timestamp: None,
};
let system = FeatureDetectionSystem::new(BootstrapStage::Stage1, version);

// Check feature support
if system.is_feature_supported(&CompilerFeature::Goroutines) {
    println!("Goroutines are supported!");
}

// Get support level
let support = system.get_feature_support(&CompilerFeature::JitCompilation);
println!("JIT compilation support: {}", support);

// Runtime detection
let result = system.detect_feature_runtime(&CompilerFeature::LlvmCodegen);
println!("LLVM available: {}", result.supported);
```

### Conditional Compilation

```rust
use cursed::bootstrap::conditional_compilation::*;

let mut compiler = ConditionalCompiler::new(Some(system));
compiler.add_feature_flag("debug".to_string());

let source = r#"
#if_feature goroutines
spn { vibez.spill("Concurrent execution") }
#else
vibez.spill("Sequential execution")
#endif
"#;

let processed = compiler.process_source(source)?;
```

### Version Negotiation

```rust
use cursed::bootstrap::version_negotiation::*;

let mut negotiator = VersionNegotiator::new(current_version);
let request = create_negotiation_request(
    current_version,
    vec![CompilerFeature::BasicTypes], // required
    vec![CompilerFeature::Goroutines], // preferred
);

let response = negotiator.negotiate(request, peer_capabilities);
match response.negotiation_result {
    NegotiationResult::FullCompatibility => {
        println!("Fully compatible!");
    },
    NegotiationResult::PartialCompatibility(limitations) => {
        println!("Partially compatible with limitations: {:?}", limitations);
    },
    _ => println!("Incompatible"),
}
```

### Diagnostic Generation

```rust
use cursed::bootstrap::diagnostic_tools::*;

let tool = DiagnosticTool::new()
    .with_feature_system(system);

let diagnostic = tool.run_full_diagnostic();
let report = tool.export_report(&diagnostic, ReportFormat::Html)?;
```

## Command-Line Interface

The `cursed-feature-detect` CLI tool provides comprehensive feature detection capabilities:

### Basic Usage

```bash
# Detect all features for current stage
cursed-feature-detect detect

# Detect specific feature with runtime testing
cursed-feature-detect detect --feature goroutines --runtime

# Generate comprehensive diagnostic report
cursed-feature-detect diagnostic --output report.html --format html

# Check compatibility with another version
cursed-feature-detect compatibility --target 0.2.0 --target-stage stage2

# Preprocess source with conditional compilation
cursed-feature-detect preprocess -i input.csd -o output.csd --flags debug,optimization
```

### Advanced Usage

```bash
# List features by category
cursed-feature-detect list-features --category "Core Language"

# Negotiate with peer compiler
cursed-feature-detect negotiate --peer 0.1.5 --required basic_types,llvm_codegen

# Generate JSON diagnostic for CI/CD
cursed-feature-detect diagnostic --format json --output ci-report.json
```

## Integration with Build System

### Makefile Integration

```makefile
# Check compatibility before building
check-compatibility:
    cursed-feature-detect compatibility --target $(TARGET_VERSION)

# Generate feature report
feature-report:
    cursed-feature-detect diagnostic --output build/feature-report.html --format html

# Preprocess conditional compilation
preprocess:
    cursed-feature-detect preprocess -i src/main.csd -o build/main.processed.csd --flags $(FEATURE_FLAGS)
```

### CI/CD Integration

```yaml
# GitHub Actions example
- name: Check Compiler Features
  run: |
    cursed-feature-detect diagnostic --format json --output feature-report.json
    cursed-feature-detect compatibility --target ${{ matrix.compiler-version }}

- name: Upload Feature Report
  uses: actions/upload-artifact@v3
  with:
    name: feature-report
    path: feature-report.json
```

## Configuration

### Environment Variables

- `CURSED_FEATURE_FLAGS`: Comma-separated list of feature flags
- `CURSED_BOOTSTRAP_STAGE`: Override detected bootstrap stage
- `CURSED_COMPILER_VERSION`: Override compiler version string

### Configuration Files

The system supports optional configuration files for custom feature matrices:

```yaml
# .cursed-features.yml
custom_features:
  experimental_optimizer: true
  debug_mode: true

fallback_strategies:
  goroutines: "sequential_execution"
  channels: "direct_communication"

compatibility_overrides:
  min_version: "0.1.0"
  max_version: "0.2.0"
```

## Best Practices

### For Library Authors

1. **Use Feature Detection**: Always check feature availability before using advanced features
2. **Provide Fallbacks**: Implement graceful degradation for unsupported features
3. **Test Across Stages**: Ensure your library works across different bootstrap stages
4. **Document Requirements**: Clearly specify minimum required features

```rust
// Good practice
if is_feature_supported(&CompilerFeature::Goroutines) {
    // Use concurrent implementation
    spawn_workers();
} else {
    // Use sequential fallback
    process_sequentially();
}
```

### For Application Developers

1. **Generate Diagnostics**: Regularly generate feature reports to understand capabilities
2. **Use Conditional Compilation**: Leverage preprocessor directives for stage-specific code
3. **Plan Migration Path**: Understand the upgrade path between bootstrap stages
4. **Monitor Compatibility**: Set up CI/CD checks for version compatibility

### For Compiler Developers

1. **Update Capability Matrix**: Keep feature matrices current with implementation status
2. **Test Feature Detection**: Ensure runtime detection accurately reflects capabilities
3. **Document Breaking Changes**: Clearly communicate when features change between stages
4. **Provide Migration Tools**: Offer automated tools for upgrading between stages

## Architecture Decisions

### Why Static + Runtime Detection?

The system uses both static capability matrices and runtime detection because:

- **Static matrices** provide fast, deterministic feature checking
- **Runtime detection** validates actual capability availability
- **Combination** ensures accuracy while maintaining performance

### Why Gradual Feature Rollout?

Features are introduced gradually across bootstrap stages to:

- **Reduce risk** of introducing instability
- **Enable testing** at each stage of development
- **Provide migration path** for users
- **Maintain backward compatibility** where possible

### Why Negotiation Protocol?

The negotiation protocol enables:

- **Inter-stage communication** between different compiler versions
- **Graceful degradation** when perfect compatibility isn't available
- **Future extensibility** as new features are added
- **Tooling interoperability** across the compiler ecosystem

## Future Enhancements

### Planned Features

1. **Dynamic Feature Loading**: Runtime loading of optional compiler plugins
2. **Feature Profiling**: Performance impact analysis of enabled features
3. **Automatic Fallback Generation**: AI-assisted generation of fallback implementations
4. **Cross-Language Interop**: Feature detection for interfacing with other languages
5. **Distributed Compilation**: Feature negotiation across network-distributed builds

### Experimental Features

1. **Machine Learning Optimization**: ML-based feature recommendation
2. **Blockchain Verification**: Cryptographic verification of compiler capabilities
3. **Quantum-Ready Features**: Preparation for quantum computing integration
4. **WebAssembly Target**: Feature detection for WASM compilation targets

## Troubleshooting

### Common Issues

1. **Feature Not Detected**: Check if runtime dependencies are installed
2. **Negotiation Failure**: Verify protocol version compatibility
3. **Compilation Errors**: Ensure feature flags match available capabilities
4. **Performance Degradation**: Review enabled experimental features

### Debug Mode

Enable debug logging for detailed feature detection information:

```bash
RUST_LOG=debug cursed-feature-detect detect --runtime
```

### Support

For issues with the feature detection system:

1. Generate a diagnostic report: `cursed-feature-detect diagnostic --format json`
2. Check the troubleshooting guide in the repository
3. Open an issue with the diagnostic report attached
4. Consult the community forum for common solutions

## Conclusion

The CURSED compiler feature detection system provides a robust foundation for building adaptable, compatible software that can evolve with the compiler through its bootstrap stages. By leveraging feature detection, conditional compilation, and diagnostic tools, developers can create software that works reliably across different compiler capabilities while taking advantage of new features as they become available.
