//! 类型定义 - 零成本抽象和类型安全设计

use serde::{Deserialize, Serialize};

/// 滤镜类型枚举 - 对用户暴露的简单接口
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FilterType {
    /// 高斯模糊 - 展示 SIMD 优化
    GaussianBlur,
    /// 边缘检测 - 展示并行计算
    EdgeDetection,
    /// 锐化滤镜
    Sharpen,
    /// 色彩调整
    ColorAdjust,
    /// 噪音降低
    NoiseReduction,
    /// 超分辨率 - 展示算法复杂性
    SuperResolution,
}

/// 滤镜参数 - 类型安全的参数传递
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilterParams {
    /// 强度参数 [0.0, 1.0]
    pub intensity: f32,
    /// 半径参数（适用于模糊等滤镜）
    pub radius: Option<f32>,
    /// 色彩调整参数
    pub color_params: Option<ColorParams>,
    /// 自定义参数映射
    pub custom: std::collections::HashMap<String, f32>,
}

/// 色彩调整参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorParams {
    pub brightness: f32,
    pub contrast: f32,
    pub saturation: f32,
    pub hue: f32,
}

/// 滤镜操作 - 批处理时使用
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilterOperation {
    pub filter_type: FilterType,
    pub params: Option<FilterParams>,
}

/// 处理后的图像结果
#[derive(Debug, Clone)]
pub struct ProcessedImage {
    /// 图像数据
    pub data: Vec<u8>,
    /// 图像尺寸
    pub dimensions: ImageDimensions,
    /// 图像格式
    pub format: ImageFormat,
    /// 处理耗时（毫秒）
    pub processing_time_ms: f64,
    /// 内存使用量（字节）
    pub memory_usage: u64,
}

/// 图像尺寸
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ImageDimensions {
    pub width: u32,
    pub height: u32,
}

/// 支持的图像格式
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ImageFormat {
    Png,
    Jpeg,
    WebP,
    Bmp,
}

/// 性能指标 - 用于展示 Rust 优势
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    /// 总处理时间（毫秒）
    pub total_time_ms: f64,
    /// 内存峰值使用量（字节）
    pub peak_memory_bytes: u64,
    /// CPU 使用率 [0.0, 1.0]
    pub cpu_usage: f32,
    /// 处理的像素数量
    pub pixels_processed: u64,
    /// 每秒处理像素数
    pub pixels_per_second: f64,
    /// 线程使用情况
    pub thread_info: ThreadMetrics,
}

/// 线程使用指标
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreadMetrics {
    /// 使用的线程数量
    pub threads_used: usize,
    /// 并行效率 [0.0, 1.0]
    pub parallel_efficiency: f32,
    /// SIMD 指令使用情况
    pub simd_utilized: bool,
}

/// 像素类型 - 零成本抽象
pub trait Pixel: Copy + Clone + Send + Sync + 'static {
    type Subpixel: Copy + Clone + Send + Sync + 'static;
    
    /// 获取像素通道数
    const CHANNEL_COUNT: u8;
    
    /// 从子像素数组创建像素
    fn from_channels(channels: &[Self::Subpixel]) -> Self;
    
    /// 转换为子像素数组
    fn to_channels(&self) -> Vec<Self::Subpixel>;
}

/// RGB 像素类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rgb<T> {
    pub data: [T; 3],
}

/// RGBA 像素类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rgba<T> {
    pub data: [T; 4],
}

/// 灰度像素类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Luma<T> {
    pub data: [T; 1],
}

// 实现默认值
impl Default for FilterParams {
    fn default() -> Self {
        Self {
            intensity: 0.5,
            radius: None,
            color_params: None,
            custom: std::collections::HashMap::new(),
        }
    }
}

impl Default for ColorParams {
    fn default() -> Self {
        Self {
            brightness: 0.0,
            contrast: 1.0,
            saturation: 1.0,
            hue: 0.0,
        }
    }
}