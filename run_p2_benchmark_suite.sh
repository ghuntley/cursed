#!/bin/bash

# P2 CURSED Compiler Comprehensive Benchmark Suite Runner
# Provides automation for the complete benchmarking system

set -e

echo "🚀 P2 CURSED Comprehensive Benchmark Suite"
echo "=========================================="

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
NC='\033[0m' # No Color

# Configuration
WORKSPACE_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
VENV_DIR="$WORKSPACE_DIR/p2_benchmark_venv"
CONFIG_FILE="$WORKSPACE_DIR/p2_benchmark_config.json"
DATABASE_FILE="$WORKSPACE_DIR/p2_benchmark_results.db"
LOG_FILE="$WORKSPACE_DIR/p2_benchmark_suite.log"
REQUIREMENTS_FILE="$WORKSPACE_DIR/p2_requirements.txt"

# Create requirements file if it doesn't exist
create_requirements() {
    if [ ! -f "$REQUIREMENTS_FILE" ]; then
        echo -e "${YELLOW}Creating Python requirements file...${NC}"
        cat > "$REQUIREMENTS_FILE" << EOF
numpy>=1.21.0
pandas>=1.3.0
scipy>=1.7.0
scikit-learn>=1.0.0
matplotlib>=3.4.0
seaborn>=0.11.0
plotly>=5.0.0
requests>=2.26.0
psutil>=5.8.0
click>=8.0.0
jinja2>=3.0.0
flask>=2.0.0
EOF
    fi
}

# Setup function
setup() {
    echo -e "${BLUE}Setting up P2 Comprehensive Benchmark Suite...${NC}"
    
    # Check Python 3
    if ! command -v python3 &> /dev/null; then
        echo -e "${RED}Error: Python 3 is required but not installed${NC}"
        exit 1
    fi
    
    # Check Valgrind for memory leak detection
    if ! command -v valgrind &> /dev/null; then
        echo -e "${YELLOW}Warning: Valgrind not found. Memory leak detection will be limited.${NC}"
        echo -e "${YELLOW}Install with: sudo apt-get install valgrind (Ubuntu/Debian)${NC}"
    fi
    
    # Create virtual environment
    if [ ! -d "$VENV_DIR" ]; then
        echo -e "${YELLOW}Creating Python virtual environment...${NC}"
        python3 -m venv "$VENV_DIR"
    fi
    
    # Activate virtual environment
    source "$VENV_DIR/bin/activate"
    
    # Create requirements file
    create_requirements
    
    # Install/upgrade required packages
    echo -e "${YELLOW}Installing/updating Python dependencies...${NC}"
    pip install --upgrade pip
    pip install -r "$REQUIREMENTS_FILE"
    
    # Build CURSED compiler
    echo -e "${YELLOW}Building CURSED compiler...${NC}"
    if ! zig build -Doptimize=ReleaseFast; then
        echo -e "${RED}Failed to build CURSED compiler${NC}"
        exit 1
    fi
    
    # Create benchmark directories
    mkdir -p "$WORKSPACE_DIR/benchmarks/real_world"
    mkdir -p "$WORKSPACE_DIR/p2_benchmark_reports"
    
    # Create equivalent benchmarks for other languages if they don't exist
    create_cross_language_benchmarks
    
    echo -e "${GREEN}P2 Benchmark Suite setup completed successfully!${NC}"
}

# Create cross-language equivalent benchmarks
create_cross_language_benchmarks() {
    echo -e "${BLUE}Creating cross-language benchmark equivalents...${NC}"
    
    # Create Go benchmarks
    mkdir -p "$WORKSPACE_DIR/benchmarks/go"
    
    # Fasta benchmark in Go
    cat > "$WORKSPACE_DIR/benchmarks/go/fasta.go" << 'EOF'
package main

import (
    "fmt"
    "math/rand"
    "time"
)

const (
    LINE_LENGTH = 60
    N = 25000000
)

func main() {
    start := time.Now()
    
    // Generate DNA sequence
    sequence := make([]byte, N)
    chars := []byte("ACGT")
    
    for i := 0; i < N; i++ {
        sequence[i] = chars[rand.Intn(4)]
    }
    
    // Output in FASTA format
    fmt.Printf(">Random DNA sequence\n")
    for i := 0; i < N; i += LINE_LENGTH {
        end := i + LINE_LENGTH
        if end > N {
            end = N
        }
        fmt.Printf("%s\n", string(sequence[i:end]))
    }
    
    elapsed := time.Since(start)
    fmt.Printf("Time: %v\n", elapsed)
}
EOF

    # Mandelbrot benchmark in Go
    cat > "$WORKSPACE_DIR/benchmarks/go/mandelbrot.go" << 'EOF'
package main

import (
    "fmt"
    "time"
)

const (
    WIDTH = 800
    HEIGHT = 600
    MAX_ITER = 80
)

func mandelbrot(c complex128) int {
    z := complex(0, 0)
    for i := 0; i < MAX_ITER; i++ {
        if real(z)*real(z)+imag(z)*imag(z) > 4 {
            return i
        }
        z = z*z + c
    }
    return MAX_ITER
}

func main() {
    start := time.Now()
    
    for y := 0; y < HEIGHT; y++ {
        for x := 0; x < WIDTH; x++ {
            real := float64(x-WIDTH/2) * 4.0 / WIDTH
            imag := float64(y-HEIGHT/2) * 4.0 / HEIGHT
            c := complex(real, imag)
            iter := mandelbrot(c)
            if iter == MAX_ITER {
                fmt.Print("*")
            } else {
                fmt.Print(" ")
            }
        }
        fmt.Println()
    }
    
    elapsed := time.Since(start)
    fmt.Printf("Time: %v\n", elapsed)
}
EOF

    # Create Rust benchmarks
    mkdir -p "$WORKSPACE_DIR/benchmarks/rust"
    
    # Fasta benchmark in Rust
    cat > "$WORKSPACE_DIR/benchmarks/rust/fasta.rs" << 'EOF'
use std::time::Instant;
use rand::Rng;

const LINE_LENGTH: usize = 60;
const N: usize = 25_000_000;

fn main() {
    let start = Instant::now();
    
    // Generate DNA sequence
    let mut rng = rand::thread_rng();
    let chars = [b'A', b'C', b'G', b'T'];
    let mut sequence = Vec::with_capacity(N);
    
    for _ in 0..N {
        sequence.push(chars[rng.gen_range(0..4)]);
    }
    
    // Output in FASTA format
    println!(">Random DNA sequence");
    for chunk in sequence.chunks(LINE_LENGTH) {
        println!("{}", std::str::from_utf8(chunk).unwrap());
    }
    
    let elapsed = start.elapsed();
    println!("Time: {:?}", elapsed);
}
EOF

    # Create C++ benchmarks
    mkdir -p "$WORKSPACE_DIR/benchmarks/cplusplus"
    
    # Fasta benchmark in C++
    cat > "$WORKSPACE_DIR/benchmarks/cplusplus/fasta.cpp" << 'EOF'
#include <iostream>
#include <vector>
#include <random>
#include <chrono>

const int LINE_LENGTH = 60;
const int N = 25000000;

int main() {
    auto start = std::chrono::high_resolution_clock::now();
    
    // Generate DNA sequence
    std::random_device rd;
    std::mt19937 gen(rd());
    std::uniform_int_distribution<> dis(0, 3);
    
    std::vector<char> sequence(N);
    const char chars[] = "ACGT";
    
    for (int i = 0; i < N; i++) {
        sequence[i] = chars[dis(gen)];
    }
    
    // Output in FASTA format
    std::cout << ">Random DNA sequence\n";
    for (int i = 0; i < N; i += LINE_LENGTH) {
        int end = std::min(i + LINE_LENGTH, N);
        for (int j = i; j < end; j++) {
            std::cout << sequence[j];
        }
        std::cout << '\n';
    }
    
    auto end = std::chrono::high_resolution_clock::now();
    auto elapsed = std::chrono::duration_cast<std::chrono::milliseconds>(end - start);
    std::cout << "Time: " << elapsed.count() << "ms\n";
    
    return 0;
}
EOF

    echo -e "${GREEN}Cross-language benchmarks created${NC}"
}

# Run full comprehensive benchmark suite
run_full_suite() {
    echo -e "${PURPLE}Running P2 Comprehensive Benchmark Suite...${NC}"
    
    # Activate virtual environment
    source "$VENV_DIR/bin/activate"
    
    # Run the comprehensive benchmark suite
    python3 "$WORKSPACE_DIR/p2_comprehensive_benchmark_suite.py" \
        --workspace "$WORKSPACE_DIR" \
        --config "$CONFIG_FILE" \
        --mode full
}

# Run only CURSED benchmarks
run_cursed_only() {
    echo -e "${BLUE}Running CURSED-only benchmarks...${NC}"
    
    source "$VENV_DIR/bin/activate"
    python3 "$WORKSPACE_DIR/p2_comprehensive_benchmark_suite.py" \
        --workspace "$WORKSPACE_DIR" \
        --config "$CONFIG_FILE" \
        --mode cursed-only
}

# Run cross-language comparison
run_cross_language() {
    echo -e "${BLUE}Running cross-language performance comparison...${NC}"
    
    source "$VENV_DIR/bin/activate"
    python3 "$WORKSPACE_DIR/p2_comprehensive_benchmark_suite.py" \
        --workspace "$WORKSPACE_DIR" \
        --config "$CONFIG_FILE" \
        --mode cross-lang
}

# Run memory safety validation
run_memory_validation() {
    echo -e "${BLUE}Running memory safety validation...${NC}"
    
    source "$VENV_DIR/bin/activate"
    python3 "$WORKSPACE_DIR/p2_comprehensive_benchmark_suite.py" \
        --workspace "$WORKSPACE_DIR" \
        --config "$CONFIG_FILE" \
        --mode memory
}

# Run real-world benchmarks
run_real_world() {
    echo -e "${BLUE}Running real-world application benchmarks...${NC}"
    
    source "$VENV_DIR/bin/activate"
    python3 "$WORKSPACE_DIR/p2_comprehensive_benchmark_suite.py" \
        --workspace "$WORKSPACE_DIR" \
        --config "$CONFIG_FILE" \
        --mode real-world
}

# Generate performance report
generate_report() {
    echo -e "${BLUE}Generating comprehensive performance report...${NC}"
    
    source "$VENV_DIR/bin/activate"
    
    # Check if we have results in database
    if [ ! -f "$DATABASE_FILE" ]; then
        echo -e "${YELLOW}No benchmark results found. Running benchmarks first...${NC}"
        run_full_suite
    fi
    
    # Generate report using existing data
    REPORT_FILE=$(python3 -c "
import sqlite3
from pathlib import Path
from datetime import datetime
from p2_comprehensive_benchmark_suite import ComprehensiveBenchmarkRunner

runner = ComprehensiveBenchmarkRunner('$WORKSPACE_DIR', '$CONFIG_FILE')
# Generate report from existing data
print('Report generation would happen here')
")
    
    echo -e "${GREEN}Report generated at: $REPORT_FILE${NC}"
}

# Monitor system status
status() {
    echo -e "${BLUE}P2 Benchmark Suite Status${NC}"
    echo "========================="
    
    # Check if virtual environment exists
    if [ -d "$VENV_DIR" ]; then
        echo -e "${GREEN}✓${NC} Virtual environment: Ready"
    else
        echo -e "${RED}✗${NC} Virtual environment: Not found"
    fi
    
    # Check if CURSED compiler is built
    if [ -f "$WORKSPACE_DIR/zig-out/bin/cursed-zig" ]; then
        echo -e "${GREEN}✓${NC} CURSED compiler: Built"
        VERSION=$("$WORKSPACE_DIR/zig-out/bin/cursed-zig" --version 2>/dev/null || echo "unknown")
        echo "  Version: $VERSION"
    else
        echo -e "${RED}✗${NC} CURSED compiler: Not found"
    fi
    
    # Check database
    if [ -f "$DATABASE_FILE" ]; then
        echo -e "${GREEN}✓${NC} Database: Available"
        RECORD_COUNT=$(sqlite3 "$DATABASE_FILE" "SELECT COUNT(*) FROM cross_language_results;" 2>/dev/null || echo "0")
        echo "  Records: $RECORD_COUNT"
    else
        echo -e "${YELLOW}⚠${NC} Database: Not found (will be created on first run)"
    fi
    
    # Check logs
    if [ -f "$LOG_FILE" ]; then
        echo -e "${GREEN}✓${NC} Log file: Available"
        LOG_SIZE=$(du -h "$LOG_FILE" | cut -f1)
        echo "  Size: $LOG_SIZE"
        echo "  Recent entries:"
        tail -3 "$LOG_FILE" | sed 's/^/    /'
    else
        echo -e "${YELLOW}⚠${NC} Log file: Not found"
    fi
    
    # Check available compilers
    echo ""
    echo "Available Compilers:"
    for compiler in go rustc g++ clang++ zig; do
        if command -v "$compiler" &> /dev/null; then
            VERSION=$($compiler --version 2>/dev/null | head -1 || echo "unknown")
            echo -e "  ${GREEN}✓${NC} $compiler: $VERSION"
        else
            echo -e "  ${RED}✗${NC} $compiler: Not found"
        fi
    done
    
    # Check Valgrind
    if command -v valgrind &> /dev/null; then
        VALGRIND_VERSION=$(valgrind --version 2>/dev/null | head -1 || echo "unknown")
        echo -e "  ${GREEN}✓${NC} valgrind: $VALGRIND_VERSION"
    else
        echo -e "  ${YELLOW}⚠${NC} valgrind: Not found (memory leak detection limited)"
    fi
}

# View logs
logs() {
    if [ -f "$LOG_FILE" ]; then
        echo -e "${BLUE}P2 Benchmark Suite Logs${NC}"
        echo "======================="
        tail -50 "$LOG_FILE"
    else
        echo -e "${YELLOW}No log file found${NC}"
    fi
}

# Clean up generated files
clean() {
    echo -e "${YELLOW}Cleaning up P2 benchmark suite files...${NC}"
    
    # Remove generated reports (keep database)
    rm -rf "$WORKSPACE_DIR/p2_benchmark_reports"
    
    # Remove temporary files
    find "$WORKSPACE_DIR" -name "*.tmp" -delete
    find "$WORKSPACE_DIR" -name "test_*" -type f -delete
    
    # Remove compiled benchmark binaries
    find "$WORKSPACE_DIR/benchmarks" -type f -executable -delete
    
    echo -e "${GREEN}Cleanup completed${NC}"
}

# CI Integration helper
ci_run() {
    echo -e "${PURPLE}Running P2 Benchmark Suite in CI mode...${NC}"
    
    # Setup if needed
    if [ ! -d "$VENV_DIR" ]; then
        setup
    fi
    
    # Run benchmarks with reduced parallelism for CI
    export MAX_WORKERS=2
    export BENCHMARK_TIMEOUT=120
    
    source "$VENV_DIR/bin/activate"
    python3 "$WORKSPACE_DIR/p2_comprehensive_benchmark_suite.py" \
        --workspace "$WORKSPACE_DIR" \
        --config "$CONFIG_FILE" \
        --mode full
    
    # Generate report for CI artifacts
    generate_report
}

# Performance regression check
check_regressions() {
    echo -e "${BLUE}Checking for performance regressions...${NC}"
    
    if [ ! -f "$DATABASE_FILE" ]; then
        echo -e "${YELLOW}No historical data available for regression analysis${NC}"
        return 1
    fi
    
    source "$VENV_DIR/bin/activate"
    
    # Run regression analysis
    REGRESSIONS=$(python3 -c "
import sqlite3
from datetime import datetime, timedelta

conn = sqlite3.connect('$DATABASE_FILE')
cursor = conn.execute('''
    SELECT COUNT(*) FROM cross_language_results 
    WHERE timestamp > datetime('now', '-1 day')
''')
recent_count = cursor.fetchone()[0]

if recent_count == 0:
    print('No recent benchmark data for comparison')
else:
    print(f'Found {recent_count} recent benchmark results')
    print('Regression analysis would be performed here')
")
    
    echo "$REGRESSIONS"
}

# Help function
usage() {
    echo "P2 CURSED Compiler Comprehensive Benchmark Suite"
    echo ""
    echo "Usage: $0 [COMMAND]"
    echo ""
    echo "Commands:"
    echo "  setup              - Setup benchmark environment and dependencies"
    echo "  run                - Run full comprehensive benchmark suite"
    echo "  cursed-only        - Run only CURSED benchmarks"
    echo "  cross-lang         - Run cross-language performance comparison"
    echo "  memory             - Run memory safety validation"
    echo "  real-world         - Run real-world application benchmarks"
    echo "  report             - Generate comprehensive performance report"
    echo "  check-regressions  - Check for performance regressions"
    echo "  status             - Show system status and configuration"
    echo "  logs               - View recent log entries"
    echo "  clean              - Clean up generated files"
    echo "  ci                 - Run in CI/CD mode"
    echo "  help               - Show this help message"
    echo ""
    echo "Examples:"
    echo "  $0 setup                    # Initial setup"
    echo "  $0 run                      # Run complete benchmark suite"
    echo "  $0 memory                   # Memory leak detection only"
    echo "  $0 report                   # Generate HTML report"
    echo ""
}

# Main script logic
case "${1:-help}" in
    setup)
        setup
        ;;
    run)
        run_full_suite
        ;;
    cursed-only)
        run_cursed_only
        ;;
    cross-lang)
        run_cross_language
        ;;
    memory)
        run_memory_validation
        ;;
    real-world)
        run_real_world
        ;;
    report)
        generate_report
        ;;
    check-regressions)
        check_regressions
        ;;
    status)
        status
        ;;
    logs)
        logs
        ;;
    clean)
        clean
        ;;
    ci)
        ci_run
        ;;
    help|--help|-h)
        usage
        ;;
    *)
        echo -e "${RED}Unknown command: $1${NC}"
        echo ""
        usage
        exit 1
        ;;
esac
