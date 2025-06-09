# CURSED Documentation Test Fixtures

This directory contains test fixtures for comprehensive integration testing of the CURSED documentation system.

## Test Files

### `sample_package.csd`
**Purpose**: Well-documented package example  
**Features**:
- Comprehensive documentation comments with proper formatting
- Examples in documentation blocks
- Argument and return value documentation
- Error handling documentation
- Interface and struct documentation
- Function documentation with usage examples

**Use Cases**:
- Testing complete documentation generation workflow
- Validating HTML output quality and completeness
- Performance benchmarking for well-documented code
- Golden file testing baseline

### `undocumented_package.csd`
**Purpose**: Package with minimal/missing documentation  
**Features**:
- Basic struct and function definitions
- Minimal or no documentation comments
- No examples or usage instructions
- Missing parameter/return documentation

**Use Cases**:
- Testing documentation coverage analysis
- Validating graceful handling of undocumented code
- Comparing output quality between documented and undocumented packages
- Testing documentation completeness reporting

### `complex_types.csd`
**Purpose**: Complex types with comprehensive documentation  
**Features**:
- Generic types with type parameter documentation
- Interface definitions with method documentation
- Complex nested structures
- Type constraints and bounds documentation
- Advanced language feature documentation

**Use Cases**:
- Testing generic type documentation rendering
- Validating complex type hierarchy visualization
- Testing constraint documentation display
- Advanced language feature documentation coverage

### `cross_references.csd`
**Purpose**: Package with extensive cross-references  
**Features**:
- Inter-function references using `[function_name]` syntax
- Cross-struct references and relationships
- Method references with `[Type.method]` syntax
- Complex reference chains and dependencies
- Related function documentation

**Use Cases**:
- Testing cross-reference resolution and linking
- Validating documentation link generation
- Testing reference validation and error handling
- Navigation testing in generated documentation

## Multi-Package Testing

The integration tests also create multi-package project structures:

### `package1/main.csd`
**Purpose**: Core utilities package  
**Features**:
- Basic utility functions
- Cross-package dependency examples
- Package-level documentation

### `package2/main.csd`
**Purpose**: Dependent package (uses package1)  
**Features**:
- Imports from package1
- Cross-package references in documentation
- Package dependency documentation

## Expected Outputs (Golden Files)

Golden file testing compares generated documentation against known-good outputs. These files would be created after initial implementation:

- `sample_package_expected.html` - Expected HTML output for well-documented package
- `complex_types_expected.html` - Expected output for generic types documentation
- `cross_references_expected.html` - Expected output with proper cross-reference links

## Test Scenarios Covered

### 1. Complete Documentation Workflow
- Source file parsing
- Documentation comment extraction
- HTML generation
- Navigation creation
- Search functionality
- CSS/JS resource inclusion

### 2. Multi-Package Projects
- Cross-package reference resolution
- Package hierarchy navigation
- Dependency documentation
- Package-level documentation aggregation

### 3. Cross-Reference Resolution
- Function-to-function references
- Type-to-type references
- Method references (Type.method)
- Package-qualified references
- Invalid reference handling

### 4. Documentation Validation
- Documentation completeness analysis
- Missing documentation detection
- Coverage percentage calculation
- Quality assessment metrics

### 5. Performance Testing
- Large codebase generation performance
- Memory usage during generation
- Output file size optimization
- Generation time benchmarking

### 6. Error Handling
- Malformed documentation comments
- Invalid cross-references
- Missing referenced items
- Syntax errors in source files

### 7. Output Formats
- HTML generation with proper structure
- Markdown export (if implemented)
- JSON export for API documentation
- Search data generation

### 8. CLI Tool Testing
- Directory processing
- Recursive file discovery
- Configuration option handling
- Error reporting and logging

## Adding New Test Cases

To add new test scenarios:

1. Create new `.csd` files in this directory with appropriate documentation
2. Add corresponding expected output files for golden testing
3. Update the integration test to include new test cases
4. Document the purpose and features of new test files in this README

## Test Execution

Run the complete documentation integration test suite:

```bash
# Run all documentation integration tests
cargo test documentation_integration_test

# Run specific test scenarios
cargo test test_complete_documentation_workflow
cargo test test_multi_package_documentation
cargo test test_cross_reference_resolution
cargo test test_performance_large_codebase

# Run with detailed output
RUST_LOG=debug cargo test documentation_integration_test -- --nocapture
```

## Maintenance

These test fixtures should be updated when:
- New CURSED language features are added
- Documentation comment syntax changes
- Cross-reference resolution logic changes
- New documentation output formats are supported
- Performance requirements change

The test files represent comprehensive examples of real-world CURSED code with various documentation patterns and should reflect current best practices for CURSED documentation.
