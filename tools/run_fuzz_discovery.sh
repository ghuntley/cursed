#!/bin/bash
# CURSED Automated Fuzz Target Discovery and Generation Script

set -e

echo "🔍 CURSED Automated Fuzz Target Discovery System"
echo "================================================"

# Configuration
PROJECT_ROOT="${1:-.}"
OUTPUT_DIR="fuzz_targets"
REPORT_FILE="fuzz_discovery_report.json"
MIN_RISK="${2:-medium}"

# Ensure we're in the right directory
cd "$PROJECT_ROOT"

# Create output directory
mkdir -p "$OUTPUT_DIR"

echo "📍 Project root: $(pwd)"
echo "📁 Output directory: $OUTPUT_DIR"
echo "🎯 Minimum risk level: $MIN_RISK"
echo ""

# Step 1: Discover fuzz targets
echo "Step 1: Discovering fuzz targets..."
python3 tools/fuzz_target_discovery.py \
    --project-root . \
    --output-dir "$OUTPUT_DIR" \
    --report "$REPORT_FILE" \
    --min-risk "$MIN_RISK" \
    --generate

echo ""

# Step 2: Generate specialized templates
echo "Step 2: Generating specialized fuzz templates..."
python3 tools/fuzz_template_generator.py \
    --report "$REPORT_FILE" \
    --output-dir "${OUTPUT_DIR}/specialized"

echo ""

# Step 3: Create comprehensive test suite
echo "Step 3: Creating comprehensive test suite..."

# Create master test runner
cat > "${OUTPUT_DIR}/run_comprehensive_fuzz_tests.sh" << 'EOF'
#!/bin/bash
# Comprehensive CURSED Fuzz Test Runner

set -e

echo "🚀 Starting comprehensive CURSED fuzz testing..."

# Configuration
FUZZ_DURATION="${1:-60}"  # Default 60 seconds per target
PARALLEL_JOBS="${2:-4}"   # Default 4 parallel jobs
MAX_MEMORY="512M"         # Memory limit per fuzzer

echo "⏱️  Fuzz duration per target: ${FUZZ_DURATION}s"
echo "🔄 Parallel jobs: $PARALLEL_JOBS"
echo "💾 Memory limit: $MAX_MEMORY"
echo ""

# Function to run a single fuzz target
run_fuzz_target() {
    local target="$1"
    local name=$(basename "$target")
    local logfile="logs/${name}.log"
    
    echo "🎯 Starting $name..."
    
    # Create corpus and output directories
    mkdir -p "corpus/$name" "crashes/$name" "outputs/$name"
    
    # Run with timeout and resource limits
    timeout "$FUZZ_DURATION" \
        ulimit -v $(echo "$MAX_MEMORY" | sed 's/M/000/') && \
        "$target" \
            -artifact_prefix="crashes/$name/" \
            -print_final_stats=1 \
            "corpus/$name" \
            > "$logfile" 2>&1 || true
    
    echo "✅ Completed $name"
}

# Prepare directories
mkdir -p logs corpus crashes outputs

# Build all targets first
echo "🔨 Building all fuzz targets..."
if [ -f "build_all_targets.sh" ]; then
    bash build_all_targets.sh
elif [ -f "../build_all_targets.sh" ]; then
    bash ../build_all_targets.sh
else
    echo "⚠️  No build script found, attempting individual builds..."
    for build_script in build_*.sh; do
        if [ -f "$build_script" ]; then
            bash "$build_script"
        fi
    done
fi

echo ""

# Find all fuzz targets
FUZZ_TARGETS=($(find . -name 'fuzz_*' -type f -executable))

if [ ${#FUZZ_TARGETS[@]} -eq 0 ]; then
    echo "❌ No fuzz targets found!"
    exit 1
fi

echo "🎯 Found ${#FUZZ_TARGETS[@]} fuzz targets:"
for target in "${FUZZ_TARGETS[@]}"; do
    echo "  - $(basename "$target")"
done
echo ""

# Run fuzz targets in parallel
echo "🚀 Starting parallel fuzzing..."
export -f run_fuzz_target
export FUZZ_DURATION MAX_MEMORY

printf '%s\n' "${FUZZ_TARGETS[@]}" | \
    xargs -n 1 -P "$PARALLEL_JOBS" -I {} bash -c 'run_fuzz_target "$@"' _ {}

echo ""
echo "📊 Fuzzing Results Summary:"
echo "=========================="

# Analyze results
total_targets=${#FUZZ_TARGETS[@]}
successful=0
crashes_found=0

for target in "${FUZZ_TARGETS[@]}"; do
    name=$(basename "$target")
    logfile="logs/${name}.log"
    
    if [ -f "$logfile" ]; then
        if grep -q "DONE" "$logfile"; then
            successful=$((successful + 1))
        fi
        
        crash_count=$(find "crashes/$name" -name "crash-*" 2>/dev/null | wc -l)
        if [ "$crash_count" -gt 0 ]; then
            echo "💥 $name: Found $crash_count crashes"
            crashes_found=$((crashes_found + crash_count))
        fi
    fi
done

echo "📈 Targets completed: $successful/$total_targets"
echo "💥 Total crashes found: $crashes_found"

if [ "$crashes_found" -gt 0 ]; then
    echo ""
    echo "🔍 Crash Analysis:"
    echo "=================="
    find crashes -name "crash-*" -exec echo "Crash: {}" \; -exec head -5 {} \; -exec echo "" \;
fi

echo ""
echo "✅ Comprehensive fuzz testing complete!"
EOF

chmod +x "${OUTPUT_DIR}/run_comprehensive_fuzz_tests.sh"

# Step 4: Create focused security test suite
echo "Step 4: Creating focused security test suite..."

cat > "${OUTPUT_DIR}/run_security_focused_tests.sh" << 'EOF'
#!/bin/bash
# Security-Focused CURSED Fuzz Testing

set -e

echo "🛡️  CURSED Security-Focused Fuzz Testing"
echo "========================================"

# Focus on high-risk targets only
SECURITY_TARGETS=(
    # Parser targets (highest risk)
    fuzz_parser_parseProgram
    fuzz_parser_parseStatement
    fuzz_parser_parseExpression
    fuzz_parser_tokenize
    
    # File I/O targets
    fuzz_file_read_file
    fuzz_file_write_file
    fuzz_file_parse_file
    
    # Network targets
    fuzz_network_http_get
    fuzz_network_tcp_connect
    
    # Memory buffer targets
    fuzz_buffer_memcpy
    fuzz_buffer_strcpy
    
    # CURSED-specific targets
    fuzz_cursed_evaluate
    fuzz_cursed_handleRequest
)

echo "🎯 Security-focused targets:"
for target in "${SECURITY_TARGETS[@]}"; do
    if [ -f "$target" ]; then
        echo "  ✅ $target"
    else
        echo "  ⚠️  $target (not found)"
    fi
done

echo ""
echo "🚀 Running security-focused fuzzing (extended duration)..."

# Run each security target for longer duration
for target in "${SECURITY_TARGETS[@]}"; do
    if [ -f "$target" ]; then
        echo "🔒 Testing $target..."
        mkdir -p "security_corpus/$(basename "$target")"
        mkdir -p "security_crashes/$(basename "$target")"
        
        # Run for 5 minutes per security target
        timeout 300 "./$target" \
            -artifact_prefix="security_crashes/$(basename "$target")/" \
            -max_len=1048576 \
            -use_value_profile=1 \
            -print_final_stats=1 \
            "security_corpus/$(basename "$target")" || true
    fi
done

echo "✅ Security-focused testing complete!"
EOF

chmod +x "${OUTPUT_DIR}/run_security_focused_tests.sh"

# Step 5: Create analysis and reporting tools
echo "Step 5: Creating analysis and reporting tools..."

cat > "${OUTPUT_DIR}/analyze_fuzz_results.py" << 'EOF'
#!/usr/bin/env python3
"""
CURSED Fuzz Results Analyzer

Analyzes fuzzing results, categorizes crashes, and generates security reports.
"""

import os
import re
import json
import subprocess
from pathlib import Path
from collections import defaultdict
from typing import Dict, List, Set

class FuzzResultsAnalyzer:
    def __init__(self, results_dir: str):
        self.results_dir = Path(results_dir)
        self.crashes = []
        self.coverage_data = {}
        self.performance_data = {}
    
    def analyze_all_results(self):
        """Analyze all fuzzing results"""
        print("🔍 Analyzing fuzz results...")
        
        self.find_crashes()
        self.analyze_coverage()
        self.analyze_performance()
        self.categorize_security_issues()
        
        return self.generate_report()
    
    def find_crashes(self):
        """Find and analyze crash files"""
        crash_dirs = list(self.results_dir.glob("**/crashes/*"))
        
        for crash_dir in crash_dirs:
            if crash_dir.is_dir():
                target_name = crash_dir.name
                crash_files = list(crash_dir.glob("crash-*"))
                
                for crash_file in crash_files:
                    crash_info = self.analyze_crash(crash_file, target_name)
                    if crash_info:
                        self.crashes.append(crash_info)
    
    def analyze_crash(self, crash_file: Path, target_name: str) -> Dict:
        """Analyze individual crash file"""
        try:
            with open(crash_file, 'rb') as f:
                crash_data = f.read()
            
            # Determine crash type based on content
            crash_type = self.categorize_crash_type(crash_data)
            
            return {
                'target': target_name,
                'file': str(crash_file),
                'size': len(crash_data),
                'type': crash_type,
                'preview': repr(crash_data[:100])
            }
        except Exception as e:
            print(f"❌ Error analyzing {crash_file}: {e}")
            return None
    
    def categorize_crash_type(self, data: bytes) -> str:
        """Categorize crash type based on input data"""
        if len(data) == 0:
            return "empty_input"
        elif len(data) > 10000:
            return "large_input"
        elif b'\x00' in data:
            return "null_bytes"
        elif all(b == data[0] for b in data):
            return "repeated_pattern"
        elif len(set(data)) < 5:
            return "low_entropy"
        else:
            return "complex_input"
    
    def analyze_coverage(self):
        """Analyze code coverage from fuzzing"""
        # Look for coverage files
        coverage_files = list(self.results_dir.glob("**/coverage.*"))
        # Implementation depends on coverage format used
        pass
    
    def analyze_performance(self):
        """Analyze performance metrics"""
        log_files = list(self.results_dir.glob("**/logs/*.log"))
        
        for log_file in log_files:
            target_name = log_file.stem
            try:
                with open(log_file, 'r') as f:
                    content = f.read()
                
                # Extract performance metrics
                exec_per_sec = self.extract_metric(content, r'exec/s:\s*(\d+)')
                total_execs = self.extract_metric(content, r'#(\d+)\s+DONE')
                
                if exec_per_sec or total_execs:
                    self.performance_data[target_name] = {
                        'executions_per_second': exec_per_sec,
                        'total_executions': total_execs
                    }
            except Exception as e:
                print(f"⚠️  Error analyzing {log_file}: {e}")
    
    def extract_metric(self, content: str, pattern: str) -> int:
        """Extract numeric metric from log content"""
        match = re.search(pattern, content)
        return int(match.group(1)) if match else 0
    
    def categorize_security_issues(self):
        """Categorize crashes by potential security impact"""
        for crash in self.crashes:
            crash['security_risk'] = self.assess_security_risk(crash)
    
    def assess_security_risk(self, crash: Dict) -> str:
        """Assess security risk level of a crash"""
        target = crash['target']
        crash_type = crash['type']
        
        # High risk: parser crashes, buffer overflows
        if 'parser' in target or 'buffer' in target:
            if crash_type in ['large_input', 'null_bytes']:
                return 'high'
        
        # Medium risk: file I/O, network crashes
        if any(keyword in target for keyword in ['file', 'network', 'io']):
            return 'medium'
        
        # Low risk: other crashes
        return 'low'
    
    def generate_report(self) -> Dict:
        """Generate comprehensive analysis report"""
        report = {
            'summary': {
                'total_crashes': len(self.crashes),
                'high_risk_crashes': len([c for c in self.crashes if c.get('security_risk') == 'high']),
                'medium_risk_crashes': len([c for c in self.crashes if c.get('security_risk') == 'medium']),
                'low_risk_crashes': len([c for c in self.crashes if c.get('security_risk') == 'low']),
                'targets_tested': len(set(c['target'] for c in self.crashes)),
                'performance_data': self.performance_data
            },
            'crashes_by_target': defaultdict(list),
            'crashes_by_type': defaultdict(list),
            'security_recommendations': []
        }
        
        # Group crashes
        for crash in self.crashes:
            report['crashes_by_target'][crash['target']].append(crash)
            report['crashes_by_type'][crash['type']].append(crash)
        
        # Generate recommendations
        report['security_recommendations'] = self.generate_security_recommendations()
        
        return report
    
    def generate_security_recommendations(self) -> List[str]:
        """Generate security recommendations based on findings"""
        recommendations = []
        
        high_risk_crashes = [c for c in self.crashes if c.get('security_risk') == 'high']
        if high_risk_crashes:
            recommendations.append(
                f"🚨 {len(high_risk_crashes)} high-risk crashes found. "
                "Immediate security review required for parser and buffer handling functions."
            )
        
        large_input_crashes = [c for c in self.crashes if c['type'] == 'large_input']
        if large_input_crashes:
            recommendations.append(
                f"📏 {len(large_input_crashes)} crashes with large inputs. "
                "Implement input size validation and limits."
            )
        
        null_byte_crashes = [c for c in self.crashes if c['type'] == 'null_bytes']
        if null_byte_crashes:
            recommendations.append(
                f"🚫 {len(null_byte_crashes)} crashes with null bytes. "
                "Review string handling and null termination logic."
            )
        
        return recommendations

def main():
    import argparse
    
    parser = argparse.ArgumentParser(description="Analyze CURSED fuzz results")
    parser.add_argument("--results-dir", default="fuzz_targets", help="Fuzz results directory")
    parser.add_argument("--output", default="fuzz_analysis_report.json", help="Output report file")
    
    args = parser.parse_args()
    
    analyzer = FuzzResultsAnalyzer(args.results_dir)
    report = analyzer.analyze_all_results()
    
    # Save report
    with open(args.output, 'w') as f:
        json.dump(report, f, indent=2, default=str)
    
    # Print summary
    print(f"\n📊 Fuzz Results Analysis Summary:")
    print(f"================================")
    print(f"Total crashes found: {report['summary']['total_crashes']}")
    print(f"High risk: {report['summary']['high_risk_crashes']}")
    print(f"Medium risk: {report['summary']['medium_risk_crashes']}")
    print(f"Low risk: {report['summary']['low_risk_crashes']}")
    print(f"Targets tested: {report['summary']['targets_tested']}")
    
    if report['security_recommendations']:
        print(f"\n🛡️  Security Recommendations:")
        for rec in report['security_recommendations']:
            print(f"  {rec}")
    
    print(f"\n📋 Full report saved to: {args.output}")

if __name__ == "__main__":
    main()
EOF

chmod +x "${OUTPUT_DIR}/analyze_fuzz_results.py"

# Step 6: Create quick start guide
echo "Step 6: Creating documentation..."

cat > "${OUTPUT_DIR}/README.md" << 'EOF'
# CURSED Automated Fuzz Testing Suite

This directory contains automatically generated fuzz targets for the CURSED programming language compiler and runtime.

## Quick Start

### 1. Run Basic Fuzzing
```bash
# Run all fuzz targets for 1 minute each
./run_comprehensive_fuzz_tests.sh 60

# Run security-focused tests (5 minutes each)
./run_security_focused_tests.sh
```

### 2. Build Individual Targets
```bash
# Build all targets
./build_all_targets.sh

# Build specific category
./build_parser.sh
./build_file_io.sh
./build_network.sh
```

### 3. Analyze Results
```bash
# Analyze all fuzzing results
python3 analyze_fuzz_results.py --results-dir .
```

## Directory Structure

- `fuzz_*.c` - LibFuzzer targets for C code
- `fuzz_*.zig` - Zig fuzz targets
- `specialized/` - Template-based specialized targets
- `corpus/` - Fuzzing input corpus
- `crashes/` - Discovered crashes
- `logs/` - Fuzzing logs

## Target Categories

### Parser Targets (Critical Priority)
- `fuzz_parser_parseProgram` - Main program parser
- `fuzz_parser_parseStatement` - Statement parser
- `fuzz_parser_parseExpression` - Expression parser
- `fuzz_parser_tokenize` - Lexer/tokenizer

### File I/O Targets
- `fuzz_file_read_file` - File reading operations
- `fuzz_file_write_file` - File writing operations
- `fuzz_file_parse_file` - File parsing

### Network Targets
- `fuzz_network_http_*` - HTTP operations
- `fuzz_network_tcp_*` - TCP operations

### Memory Targets
- `fuzz_buffer_*` - Buffer operations
- `fuzz_memory_*` - Memory management

## Security Focus Areas

1. **Input Validation**: Parser targets test malformed CURSED code
2. **Buffer Safety**: Memory targets test buffer overflows
3. **File System**: File I/O targets test path traversal
4. **Network Security**: Network targets test protocol parsing

## Interpreting Results

### Crash Categories
- `empty_input` - Crashes with empty input
- `large_input` - Crashes with oversized input
- `null_bytes` - Crashes involving null bytes
- `repeated_pattern` - Crashes with repeated patterns
- `low_entropy` - Crashes with simple patterns

### Risk Levels
- **High**: Parser, buffer, and memory management crashes
- **Medium**: File I/O and network crashes
- **Low**: Other functional crashes

## Extending the Test Suite

### Adding New Targets
1. Add function signatures to `fuzz_target_discovery.py`
2. Run discovery: `python3 tools/fuzz_target_discovery.py --generate`
3. Customize generated templates as needed

### Custom Test Cases
Add test cases to `test_cases_*.txt` files and rebuild targets.

## Performance Tuning

### Resource Limits
- Default memory limit: 512MB per fuzzer
- Default timeout: 60 seconds per target
- Parallel jobs: 4 (adjust based on CPU cores)

### Optimization
```bash
# High-performance fuzzing
./run_comprehensive_fuzz_tests.sh 300 8  # 5min, 8 jobs

# Quick smoke test
./run_comprehensive_fuzz_tests.sh 10 1   # 10sec, 1 job
```

## Continuous Integration

Add to CI pipeline:
```yaml
- name: Security Fuzzing
  run: |
    cd fuzz_targets
    ./run_security_focused_tests.sh
    python3 analyze_fuzz_results.py
```

## Troubleshooting

### Build Issues
- Ensure clang with fuzzer support is installed
- Check that CURSED compiler builds successfully
- Verify all dependencies are available

### Runtime Issues
- Increase memory limits for large inputs
- Check file permissions for corpus directories
- Monitor disk space for crash files

## Security Advisory

⚠️ **Important**: This fuzzing suite may discover security vulnerabilities. 
Handle any discovered crashes as potential security issues and follow 
responsible disclosure practices.
EOF

# Final summary
echo ""
echo "🎉 CURSED Fuzz Target Discovery Complete!"
echo "========================================"
echo ""
echo "📁 Generated files:"
echo "  - Discovery report: $REPORT_FILE"
echo "  - Fuzz targets: $OUTPUT_DIR/"
echo "  - Specialized targets: $OUTPUT_DIR/specialized/"
echo ""
echo "🚀 Next steps:"
echo "  1. Review discovery report: cat $REPORT_FILE"
echo "  2. Run comprehensive tests: cd $OUTPUT_DIR && ./run_comprehensive_fuzz_tests.sh"
echo "  3. Run security tests: cd $OUTPUT_DIR && ./run_security_focused_tests.sh"
echo "  4. Analyze results: cd $OUTPUT_DIR && python3 analyze_fuzz_results.py"
echo ""
echo "📖 Documentation: $OUTPUT_DIR/README.md"
echo ""

# Show quick stats from discovery
if [ -f "$REPORT_FILE" ]; then
    echo "📊 Discovery Summary:"
    python3 -c "
import json
with open('$REPORT_FILE', 'r') as f:
    data = json.load(f)
    summary = data['summary']
    print(f'  Total functions: {summary[\"total_functions\"]}')
    print(f'  Critical risk: {summary[\"critical_risk\"]}')
    print(f'  High risk: {summary[\"high_risk\"]}')
    print(f'  Medium risk: {summary[\"medium_risk\"]}')
    print(f'  Languages: {summary[\"languages\"]}')
"
fi

echo ""
echo "✅ Fuzz target discovery system ready!"
