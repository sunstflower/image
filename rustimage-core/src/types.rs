//! 类型定义 - 零成本抽象和类型安全设计
//! 
//! 本模块遵循《软件设计哲学》的核心理念：
//! - 深模块设计：简单的类型接口，隐藏复杂的实现细节
//! - 信息隐藏：只暴露必要的公共接口，内部实现细节保持私有
//! - 零成本抽象：利用Rust的类型系统在编译时进行优化

use serde::{Deserialize, Serialize};
use std::collections::HashMap;  
use std::fmt; // 格式化输出

// =============================================================================
// 公共API类型 - 对外暴露的简单接口
// =============================================================================

/// 支持的图像格式
/// 
/// 这是一个"深"的枚举：接口简单（只是格式名），但内部隐藏了
/// 复杂的编解码实现、格式特性、兼容性处理等
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]  // 实现序列化/反序列化
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

/// 转换选项构建器 - 使用构建器模式简化复杂配置
/// 
/// 遵循设计哲学：提供简单的链式API，隐藏复杂的参数验证和默认值逻辑
#[derive(Debug, Clone)]
pub struct ConversionOptionsBuilder {
    options: ConversionOptions, // 私有：防止直接修改
}

/// 转换选项 - 内部结构，通过Builder暴露
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversionOptions {
    // 私有字段，只能通过Builder或方法访问
    /// quality: 质量参数 [0.0, 1.0] (适用于有损格式)
    quality: Option<f32>,
    /// compression_level: 压缩级别 [0, 9] (适用于无损格式)
    compression_level: Option<u8>, 
    /// progressive: 是否启用渐进式编码
    progressive: Option<bool>,
    /// preserve_dimensions: 是否保持原图尺寸
    preserve_dimensions: bool,
    /// preserve_color_space: 是否保持色彩空间
    preserve_color_space: bool,
    /// preserve_metadata: 是否保持元数据
    preserve_metadata: bool,
    /// custom: 自定义参数映射
    custom: HashMap<String, String>,
}

/// 转换任务 - 简单的值对象
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversionTask {
    /// from_format: 源格式
    pub from_format: ImageFormat,
    /// to_format: 目标格式
    pub to_format: ImageFormat,
    /// options: 转换选项
    pub options: Option<ConversionOptions>,
}

/// 图像输入 - 封装输入数据和元信息
#[derive(Debug, Clone)]
pub struct ImageInput {
    /// data: 图像数据
    data: Vec<u8>,                    
    /// format: 图像格式
    format: ImageFormat,              
    /// filename: 文件名（可选）
    filename: Option<String>,         
}

/// 转换结果 - 不可变的结果对象
#[derive(Debug, Clone)]
pub struct ConvertedImage {
    // 结果数据 - 私有，通过getter访问
    /// data: 转换后的图像数据
    data: Vec<u8>,
    /// dimensions: 图像尺寸
    dimensions: ImageDimensions,
    /// format: 目标格式
    format: ImageFormat,
    
    // 性能指标 - 私有，通过getter访问
    /// conversion_time_ms: 转换耗时（毫秒）
    conversion_time_ms: f64,
    /// original_size: 原始大小（字节）
    original_size: u64,
    /// converted_size: 转换后大小（字节）
    converted_size: u64,
    /// compression_ratio: 压缩比
    compression_ratio: f32,
    
    /// quality_metrics: 质量评估指标（可选）
    quality_metrics: Option<QualityMetrics>,
}

/// 图像尺寸 - 简单的值类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ImageDimensions {
    pub width: u32,
    pub height: u32,
}

/// 质量评估指标 - 只读数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityMetrics {
    psnr: f32,                        // 峰值信噪比
    ssim: f32,                        // 结构相似性指数  
    perceptual_similarity: f32,       // 感知哈希相似度
}

// =============================================================================
// 格式特性系统 - 零成本抽象的体现
// =============================================================================

/// 格式特性 - 编译时已知的常量信息
pub struct FormatInfo {
    /// name: 格式名称
    pub name: &'static str,
    /// description: 格式描述
    pub description: &'static str,
    /// extensions: 文件扩展名
    pub extensions: &'static [&'static str],
    /// mime_type: MIME类型
    pub mime_type: &'static str,
    /// capabilities: 格式能力
    pub capabilities: FormatCapabilities,
    /// limits: 格式限制
    pub limits: FormatLimits,
}

/// 格式能力 - 使用位标志进行零成本抽象
#[derive(Debug, Clone, Copy)]
pub struct FormatCapabilities {
    /// flags: 格式标志
    flags: u8,
}

/// 格式限制 - 编译时常量
#[derive(Debug, Clone, Copy)]
pub struct FormatLimits {
    /// max_width: 最大宽度
    pub max_width: Option<u32>,
    /// max_height: 最大高度
    pub max_height: Option<u32>,
    /// max_file_size: 最大文件大小
    pub max_file_size: Option<u64>,
    /// supported_bit_depths: 支持的位深度
    pub supported_bit_depths: &'static [u8],
}



/// 性能指标 - 对外的简单接口
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    /// timing: 时间相关指标
    pub timing: TimingMetrics,
    /// memory: 内存相关指标
    pub memory: MemoryMetrics, 
    /// throughput: 吞吐量指标
    pub throughput: ThroughputMetrics,
    /// system: 系统指标
    pub system: SystemMetrics,
}

/// 时间相关指标
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimingMetrics {
    /// total_time_ms: 总时间（毫秒）
    pub total_time_ms: f64,
    /// decode_time_ms: 解码时间（毫秒）
    pub decode_time_ms: f64,
    /// encode_time_ms: 编码时间（毫秒）
    pub encode_time_ms: f64,
    /// processing_time_ms: 处理时间（毫秒）
    pub processing_time_ms: f64,
}

/// 内存相关指标
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryMetrics {
    /// peak_memory_bytes: 峰值内存使用量（字节）
    pub peak_memory_bytes: u64,
    /// allocations_count: 分配次数
    pub allocations_count: u32,
    /// deallocations_count: 释放次数
    pub deallocations_count: u32,
}

/// 吞吐量指标
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThroughputMetrics {
    /// images_per_second: 每秒处理的图像数量
    pub images_per_second: f64,
    /// bytes_per_second: 每秒处理的字节数
    pub bytes_per_second: f64,
    /// pixels_per_second: 每秒处理的像素数量
    pub pixels_per_second: f64,
}

/// 系统指标
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemMetrics {
    /// cpu_usage_percent: CPU使用率（百分比）
    pub cpu_usage_percent: f32,
    /// threads_used: 使用的线程数量
    pub threads_used: usize,
    /// parallel_efficiency: 并行效率（百分比）
    pub parallel_efficiency: f32,
    /// simd_utilized: 是否使用SIMD指令
    pub simd_utilized: bool,
}

/// 像素特性 - 编译时多态
pub trait Pixel: Copy + Clone + Send + Sync + 'static {
    /// Subpixel: 子像素类型
    type Subpixel: Copy + Clone + Send + Sync + 'static;
    
    /// CHANNEL_COUNT: 通道数量
    const CHANNEL_COUNT: u8;
    /// HAS_ALPHA: 是否包含透明通道
    const HAS_ALPHA: bool;
    /// BITS_PER_CHANNEL: 每个通道的位数
    const BITS_PER_CHANNEL: u8;
    
    /// 从子像素数组创建像素 - 编译时内联
    fn from_channels(channels: &[Self::Subpixel]) -> Self;
    
    /// 转换为子像素数组 - 编译时内联
    fn to_channels(&self) -> [Self::Subpixel; Self::CHANNEL_COUNT as usize];
    
    /// 获取亮度值 - 编译时特化
    fn luminance(&self) -> Self::Subpixel;
}

/// RGB像素 - 零开销封装
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rgb<T> {
    /// r: 红色通道
    pub r: T,
    /// g: 绿色通道
    pub g: T, 
    /// b: 蓝色通道
    pub b: T,
}

/// RGBA像素 - 零开销封装
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rgba<T> {
    /// r: 红色通道
    pub r: T,
    /// g: 绿色通道
    pub g: T,
    /// b: 蓝色通道
    pub b: T,
    /// a: 透明通道
    pub a: T,
}

/// 灰度像素 - 零开销封装
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Luma<T> {
    pub l: T,
}

// =============================================================================
// 实现块 - 信息隐藏的体现
// =============================================================================

impl ConversionOptionsBuilder {
    /// 创建新的构建器 - 深模块的入口点
    pub fn new() -> Self {
        Self {
            options: ConversionOptions::default(),
        }
    }
    
    /// 设置质量参数 - 带验证的Builder方法
    pub fn quality(mut self, quality: f32) -> Self {
        self.options.quality = Some(quality.clamp(0.0, 1.0));
        self
    }
    
    /// 设置压缩级别 - 带验证的Builder方法
    pub fn compression_level(mut self, level: u8) -> Self {
        self.options.compression_level = Some(level.min(9));
        self
    }
    
    /// 设置渐进式编码
    pub fn progressive(mut self, progressive: bool) -> Self {
        self.options.progressive = Some(progressive);
        self
    }
    
    /// 是否保持尺寸
    pub fn preserve_dimensions(mut self, preserve: bool) -> Self {
        self.options.preserve_dimensions = preserve;
        self
    }
    
    /// 是否保持色彩空间
    pub fn preserve_color_space(mut self, preserve: bool) -> Self {
        self.options.preserve_color_space = preserve;
        self
    }
    
    /// 是否保持元数据
    pub fn preserve_metadata(mut self, preserve: bool) -> Self {
        self.options.preserve_metadata = preserve;
        self
    }
    
    /// 添加自定义参数
    pub fn custom_param<K: Into<String>, V: Into<String>>(mut self, key: K, value: V) -> Self {
        self.options.custom.insert(key.into(), value.into());
        self
    }
    
    /// 构建最终的选项对象
    pub fn build(self) -> ConversionOptions {
        self.options
    }
}

impl Default for ConversionOptionsBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl ConversionOptions {
    /// 获取质量参数 - 只读访问
    pub fn quality(&self) -> Option<f32> {
        self.quality
    }
    
    /// 获取压缩级别 - 只读访问
    pub fn compression_level(&self) -> Option<u8> {
        self.compression_level
    }
    
    /// 是否渐进式编码 - 只读访问
    pub fn is_progressive(&self) -> Option<bool> {
        self.progressive
    }
    
    /// 是否保持尺寸 - 只读访问
    pub fn preserves_dimensions(&self) -> bool {
        self.preserve_dimensions
    }
    
    /// 是否保持色彩空间 - 只读访问
    pub fn preserves_color_space(&self) -> bool {
        self.preserve_color_space
    }
    
    /// 是否保持元数据 - 只读访问
    pub fn preserves_metadata(&self) -> bool {
        self.preserve_metadata
    }
    
    /// 获取自定义参数 - 只读访问
    pub fn custom_param(&self, key: &str) -> Option<&str> {
        self.custom.get(key).map(|s| s.as_str())
    }
    
    /// 获取所有自定义参数的键 - 只读访问
    pub fn custom_param_keys(&self) -> impl Iterator<Item = &str> {
        self.custom.keys().map(|s| s.as_str())
    }
}

impl Default for ConversionOptions {
    fn default() -> Self {
        Self {
            quality: Some(0.8),
            compression_level: Some(6),
            progressive: Some(false),
            preserve_dimensions: true,
            preserve_color_space: true,
            preserve_metadata: false,
            custom: HashMap::new(),
        }
    }
}

impl ImageInput {
    /// 创建新的图像输入 - 工厂方法
    pub fn new(data: Vec<u8>, format: ImageFormat) -> Self {
        Self {
            data,
            format,
            filename: None,
        }
    }
    
    /// 带文件名的创建方法
    pub fn with_filename(data: Vec<u8>, format: ImageFormat, filename: String) -> Self {
        Self {
            data,
            format,
            filename: Some(filename),
        }
    }
    
    /// 获取数据 - 只读访问
    pub fn data(&self) -> &[u8] {
        &self.data
    }
    
    /// 获取格式 - 只读访问
    pub fn format(&self) -> ImageFormat {
        self.format
    }
    
    /// 获取文件名 - 只读访问
    pub fn filename(&self) -> Option<&str> {
        self.filename.as_deref()
    }
    
    /// 获取数据大小
    pub fn size(&self) -> usize {
        self.data.len()
    }
}

impl ConvertedImage {
    /// 创建转换结果 - 包内可见的构造器
    pub(crate) fn new(
        data: Vec<u8>,
        dimensions: ImageDimensions,
        format: ImageFormat,
        conversion_time_ms: f64,
        original_size: u64,
    ) -> Self {
        let converted_size = data.len() as u64;
        let compression_ratio = if original_size > 0 {
            converted_size as f32 / original_size as f32
        } else {
            1.0
        };
        
        Self {
            data,
            dimensions,
            format,
            conversion_time_ms,
            original_size,
            converted_size,
            compression_ratio,
            quality_metrics: None,
        }
    }
    
    /// 添加质量指标 - 包内可见
    pub(crate) fn with_quality_metrics(mut self, metrics: QualityMetrics) -> Self {
        self.quality_metrics = Some(metrics);
        self
    }
    
    // 只读访问器方法
    pub fn data(&self) -> &[u8] { &self.data }
    pub fn dimensions(&self) -> ImageDimensions { self.dimensions }
    pub fn format(&self) -> ImageFormat { self.format }
    pub fn conversion_time_ms(&self) -> f64 { self.conversion_time_ms }
    pub fn original_size(&self) -> u64 { self.original_size }
    pub fn converted_size(&self) -> u64 { self.converted_size }
    pub fn compression_ratio(&self) -> f32 { self.compression_ratio }
    pub fn quality_metrics(&self) -> Option<&QualityMetrics> { self.quality_metrics.as_ref() }
    
    /// 计算压缩节省的字节数
    pub fn bytes_saved(&self) -> i64 {
        self.original_size as i64 - self.converted_size as i64
    }
    
    /// 计算压缩率百分比
    pub fn compression_percentage(&self) -> f32 {
        if self.original_size > 0 {
            (1.0 - self.compression_ratio) * 100.0
        } else {
            0.0
        }
    }
}

impl QualityMetrics {
    /// 创建质量指标 - 包内构造器
    pub(crate) fn new(psnr: f32, ssim: f32, perceptual_similarity: f32) -> Self {
        Self {
            psnr,
            ssim,
            perceptual_similarity,
        }
    }
    
    // 只读访问器
    pub fn psnr(&self) -> f32 { self.psnr }
    pub fn ssim(&self) -> f32 { self.ssim }
    pub fn perceptual_similarity(&self) -> f32 { self.perceptual_similarity }
    
    /// 综合质量评分 (0.0-1.0)
    pub fn overall_quality(&self) -> f32 {
        // 加权计算综合质量
        let normalized_psnr = (self.psnr / 50.0).min(1.0);
        let weighted_score = normalized_psnr * 0.4 + self.ssim * 0.4 + self.perceptual_similarity * 0.2;
        weighted_score.clamp(0.0, 1.0)
    }
}

// 格式能力标志常量 - 编译时常量
impl FormatCapabilities {
    const LOSSY: u8 = 1 << 0;
    const TRANSPARENCY: u8 = 1 << 1;
    const ANIMATION: u8 = 1 << 2;
    const PROGRESSIVE: u8 = 1 << 3;
    const METADATA: u8 = 1 << 4;
    const HDR: u8 = 1 << 5;
    
    pub const fn new() -> Self {
        Self { flags: 0 }
    }
    
    pub const fn with_lossy(mut self) -> Self {
        self.flags |= Self::LOSSY;
        self
    }
    
    pub const fn with_transparency(mut self) -> Self {
        self.flags |= Self::TRANSPARENCY;
        self
    }
    
    pub const fn with_animation(mut self) -> Self {
        self.flags |= Self::ANIMATION;
        self
    }
    
    pub const fn with_progressive(mut self) -> Self {
        self.flags |= Self::PROGRESSIVE;
        self
    }
    
    pub const fn with_metadata(mut self) -> Self {
        self.flags |= Self::METADATA;
        self
    }
    
    pub const fn with_hdr(mut self) -> Self {
        self.flags |= Self::HDR;
        self
    }
    
    // 检查能力的方法 - 内联编译时优化
    #[inline]
    pub fn supports_lossy(&self) -> bool { self.flags & Self::LOSSY != 0 }
    
    #[inline]
    pub fn supports_transparency(&self) -> bool { self.flags & Self::TRANSPARENCY != 0 }
    
    #[inline]
    pub fn supports_animation(&self) -> bool { self.flags & Self::ANIMATION != 0 }
    
    #[inline]
    pub fn supports_progressive(&self) -> bool { self.flags & Self::PROGRESSIVE != 0 }
    
    #[inline]
    pub fn supports_metadata(&self) -> bool { self.flags & Self::METADATA != 0 }
    
    #[inline]
    pub fn supports_hdr(&self) -> bool { self.flags & Self::HDR != 0 }
}

impl ImageFormat {
    /// 获取格式信息 - 编译时常量查找
    pub const fn info(&self) -> FormatInfo {
        match self {
            ImageFormat::Jpeg => FormatInfo {
                name: "JPEG",
                description: "Joint Photographic Experts Group - 有损压缩，适合照片",
                extensions: &["jpg", "jpeg"],
                mime_type: "image/jpeg",
                capabilities: FormatCapabilities::new()
                    .with_lossy()
                    .with_progressive()
                    .with_metadata(),
                limits: FormatLimits {
                    max_width: Some(65535),
                    max_height: Some(65535),
                    max_file_size: None,
                    supported_bit_depths: &[8],
                },
            },
            ImageFormat::Png => FormatInfo {
                name: "PNG",
                description: "Portable Network Graphics - 无损压缩，支持透明度",
                extensions: &["png"],
                mime_type: "image/png",
                capabilities: FormatCapabilities::new()
                    .with_transparency()
                    .with_metadata(),
                limits: FormatLimits {
                    max_width: None,
                    max_height: None,
                    max_file_size: None,
                    supported_bit_depths: &[8, 16],
                },
            },
            ImageFormat::WebP => FormatInfo {
                name: "WebP",
                description: "Web Picture Format - 现代格式，支持有损和无损",
                extensions: &["webp"],
                mime_type: "image/webp",
                capabilities: FormatCapabilities::new()
                    .with_lossy()
                    .with_transparency()
                    .with_animation()
                    .with_metadata(),
                limits: FormatLimits {
                    max_width: Some(16383),
                    max_height: Some(16383),
                    max_file_size: None,
                    supported_bit_depths: &[8],
                },
            },
            ImageFormat::Avif => FormatInfo {
                name: "AVIF",
                description: "AV1 Image File Format - 新一代格式，最高压缩比",
                extensions: &["avif"],
                mime_type: "image/avif",
                capabilities: FormatCapabilities::new()
                    .with_lossy()
                    .with_transparency()
                    .with_animation()
                    .with_hdr()
                    .with_metadata(),
                limits: FormatLimits {
                    max_width: None,
                    max_height: None,
                    max_file_size: None,
                    supported_bit_depths: &[8, 10, 12],
                },
            },
            ImageFormat::Bmp => FormatInfo {
                name: "BMP",
                description: "Windows Bitmap - 无压缩，兼容性好",
                extensions: &["bmp"],
                mime_type: "image/bmp",
                capabilities: FormatCapabilities::new(),
                limits: FormatLimits {
                    max_width: None,
                    max_height: None,
                    max_file_size: Some(4u64 * 1024 * 1024 * 1024), // 4GB
                    supported_bit_depths: &[8, 16, 24, 32],
                },
            },
            ImageFormat::Tiff => FormatInfo {
                name: "TIFF",
                description: "Tagged Image File Format - 专业格式，支持多页",
                extensions: &["tiff", "tif"],
                mime_type: "image/tiff",
                capabilities: FormatCapabilities::new()
                    .with_transparency()
                    .with_metadata(),
                limits: FormatLimits {
                    max_width: None,
                    max_height: None,
                    max_file_size: None,
                    supported_bit_depths: &[8, 16, 32],
                },
            },
            ImageFormat::Gif => FormatInfo {
                name: "GIF",
                description: "Graphics Interchange Format - 支持动画，256色",
                extensions: &["gif"],
                mime_type: "image/gif",
                capabilities: FormatCapabilities::new()
                    .with_transparency()
                    .with_animation(),
                limits: FormatLimits {
                    max_width: Some(65535),
                    max_height: Some(65535),
                    max_file_size: None,
                    supported_bit_depths: &[8],
                },
            },
            ImageFormat::Ico => FormatInfo {
                name: "ICO",
                description: "Windows Icon Format - 图标格式，多尺寸",
                extensions: &["ico"],
                mime_type: "image/x-icon",
                capabilities: FormatCapabilities::new()
                    .with_transparency(),
                limits: FormatLimits {
                    max_width: Some(256),
                    max_height: Some(256),
                    max_file_size: Some(1024 * 1024), // 1MB
                    supported_bit_depths: &[8, 16, 24, 32],
                },
            },
        }
    }
    
    /// 获取文件扩展名 - 编译时内联
    #[inline]
    pub fn extension(&self) -> &'static str {
        self.info().extensions[0]
    }
    
    /// 获取MIME类型 - 编译时内联
    #[inline]
    pub fn mime_type(&self) -> &'static str {
        self.info().mime_type
    }
    
    /// 检查是否支持有损压缩 - 编译时优化
    #[inline]
    pub fn supports_lossy(&self) -> bool {
        self.info().capabilities.supports_lossy()
    }
    
    /// 检查是否支持透明度 - 编译时优化
    #[inline]
    pub fn supports_transparency(&self) -> bool {
        self.info().capabilities.supports_transparency()
    }
    
    /// 检查是否支持动画 - 编译时优化
    #[inline]
    pub fn supports_animation(&self) -> bool {
        self.info().capabilities.supports_animation()
    }
}

// =============================================================================
// 像素类型实现 - 零成本抽象的完美示例
// =============================================================================

impl<T> Pixel for Rgb<T> 
where 
    T: Copy + Clone + Send + Sync + 'static
{
    type Subpixel = T;
    const CHANNEL_COUNT: u8 = 3;
    const HAS_ALPHA: bool = false;
    const BITS_PER_CHANNEL: u8 = std::mem::size_of::<T>() as u8 * 8;
    
    #[inline]
    fn from_channels(channels: &[Self::Subpixel]) -> Self {
        assert!(channels.len() >= 3);
        Self { r: channels[0], g: channels[1], b: channels[2] }
    }
    
    #[inline]
    fn to_channels(&self) -> [Self::Subpixel; 3] {
        [self.r, self.g, self.b]
    }
    
    #[inline]
    fn luminance(&self) -> Self::Subpixel where T: From<u8> + Into<f32> {
        // ITU-R BT.709 标准的亮度计算
        let r_f32: f32 = self.r.into();
        let g_f32: f32 = self.g.into();
        let b_f32: f32 = self.b.into();
        let luminance = 0.2126 * r_f32 + 0.7152 * g_f32 + 0.0722 * b_f32;
        T::from(luminance as u8)
    }
}

impl<T> Pixel for Rgba<T> 
where 
    T: Copy + Clone + Send + Sync + 'static
{
    type Subpixel = T;
    const CHANNEL_COUNT: u8 = 4;
    const HAS_ALPHA: bool = true;
    const BITS_PER_CHANNEL: u8 = std::mem::size_of::<T>() as u8 * 8;
    
    #[inline]
    fn from_channels(channels: &[Self::Subpixel]) -> Self {
        assert!(channels.len() >= 4);
        Self { r: channels[0], g: channels[1], b: channels[2], a: channels[3] }
    }
    
    #[inline]
    fn to_channels(&self) -> [Self::Subpixel; 4] {
        [self.r, self.g, self.b, self.a]
    }
    
    #[inline]
    fn luminance(&self) -> Self::Subpixel where T: From<u8> + Into<f32> {
        let r_f32: f32 = self.r.into();
        let g_f32: f32 = self.g.into();
        let b_f32: f32 = self.b.into();
        let luminance = 0.2126 * r_f32 + 0.7152 * g_f32 + 0.0722 * b_f32;
        T::from(luminance as u8)
    }
}

impl<T> Pixel for Luma<T> 
where 
    T: Copy + Clone + Send + Sync + 'static
{
    type Subpixel = T;
    const CHANNEL_COUNT: u8 = 1;
    const HAS_ALPHA: bool = false;
    const BITS_PER_CHANNEL: u8 = std::mem::size_of::<T>() as u8 * 8;
    
    #[inline]
    fn from_channels(channels: &[Self::Subpixel]) -> Self {
        assert!(!channels.is_empty());
        Self { l: channels[0] }
    }
    
    #[inline]
    fn to_channels(&self) -> [Self::Subpixel; 1] {
        [self.l]
    }
    
    #[inline]
    fn luminance(&self) -> Self::Subpixel {
        self.l
    }
}

// =============================================================================
// 显示实现 - 用户友好的调试输出
// =============================================================================

impl fmt::Display for ImageFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.info().name)
    }
}

impl fmt::Display for ImageDimensions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}×{}", self.width, self.height)
    }
}

impl fmt::Display for ConvertedImage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} image {}×{} ({} -> {} bytes, {:.1}% compression)", 
               self.format, 
               self.dimensions.width, 
               self.dimensions.height,
               self.original_size,
               self.converted_size,
               self.compression_percentage())
    }
}

// =============================================================================
// 类型别名 - 常用组合的简化
// =============================================================================

/// 常用的8位RGB像素类型
pub type Rgb8 = Rgb<u8>;

/// 常用的8位RGBA像素类型  
pub type Rgba8 = Rgba<u8>;

/// 常用的8位灰度像素类型
pub type Luma8 = Luma<u8>;

/// 16位RGB像素类型（用于高精度处理）
pub type Rgb16 = Rgb<u16>;

/// 16位RGBA像素类型
pub type Rgba16 = Rgba<u16>;

/// 16位灰度像素类型
pub type Luma16 = Luma<u16>;