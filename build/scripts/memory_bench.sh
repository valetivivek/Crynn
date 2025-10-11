#!/bin/bash

# Crynn Browser Memory Benchmarking Script
# This script measures memory usage across different scenarios

set -e

PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
BUILD_DIR="$PROJECT_ROOT/target/release"
BINARY="$BUILD_DIR/crynn-shell"
RESULTS_DIR="$PROJECT_ROOT/build/results"
LOG_FILE="$RESULTS_DIR/memory_bench_$(date +%Y%m%d_%H%M%S).log"

# Create results directory
mkdir -p "$RESULTS_DIR"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${GREEN}Crynn Browser Memory Benchmark${NC}"
echo "=================================="
echo "Binary: $BINARY"
echo "Results: $LOG_FILE"
echo ""

# Check if binary exists
if [ ! -f "$BINARY" ]; then
    echo -e "${RED}Error: Binary not found at $BINARY${NC}"
    echo "Please build the project first with: cargo build --release"
    exit 1
fi

# Function to measure memory usage
measure_memory() {
    local scenario="$1"
    local duration="$2"
    
    echo -e "${YELLOW}Measuring: $scenario${NC}"
    
    # Start the application in background
    "$BINARY" &
    local pid=$!
    
    # Wait for startup
    sleep 5
    
    # Measure memory for specified duration
    local total_rss=0
    local measurements=0
    
    for i in $(seq 1 $duration); do
        local rss=$(ps -o rss= -p $pid 2>/dev/null || echo "0")
        if [ "$rss" != "0" ]; then
            total_rss=$((total_rss + rss))
            measurements=$((measurements + 1))
        fi
        sleep 1
    done
    
    # Calculate average
    local avg_rss=0
    if [ $measurements -gt 0 ]; then
        avg_rss=$((total_rss / measurements))
    fi
    
    # Convert KB to MB
    local avg_mb=$((avg_rss / 1024))
    
    echo "  Average RSS: ${avg_mb} MB"
    echo "$scenario,$avg_mb" >> "$LOG_FILE"
    
    # Kill the process
    kill $pid 2>/dev/null || true
    wait $pid 2>/dev/null || true
    
    sleep 2
}

# Memory budget targets
COLD_START_TARGET=80
IDLE_TAB_TARGET=120
TEN_TABS_TARGET=450

echo "Memory Budget Targets:"
echo "  Cold Start: ${COLD_START_TARGET} MB"
echo "  Idle Tab: ${IDLE_TAB_TARGET} MB" 
echo "  10 Tabs: ${TEN_TABS_TARGET} MB"
echo ""

# Initialize log file
echo "scenario,memory_mb" > "$LOG_FILE"

# Benchmark scenarios
echo "Starting benchmarks..."
echo ""

# 1. Cold Start Test
measure_memory "cold_start" 10

# 2. Idle Tab Test  
measure_memory "idle_tab" 30

# 3. Navigation Test
echo -e "${YELLOW}Measuring: navigation_test${NC}"
"$BINARY" &
NAV_PID=$!
sleep 5

# Simulate navigation (this would need actual implementation)
sleep 30
nav_rss=$(ps -o rss= -p $NAV_PID 2>/dev/null || echo "0")
nav_mb=$((nav_rss / 1024))
echo "  Navigation RSS: ${nav_mb} MB"
echo "navigation_test,$nav_mb" >> "$LOG_FILE"

kill $NAV_PID 2>/dev/null || true
wait $NAV_PID 2>/dev/null || true
sleep 2

# 4. Email Sync Test
echo -e "${YELLOW}Measuring: email_sync${NC}"
"$BINARY" &
EMAIL_PID=$!
sleep 5

# Simulate email sync (this would need actual implementation)
sleep 20
email_rss=$(ps -o rss= -p $EMAIL_PID 2>/dev/null || echo "0")
email_mb=$((email_rss / 1024))
echo "  Email Sync RSS: ${email_mb} MB"
echo "email_sync,$email_mb" >> "$LOG_FILE"

kill $EMAIL_PID 2>/dev/null || true
wait $EMAIL_PID 2>/dev/null || true
sleep 2

# 5. VPN Control Test
echo -e "${YELLOW}Measuring: vpn_control${NC}"
"$BINARY" &
VPN_PID=$!
sleep 5

# Simulate VPN control (this would need actual implementation)
sleep 15
vpn_rss=$(ps -o rss= -p $VPN_PID 2>/dev/null || echo "0")
vpn_mb=$((vpn_rss / 1024))
echo "  VPN Control RSS: ${vpn_mb} MB"
echo "vpn_control,$vpn_mb" >> "$LOG_FILE"

kill $VPN_PID 2>/dev/null || true
wait $VPN_PID 2>/dev/null || true

echo ""
echo -e "${GREEN}Benchmark Complete${NC}"
echo "Results saved to: $LOG_FILE"
echo ""

# Generate summary report
echo "Memory Usage Summary:"
echo "===================="
while IFS=',' read -r scenario memory; do
    if [ "$scenario" != "scenario" ]; then
        case $scenario in
            "cold_start")
                if [ $memory -le $COLD_START_TARGET ]; then
                    echo -e "  $scenario: ${GREEN}${memory} MB${NC} (target: ${COLD_START_TARGET} MB) ✓"
                else
                    echo -e "  $scenario: ${RED}${memory} MB${NC} (target: ${COLD_START_TARGET} MB) ✗"
                fi
                ;;
            "idle_tab")
                if [ $memory -le $IDLE_TAB_TARGET ]; then
                    echo -e "  $scenario: ${GREEN}${memory} MB${NC} (target: ${IDLE_TAB_TARGET} MB) ✓"
                else
                    echo -e "  $scenario: ${RED}${memory} MB${NC} (target: ${IDLE_TAB_TARGET} MB) ✗"
                fi
                ;;
            *)
                echo -e "  $scenario: ${YELLOW}${memory} MB${NC}"
                ;;
        esac
    fi
done < "$LOG_FILE"

echo ""
echo "For detailed analysis, see: $LOG_FILE"
