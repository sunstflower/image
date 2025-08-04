#!/bin/bash
# 构建脚本 - 整合 Rust WASM 和 React 前端构建

set -e

echo "🚀 Building RustImage - High-Performance Image Processing Tool"

# 颜色定义
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

# 1. 构建 Rust 核心库
echo -e "${BLUE}📦 Building Rust core library...${NC}"
cd rustimage-core
cargo build --release
cd ..

# 2. 构建 WebAssembly 模块
echo -e "${BLUE}🕸️  Building WebAssembly module...${NC}"
cd rustimage-wasm

# 检查 wasm-pack 是否安装
if ! command -v wasm-pack &> /dev/null; then
    echo -e "${RED}❌ wasm-pack not found. Installing...${NC}"
    curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
fi

# 构建 WASM 包
wasm-pack build --target web --out-dir ../frontend/public/wasm --release

echo -e "${GREEN}✅ WASM module built successfully${NC}"
cd ..

# 3. 构建前端应用
echo -e "${BLUE}⚛️  Building React frontend...${NC}"
cd frontend

# 安装依赖（如果需要）
if [ ! -d "node_modules" ]; then
    echo -e "${YELLOW}📦 Installing frontend dependencies...${NC}"
    npm install
fi

# 构建生产版本
npm run build

echo -e "${GREEN}✅ Frontend built successfully${NC}"
cd ..

# 4. 运行基准测试（可选）
if [ "$1" = "--benchmark" ]; then
    echo -e "${BLUE}⚡ Running performance benchmarks...${NC}"
    cd rustimage-core
    cargo bench
    cd ..
fi

echo -e "${GREEN}🎉 Build completed successfully!${NC}"
echo -e "${YELLOW}📁 Frontend build output: frontend/dist${NC}"
echo -e "${YELLOW}📁 WASM module output: frontend/public/wasm${NC}"