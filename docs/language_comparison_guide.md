# Language Comparison Guide

## Overview

This guide explains how to use the CURSED language comparison benchmarks to compare the performance of CURSED against other similar programming languages like Go, Rust, Java, and JavaScript.

## Running the Comparison

To run the language comparison benchmarks, you need to have the following installed:

- CURSED compiler and runtime (built in release mode)
- Go (optional, for Go comparisons)
- Rust (optional, for Rust comparisons)
- Java (optional, for Java comparisons)
- Node.js (optional, for JavaScript comparisons)

Use the following command to run the language comparison benchmarks:

```bash
# Using the benchmark binary:
./target/release/benchmark language

# Or using the dedicated language comparison binary:
./target/release/language_benchmark
```

## Output Formats

You can specify different output formats:

```bash
# Output to console (default)
./target/release/language_benchmark

# Output to JSON file
./target/release/language_benchmark json results

# Output to CSV file
./target/release/language_benchmark csv results

# Output to Markdown file
./target/release/language_benchmark markdown results
```

## Benchmark Algorithms

The language comparison benchmarks include the following algorithms:

1. **Binary Trees**: Tests memory allocation/deallocation performance by creating and traversing many binary trees of various depths. This benchmark is particularly effective at testing garbage collector efficiency.

2. **N-Bodies**: Simulates the Jovian planets' movements using a simple N-body simulation. This benchmark tests floating-point performance and general computational efficiency.

3. **String Processing**: Creates and manipulates strings in various ways, including concatenation, replacement, and extraction. This tests string handling performance, which is crucial for many real-world applications.

## Understanding the Results

The benchmark results include two primary metrics:

1. **Execution Time**: How long each language took to run the benchmark, measured in milliseconds. Lower is better.

2. **Memory Usage**: How much memory each language used during the benchmark, measured in megabytes. Lower is better.

The results are presented with relative comparisons, where the fastest language is given a baseline of 1.0x, and other languages are compared proportionally. For example, if Language A takes 100ms and Language B takes 200ms, then B will be reported as 2.0x slower than A.

## Extending the Benchmarks

If you want to add your own benchmarks or add support for other languages:

1. Create a new benchmark template in `benchmarks/templates/`
2. Update the `language_comparison.rs` file to include your new benchmark
3. Run the benchmarks with the new algorithm

## Limitations

Areas where the current benchmark framework may be limited:

1. **JIT Warmup**: Some languages (like JavaScript) benefit significantly from JIT warmup. The benchmarks attempt to account for this with a warmup phase, but results may still be affected.

2. **Memory Reporting**: Memory usage is reported differently by different language runtimes. The benchmarks attempt to standardize this, but exact memory comparison across languages may not be perfectly precise.

3. **Language Features**: Some benchmarks may benefit from language-specific features or optimization techniques. The benchmarks try to implement the same algorithm in each language, but idiomatic approaches may vary.

## Example Results

Here's an example of what the results might look like:

```
=== Binary Trees Benchmark ===

Execution Time:
| Language   | Time (ms) | Relative |
|------------|-----------|----------|
| Rust       | 124.5     | 1.0x     |
| Go         | 152.3     | 1.2x     |
| CURSED     | 178.9     | 1.4x     |
| Java       | 196.7     | 1.6x     |
| JavaScript | 289.1     | 2.3x     |

Memory Usage:
| Language   | Memory (MB) | Relative |
|------------|-------------|----------|
| Rust       | 8.2         | 1.0x     |
| CURSED     | 12.5        | 1.5x     |
| Go         | 18.7        | 2.3x     |
| Java       | 24.3        | 3.0x     |
| JavaScript | 31.5        | 3.8x     |
```

## Future Improvements

Planned improvements for the language comparison benchmarks:

1. **More Algorithms**: Add more diverse benchmarks like sorting, graph algorithms, and ML tasks
2. **More Languages**: Add support for more languages like C++, Python, etc.
3. **Profile Visualization**: Generate charts and graphs showing performance comparisons
4. **Historical Tracking**: Track performance changes over time as CURSED evolves