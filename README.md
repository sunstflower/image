# RustImage - 高性能图像格式转换工具

> 基于《软件设计哲学》理念的深模块设计，展示 Rust + WebAssembly 的性能优势

## 🎯 项目概述

RustImage 是一个专注于图像格式转换的工具，完美融合了《软件设计哲学》的核心理念：

- **深模块设计**: 简化对外接口 `convert_format()`，隐藏内部复杂性
- **信息隐藏**: 封装复杂的编解码算法
- **零成本抽象**: 充分利用 Rust 的性能优势

## 🏗️ 架构设计

### 深模块体现

```
用户接口 (简单)
├── convert_format(image, from, to, options?) -> Result
├── batch_convert(images[], tasks[]) -> Result[]  
├── detect_format(image) -> Format
└── get_performance_metrics() -> Stats

内部实现 (复杂但隐藏)
├── JPEG/PNG/WebP/AVIF 编解码器
├── 格式检测算法
├── 质量评估系统
├── 多线程并行处理
└── 性能监控系统
```

### 支持的格式

- **JPEG** - 有损压缩，适合照片
- **PNG** - 无损压缩，支持透明度  
- **WebP** - 现代格式，高压缩比
- **AVIF** - 新一代格式，最高压缩比
- **BMP** - 无压缩，兼容性好
- **TIFF** - 专业格式，支持多层
- **GIF** - 支持动画
- **ICO** - 图标格式

### 技术栈

**后端 (Rust + WebAssembly)**
- `rustimage-core`: 核心格式转换库
- `rustimage-wasm`: WebAssembly 绑定
- 零成本抽象和编译时优化

**前端 (React + TypeScript)**
- React 18 + Vite 快速开发
- TypeScript 类型安全
- Zustand 状态管理

## 🚀 开发环境设置

### 环境要求

- Rust 1.70+
- Node.js 18+
- wasm-pack (需要手动安装)

### 安装 wasm-pack

```bash
# 安装 wasm-pack
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

# 或者使用 cargo 安装
cargo install wasm-pack
```

## 🔧 手动构建步骤

### 开发环境启动

#### 1. 构建 Rust 核心库

```bash
# 进入核心库目录
cd rustimage-core

# 开发构建（包含调试信息）
cargo build

# 或者发布构建（优化版本）
cargo build --release

# 返回根目录
cd ..
```

#### 2. 构建 WebAssembly 模块

```bash
# 进入 WASM 模块目录
cd rustimage-wasm

# 开发模式构建（包含调试信息，构建快）
wasm-pack build --target web --out-dir ../frontend/public/wasm --dev

# 或者发布模式（优化版本，体积小）
wasm-pack build --target web --out-dir ../frontend/public/wasm --release

# 返回根目录
cd ..
```

#### 3. 启动前端开发服务器

```bash
# 进入前端目录
cd frontend

# 安装依赖（首次运行）
npm install

# 启动开发服务器
npm run dev

# 浏览器访问: http://localhost:5173
```

### 生产环境构建

#### 1. 构建优化版本的所有组件

```bash
# 1. 构建 Rust 核心库（发布版本）
cd rustimage-core
cargo build --release
cd ..

# 2. 构建 WASM 模块（发布版本）
cd rustimage-wasm
wasm-pack build --target web --out-dir ../frontend/public/wasm --release
cd ..

# 3. 构建前端（生产版本）
cd frontend
npm install  # 确保依赖已安装
npm run build
cd ..
```

#### 2. 输出文件位置

- **前端构建输出**: `frontend/dist/`
- **WASM模块输出**: `frontend/public/wasm/`

## 🧪 测试和验证

### 运行测试

```bash
# 测试 Rust 核心库
cd rustimage-core
cargo test
cd ..

# 测试 WASM 绑定
cd rustimage-wasm  
cargo test
cd ..

# 测试前端
cd frontend
npm test
cd ..
```

### 性能基准测试

```bash
# 运行 Rust 性能基准
cd rustimage-core
cargo bench
cd ..
```

### 验证构建结果

```bash
# 检查 WASM 文件是否生成
ls -la frontend/public/wasm/

# 应该看到：
# - rustimage_wasm.js
# - rustimage_wasm_bg.wasm
# - rustimage_wasm.d.ts
```

## 📁 项目结构

```
rustimge/
├── rustimage-core/          # Rust 核心库
│   ├── src/
│   │   ├── lib.rs           # 主要对外接口
│   │   ├── types.rs         # 类型定义
│   │   ├── converter.rs     # 格式转换器
│   │   ├── codecs.rs        # 编解码引擎
│   │   ├── performance.rs   # 性能监控
│   │   └── error.rs         # 错误处理
│   └── Cargo.toml
├── rustimage-wasm/          # WebAssembly 绑定
│   ├── src/lib.rs
│   └── Cargo.toml
├── frontend/                # React 前端
│   ├── src/
│   │   ├── types/           # TypeScript 类型
│   │   ├── hooks/           # React Hooks
│   │   ├── components/      # React 组件
│   │   └── stores/          # 状态管理
│   ├── public/wasm/         # WASM 模块输出
│   ├── vite.config.ts
│   └── package.json
└── Cargo.toml              # Workspace 配置
```

## 🎨 核心功能

### 深模块接口

```rust
// 极简的对外接口
pub fn convert_format(
    image_data: &[u8],
    from_format: ImageFormat,
    to_format: ImageFormat,
    options: Option<ConversionOptions>,
) -> Result<ConvertedImage>
```

### 支持的转换

- **有损 ↔ 无损**: JPEG ↔ PNG
- **现代格式**: WebP, AVIF
- **批量转换**: 多文件并行处理
- **质量控制**: 压缩比、渐进式编码
- **格式检测**: 自动识别图像格式

## 💡 设计理念体现

### 深模块设计

- **对外**: 只需调用 `convert_format()`
- **对内**: 复杂的编解码算法完全隐藏
- **好处**: 用户无需了解各种格式的实现细节

### 信息隐藏

- **接口层**: 简单的枚举和结构体
- **实现层**: JPEG DCT、PNG滤波、WebP预测等算法
- **好处**: 降低使用复杂度，提高可维护性

### 零成本抽象

```rust
// 编译时特化，运行时零开销
trait Codec<P: Pixel> {
    fn encode(&self, buffer: &ImageBuffer<P>) -> Result<Vec<u8>>;
}

// 自动优化的格式检测
fn detect_format(data: &[u8]) -> Result<ImageFormat>
```

## 📊 性能指标

预期性能提升：
- **转换速度**: 2-5x 比 JavaScript 快
- **内存效率**: 30-50% 内存减少
- **批处理**: 充分利用多核并行
- **压缩比**: 现代格式显著减小文件体积

## 🔧 常见问题

### WASM 构建失败
```bash
# 确保 wasm-pack 已安装
wasm-pack --version

# 如果未安装
cargo install wasm-pack
```

### 前端无法加载 WASM
```bash
# 确保 WASM 文件已生成
ls frontend/public/wasm/

# 重新构建 WASM
cd rustimage-wasm
wasm-pack build --target web --out-dir ../frontend/public/wasm --dev
```

### 依赖冲突
```bash
# 清理并重新安装
cd frontend
rm -rf node_modules package-lock.json
npm install
```

## 📈 后续计划

1. **更多格式支持**: HEIC, JXL等
2. **质量评估**: PSNR, SSIM指标
3. **批处理优化**: 更高效的并行策略
4. **Web Workers**: 前端多线程处理

---

这个项目完美展示了《软件设计哲学》在现代 Web 开发中的应用，通过深模块设计实现了简单易用的接口和强大的格式转换功能。