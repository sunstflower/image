//! 编解码引擎 - 深模块设计的图像格式处理核心
//!
//! 本模块遵循《软件设计哲学》的核心理念：
//! - **深模块设计**：简单的编解码接口，隐藏复杂的算法实现
//! - **信息隐藏**：每个编解码器的实现细节完全私有
//! - **分层架构**：CodecEngine -> 具体Codec -> 底层算法
//! - **零成本抽象**：编译时特化和内联优化

use crate::{
    error::{ImageError, Result},
    types::*,
};
use rayon::prelude::*;
use std::sync::Arc;

// =============================================================================
// 公共API - 深模块的简单接口
// =============================================================================

/// 编解码引擎 - 对外的统一入口点
/// 
/// 这是一个"深模块"：提供简单的encode/decode接口，
/// 内部管理8种不同格式的复杂编解码器
pub struct CodecEngine {
    // 私有字段：隐藏所有实现细节
    codecs: CodecRegistry, // 管理所有编解码器实例    
    config: CodecConfig, // 编解码器配置
}

/// 编解码器配置 - 使用构建器模式
#[derive(Debug, Clone)]
pub struct CodecConfig {
    /// 是否启用并行处理
    pub parallel_enabled: bool,
    /// 线程池大小
    pub thread_pool_size: Option<usize>,
    /// 是否启用SIMD优化  
    pub simd_enabled: bool,
    /// 内存限制（字节）
    pub memory_limit: Option<u64>,
    /// 质量优先级
    pub quality_priority: QualityPriority,
}

/// 质量优先级策略
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QualityPriority {
    /// 速度优先
    Speed,
    /// 平衡速度和质量
    Balanced,
    /// 质量优先
    Quality,
}

/// 编解码器配置构建器
#[derive(Debug, Clone)]
pub struct CodecConfigBuilder {
    config: CodecConfig,
}

/// 图像缓冲区 - 零成本抽象的像素容器
#[derive(Debug, Clone)]
pub struct ImageBuffer<P: Pixel> {
    /// 像素数据 - 私有：防止直接修改
    pixels: Vec<P>,
    /// 图像尺寸 - 私有：通过方法访问
    dimensions: ImageDimensions,
    /// 像素格式 - 私有：类型安全
    pixel_format: PixelFormat,
}

/// 像素格式枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PixelFormat {
    /// 8位RGB
    Rgb8,
    /// 8位RGBA
    Rgba8,
    /// 8位灰度
    Gray8,
    /// 16位RGB
    Rgb16,
    /// 16位RGBA
    Rgba16,
    /// 16位灰度
    Gray16,
}

// =============================================================================
// 内部类型 - 信息隐藏的体现
// =============================================================================

/// 编解码器注册表 - 私有：管理所有编解码器实例
struct CodecRegistry {
    /// JPEG编解码器
    jpeg: Box<dyn Codec<Rgba8> + Send + Sync>,
    /// PNG编解码器
    png: Box<dyn Codec<Rgba8> + Send + Sync>,
    /// WebP编解码器
    webp: Box<dyn Codec<Rgba8> + Send + Sync>,
    /// AVIF编解码器
    avif: Box<dyn Codec<Rgba8> + Send + Sync>,
    /// BMP编解码器
    bmp: Box<dyn Codec<Rgba8> + Send + Sync>,
    /// TIFF编解码器
    tiff: Box<dyn Codec<Rgba8> + Send + Sync>,
    /// GIF编解码器
    gif: Box<dyn Codec<Rgba8> + Send + Sync>,
    /// ICO编解码器
    ico: Box<dyn Codec<Rgba8> + Send + Sync>,
}

// =============================================================================
// 编解码器抽象 - 零成本抽象接口
// =============================================================================

/// 编解码器特征 - 统一的编解码接口
/// 
/// 使用泛型和关联类型实现零成本抽象
pub trait Codec<P: Pixel>: Send + Sync {
    /// 解码图像数据
    /// 
    /// # 参数
    /// - `data`: 原始图像数据
    /// 
    /// # 返回
    /// - `Ok(ImageBuffer)`: 解码成功的像素缓冲区
    /// - `Err(ImageError)`: 解码失败的错误信息
    fn decode(&self, data: &[u8]) -> Result<ImageBuffer<P>>;
    
    /// 编码图像数据
    /// 
    /// # 参数
    /// - `buffer`: 像素缓冲区
    /// - `options`: 编码选项
    /// 
    /// # 返回
    /// - `Ok(Vec<u8>)`: 编码后的图像数据
    /// - `Err(ImageError)`: 编码失败的错误信息
    fn encode(&self, buffer: &ImageBuffer<P>, options: &ConversionOptions) -> Result<Vec<u8>>;
    
    /// 获取编解码器信息
    fn info(&self) -> CodecInfo;
    
    /// 验证数据格式
    fn validate_format(&self, data: &[u8]) -> bool;
    
    /// 获取默认编码选项
    fn default_options(&self) -> ConversionOptions {
        ConversionOptionsBuilder::new().build()
    }
    
    /// 预处理 - 可选的性能优化
    fn preprocess(&mut self, _dimensions: ImageDimensions) -> Result<()> {
        Ok(())
    }
    
    /// 后处理 - 资源清理
    fn postprocess(&mut self) -> Result<()> {
        Ok(())
    }
}

/// 编解码器信息
#[derive(Debug, Clone)]
pub struct CodecInfo {
    /// 格式类型
    pub format: ImageFormat,
    /// 编解码器名称
    pub name: String,
    /// 版本信息
    pub version: String,
    /// 是否支持解码
    pub supports_decode: bool,
    /// 是否支持编码
    pub supports_encode: bool,
    /// 性能等级
    pub performance_level: PerformanceLevel,
    /// 质量特性
    pub quality_features: QualityFeatures,
}

/// 性能等级
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PerformanceLevel {
    /// 快速但质量较低
    Fast,
    /// 速度和质量平衡
    Balanced,
    /// 高质量但较慢
    HighQuality,
}

/// 质量特性
#[derive(Debug, Clone)]
pub struct QualityFeatures {
    /// 支持无损压缩
    pub supports_lossless: bool,
    /// 支持有损压缩
    pub supports_lossy: bool,
    /// 支持渐进式编码
    pub supports_progressive: bool,
    /// 支持透明度
    pub supports_transparency: bool,
    /// 支持动画
    pub supports_animation: bool,
    /// 最大质量等级
    pub max_quality_level: u8,
}

// =============================================================================
// 具体编解码器实现 - 每个都是独立的深模块
// =============================================================================

/// JPEG编解码器 - 有损压缩专家
struct JpegCodec {
    /// 预计算的量化表 - 私有：优化实现
    quality_tables: Option<Arc<[u8; 64]>>,
    /// 优化级别 - 私有：内部配置
    optimization_level: u8,
    /// DCT实现类型 - 私有：算法选择
    dct_impl: DctImplementation,
}

/// DCT实现类型 - 私有枚举
#[derive(Debug, Clone, Copy)]
enum DctImplementation {
    /// 标准实现
    Standard,
    /// SIMD优化实现
    Simd,
    /// 查表优化实现
    LookupTable,
}

/// PNG编解码器 - 无损压缩专家
struct PngCodec {
    /// 压缩策略 - 私有：算法选择
    compression_strategy: CompressionStrategy,
    /// 滤波器类型 - 私有：预处理选择
    filter_type: PngFilter,
    /// zlib压缩级别 - 私有：压缩参数
    compression_level: u8,
}

/// PNG压缩策略 - 私有枚举
#[derive(Debug, Clone, Copy)]
enum CompressionStrategy {
    Default,
    Filtered,
    HuffmanOnly,
    Rle,
    Fixed,
}

/// PNG滤波器类型 - 私有枚举
#[derive(Debug, Clone, Copy)]
enum PngFilter {
    None,
    Sub,
    Up,
    Average,
    Paeth,
    /// 自适应选择最佳滤波器
    Adaptive,
}

/// WebP编解码器 - 现代格式处理器
struct WebPCodec {
    /// 编码模式 - 私有：模式选择
    encoding_mode: WebPMode,
    /// 预处理选项 - 私有：优化配置
    preprocessing: WebPPreprocessing,
    /// 预测模式 - 私有：压缩优化
    prediction_mode: WebPPrediction,
}

/// WebP编码模式 - 私有枚举
#[derive(Debug, Clone, Copy)]
enum WebPMode {
    /// 有损压缩
    Lossy,
    /// 无损压缩
    Lossless,
    /// 混合模式，自动选择
    Mixed,
}

/// WebP预处理选项 - 私有结构体
#[derive(Debug, Clone)]
struct WebPPreprocessing {
    /// Sharp YUV预处理
    sharp_yuv: bool,
    /// 自动滤波
    auto_filter: bool,
    /// 透明度压缩
    alpha_compression: bool,
}

/// WebP预测模式 - 私有枚举
#[derive(Debug, Clone, Copy)]
enum WebPPrediction {
    None,
    Horizontal,
    Vertical,
    Both,
}

/// AVIF编解码器 - 下一代格式处理器
struct AvifCodec {
    /// 编码器设置 - 私有：配置管理
    encoder_settings: AvifEncoderSettings,
    /// 分块模式 - 私有：并行优化
    tiling_mode: TilingMode,
    /// 色彩空间 - 私有：颜色管理
    color_space: AvifColorSpace,
}

/// AVIF编码器设置 - 私有结构体
#[derive(Debug, Clone)]
struct AvifEncoderSettings {
    /// 编码速度 (0-10, 0最慢但质量最好)
    speed: u8,
    /// 质量参数 (0-100)
    quality: u8,
    /// Alpha通道质量
    quality_alpha: u8,
    /// 像素格式
    pixel_format: AvifPixelFormat,
}

/// AVIF分块模式 - 私有枚举
#[derive(Debug, Clone, Copy)]
enum TilingMode {
    Disabled,
    Auto,
    Custom { cols: u8, rows: u8 },
}

/// AVIF色彩空间 - 私有枚举
#[derive(Debug, Clone, Copy)]
enum AvifColorSpace {
    Bt709,
    Bt2020,
    Srgb,
}

/// AVIF像素格式 - 私有枚举
#[derive(Debug, Clone, Copy)]
enum AvifPixelFormat {
    Yuv420,
    Yuv422,
    Yuv444,
}

// 其他格式的简化编解码器
struct BmpCodec {
    support_compression: bool,
}

struct TiffCodec {
    compression_type: TiffCompression,
}

#[derive(Debug, Clone, Copy)]
enum TiffCompression {
    None,
    Lzw,
    Deflate,
    PackBits,
    Jpeg,
}

struct GifCodec {
    animation_support: bool,
    palette_optimization: bool,
}

struct IcoCodec {
    supported_sizes: Vec<u32>,
}

// =============================================================================
// 公共实现 - 深模块接口的实现
// =============================================================================

impl CodecEngine {
    /// 创建新的编解码引擎 - 主要构造函数
    /// 
    /// # 参数
    /// - `config`: 编解码器配置
    /// 
    /// # 返回
    /// - `Ok(CodecEngine)`: 创建成功的引擎实例
    /// - `Err(ImageError)`: 创建失败的错误信息
    pub fn new(config: CodecConfig) -> Result<Self> {
        let codecs = CodecRegistry::new(&config)?;
        
        Ok(Self {
            codecs,
            config,
        })
    }
    
    /// 使用默认配置创建引擎
    pub fn with_defaults() -> Result<Self> {
        Self::new(CodecConfig::default())
    }
    
    /// 解码图像数据 - 深模块的主要接口
    /// 
    /// 这个简单的接口隐藏了复杂的格式检测、编解码器选择、
    /// 并行处理、错误恢复等逻辑
    pub fn decode<P: Pixel>(&self, data: &[u8], format: ImageFormat) -> Result<ImageBuffer<P>>
    where
        P: From<Rgba8> + 'static,
    {
        // 1. 格式验证 - 内部逻辑
        self.validate_format_data(data, format)?;
        
        // 2. 获取对应的编解码器 - 信息隐藏
        let codec = self.codecs.get_codec(format)?;
        
        // 3. 执行解码 - 委托给具体实现
        let rgba_buffer = codec.decode(data)?;
        
        // 4. 像素格式转换 - 零成本抽象
        self.convert_buffer::<Rgba8, P>(rgba_buffer)
    }
    
    /// 编码图像数据 - 深模块的主要接口
    pub fn encode<P: Pixel>(
        &self,
        buffer: &ImageBuffer<P>,
        format: ImageFormat,
        options: &ConversionOptions,
    ) -> Result<Vec<u8>>
    where
        P: Into<Rgba8> + Copy + 'static,
    {
        // 1. 参数验证 - 内部逻辑
        self.validate_encode_params(format, options)?;
        
        // 2. 像素格式转换 - 零成本抽象
        let rgba_buffer = self.convert_buffer::<P, Rgba8>(buffer.clone())?;
        
        // 3. 获取编解码器并编码 - 委托给具体实现
        let codec = self.codecs.get_codec(format)?;
        codec.encode(&rgba_buffer, options)
    }
    
    /// 检测图像格式 - 便民方法
    pub fn detect_format(&self, data: &[u8]) -> Result<ImageFormat> {
        FormatDetector::detect(data)
    }
    
    /// 获取支持的格式列表
    pub fn supported_formats(&self) -> Vec<ImageFormat> {
        vec![
            ImageFormat::Jpeg,
            ImageFormat::Png,
            ImageFormat::WebP,
            ImageFormat::Avif,
            ImageFormat::Bmp,
            ImageFormat::Tiff,
            ImageFormat::Gif,
            ImageFormat::Ico,
        ]
    }
    
    /// 检查格式转换是否支持
    pub fn supports_conversion(&self, from: ImageFormat, to: ImageFormat) -> bool {
        self.supported_formats().contains(&from) && self.supported_formats().contains(&to)
    }
    
    /// 获取编解码器信息
    pub fn get_codec_info(&self, format: ImageFormat) -> Result<CodecInfo> {
        let codec = self.codecs.get_codec(format)?;
        Ok(codec.info())
    }
    
    /// 更新配置 - 运行时重配置
    pub fn update_config(&mut self, config: CodecConfig) -> Result<()> {
        // 重新创建编解码器注册表
        self.codecs = CodecRegistry::new(&config)?;
        self.config = config;
        Ok(())
    }
}

// =============================================================================
// 私有实现方法 - 信息隐藏
// =============================================================================

impl CodecEngine {
    /// 验证格式和数据的匹配性 - 私有方法
    fn validate_format_data(&self, data: &[u8], format: ImageFormat) -> Result<()> {
        if data.is_empty() {
            return Err(ImageError::InvalidFormat {
                format: format!("Empty data for format {}", format),
            });
        }
        
        let codec = self.codecs.get_codec(format)?;
        if !codec.validate_format(data) {
            return Err(ImageError::InvalidFormat {
                format: format!("Data does not match format {}", format),
            });
        }
        
        Ok(())
    }
    
    /// 验证编码参数 - 私有方法
    fn validate_encode_params(&self, format: ImageFormat, options: &ConversionOptions) -> Result<()> {
        let info = format.info();
        
        // 检查有损格式的质量参数
        if info.capabilities.supports_lossy() {
            if let Some(quality) = options.quality() {
                if !(0.0..=1.0).contains(&quality) {
                    return Err(ImageError::InvalidParameters {
                        details: format!("Quality {} out of range [0.0, 1.0]", quality),
                    });
                }
            }
        }
        
        // 检查无损格式的压缩级别
        if !info.capabilities.supports_lossy() {
            if let Some(level) = options.compression_level() {
                if level > 9 {
                    return Err(ImageError::InvalidParameters {
                        details: format!("Compression level {} out of range [0, 9]", level),
                    });
                }
            }
        }
        
        Ok(())
    }
    
    /// 像素格式转换 - 零成本抽象的私有实现
    fn convert_buffer<From, To>(&self, buffer: ImageBuffer<From>) -> Result<ImageBuffer<To>>
    where
        From: Pixel + Into<To>,
        To: Pixel,
    {
        let converted_pixels: Vec<To> = buffer.pixels
            .into_iter()
            .map(|pixel| pixel.into())
            .collect();
        
        Ok(ImageBuffer {
            pixels: converted_pixels,
            dimensions: buffer.dimensions,
            pixel_format: self.infer_pixel_format::<To>(),
        })
    }
    
    /// 推断像素格式 - 编译时特化
    fn infer_pixel_format<P: Pixel>(&self) -> PixelFormat {
        match (P::CHANNEL_COUNT, P::BITS_PER_CHANNEL, P::HAS_ALPHA) {
            (3, 8, false) => PixelFormat::Rgb8,
            (4, 8, true) => PixelFormat::Rgba8,
            (1, 8, false) => PixelFormat::Gray8,
            (3, 16, false) => PixelFormat::Rgb16,
            (4, 16, true) => PixelFormat::Rgba16,
            (1, 16, false) => PixelFormat::Gray16,
            _ => PixelFormat::Rgba8, // 默认回退
        }
    }
}

// =============================================================================
// 编解码器注册表实现 - 私有管理逻辑
// =============================================================================

impl CodecRegistry {
    /// 创建新的编解码器注册表 - 私有构造函数
    fn new(config: &CodecConfig) -> Result<Self> {
        Ok(Self {
            jpeg: Box::new(JpegCodec::new(config)?),
            png: Box::new(PngCodec::new(config)?),
            webp: Box::new(WebPCodec::new(config)?),
            avif: Box::new(AvifCodec::new(config)?),
            bmp: Box::new(BmpCodec::new(config)?),
            tiff: Box::new(TiffCodec::new(config)?),
            gif: Box::new(GifCodec::new(config)?),
            ico: Box::new(IcoCodec::new(config)?),
        })
    }
    
    /// 获取指定格式的编解码器 - 私有方法
    fn get_codec(&self, format: ImageFormat) -> Result<&dyn Codec<Rgba8>> {
        let codec: &dyn Codec<Rgba8> = match format {
            ImageFormat::Jpeg => self.jpeg.as_ref(),
            ImageFormat::Png => self.png.as_ref(),
            ImageFormat::WebP => self.webp.as_ref(),
            ImageFormat::Avif => self.avif.as_ref(),
            ImageFormat::Bmp => self.bmp.as_ref(),
            ImageFormat::Tiff => self.tiff.as_ref(),
            ImageFormat::Gif => self.gif.as_ref(),
            ImageFormat::Ico => self.ico.as_ref(),
        };
        Ok(codec)
    }
}

// =============================================================================
// 图像缓冲区实现 - 零成本抽象容器
// =============================================================================

impl<P: Pixel> ImageBuffer<P> {
    /// 创建新的图像缓冲区
    pub fn new(width: u32, height: u32, pixel_format: PixelFormat) -> Self {
        let capacity = (width * height) as usize;
        let pixels = Vec::with_capacity(capacity);
        
        Self {
            pixels,
            dimensions: ImageDimensions { width, height },
            pixel_format,
        }
    }
    
    /// 从原始数据创建缓冲区
    pub fn from_raw(
        width: u32,
        height: u32,
        data: Vec<P>,
        pixel_format: PixelFormat,
    ) -> Result<Self> {
        let expected_len = (width * height) as usize;
        if data.len() != expected_len {
            return Err(ImageError::InvalidParameters {
                details: format!(
                    "Data length {} does not match dimensions {}×{} (expected {})",
                    data.len(), width, height, expected_len
                ),
            });
        }
        
        Ok(Self {
            pixels: data,
            dimensions: ImageDimensions { width, height },
            pixel_format,
        })
    }
    
    /// 获取像素 - 边界检查的安全访问
    #[inline]
    pub fn get_pixel(&self, x: u32, y: u32) -> Option<&P> {
        if x >= self.dimensions.width || y >= self.dimensions.height {
            return None;
        }
        
        let index = (y * self.dimensions.width + x) as usize;
        self.pixels.get(index)
    }
    
    /// 设置像素 - 边界检查的安全修改
    #[inline]
    pub fn put_pixel(&mut self, x: u32, y: u32, pixel: P) -> Result<()> {
        if x >= self.dimensions.width || y >= self.dimensions.height {
            return Err(ImageError::InvalidParameters {
                details: format!("Pixel coordinates ({}, {}) out of bounds", x, y),
            });
        }
        
        let index = (y * self.dimensions.width + x) as usize;
        if index < self.pixels.len() {
            self.pixels[index] = pixel;
            Ok(())
        } else {
            Err(ImageError::InvalidParameters {
                details: "Buffer not properly initialized".to_string(),
            })
        }
    }
    
    // 只读访问器方法
    pub fn dimensions(&self) -> ImageDimensions { self.dimensions }
    pub fn pixel_format(&self) -> PixelFormat { self.pixel_format }
    pub fn as_slice(&self) -> &[P] { &self.pixels }
    pub fn len(&self) -> usize { self.pixels.len() }
    pub fn is_empty(&self) -> bool { self.pixels.is_empty() }
    
    /// 获取可变切片 - 高级用户的直接访问  
    pub fn as_mut_slice(&mut self) -> &mut [P] {
        &mut self.pixels
    }
}

// =============================================================================
// 格式检测器 - 独立的工具模块
// =============================================================================

/// 格式检测器 - 静态工具类
pub struct FormatDetector;

impl FormatDetector {
    /// 从数据检测格式 - 主要检测方法
    pub fn detect(data: &[u8]) -> Result<ImageFormat> {
        if data.len() < 8 {
            return Err(ImageError::InvalidFormat {
                format: "Data too short for format detection".to_string(),
            });
        }
        
        // JPEG文件头检测
        if data.starts_with(&[0xFF, 0xD8, 0xFF]) {
            return Ok(ImageFormat::Jpeg);
        }
        
        // PNG文件头检测
        if data.starts_with(&[0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A]) {
            return Ok(ImageFormat::Png);
        }
        
        // WebP文件头检测
        if data.len() >= 12 && 
           data[0..4] == [0x52, 0x49, 0x46, 0x46] && 
           data[8..12] == [0x57, 0x45, 0x42, 0x50] {
            return Ok(ImageFormat::WebP);
        }
        
        // AVIF文件头检测
        if data.len() >= 12 && 
           data[4..8] == [0x66, 0x74, 0x79, 0x70] && 
           data[8..12] == [0x61, 0x76, 0x69, 0x66] {
            return Ok(ImageFormat::Avif);
        }
        
        // BMP文件头检测
        if data.starts_with(&[0x42, 0x4D]) {
            return Ok(ImageFormat::Bmp);
        }
        
        // TIFF文件头检测 (小端和大端)
        if data.starts_with(&[0x49, 0x49, 0x2A, 0x00]) || 
           data.starts_with(&[0x4D, 0x4D, 0x00, 0x2A]) {
            return Ok(ImageFormat::Tiff);
        }
        
        // GIF文件头检测
        if data.starts_with(b"GIF87a") || data.starts_with(b"GIF89a") {
            return Ok(ImageFormat::Gif);
        }
        
        // ICO文件头检测
        if data.len() >= 4 && data[0..4] == [0x00, 0x00, 0x01, 0x00] {
            return Ok(ImageFormat::Ico);
        }
        
        Err(ImageError::InvalidFormat {
            format: "Unknown format - no matching file signature".to_string(),
        })
    }
    
    /// 从扩展名猜测格式 - 辅助方法
    pub fn guess_from_extension(extension: &str) -> Option<ImageFormat> {
        match extension.to_lowercase().as_str() {
            "jpg" | "jpeg" => Some(ImageFormat::Jpeg),
            "png" => Some(ImageFormat::Png),
            "webp" => Some(ImageFormat::WebP),
            "avif" => Some(ImageFormat::Avif),
            "bmp" => Some(ImageFormat::Bmp),
            "tiff" | "tif" => Some(ImageFormat::Tiff),
            "gif" => Some(ImageFormat::Gif),
            "ico" => Some(ImageFormat::Ico),
            _ => None,
        }
    }
}

// =============================================================================
// 配置构建器实现 - 构建器模式
// =============================================================================

impl CodecConfigBuilder {
    /// 创建新的配置构建器
    pub fn new() -> Self {
        Self {
            config: CodecConfig::default(),
        }
    }
    
    /// 启用/禁用并行处理
    pub fn parallel(mut self, enabled: bool) -> Self {
        self.config.parallel_enabled = enabled;
        self
    }
    
    /// 设置线程池大小
    pub fn thread_pool_size(mut self, size: usize) -> Self {
        self.config.thread_pool_size = Some(size);
        self
    }
    
    /// 启用/禁用SIMD优化
    pub fn simd(mut self, enabled: bool) -> Self {
        self.config.simd_enabled = enabled;
        self
    }
    
    /// 设置内存限制
    pub fn memory_limit(mut self, limit: u64) -> Self {
        self.config.memory_limit = Some(limit);
        self
    }
    
    /// 设置质量优先级
    pub fn quality_priority(mut self, priority: QualityPriority) -> Self {
        self.config.quality_priority = priority;
        self
    }
    
    /// 构建最终配置
    pub fn build(self) -> CodecConfig {
        self.config
    }
}

impl Default for CodecConfigBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for CodecConfig {
    fn default() -> Self {
        Self {
            parallel_enabled: true,
            thread_pool_size: None,
            simd_enabled: true,
            memory_limit: None,
            quality_priority: QualityPriority::Balanced,
        }
    }
}

// =============================================================================
// 具体编解码器的存根实现 - 为了编译通过
// =============================================================================

// 简化的存根实现宏
macro_rules! impl_codec_stub {
    ($codec:ident, $format:expr, $name:expr, $lossy:expr, $transparency:expr, $animation:expr) => {
        impl $codec {
            fn new(_config: &CodecConfig) -> Result<Self> {
                Ok(Self::default())
            }
        }
        
        impl Codec<Rgba8> for $codec {
            fn decode(&self, _data: &[u8]) -> Result<ImageBuffer<Rgba8>> {
                Err(ImageError::UnsupportedOperation {
                    operation: format!("{} decode not yet implemented", $name),
                })
            }
            
            fn encode(&self, _buffer: &ImageBuffer<Rgba8>, _options: &ConversionOptions) -> Result<Vec<u8>> {
                Err(ImageError::UnsupportedOperation {
                    operation: format!("{} encode not yet implemented", $name),
                })
            }
            
            fn info(&self) -> CodecInfo {
                CodecInfo {
                    format: $format,
                    name: format!("{} Codec", $name),
                    version: "1.0.0".to_string(),
                    supports_decode: false, // TODO: 实现后改为true
                    supports_encode: false, // TODO: 实现后改为true
                    performance_level: PerformanceLevel::Balanced,
                    quality_features: QualityFeatures {
                        supports_lossless: !$lossy,
                        supports_lossy: $lossy,
                        supports_progressive: false,
                        supports_transparency: $transparency,
                        supports_animation: $animation,
                        max_quality_level: 100,
                    },
                }
            }
            
            fn validate_format(&self, _data: &[u8]) -> bool {
                false // TODO: 实现具体的格式验证
            }
        }
    };
}

// Default实现
impl Default for JpegCodec {
    fn default() -> Self {
        Self {
            quality_tables: None,
            optimization_level: 6,
            dct_impl: DctImplementation::Standard,
        }
    }
}

impl Default for PngCodec {
    fn default() -> Self {
        Self {
            compression_strategy: CompressionStrategy::Default,
            filter_type: PngFilter::Adaptive,
            compression_level: 6,
        }
    }
}

impl Default for WebPCodec {
    fn default() -> Self {
        Self {
            encoding_mode: WebPMode::Mixed,
            preprocessing: WebPPreprocessing {
                sharp_yuv: false,
                auto_filter: true,
                alpha_compression: true,
            },
            prediction_mode: WebPPrediction::Both,
        }
    }
}

impl Default for AvifCodec {
    fn default() -> Self {
        Self {
            encoder_settings: AvifEncoderSettings {
                speed: 6,
                quality: 80,
                quality_alpha: 80,
                pixel_format: AvifPixelFormat::Yuv420,
            },
            tiling_mode: TilingMode::Auto,
            color_space: AvifColorSpace::Srgb,
        }
    }
}

impl Default for BmpCodec {
    fn default() -> Self {
        Self {
            support_compression: false,
        }
    }
}

impl Default for TiffCodec {
    fn default() -> Self {
        Self {
            compression_type: TiffCompression::Deflate,
        }
    }
}

impl Default for GifCodec {
    fn default() -> Self {
        Self {
            animation_support: true,
            palette_optimization: true,
        }
    }
}

impl Default for IcoCodec {
    fn default() -> Self {
        Self {
            supported_sizes: vec![16, 24, 32, 48, 64, 128, 256],
        }
    }
}

// 应用存根实现
impl_codec_stub!(JpegCodec, ImageFormat::Jpeg, "JPEG", true, false, false);
impl_codec_stub!(PngCodec, ImageFormat::Png, "PNG", false, true, false);
impl_codec_stub!(WebPCodec, ImageFormat::WebP, "WebP", true, true, true);
impl_codec_stub!(AvifCodec, ImageFormat::Avif, "AVIF", true, true, true);
impl_codec_stub!(BmpCodec, ImageFormat::Bmp, "BMP", false, false, false);
impl_codec_stub!(TiffCodec, ImageFormat::Tiff, "TIFF", false, true, false);
impl_codec_stub!(GifCodec, ImageFormat::Gif, "GIF", false, true, true);
impl_codec_stub!(IcoCodec, ImageFormat::Ico, "ICO", false, true, false);