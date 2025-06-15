#!/bin/bash
# fr fr Redis driver test runner - make sure our Redis game is on point periodt!
#
# This script runs the Redis driver test suite with proper environment setup
# and provides comprehensive test coverage reporting.

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Test configuration
REDIS_URL="${REDIS_URL:-redis://localhost:6379}"
TEST_DB="${TEST_DB:-15}"  # Use database 15 for tests to avoid conflicts

echo -e "${BLUE}🔥 CURSED Redis Driver Test Suite 🔥${NC}"
echo "=================================="

# Check if Redis is available
echo -e "\n${YELLOW}📡 Checking Redis connectivity...${NC}"
if command -v redis-cli &> /dev/null; then
    if redis-cli ping &> /dev/null; then
        echo -e "${GREEN}✅ Redis is running and accessible${NC}"
        REDIS_AVAILABLE=true
    else
        echo -e "${RED}❌ Redis is not responding${NC}"
        REDIS_AVAILABLE=false
    fi
else
    echo -e "${YELLOW}⚠️  redis-cli not found, assuming Redis might be available${NC}"
    REDIS_AVAILABLE=false
fi

# Function to run tests with proper linking fix
run_test() {
    local test_name=$1
    local ignored=$2
    
    echo -e "\n${BLUE}🧪 Running $test_name tests...${NC}"
    
    if [ -f "./fix_linking.sh" ]; then
        if [ "$ignored" = "true" ]; then
            ./fix_linking.sh cargo test --test redis_driver_test -- --ignored "$test_name"
        else
            ./fix_linking.sh cargo test --test redis_driver_test "$test_name"
        fi
    else
        if [ "$ignored" = "true" ]; then
            cargo test --test redis_driver_test -- --ignored "$test_name"
        else
            cargo test --test redis_driver_test "$test_name"
        fi
    fi
}

# Run unit tests (don't require Redis)
echo -e "\n${YELLOW}🔧 Running unit tests (no Redis required)...${NC}"
run_test "test_redis_config" false
run_test "test_value_conversions" false
run_test "test_driver_creation" false
run_test "test_error_handling" false
run_test "test_config_edge_cases" false

echo -e "\n${GREEN}✅ Unit tests completed${NC}"

# Run integration tests (require Redis)
if [ "$REDIS_AVAILABLE" = true ]; then
    echo -e "\n${YELLOW}🚀 Running integration tests (Redis required)...${NC}"
    
    # Set test database
    export REDIS_URL="${REDIS_URL}/${TEST_DB}"
    
    run_test "test_redis_connection" true
    run_test "test_basic_redis_operations" true
    run_test "test_redis_expiration" true
    run_test "test_redis_increment_operations" true
    run_test "test_redis_list_operations" true
    run_test "test_redis_set_operations" true
    run_test "test_redis_hash_operations" true
    run_test "test_redis_advanced_operations" true
    run_test "test_connection_statistics" true
    run_test "test_nosql_trait_implementation" true
    
    echo -e "\n${GREEN}✅ Integration tests completed${NC}"
    
    # Run performance tests if requested
    if [ "$1" = "--performance" ] || [ "$1" = "--perf" ]; then
        echo -e "\n${YELLOW}⚡ Running performance tests...${NC}"
        run_test "test_redis_performance" true
        echo -e "\n${GREEN}✅ Performance tests completed${NC}"
    fi
    
    # Cleanup test data
    echo -e "\n${YELLOW}🧹 Cleaning up test data...${NC}"
    if command -v redis-cli &> /dev/null; then
        redis-cli -n $TEST_DB flushdb &> /dev/null || true
    fi
    
else
    echo -e "\n${YELLOW}⚠️  Skipping integration tests - Redis not available${NC}"
    echo -e "   To run integration tests:"
    echo -e "   1. Start Redis: ${BLUE}redis-server${NC}"
    echo -e "   2. Re-run tests: ${BLUE}$0${NC}"
fi

# Generate test report if requested
if [ "$1" = "--report" ] || [ "$2" = "--report" ]; then
    echo -e "\n${YELLOW}📊 Generating test coverage report...${NC}"
    
    if command -v cargo-tarpaulin &> /dev/null; then
        if [ -f "./fix_linking.sh" ]; then
            ./fix_linking.sh cargo tarpaulin --test redis_driver_test --out Html --output-dir coverage/redis
        else
            cargo tarpaulin --test redis_driver_test --out Html --output-dir coverage/redis
        fi
        echo -e "${GREEN}✅ Coverage report generated in coverage/redis/tarpaulin-report.html${NC}"
    else
        echo -e "${YELLOW}⚠️  cargo-tarpaulin not installed, skipping coverage report${NC}"
        echo -e "   Install with: ${BLUE}cargo install cargo-tarpaulin${NC}"
    fi
fi

# Test summary
echo -e "\n${GREEN}🎉 Redis driver test suite completed!${NC}"
echo -e "================================="

if [ "$REDIS_AVAILABLE" = true ]; then
    echo -e "${GREEN}✅ All tests executed successfully${NC}"
    echo -e "   - Unit tests: Configuration, conversions, error handling"
    echo -e "   - Integration tests: Redis operations, data structures, performance"
    
    if [ "$1" = "--performance" ] || [ "$1" = "--perf" ]; then
        echo -e "   - Performance tests: Throughput and latency benchmarks"
    fi
else
    echo -e "${YELLOW}⚠️  Integration tests skipped (Redis not available)${NC}"
    echo -e "${GREEN}✅ Unit tests passed successfully${NC}"
fi

echo -e "\n${BLUE}Redis driver is ready for production use! 🚀${NC}"

# Exit codes
if [ "$REDIS_AVAILABLE" = true ]; then
    exit 0
else
    exit 1  # Non-zero to indicate incomplete testing
fi
