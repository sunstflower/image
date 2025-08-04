# RustImage - 高性能图像处理工具

> 基于《软件设计哲学》理念的深模块设计，展示 Rust + WebAssembly 的性能优势

## 🎯 项目概述

RustImage 是一个现代化的图像处理工具，完美融合了《软件设计哲学》的核心理念：

- **深模块设计**: 简化对外接口，隐藏内部复杂性
- **信息隐藏**: 封装复杂的图像处理算法
- **零成本抽象**: 充分利用 Rust 的性能优势

## 🏗️ 架构设计

### 深模块体现

```
用户接口 (简单)
├── process_image(image, filter_type, params?) -> Result
├── batch_process(images[], operations[]) -> Result[]  
└── get_performance_metrics() -> PerformanceStats

内部实现 (复杂但隐藏)
├── 图像解码/编码引擎
├── SIMD 优化的滤镜算法
├── 多线程并行处理
├── 内存池管理
└── 性能监控系统
```

### 技术栈

**后端 (Rust + WebAssembly)**
- `rustimage-core`: 核心图像处理库
- `rustimage-wasm`: WebAssembly 绑定
- 零成本抽象和编译时优化

**前端 (React + TypeScript)**
- React 18 + Vite 快速开发
- TypeScript 类型安全
- Zustand 状态管理
- Tailwind CSS 样式系统

## 🚀 快速开始

### 环境要求

- Rust 1.70+
- Node.js 18+
- wasm-pack (自动安装)

### 开发环境

```bash
# 启动开发环境
./dev.sh

# 前端开发服务器: http://localhost:5173
```

### 生产构建

```bash
# 完整构建
./build.sh

# 包含性能基准测试
./build.sh --benchmark
```

## 📁 项目结构

```
rustimge/
├── rustimage-core/          # Rust 核心库
│   ├── src/
│   │   ├── lib.rs           # 主要对外接口
│   │   ├── types.rs         # 类型定义
│   │   ├── processor.rs     # 图像处理器
│   │   ├── filters.rs       # 滤镜引擎
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
│   │   ├── stores/          # 状态管理
│   │   └── utils/           # 工具函数
│   ├── vite.config.ts
│   └── package.json
├── build.sh                 # 构建脚本
├── dev.sh                   # 开发脚本
└── Cargo.toml              # Workspace 配置
```

## 🎨 核心功能

### 1. 深模块接口

```rust
// 极简的对外接口
pub fn process_image(
    image_data: &[u8],
    filter_type: FilterType,
    params: Option<FilterParams>,
) -> Result<ProcessedImage>
```

### 2. 支持的滤镜

- **高斯模糊**: SIMD 优化展示
- **边缘检测**: 并行计算展示  
- **锐化滤镜**: 卷积算法优化
- **色彩调整**: 查找表优化
- **噪音降低**: 算法复杂性展示
- **超分辨率**: 神经网络算法

### 3. 性能优势展示

- **实时对比**: Rust+WASM vs JavaScript
- **性能监控**: CPU、内存、处理速度
- **基准测试**: 自动化性能评估
- **可视化图表**: 实时性能曲线

## 💡 设计理念体现

### 深模块设计

- **对外**: 只需调用 `process_image()`
- **对内**: 复杂的算法实现完全隐藏
- **好处**: 用户无需了解内部实现细节

### 信息隐藏

- **接口层**: 简单的枚举和结构体
- **实现层**: SIMD、多线程、内存管理
- **好处**: 降低使用复杂度，提高可维护性

### 零成本抽象

```rust
// 编译时特化，运行时零开销
trait ImageFilter<T: Pixel> {
    fn apply(&self, image: &mut ImageBuffer<T>) -> Result<()>;
}

// 自动向量化
#[inline]
fn process_pixels<F>(pixels: &mut [u8], func: F) 
where F: Fn(u8) -> u8 
```

## 📊 性能指标

预期性能提升：
- **处理速度**: 2-5x 比 JavaScript 快
- **内存效率**: 30-50% 内存减少
- **稳定性**: 零 GC 暂停
- **并行化**: 充分利用多核 CPU

## 🔧 开发指南

### 添加新滤镜

1. 在 `rustimage-core/src/filters.rs` 定义滤镜结构
2. 实现 `Filter` trait
3. 在 `FilterEngine` 中注册
4. 在前端添加对应的 UI 控件

### 性能优化

1. 使用 SIMD 指令加速
2. 实现并行处理算法
3. 优化内存访问模式
4. 添加性能监控点

## 📈 后续计划

1. **更多滤镜算法**: 添加更多专业滤镜
2. **批处理优化**: 提高批量处理效率
3. **GPU 加速**: 集成 WebGPU 支持
4. **移动端适配**: PWA 和响应式优化

---

这个项目完美展示了《软件设计哲学》在现代 Web 开发中的应用，通过深模块设计实现了简单易用的接口和强大的功能。