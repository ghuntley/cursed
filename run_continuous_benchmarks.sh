#!/bin/bash

# CURSED Compiler Continuous Benchmark Runner
# This script starts the continuous benchmark harness and dashboard

set -e

echo "🚀 CURSED Continuous Benchmark System"
echo "====================================="

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
WORKSPACE_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
VENV_DIR="$WORKSPACE_DIR/benchmark_venv"
CONFIG_FILE="$WORKSPACE_DIR/benchmark_config.json"
DATABASE_FILE="$WORKSPACE_DIR/benchmark_results.db"
LOG_FILE="$WORKSPACE_DIR/benchmark_harness.log"

# Functions
check_requirements() {
    echo -e "${BLUE}Checking requirements...${NC}"
    
    # Check Python 3
    if ! command -v python3 &> /dev/null; then
        echo -e "${RED}Error: Python 3 is required but not installed${NC}"
        exit 1
    fi
    
    # Check if virtual environment exists
    if [ ! -d "$VENV_DIR" ]; then
        echo -e "${YELLOW}Creating Python virtual environment...${NC}"
        python3 -m venv "$VENV_DIR"
    fi
    
    # Activate virtual environment
    source "$VENV_DIR/bin/activate"
    
    # Install/upgrade required packages
    echo -e "${YELLOW}Installing/updating Python dependencies...${NC}"
    pip install --upgrade pip
    
    if [ -f "$WORKSPACE_DIR/requirements.txt" ]; then
        pip install -r "$WORKSPACE_DIR/requirements.txt"
    else
        # Install essential packages if requirements.txt doesn't exist
        pip install numpy scipy matplotlib plotly pandas requests flask
    fi
    
    echo -e "${GREEN}✅ Requirements check completed${NC}"
}

build_compiler() {
    echo -e "${BLUE}Building CURSED compiler...${NC}"
    
    cd "$WORKSPACE_DIR"
    
    # Clean build
    rm -rf zig-cache/ zig-out/
    
    # Build compiler
    if zig build; then
        echo -e "${GREEN}✅ Compiler built successfully${NC}"
    else
        echo -e "${RED}❌ Compiler build failed${NC}"
        exit 1
    fi
}

setup_benchmark_files() {
    echo -e "${BLUE}Setting up benchmark files...${NC}"
    
    # Create test files if they don't exist
    mkdir -p "$WORKSPACE_DIR/benchmark_reports"
    
    # Create a simple test benchmark if comprehensive ones don't exist
    if [ ! -f "$WORKSPACE_DIR/comprehensive_stdlib_test.csd" ]; then
        cat > "$WORKSPACE_DIR/comprehensive_stdlib_test.csd" << 'EOF'
# Simple stdlib test for benchmarking
yeet "mathz"
yeet "stringz"
yeet "arrayz"

sus x drip = 42
sus y drip = abs_normie(-10)
sus result drip = x + y

sus arr []drip = [1, 2, 3, 4, 5]
sus length drip = len(arr)

vibez.spill("Benchmark test completed successfully")
vibez.spill("Result:", result)
vibez.spill("Array length:", length)
EOF
    fi
    
    if [ ! -f "$WORKSPACE_DIR/advanced_features_test.csd" ]; then
        cat > "$WORKSPACE_DIR/advanced_features_test.csd" << 'EOF'
# Advanced features test for benchmarking
sus counter drip = 0

# Function definition
slay increment(x drip) drip {
    damn x + 1
}

# Loop test
bestie (counter < 1000) {
    counter = increment(counter)
}

# Array operations
sus numbers []drip = [1, 2, 3, 4, 5]
sus total drip = 0
sus i drip = 0

bestie (i < len(numbers)) {
    total = total + numbers[i]
    i = i + 1
}

vibez.spill("Advanced features test completed")
vibez.spill("Counter:", counter)
vibez.spill("Total:", total)
EOF
    fi
    
    echo -e "${GREEN}✅ Benchmark files ready${NC}"
}

start_monitoring() {
    echo -e "${BLUE}Starting continuous benchmark monitoring...${NC}"
    
    source "$VENV_DIR/bin/activate"
    cd "$WORKSPACE_DIR"
    
    # Check if already running
    if pgrep -f "continuous_benchmark_harness.py" > /dev/null; then
        echo -e "${YELLOW}Warning: Benchmark harness is already running${NC}"
        echo "Use 'pkill -f continuous_benchmark_harness.py' to stop it first"
        return 1
    fi
    
    # Start the benchmark harness in background
    echo -e "${GREEN}Starting benchmark harness...${NC}"
    nohup python3 continuous_benchmark_harness.py \
        --workspace "$WORKSPACE_DIR" \
        --config "$CONFIG_FILE" \
        --mode monitor \
        > "$LOG_FILE" 2>&1 &
    
    HARNESS_PID=$!
    echo "Benchmark harness started with PID: $HARNESS_PID"
    
    # Wait a moment to check if it started successfully
    sleep 3
    if ! kill -0 $HARNESS_PID 2>/dev/null; then
        echo -e "${RED}❌ Benchmark harness failed to start${NC}"
        echo "Check log file: $LOG_FILE"
        return 1
    fi
    
    echo -e "${GREEN}✅ Benchmark harness is running${NC}"
    echo "Log file: $LOG_FILE"
    echo "Database: $DATABASE_FILE"
    return 0
}

start_dashboard() {
    echo -e "${BLUE}Starting benchmark dashboard...${NC}"
    
    source "$VENV_DIR/bin/activate"
    cd "$WORKSPACE_DIR"
    
    # Check if dashboard is already running
    if pgrep -f "benchmark_dashboard.py" > /dev/null; then
        echo -e "${YELLOW}Warning: Dashboard is already running${NC}"
        echo "Use 'pkill -f benchmark_dashboard.py' to stop it first"
        return 1
    fi
    
    # Start the dashboard
    echo -e "${GREEN}Starting dashboard on http://localhost:5000${NC}"
    nohup python3 benchmark_dashboard.py \
        --database "$DATABASE_FILE" \
        --host "0.0.0.0" \
        --port 5000 \
        > dashboard.log 2>&1 &
    
    DASHBOARD_PID=$!
    echo "Dashboard started with PID: $DASHBOARD_PID"
    
    # Wait a moment to check if it started successfully
    sleep 3
    if ! kill -0 $DASHBOARD_PID 2>/dev/null; then
        echo -e "${RED}❌ Dashboard failed to start${NC}"
        echo "Check log file: dashboard.log"
        return 1
    fi
    
    echo -e "${GREEN}✅ Dashboard is running at http://localhost:5000${NC}"
    return 0
}

run_single_benchmark() {
    echo -e "${BLUE}Running single benchmark cycle...${NC}"
    
    source "$VENV_DIR/bin/activate"
    cd "$WORKSPACE_DIR"
    
    python3 continuous_benchmark_harness.py \
        --workspace "$WORKSPACE_DIR" \
        --config "$CONFIG_FILE" \
        --mode single
    
    echo -e "${GREEN}✅ Single benchmark cycle completed${NC}"
}

generate_report() {
    local days=${1:-7}
    echo -e "${BLUE}Generating performance report for last $days days...${NC}"
    
    source "$VENV_DIR/bin/activate"
    cd "$WORKSPACE_DIR"
    
    local report_path=$(python3 continuous_benchmark_harness.py \
        --workspace "$WORKSPACE_DIR" \
        --config "$CONFIG_FILE" \
        --mode report \
        --days "$days")
    
    echo -e "${GREEN}✅ Report generated: $report_path${NC}"
}

show_status() {
    echo -e "${BLUE}Benchmark System Status${NC}"
    echo "======================="
    
    # Check harness status
    if pgrep -f "continuous_benchmark_harness.py" > /dev/null; then
        local harness_pid=$(pgrep -f "continuous_benchmark_harness.py")
        echo -e "${GREEN}✅ Benchmark Harness: RUNNING (PID: $harness_pid)${NC}"
    else
        echo -e "${RED}❌ Benchmark Harness: NOT RUNNING${NC}"
    fi
    
    # Check dashboard status
    if pgrep -f "benchmark_dashboard.py" > /dev/null; then
        local dashboard_pid=$(pgrep -f "benchmark_dashboard.py")
        echo -e "${GREEN}✅ Dashboard: RUNNING (PID: $dashboard_pid)${NC}"
        echo -e "   ${BLUE}URL: http://localhost:5000${NC}"
    else
        echo -e "${RED}❌ Dashboard: NOT RUNNING${NC}"
    fi
    
    # Check database
    if [ -f "$DATABASE_FILE" ]; then
        local db_size=$(du -h "$DATABASE_FILE" | cut -f1)
        echo -e "${GREEN}✅ Database: EXISTS ($db_size)${NC}"
        echo -e "   ${BLUE}Path: $DATABASE_FILE${NC}"
    else
        echo -e "${YELLOW}⚠️  Database: NOT FOUND${NC}"
    fi
    
    # Check log file
    if [ -f "$LOG_FILE" ]; then
        local log_size=$(du -h "$LOG_FILE" | cut -f1)
        local log_lines=$(wc -l < "$LOG_FILE")
        echo -e "${GREEN}✅ Log File: EXISTS ($log_size, $log_lines lines)${NC}"
        echo -e "   ${BLUE}Path: $LOG_FILE${NC}"
        
        # Show recent log entries
        echo -e "\n${BLUE}Recent Log Entries:${NC}"
        tail -5 "$LOG_FILE" 2>/dev/null || echo "No recent log entries"
    else
        echo -e "${YELLOW}⚠️  Log File: NOT FOUND${NC}"
    fi
}

stop_services() {
    echo -e "${BLUE}Stopping benchmark services...${NC}"
    
    # Stop harness
    if pgrep -f "continuous_benchmark_harness.py" > /dev/null; then
        echo "Stopping benchmark harness..."
        pkill -f "continuous_benchmark_harness.py"
        sleep 2
        echo -e "${GREEN}✅ Benchmark harness stopped${NC}"
    fi
    
    # Stop dashboard
    if pgrep -f "benchmark_dashboard.py" > /dev/null; then
        echo "Stopping dashboard..."
        pkill -f "benchmark_dashboard.py"
        sleep 2
        echo -e "${GREEN}✅ Dashboard stopped${NC}"
    fi
    
    echo -e "${GREEN}✅ All services stopped${NC}"
}

show_help() {
    echo "CURSED Continuous Benchmark System"
    echo "=================================="
    echo
    echo "Usage: $0 [COMMAND]"
    echo
    echo "Commands:"
    echo "  setup       - Set up the benchmark environment"
    echo "  start       - Start continuous monitoring and dashboard"
    echo "  monitor     - Start only the benchmark monitoring"
    echo "  dashboard   - Start only the web dashboard"
    echo "  single      - Run a single benchmark cycle"
    echo "  report [N]  - Generate performance report for last N days (default: 7)"
    echo "  status      - Show system status"
    echo "  stop        - Stop all services"
    echo "  logs        - Show recent log entries"
    echo "  help        - Show this help message"
    echo
    echo "Examples:"
    echo "  $0 setup                    # Initial setup"
    echo "  $0 start                    # Start full system"
    echo "  $0 single                   # Run one benchmark cycle"
    echo "  $0 report 30                # Generate 30-day report"
    echo "  $0 status                   # Check system status"
}

show_logs() {
    echo -e "${BLUE}Recent Benchmark Logs${NC}"
    echo "====================="
    
    if [ -f "$LOG_FILE" ]; then
        tail -20 "$LOG_FILE"
    else
        echo "No log file found at: $LOG_FILE"
    fi
    
    echo
    echo -e "${BLUE}Recent Dashboard Logs${NC}"
    echo "======================"
    
    if [ -f "dashboard.log" ]; then
        tail -20 "dashboard.log"
    else
        echo "No dashboard log file found"
    fi
}

# Main command handling
case "${1:-help}" in
    setup)
        check_requirements
        build_compiler
        setup_benchmark_files
        echo -e "\n${GREEN}✅ Setup completed successfully!${NC}"
        echo -e "${BLUE}Next steps:${NC}"
        echo "  • Run '$0 start' to begin continuous monitoring"
        echo "  • Run '$0 dashboard' to start the web interface"
        echo "  • Run '$0 single' to test with a single benchmark cycle"
        ;;
    
    start)
        check_requirements
        build_compiler
        setup_benchmark_files
        
        if start_monitoring && start_dashboard; then
            echo -e "\n${GREEN}🎉 Continuous benchmark system is now running!${NC}"
            echo -e "${BLUE}Access the dashboard at: http://localhost:5000${NC}"
            echo -e "${BLUE}Monitor logs with: $0 logs${NC}"
            echo -e "${BLUE}Check status with: $0 status${NC}"
        else
            echo -e "\n${RED}❌ Failed to start benchmark system${NC}"
            exit 1
        fi
        ;;
    
    monitor)
        check_requirements
        build_compiler
        setup_benchmark_files
        start_monitoring
        ;;
    
    dashboard)
        check_requirements
        start_dashboard
        ;;
    
    single)
        check_requirements
        build_compiler
        setup_benchmark_files
        run_single_benchmark
        ;;
    
    report)
        generate_report "${2:-7}"
        ;;
    
    status)
        show_status
        ;;
    
    stop)
        stop_services
        ;;
    
    logs)
        show_logs
        ;;
    
    help|--help|-h)
        show_help
        ;;
    
    *)
        echo -e "${RED}Unknown command: $1${NC}"
        echo
        show_help
        exit 1
        ;;
esac
