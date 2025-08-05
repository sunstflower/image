//! 类型定义 - 零成本抽象和类型安全设计

use serde::{Deserialize, Serialize};

/// 支持的图像格式 - 对用户暴露的简单接口
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ImageFormat {
    /// JPEG格式 - 有损压缩，适合照片
    Jpeg,
    /// PNG格式 - 无损压缩，支持透明度
    Png,
    /// WebP格式 - 现代格式，高压缩比
    WebP,
    /// AVIF格式 - 新一代格式，最高压缩比
    Avif,
    /// BMP格式 - 无压缩，兼容性好
    Bmp,
    /// TIFF格式 - 专业格式，支持多层
    Tiff,
    /// GIF格式 - 支持动画
    Gif,
    /// ICO格式 - 图标格式
    Ico,
}

/// 转换选项 - 类型安全的参数传递
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversionOptions {
    /// 质量参数 [0.0, 1.0] (适用于有损格式)
    pub quality: Option<f32>,
    /// 压缩级别 [0, 9] (适用于无损格式)
    pub compression_level: Option<u8>,
    /// 是否启用渐进式编码
    pub progressive: Option<bool>,
    /// 是否保持原图尺寸
    pub preserve_dimensions: bool,
    /// 是否保持色彩空间
    pub preserve_color_space: bool,
    /// 是否保持元数据
    pub preserve_metadata: bool,
    /// 自定义参数映射
    pub custom: std::collections::HashMap<String, String>,
}

/// 转换任务
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversionTask {
    pub from_format: ImageFormat,
    pub to_format: ImageFormat,
    pub options: Option<ConversionOptions>,
}

/// 图像输入信息
#[derive(Debug, Clone)]
pub struct ImageInput {
    /// 图像数据
    pub data: Vec<u8>,
    /// 原始格式
    pub format: ImageFormat,
    /// 文件名（可选）
    pub filename: Option<String>,
}

/// 转换后的图像结果
#[derive(Debug, Clone)]
pub struct ConvertedImage {
    /// 转换后的图像数据
    pub data: Vec<u8>,
    /// 图像尺寸
    pub dimensions: ImageDimensions,
    /// 目标格式
    pub format: ImageFormat,
    /// 转换耗时（毫秒）
    pub conversion_time_ms: f64,
    /// 原始大小（字节）
    pub original_size: u64,
    /// 转换后大小（字节）
    pub converted_size: u64,
    /// 压缩比
    pub compression_ratio: f32,
    /// 转换质量评估
    pub quality_metrics: Option<QualityMetrics>,
}

/// 图像尺寸
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ImageDimensions {
    pub width: u32,
    pub height: u32,
}

/// 格式信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormatInfo {
    /// 格式名称
    pub name: String,
    /// 格式描述
    pub description: String,
    /// 文件扩展名
    pub extensions: Vec<String>,
    /// MIME 类型
    pub mime_type: String,
    /// 是否支持透明度
    pub supports_transparency: bool,
    /// 是否支持动画
    pub supports_animation: bool,
    /// 是否有损压缩
    pub is_lossy: bool,
    /// 最大支持尺寸
    pub max_dimensions: Option<ImageDimensions>,
    /// 支持的色彩深度
    pub color_depths: Vec<u8>,
}

/// 质量评估指标
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityMetrics {
    /// 峰值信噪比 (PSNR)
    pub psnr: f32,
    /// 结构相似性指数 (SSIM)
    pub ssim: f32,
    /// 感知哈希相似度
    pub perceptual_similarity: f32,
}

/// 性能指标 - 用于展示 Rust 优势
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    /// 总转换时间（毫秒）
    pub total_time_ms: f64,
    /// 内存峰值使用量（字节）
    pub peak_memory_bytes: u64,
    /// CPU 使用率 [0.0, 1.0]
    pub cpu_usage: f32,
    /// 处理的图像数量
    pub images_processed: u64,
    /// 每秒处理图像数
    pub images_per_second: f64,
    /// 总数据量（字节）
    pub total_data_bytes: u64,
    /// 数据处理速度（MB/s）
    pub throughput_mbps: f64,
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

// 格式特性 trait
pub trait ImageFormatTrait {
    /// 获取格式信息
    fn info() -> FormatInfo;
    
    /// 检测是否为该格式
    fn detect(data: &[u8]) -> bool;
    
    /// 获取默认选项
    fn default_options() -> ConversionOptions;
}

// 实现默认值
impl Default for ConversionOptions {
    fn default() -> Self {
        Self {
            quality: Some(0.8),
            compression_level: Some(6),
            progressive: Some(false),
            preserve_dimensions: true,
            preserve_color_space: true,
            preserve_metadata: false,
            custom: std::collections::HashMap::new(),
        }
    }
}

impl ImageFormat {
    /// 获取格式的文件扩展名
    pub fn extension(&self) -> &'static str {
        match self {
            ImageFormat::Jpeg => "jpg",
            ImageFormat::Png => "png",
            ImageFormat::WebP => "webp",
            ImageFormat::Avif => "avif",
            ImageFormat::Bmp => "bmp",
            ImageFormat::Tiff => "tiff",
            ImageFormat::Gif => "gif",
            ImageFormat::Ico => "ico",
        }
    }
    
    /// 获取 MIME 类型
    pub fn mime_type(&self) -> &'static str {
        match self {
            ImageFormat::Jpeg => "image/jpeg",
            ImageFormat::Png => "image/png",
            ImageFormat::WebP => "image/webp",
            ImageFormat::Avif => "image/avif",
            ImageFormat::Bmp => "image/bmp",
            ImageFormat::Tiff => "image/tiff",
            ImageFormat::Gif => "image/gif",
            ImageFormat::Ico => "image/x-icon",
        }
    }
    
    /// 是否为有损格式
    pub fn is_lossy(&self) -> bool {
        matches!(self, ImageFormat::Jpeg | ImageFormat::WebP | ImageFormat::Avif)
    }
    
    /// 是否支持透明度
    pub fn supports_transparency(&self) -> bool {
        matches!(self, ImageFormat::Png | ImageFormat::WebP | ImageFormat::Avif | ImageFormat::Gif | ImageFormat::Ico)
    }
    
    /// 是否支持动画
    pub fn supports_animation(&self) -> bool {
        matches!(self, ImageFormat::Gif | ImageFormat::WebP | ImageFormat::Avif)
    }
}