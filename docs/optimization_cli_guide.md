# CURSED Optimization CLI Guide

This guide covers the comprehensive optimization commands available in the CURSED compiler CLI for performance analysis, benchmarking, profiling, and optimization configuration.

## Overview

The CURSED optimization CLI provides powerful tools for:
- **Performance Analysis**: Analyze compilation performance and identify optimization opportunities
- **Benchmarking**: Compare compilation speed across different optimization levels
- **Profiling**: Profile compilation pipeline to identify bottlenecks
- **Configuration Management**: Enable/disable optimization passes and configure settings
- **Reporting**: Generate detailed performance reports and recommendations

## Commands

### `cursed optimize analyze`

Analyze compilation performance and suggest optimizations for a specific file.

```bash
# Basic analysis
cursed optimize analyze hello.csd

# Detailed analysis with suggestions
cursed optimize analyze hello.csd --detailed --suggestions

# Output to file in different formats
cursed optimize analyze hello.csd --output report.md --format markdown
cursed optimize analyze hello.csd --output report.json --format json
cursed optimize analyze hello.csd --output report.txt --format table
```

**Options:**
- `--output, -o FILE`: Output file for analysis report
- `--format, -f FORMAT`: Report format (json, markdown, table)
- `--detailed`: Generate detailed analysis report
- `--suggestions`: Include optimization suggestions

### `cursed optimize benchmark`

Benchmark compilation speed across different optimization levels.

```bash
# Basic benchmark
cursed optimize benchmark hello.csd

# Custom optimization levels
cursed optimize benchmark hello.csd --levels "0,2,3"

# More iterations for accuracy
cursed optimize benchmark hello.csd --iterations 10 --warmup 5

# Parallel benchmarking
cursed optimize benchmark hello.csd --parallel

# Compare with previous results
cursed optimize benchmark hello.csd --compare previous_results.json
```

**Options:**
- `--levels, -l LEVELS`: Optimization levels to benchmark (default: "0,1,2,3,s,z")
- `--iterations, -i N`: Number of iterations per level (default: 5)
- `--warmup, -w N`: Number of warmup iterations (default: 2)
- `--timeout, -t SECONDS`: Timeout for each compilation (default: 300)
- `--output, -o FILE`: Output file for benchmark results
- `--compare FILE`: Compare with previous benchmark results
- `--parallel, -p`: Run benchmarks in parallel

### `cursed optimize profile`

Profile compilation pipeline and identify bottlenecks.

```bash
# Basic profiling
cursed optimize profile hello.csd

# Profile with specific optimization level
cursed optimize profile hello.csd --opt-level 3

# Detailed phase profiling
cursed optimize profile hello.csd --phases

# Memory tracking
cursed optimize profile hello.csd --memory

# Generate flamegraph
cursed optimize profile hello.csd --flamegraph --output profile.svg
```

**Options:**
- `--opt-level, -O LEVEL`: Optimization level to profile (default: 2)
- `--phases`: Profile individual compilation phases
- `--memory`: Track memory usage during compilation
- `--sample-rate HZ`: Profiling sample rate in Hz (default: 1000)
- `--output, -o FILE`: Output file for profiling report
- `--flamegraph`: Generate flamegraph output

### `cursed optimize enable`

Enable specific optimization passes.

```bash
# Enable single pass
cursed optimize enable inline

# Enable multiple passes
cursed optimize enable "inline,vectorize,loop-unroll"

# Apply globally
cursed optimize enable "aggressive-inline" --global

# Apply to current project only
cursed optimize enable "vectorize" --project
```

**Options:**
- `--global, -g`: Apply to global configuration
- `--project, -p`: Apply to current project only

### `cursed optimize disable`

Disable specific optimization passes.

```bash
# Disable single pass
cursed optimize disable aggressive-inline

# Disable multiple passes
cursed optimize disable "experimental-vectorize,unstable-opts"

# Apply globally
cursed optimize disable "aggressive-inline" --global
```

**Options:**
- `--global, -g`: Apply to global configuration
- `--project, -p`: Apply to current project only

### `cursed optimize config`

Configure optimization settings.

```bash
# Show current configuration
cursed optimize config --show

# Set configuration values
cursed optimize config --set "inline_threshold=500"
cursed optimize config --set "optimization_remarks=true"

# Unset configuration values
cursed optimize config --unset "experimental_feature"

# Set default optimization level
cursed optimize config --default-level 3

# Export configuration
cursed optimize config --export my_config.json

# Import configuration
cursed optimize config --import my_config.json

# Modify global configuration
cursed optimize config --show --global
```

**Options:**
- `--show`: Show current configuration
- `--set KEY=VALUE`: Set configuration value (can be used multiple times)
- `--unset KEY`: Unset configuration value (can be used multiple times)
- `--default-level LEVEL`: Set default optimization level
- `--global, -g`: Modify global configuration
- `--export FILE`: Export configuration to file
- `--import FILE`: Import configuration from file

### `cursed optimize reset`

Reset to default optimization configuration.

```bash
# Reset project configuration (with confirmation prompt)
cursed optimize reset --project

# Reset global configuration (with confirmation prompt)
cursed optimize reset --global

# Reset without confirmation prompt
cursed optimize reset --project --confirm
cursed optimize reset --global --confirm
```

**Options:**
- `--global, -g`: Reset global configuration
- `--project, -p`: Reset project configuration
- `--confirm`: Confirm reset without prompting

## Configuration

### Configuration Files

Optimization settings are stored in JSON configuration files:

- **Global**: `~/.cursed/optimization.json`
- **Project**: `.cursed/optimization.json`

Project configuration takes precedence over global configuration.

### Configuration Structure

```json
{
  "default_level": "O2",
  "enabled_passes": [
    "inline",
    "dce", 
    "mem2reg",
    "gvn"
  ],
  "disabled_passes": [],
  "custom_params": {
    "inline_threshold": "500",
    "optimization_remarks": "true"
  },
  "benchmark_config": {
    "iterations": 5,
    "timeout_seconds": 300,
    "warmup_iterations": 2,
    "test_files": []
  },
  "profiling_config": {
    "detailed_timing": true,
    "memory_tracking": false,
    "sample_rate": 1000,
    "output_format": "markdown"
  }
}
```

### Optimization Levels

- **O0**: No optimization (fastest compilation)
- **O1**: Basic optimizations
- **O2**: Standard optimizations (default)
- **O3**: Aggressive optimizations
- **Os**: Optimize for size
- **Oz**: Optimize aggressively for size

### Available Optimization Passes

- **inline**: Function inlining
- **dce**: Dead code elimination
- **mem2reg**: Promote memory to register
- **gvn**: Global value numbering
- **loop-unroll**: Loop unrolling
- **vectorize**: Auto-vectorization
- **constant-fold**: Constant folding
- **licm**: Loop invariant code motion
- **aggressive-inline**: Aggressive function inlining
- **experimental-vectorize**: Experimental vectorization

## Examples

### Performance Analysis Workflow

```bash
# 1. Analyze current performance
cursed optimize analyze my_program.csd --detailed --suggestions

# 2. Benchmark different optimization levels
cursed optimize benchmark my_program.csd --output baseline.json

# 3. Enable recommended optimizations
cursed optimize enable "loop-unroll,vectorize"

# 4. Benchmark again to measure improvement
cursed optimize benchmark my_program.csd --compare baseline.json

# 5. Profile to identify remaining bottlenecks
cursed optimize profile my_program.csd --phases --memory
```

### Configuration Management

```bash
# Set up custom optimization configuration
cursed optimize config --default-level 3
cursed optimize config --set "inline_threshold=1000"
cursed optimize config --set "optimization_remarks=true"
cursed optimize enable "loop-unroll,vectorize,constant-fold"

# Export configuration for sharing
cursed optimize config --export team_config.json

# Import configuration on another machine
cursed optimize config --import team_config.json

# Reset to defaults when needed
cursed optimize reset --project --confirm
```

### Automated Performance Testing

```bash
# Set up benchmark configuration
cursed optimize config --set "benchmark_iterations=10"
cursed optimize config --set "benchmark_timeout=600"

# Run comprehensive benchmark suite
cursed optimize benchmark app.csd --levels "0,1,2,3,s,z" --parallel

# Profile the best performing configuration
cursed optimize profile app.csd --opt-level 3 --phases --flamegraph
```

## Integration with Build Systems

### Makefile Integration

```makefile
# Performance analysis target
analyze:
	cursed optimize analyze src/main.csd --output analysis.md

# Benchmark target
benchmark:
	cursed optimize benchmark src/main.csd --output benchmark.json

# Optimized build target
build-optimized:
	cursed optimize config --default-level 3
	cursed build src/main.csd --output optimized_binary

.PHONY: analyze benchmark build-optimized
```

### CI/CD Integration

```yaml
# GitHub Actions example
- name: Performance Analysis
  run: |
    cursed optimize analyze src/main.csd --format json --output analysis.json
    cursed optimize benchmark src/main.csd --output benchmark.json

- name: Upload Performance Reports
  uses: actions/upload-artifact@v3
  with:
    name: performance-reports
    path: |
      analysis.json
      benchmark.json
```

## Troubleshooting

### Common Issues

**Configuration not found:**
```bash
# Create default configuration
cursed optimize config --show
```

**Benchmark timeouts:**
```bash
# Increase timeout
cursed optimize benchmark app.csd --timeout 600
```

**Memory issues during profiling:**
```bash
# Reduce sample rate
cursed optimize profile app.csd --sample-rate 500
```

### Debug Mode

Enable verbose output for debugging:
```bash
cursed --verbose optimize analyze app.csd
```

## Best Practices

1. **Start with analysis**: Always begin with `cursed optimize analyze` to understand current performance
2. **Baseline first**: Create benchmark baselines before making optimization changes
3. **Incremental changes**: Enable optimizations incrementally and measure impact
4. **Profile bottlenecks**: Use profiling to identify specific performance bottlenecks
5. **Project-specific configs**: Use project-specific configurations for different codebases
6. **Version control configs**: Include `.cursed/optimization.json` in version control for team consistency

## Advanced Usage

### Custom Optimization Passes

```bash
# Enable experimental passes
cursed optimize enable "experimental-vectorize,llvm-experimental"

# Set custom parameters
cursed optimize config --set "vectorize_width=16"
cursed optimize config --set "inline_cost_threshold=2000"
```

### Performance Regression Detection

```bash
# Set up baseline
cursed optimize benchmark app.csd --output baseline.json

# After changes, compare performance
cursed optimize benchmark app.csd --compare baseline.json

# Automated regression check
if cursed optimize benchmark app.csd --compare baseline.json | grep -q "regression"; then
  echo "Performance regression detected!"
  exit 1
fi
```
