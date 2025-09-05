#!/bin/bash

# CURSED Web Dashboard Build Script
# Builds all components of the web dashboard application

set -e

echo "🔥 Building CURSED Web Dashboard Application"
echo "============================================"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Build directory
BUILD_DIR="build"
mkdir -p "$BUILD_DIR"

echo -e "${BLUE}Step 1: Building CURSED compiler...${NC}"
if zig build; then
    echo -e "${GREEN}✅ Compiler build successful${NC}"
else
    echo -e "${RED}❌ Compiler build failed${NC}"
    exit 1
fi

echo -e "\n${BLUE}Step 2: Building backend server...${NC}"
if ./zig-out/bin/cursed-zig --compile examples/web-dashboard/backend/server.💀 -o "$BUILD_DIR/dashboard-server"; then
    echo -e "${GREEN}✅ Backend server compiled successfully${NC}"
else
    echo -e "${YELLOW}⚠️  Backend server compilation failed, using interpreter mode${NC}"
    echo "#!/bin/bash" > "$BUILD_DIR/dashboard-server"
    echo "exec $(pwd)/zig-out/bin/cursed-zig examples/web-dashboard/backend/server.💀" >> "$BUILD_DIR/dashboard-server"
    chmod +x "$BUILD_DIR/dashboard-server"
fi

echo -e "\n${BLUE}Step 3: Building CLI administration tool...${NC}"
if ./zig-out/bin/cursed-zig --compile examples/web-dashboard/cli/admin.💀 -o "$BUILD_DIR/dashboard-admin"; then
    echo -e "${GREEN}✅ CLI tool compiled successfully${NC}"
else
    echo -e "${YELLOW}⚠️  CLI tool compilation failed, using interpreter mode${NC}"
    echo "#!/bin/bash" > "$BUILD_DIR/dashboard-admin"
    echo "exec $(pwd)/zig-out/bin/cursed-zig examples/web-dashboard/cli/admin.💀" >> "$BUILD_DIR/dashboard-admin"
    chmod +x "$BUILD_DIR/dashboard-admin"
fi

echo -e "\n${BLUE}Step 4: Building frontend WebAssembly module...${NC}"
if ./zig-out/bin/cursed-zig --compile --target=wasm32-freestanding examples/web-dashboard/frontend/app.💀 -o "$BUILD_DIR/dashboard-frontend.wasm"; then
    echo -e "${GREEN}✅ Frontend WASM compiled successfully${NC}"
else
    echo -e "${YELLOW}⚠️  Frontend WASM compilation not available yet${NC}"
    echo -e "${YELLOW}   Using JavaScript simulation in HTML file${NC}"
fi

echo -e "\n${BLUE}Step 5: Copying static assets...${NC}"
cp examples/web-dashboard/frontend/index.html "$BUILD_DIR/"
cp examples/web-dashboard/config/server.json "$BUILD_DIR/"
mkdir -p "$BUILD_DIR/data"

echo -e "${GREEN}✅ Static assets copied${NC}"

echo -e "\n${BLUE}Step 6: Testing builds...${NC}"

# Test backend (just validate syntax)
echo -e "Testing backend server..."
if ./zig-out/bin/cursed-zig examples/web-dashboard/backend/server.💀 --check; then
    echo -e "${GREEN}✅ Backend server syntax valid${NC}"
else
    echo -e "${YELLOW}⚠️  Backend server syntax check failed${NC}"
fi

# Test CLI tool
echo -e "Testing CLI administration tool..."
if ./zig-out/bin/cursed-zig examples/web-dashboard/cli/admin.💀 --check; then
    echo -e "${GREEN}✅ CLI tool syntax valid${NC}"
else
    echo -e "${YELLOW}⚠️  CLI tool syntax check failed${NC}"
fi

# Test frontend
echo -e "Testing frontend application..."
if ./zig-out/bin/cursed-zig examples/web-dashboard/frontend/app.💀 --check; then
    echo -e "${GREEN}✅ Frontend application syntax valid${NC}"
else
    echo -e "${YELLOW}⚠️  Frontend application syntax check failed${NC}"
fi

echo -e "\n${BLUE}Step 7: Creating deployment package...${NC}"
tar -czf "$BUILD_DIR/cursed-web-dashboard.tar.gz" -C "$BUILD_DIR" .
echo -e "${GREEN}✅ Deployment package created: $BUILD_DIR/cursed-web-dashboard.tar.gz${NC}"

echo -e "\n${GREEN}🎉 Build Complete!${NC}"
echo "========================================"
echo -e "Built components:"
echo -e "  📦 Backend Server: ${BLUE}$BUILD_DIR/dashboard-server${NC}"
echo -e "  🔧 CLI Admin Tool: ${BLUE}$BUILD_DIR/dashboard-admin${NC}"
echo -e "  🌐 Frontend HTML: ${BLUE}$BUILD_DIR/index.html${NC}"
echo -e "  ⚙️  Configuration: ${BLUE}$BUILD_DIR/server.json${NC}"
echo -e "  📦 Full Package: ${BLUE}$BUILD_DIR/cursed-web-dashboard.tar.gz${NC}"

echo -e "\n${BLUE}Usage:${NC}"
echo -e "  Start server: ${YELLOW}$BUILD_DIR/dashboard-server${NC}"
echo -e "  CLI admin: ${YELLOW}$BUILD_DIR/dashboard-admin help${NC}"
echo -e "  Open frontend: ${YELLOW}open $BUILD_DIR/index.html${NC}"

echo -e "\n${BLUE}Quick Start:${NC}"
echo -e "  cd $BUILD_DIR"
echo -e "  ./dashboard-admin create-user admin admin@example.com admin123 --admin"
echo -e "  ./dashboard-server &"
echo -e "  open index.html"
