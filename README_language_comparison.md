# CURSED Language Benchmark Comparison System

## Overview

The language comparison system is designed to benchmark CURSED against other programming languages like Go, Rust, Java, and JavaScript. It implements standard algorithms in each language and measures their performance in terms of execution time and memory usage.

## Implemented Benchmarks

The framework includes the following benchmark algorithms:

1. **Binary Trees**: Tests memory allocation and garbage collection by creating and traversing binary trees
2. **N-Bodies**: Simulates a physical system with Jovian planets, testing floating-point and computation performance
3. **String Processing**: Tests string manipulation operations across languages
4. **Mandelbrot**: Calculates the Mandelbrot set, testing computational and floating-point performance
5. **Fannkuch**: Implements the Fannkuch algorithm, which tests indexed-access and integer operations
6. **FASTA**: Generates random DNA sequences, testing string generation and random number generation

## Running the Benchmarks

You can run the language comparison benchmarks with:

```bash
# Using benchmark binary
./target/debug/benchmark language

# Or using the dedicated binary
./target/debug/language_benchmark
```

To specify an output format:

```bash
./target/debug/language_benchmark markdown results.md
```

Supported output formats:
- `console`: Default, prints results to the terminal
- `json`: Saves results in JSON format
- `csv`: Saves results in CSV format
- `markdown`: Saves results in Markdown format (most readable)

## Implementation Details

1. The framework uses a modular design with benchmark files for each algorithm in each language
2. Files are stored in their respective language directories under the `benchmarks/` folder
3. The system automatically generates implementations for all available languages
4. For most accurate results, build in release mode before benchmarking

## Adding New Languages

To add support for a new language:

1. Add the language to the `Language` enum in `src/benchmark/language_comparison.rs`
2. Implement a template generator function in the same file
3. Create template files for each algorithm in the new language

## Adding New Benchmarks

To add a new benchmark algorithm:

1. Add it to the `Algorithm` enum in `src/benchmark/language_comparison.rs`
2. Implement templates for each supported language
3. Update the description and test functions

## Requirements

To run all language comparisons, you need:

- Cursed compiler (built from this repository)
- Go compiler (for Go benchmarks)
- Rust compiler (for Rust benchmarks)
- Java JDK (for Java benchmarks)
- Node.js (for JavaScript benchmarks)

The system will skip benchmarks for languages that aren't installed.

## Understanding the Results

The benchmark results provide two key metrics:

1. **Execution Time**: How long each language took to execute the algorithm, measured in milliseconds. Lower is better.

2. **Memory Usage**: The peak memory consumption during execution, measured in kilobytes. Lower is better.

Results are presented with relative comparisons, where the fastest language is given a baseline of 1.0x, and other languages are compared proportionally.

## Example Results

Here's an example of what the benchmark results might look like:

```markdown
## Binary Trees

Allocate and deallocate many binary trees

### Execution Time Comparison

| Language | Time (ms) | Relative Performance |
| --- | ---: | ---: |
| Rust | 124.50 | 1.00x |
| Go | 152.30 | 1.22x |
| CURSED | 178.90 | 1.44x |
| Java | 196.70 | 1.58x |
| JavaScript | 289.10 | 2.32x |

### Memory Usage Comparison

| Language | Memory (MB) | Relative Memory Usage |
| --- | ---: | ---: |
| Rust | 8.20 | 1.00x |
| CURSED | 12.50 | 1.52x |
| Go | 18.70 | 2.28x |
| Java | 24.30 | 2.96x |
| JavaScript | 31.50 | 3.84x |
```

## Troubleshooting

If you encounter issues running the benchmarks:

1. **Missing language**: Ensure you have the language's compiler/runtime installed and available in your PATH
2. **Compilation errors**: Check if the template files have any issues
3. **Runtime errors**: Ensure you have the necessary permissions and resources to run the benchmarks
4. **Zero execution time**: The benchmark might be too small - try increasing the workload in the template files