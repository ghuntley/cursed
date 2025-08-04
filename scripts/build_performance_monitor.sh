#!/bin/bash

# CURSED Build Performance Monitor
# Real-time monitoring and optimization of build performance

set -euo pipefail

# Configuration
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
MONITOR_LOG_DIR="$PROJECT_ROOT/build_performance_logs"
METRICS_DB="$MONITOR_LOG_DIR/build_metrics.db"
PERFORMANCE_LOG="$MONITOR_LOG_DIR/performance.log"
REALTIME_LOG="$MONITOR_LOG_DIR/realtime.log"

# Colors
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
RED='\033[0;31m'
CYAN='\033[0;36m'
MAGENTA='\033[0;35m'
NC='\033[0m'

# Performance thresholds
SLOW_BUILD_THRESHOLD=30  # seconds
MEMORY_WARNING_THRESHOLD=1024  # MB
CPU_WARNING_THRESHOLD=80  # percentage

# Monitoring configuration
MONITOR_INTERVAL=1  # seconds
SAVE_INTERVAL=10    # seconds
MAX_LOG_SIZE_MB=100

# Logging functions
log_info() {
    echo -e "${BLUE}[MONITOR]${NC} $1" | tee -a "$PERFORMANCE_LOG"
}

log_success() {
    echo -e "${GREEN}[MONITOR]${NC} $1" | tee -a "$PERFORMANCE_LOG"
}

log_warning() {
    echo -e "${YELLOW}[MONITOR]${NC} $1" | tee -a "$PERFORMANCE_LOG"
}

log_error() {
    echo -e "${RED}[MONITOR]${NC} $1" | tee -a "$PERFORMANCE_LOG"
}

log_realtime() {
    echo "$(date -Iseconds),$1" >> "$REALTIME_LOG"
}

# Initialize monitoring system
init_monitor() {
    log_info "📊 Initializing CURSED build performance monitor"
    
    # Create directories
    mkdir -p "$MONITOR_LOG_DIR"
    
    # Initialize logs
    > "$PERFORMANCE_LOG"
    > "$REALTIME_LOG"
    
    # Create CSV header for realtime log
    echo "timestamp,build_phase,cpu_percent,memory_mb,io_read_mb,io_write_mb,processes,threads" > "$REALTIME_LOG"
    
    log_success "Performance monitor initialized"
}

# Get system metrics
get_system_metrics() {
    local metrics=""
    
    # CPU usage
    local cpu_percent
    if command -v top >/dev/null 2>&1; then
        cpu_percent=$(top -l 1 -n 0 | grep "CPU usage" | awk '{print $3}' | sed 's/%//' 2>/dev/null || echo "0")
    elif command -v vmstat >/dev/null 2>&1; then
        cpu_percent=$(vmstat 1 2 | tail -1 | awk '{print 100-$15}' 2>/dev/null || echo "0")
    else
        cpu_percent="0"
    fi
    
    # Memory usage
    local memory_mb
    if command -v free >/dev/null 2>&1; then
        memory_mb=$(free -m | awk 'NR==2{printf "%.0f", $3}' 2>/dev/null || echo "0")
    elif command -v vm_stat >/dev/null 2>&1; then
        memory_mb=$(vm_stat | grep "Pages active" | awk '{print $3}' | sed 's/\.//' | awk '{print $1*4096/1024/1024}' 2>/dev/null || echo "0")
    else
        memory_mb="0"
    fi
    
    # I/O statistics (simplified)
    local io_read_mb="0"
    local io_write_mb="0"
    if command -v iostat >/dev/null 2>&1; then
        local io_stats
        io_stats=$(iostat -d 1 2 | tail -1 2>/dev/null || echo "0 0")
        io_read_mb=$(echo "$io_stats" | awk '{print $3/1024}' 2>/dev/null || echo "0")
        io_write_mb=$(echo "$io_stats" | awk '{print $4/1024}' 2>/dev/null || echo "0")
    fi
    
    # Process count
    local processes
    processes=$(ps aux | wc -l 2>/dev/null || echo "0")
    
    # Thread count (for zig processes)
    local threads
    threads=$(ps -T | grep -c zig 2>/dev/null || echo "0")
    
    echo "$cpu_percent,$memory_mb,$io_read_mb,$io_write_mb,$processes,$threads"
}

# Monitor build process in real-time
monitor_realtime() {
    local build_pid="$1"
    local build_phase="${2:-unknown}"
    
    log_info "🔍 Starting real-time monitoring for PID $build_pid"
    
    while kill -0 "$build_pid" 2>/dev/null; do
        local metrics
        metrics=$(get_system_metrics)
        log_realtime "$build_phase,$metrics"
        
        # Parse metrics for warnings
        local cpu_percent
        local memory_mb
        cpu_percent=$(echo "$metrics" | cut -d',' -f1)
        memory_mb=$(echo "$metrics" | cut -d',' -f2)
        
        # Check thresholds
        if (( $(echo "$cpu_percent > $CPU_WARNING_THRESHOLD" | bc -l 2>/dev/null || echo "0") )); then
            log_warning "⚠️ High CPU usage: ${cpu_percent}%"
        fi
        
        if (( memory_mb > MEMORY_WARNING_THRESHOLD )); then
            log_warning "⚠️ High memory usage: ${memory_mb}MB"
        fi
        
        sleep "$MONITOR_INTERVAL"
    done
    
    log_success "✅ Real-time monitoring completed for PID $build_pid"
}

# Benchmark build command
benchmark_build() {
    local build_command=("$@")
    local start_time
    local end_time
    local build_time
    local peak_memory=0
    local peak_cpu=0
    
    log_info "🚀 Benchmarking build command: ${build_command[*]}"
    start_time=$(date +%s.%N)
    
    # Start build in background
    "${build_command[@]}" &
    local build_pid=$!
    
    # Monitor in background
    (monitor_realtime "$build_pid" "build") &
    local monitor_pid=$!
    
    # Wait for build completion
    if wait "$build_pid"; then
        end_time=$(date +%s.%N)
        build_time=$(echo "$end_time - $start_time" | bc)
        
        # Stop monitoring
        kill "$monitor_pid" 2>/dev/null || true
        wait "$monitor_pid" 2>/dev/null || true
        
        # Calculate peak metrics
        if [[ -f "$REALTIME_LOG" ]]; then
            peak_memory=$(tail -n +2 "$REALTIME_LOG" | cut -d',' -f3 | sort -n | tail -1)
            peak_cpu=$(tail -n +2 "$REALTIME_LOG" | cut -d',' -f2 | sort -n | tail -1)
        fi
        
        # Log results
        log_success "✅ Build completed successfully"
        log_info "⏱️ Build time: ${build_time}s"
        log_info "🧠 Peak memory: ${peak_memory}MB"
        log_info "⚡ Peak CPU: ${peak_cpu}%"
        
        # Check for slow builds
        if (( $(echo "$build_time > $SLOW_BUILD_THRESHOLD" | bc -l 2>/dev/null || echo "0") )); then
            log_warning "🐌 Slow build detected (>${SLOW_BUILD_THRESHOLD}s)"
            suggest_optimizations "$build_time" "$peak_memory" "$peak_cpu"
        fi
        
        # Store metrics
        store_build_metrics "${build_command[*]}" "$build_time" "$peak_memory" "$peak_cpu" "success"
        
        return 0
    else
        # Build failed
        kill "$monitor_pid" 2>/dev/null || true
        wait "$monitor_pid" 2>/dev/null || true
        
        end_time=$(date +%s.%N)
        build_time=$(echo "$end_time - $start_time" | bc)
        
        log_error "❌ Build failed after ${build_time}s"
        store_build_metrics "${build_command[*]}" "$build_time" "0" "0" "failed"
        
        return 1
    fi
}

# Store build metrics in simple log format
store_build_metrics() {
    local command="$1"
    local build_time="$2"
    local peak_memory="$3"
    local peak_cpu="$4"
    local status="$5"
    
    local metrics_file="$MONITOR_LOG_DIR/build_history.csv"
    
    # Create header if file doesn't exist
    if [[ ! -f "$metrics_file" ]]; then
        echo "timestamp,command,build_time,peak_memory_mb,peak_cpu_percent,status" > "$metrics_file"
    fi
    
    # Append metrics
    echo "$(date -Iseconds),\"$command\",$build_time,$peak_memory,$peak_cpu,$status" >> "$metrics_file"
}

# Suggest optimizations based on metrics
suggest_optimizations() {
    local build_time="$1"
    local peak_memory="$2"
    local peak_cpu="$3"
    
    log_info "💡 Performance optimization suggestions:"
    
    # Build time optimizations
    if (( $(echo "$build_time > 60" | bc -l 2>/dev/null || echo "0") )); then
        echo "  🔧 Consider enabling parallel compilation: -j$(nproc)"
        echo "  📦 Enable build caching for faster incremental builds"
        echo "  ⚡ Use ReleaseFast mode for faster compilation"
    fi
    
    # Memory optimizations
    if (( peak_memory > 2048 )); then
        echo "  🧠 High memory usage detected:"
        echo "     - Consider reducing parallel jobs"
        echo "     - Enable incremental compilation"
        echo "     - Use memory-efficient build options"
    fi
    
    # CPU optimizations
    if (( $(echo "$peak_cpu < 50" | bc -l 2>/dev/null || echo "0") )); then
        echo "  ⚡ Low CPU utilization detected:"
        echo "     - Increase parallel jobs: -j$(($(nproc) * 2))"
        echo "     - Enable multithreaded compilation"
    fi
}

# Generate performance report
generate_performance_report() {
    local report_file="$MONITOR_LOG_DIR/performance_report.md"
    local metrics_file="$MONITOR_LOG_DIR/build_history.csv"
    
    if [[ ! -f "$metrics_file" ]]; then
        log_warning "No build metrics found"
        return 1
    fi
    
    log_info "📊 Generating performance report"
    
    # Calculate statistics
    local total_builds
    local successful_builds
    local failed_builds
    local avg_build_time
    local avg_memory
    local avg_cpu
    
    total_builds=$(tail -n +2 "$metrics_file" | wc -l)
    successful_builds=$(tail -n +2 "$metrics_file" | grep ",success$" | wc -l)
    failed_builds=$(tail -n +2 "$metrics_file" | grep ",failed$" | wc -l)
    
    if [[ $successful_builds -gt 0 ]]; then
        avg_build_time=$(tail -n +2 "$metrics_file" | grep ",success$" | cut -d',' -f3 | awk '{sum+=$1} END {print sum/NR}' 2>/dev/null || echo "0")
        avg_memory=$(tail -n +2 "$metrics_file" | grep ",success$" | cut -d',' -f4 | awk '{sum+=$1} END {print sum/NR}' 2>/dev/null || echo "0")
        avg_cpu=$(tail -n +2 "$metrics_file" | grep ",success$" | cut -d',' -f5 | awk '{sum+=$1} END {print sum/NR}' 2>/dev/null || echo "0")
    else
        avg_build_time="0"
        avg_memory="0"
        avg_cpu="0"
    fi
    
    # Generate report
    {
        echo "# CURSED Build Performance Report"
        echo "Generated: $(date)"
        echo ""
        echo "## Summary Statistics"
        echo "- Total builds: $total_builds"
        echo "- Successful builds: $successful_builds"
        echo "- Failed builds: $failed_builds"
        echo "- Success rate: $(echo "scale=1; $successful_builds * 100 / $total_builds" | bc 2>/dev/null || echo "0")%"
        echo ""
        echo "## Performance Metrics (Successful Builds)"
        echo "- Average build time: ${avg_build_time}s"
        echo "- Average peak memory: ${avg_memory}MB"
        echo "- Average peak CPU: ${avg_cpu}%"
        echo ""
        echo "## Recent Build History"
        echo "| Timestamp | Build Time | Memory (MB) | CPU (%) | Status |"
        echo "|-----------|------------|-------------|---------|--------|"
        
        tail -n 10 "$metrics_file" | while IFS=',' read -r timestamp command build_time memory cpu status; do
            # Format timestamp
            local formatted_time
            formatted_time=$(echo "$timestamp" | cut -d'T' -f2 | cut -d'+' -f1)
            echo "| $formatted_time | $build_time | $memory | $cpu | $status |"
        done
        
        echo ""
        echo "## Performance Trends"
        if [[ $total_builds -gt 5 ]]; then
            local recent_avg
            local older_avg
            recent_avg=$(tail -n 5 "$metrics_file" | grep ",success$" | cut -d',' -f3 | awk '{sum+=$1} END {print sum/NR}' 2>/dev/null || echo "0")
            older_avg=$(head -n -5 "$metrics_file" | tail -n +2 | grep ",success$" | cut -d',' -f3 | awk '{sum+=$1} END {print sum/NR}' 2>/dev/null || echo "0")
            
            if (( $(echo "$recent_avg < $older_avg" | bc -l 2>/dev/null || echo "0") )); then
                echo "📈 **Performance improving**: Recent builds are faster"
            elif (( $(echo "$recent_avg > $older_avg" | bc -l 2>/dev/null || echo "0") )); then
                echo "📉 **Performance declining**: Recent builds are slower"
            else
                echo "📊 **Performance stable**: No significant trend"
            fi
        else
            echo "Insufficient data for trend analysis"
        fi
        
        echo ""
        echo "## Optimization Recommendations"
        
        # Performance-based recommendations
        if (( $(echo "$avg_build_time > 30" | bc -l 2>/dev/null || echo "0") )); then
            echo "- ⚡ **Enable parallel compilation** to reduce build times"
            echo "- 📦 **Use build caching** for incremental builds"
        fi
        
        if (( $(echo "$avg_memory > 1024" | bc -l 2>/dev/null || echo "0") )); then
            echo "- 🧠 **Optimize memory usage** with incremental compilation"
            echo "- 🔧 **Reduce parallel jobs** if memory constrained"
        fi
        
        if (( $(echo "$avg_cpu < 60" | bc -l 2>/dev/null || echo "0") )); then
            echo "- ⚡ **Increase parallelism** to better utilize CPU"
            echo "- 🚀 **Enable more aggressive optimization flags**"
        fi
        
    } > "$report_file"
    
    log_success "📊 Performance report generated: $report_file"
}

# Compare build performance
compare_builds() {
    local command1="$1"
    local command2="$2"
    local iterations="${3:-3}"
    
    log_info "⚖️ Comparing build performance: $iterations iterations each"
    
    local times1=()
    local times2=()
    
    # Benchmark first command
    log_info "Testing command 1: $command1"
    for ((i=1; i<=iterations; i++)); do
        log_info "Iteration $i/$iterations"
        local start_time
        start_time=$(date +%s.%N)
        
        if eval "$command1" > /dev/null 2>&1; then
            local end_time
            end_time=$(date +%s.%N)
            local build_time
            build_time=$(echo "$end_time - $start_time" | bc)
            times1+=("$build_time")
            log_info "✅ Completed in ${build_time}s"
        else
            log_error "❌ Build failed"
            times1+=("0")
        fi
    done
    
    # Benchmark second command
    log_info "Testing command 2: $command2"
    for ((i=1; i<=iterations; i++)); do
        log_info "Iteration $i/$iterations"
        local start_time
        start_time=$(date +%s.%N)
        
        if eval "$command2" > /dev/null 2>&1; then
            local end_time
            end_time=$(date +%s.%N)
            local build_time
            build_time=$(echo "$end_time - $start_time" | bc)
            times2+=("$build_time")
            log_info "✅ Completed in ${build_time}s"
        else
            log_error "❌ Build failed"
            times2+=("0")
        fi
    done
    
    # Calculate averages
    local avg1=0
    local avg2=0
    
    for time in "${times1[@]}"; do
        avg1=$(echo "$avg1 + $time" | bc)
    done
    avg1=$(echo "scale=2; $avg1 / ${#times1[@]}" | bc)
    
    for time in "${times2[@]}"; do
        avg2=$(echo "$avg2 + $time" | bc)
    done
    avg2=$(echo "scale=2; $avg2 / ${#times2[@]}" | bc)
    
    # Compare results
    log_success "🏁 Comparison Results:"
    echo "  Command 1 average: ${avg1}s"
    echo "  Command 2 average: ${avg2}s"
    
    if (( $(echo "$avg1 < $avg2" | bc -l) )); then
        local improvement
        improvement=$(echo "scale=1; ($avg2 - $avg1) * 100 / $avg2" | bc)
        log_success "🚀 Command 1 is faster by ${improvement}%"
    elif (( $(echo "$avg2 < $avg1" | bc -l) )); then
        local improvement
        improvement=$(echo "scale=1; ($avg1 - $avg2) * 100 / $avg1" | bc)
        log_success "🚀 Command 2 is faster by ${improvement}%"
    else
        log_info "⚖️ Both commands have similar performance"
    fi
}

# Clean up old logs
cleanup_logs() {
    log_info "🧹 Cleaning up old performance logs"
    
    # Find large log files
    find "$MONITOR_LOG_DIR" -name "*.log" -size "+${MAX_LOG_SIZE_MB}M" | while read -r log_file; do
        log_warning "Large log file detected: $(basename "$log_file")"
        # Keep last 1000 lines
        tail -n 1000 "$log_file" > "$log_file.tmp" && mv "$log_file.tmp" "$log_file"
        log_info "Truncated: $(basename "$log_file")"
    done
    
    # Remove old real-time logs (keep last 7 days)
    find "$MONITOR_LOG_DIR" -name "realtime_*.log" -mtime +7 -delete 2>/dev/null || true
    
    log_success "Log cleanup completed"
}

# Main function
main() {
    local command="${1:-help}"
    
    case "$command" in
        "init")
            init_monitor
            ;;
        "benchmark")
            shift
            if [[ $# -eq 0 ]]; then
                log_error "Usage: $0 benchmark <build_command>"
                exit 1
            fi
            init_monitor
            benchmark_build "$@"
            ;;
        "compare")
            if [[ $# -lt 3 ]]; then
                log_error "Usage: $0 compare '<command1>' '<command2>' [iterations]"
                exit 1
            fi
            init_monitor
            compare_builds "$2" "$3" "${4:-3}"
            ;;
        "report")
            generate_performance_report
            ;;
        "cleanup")
            cleanup_logs
            ;;
        "monitor")
            if [[ $# -lt 2 ]]; then
                log_error "Usage: $0 monitor <pid>"
                exit 1
            fi
            init_monitor
            monitor_realtime "$2" "manual"
            ;;
        "help"|*)
            echo "CURSED Build Performance Monitor"
            echo "Usage: $0 <command> [options]"
            echo ""
            echo "Commands:"
            echo "  init                     Initialize monitoring system"
            echo "  benchmark <command>      Benchmark build command with monitoring"
            echo "  compare <cmd1> <cmd2>    Compare two build commands"
            echo "  monitor <pid>            Monitor specific process"
            echo "  report                   Generate performance report"
            echo "  cleanup                  Clean up old logs"
            echo "  help                     Show this help"
            echo ""
            echo "Examples:"
            echo "  $0 benchmark zig build"
            echo "  $0 benchmark zig build -Doptimize=ReleaseFast"
            echo "  $0 compare 'zig build' 'zig build -j8'"
            echo "  $0 report"
            ;;
    esac
}

# Initialize and run
cd "$PROJECT_ROOT"
main "$@"
