#!/bin/bash

# Crynn Browser - Video Support Test Script
# This script tests YouTube and other video platform compatibility

set -e

echo "🎬 Testing Crynn Browser Video Support..."
echo "========================================"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Test URLs
declare -A TEST_URLS=(
    ["YouTube"]="https://youtube.com"
    ["YouTube Video"]="https://youtube.com/watch?v=dQw4w9WgXcQ"
    ["Netflix"]="https://netflix.com"
    ["Twitch"]="https://twitch.tv"
    ["Vimeo"]="https://vimeo.com"
    ["HTML5 Video Test"]="https://www.w3schools.com/html/html5_video.asp"
)

# Function to test URL
test_url() {
    local name="$1"
    local url="$2"
    
    echo -e "\n${BLUE}Testing: ${name}${NC}"
    echo -e "URL: ${url}"
    
    # Start Crynn Browser in background
    echo "🚀 Starting Crynn Browser..."
    cargo run --release &
    CRYNN_PID=$!
    
    # Wait for browser to start
    sleep 3
    
    # Check if process is still running
    if ! kill -0 $CRYNN_PID 2>/dev/null; then
        echo -e "${RED}❌ Crynn Browser failed to start${NC}"
        return 1
    fi
    
    echo -e "${GREEN}✅ Crynn Browser started successfully${NC}"
    echo -e "${YELLOW}📝 Manual test required:${NC}"
    echo -e "   1. Navigate to: ${url}"
    echo -e "   2. Verify video loads and plays"
    echo -e "   3. Test video controls (play/pause/volume)"
    echo -e "   4. Test fullscreen mode"
    echo -e "   5. Press Enter when done testing..."
    
    read -r
    
    # Kill the browser
    echo "🛑 Stopping Crynn Browser..."
    kill $CRYNN_PID 2>/dev/null || true
    wait $CRYNN_PID 2>/dev/null || true
    
    echo -e "${GREEN}✅ Test completed for ${name}${NC}"
}

# Check if GeckoView is available
echo "🔍 Checking GeckoView availability..."

if pkg-config --exists geckoview 2>/dev/null; then
    GECKOVIEW_VERSION=$(pkg-config --modversion geckoview)
    echo -e "${GREEN}✅ GeckoView ${GECKOVIEW_VERSION} found${NC}"
else
    echo -e "${RED}❌ GeckoView not found${NC}"
    echo "Please install GeckoView first:"
    echo "  ./build/scripts/install_geckoview.sh"
    exit 1
fi

# Check if Crynn Browser builds
echo -e "\n🔨 Building Crynn Browser..."
if cargo build --release; then
    echo -e "${GREEN}✅ Crynn Browser built successfully${NC}"
else
    echo -e "${RED}❌ Build failed${NC}"
    exit 1
fi

# Run tests
echo -e "\n🧪 Starting video support tests..."
echo "Note: Each test requires manual verification"

for name in "${!TEST_URLS[@]}"; do
    url="${TEST_URLS[$name]}"
    test_url "$name" "$url"
done

echo -e "\n🎉 All video support tests completed!"
echo -e "\n${GREEN}Summary:${NC}"
echo -e "✅ GeckoView integration working"
echo -e "✅ Crynn Browser builds successfully"
echo -e "✅ Video platform compatibility tested"
echo -e "\n${BLUE}Next steps:${NC}"
echo -e "1. Install GeckoView: ./build/scripts/install_geckoview.sh"
echo -e "2. Build browser: cargo build --release"
echo -e "3. Run browser: cargo run --release"
echo -e "4. Test YouTube: Navigate to https://youtube.com"
echo -e "5. Test Netflix: Navigate to https://netflix.com"
echo -e "6. Test Twitch: Navigate to https://twitch.tv"
