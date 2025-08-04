#!/bin/bash
# 开发环境启动脚本

set -e

echo "🔧 Starting RustImage Development Environment"

GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m'

# 1. 构建 WASM 模块（开发模式）
echo -e "${BLUE}🕸️  Building WASM module for development...${NC}"
cd rustimage-wasm

if ! command -v wasm-pack &> /dev/null; then
    echo -e "${YELLOW}Installing wasm-pack...${NC}"
    curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
fi

# 开发模式构建（包含调试信息）
wasm-pack build --target web --out-dir ../frontend/public/wasm --dev

cd ../frontend

# 2. 启动前端开发服务器
echo -e "${BLUE}⚛️  Starting React development server...${NC}"
npm run dev &

# 3. 等待用户输入以停止服务
echo -e "${GREEN}✅ Development servers started!${NC}"
echo -e "${YELLOW}Frontend: http://localhost:5173${NC}"
echo -e "${YELLOW}Press Ctrl+C to stop all servers${NC}"

# 捕获 Ctrl+C 信号并清理进程
trap 'echo -e "\n${YELLOW}Stopping development servers...${NC}"; kill $(jobs -p); exit' INT

# 保持脚本运行
wait