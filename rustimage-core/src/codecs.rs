//! 编解码引擎 - 封装复杂的图像格式处理算法

use crate::{
    error::{ImageError, Result},
    types::*,
    converter::{ImageBuffer, PixelFormat},
};
use rayon::prelude::*;
use image::{DynamicImage, ImageFormat as ImgFmt, ImageBuffer as ImgBuffer, Rgba as ImgRgba};
use std::io::Cursor;

/// 编解码引擎 - 管理所有格式的编解码器
pub struct CodecEngine {
    /// JPEG 编解码器
    jpeg_codec: JpegCodec,
    /// PNG 编解码器
    png_codec: PngCodec,
    /// WebP 编解码器
    webp_codec: WebPCodec,
    /// AVIF 编解码器
    avif_codec: AvifCodec,
    /// BMP 编解码器
    bmp_codec: BmpCodec,
    /// TIFF 编解码器
    tiff_codec: TiffCodec,
    /// GIF 编解码器
    gif_codec: GifCodec,
    /// ICO 编解码器
    ico_codec: IcoCodec,
}

impl CodecEngine {
    /// 创建新的编解码引擎
    pub fn new() -> Self {
        Self {
            jpeg_codec: JpegCodec::new(),
            png_codec: PngCodec::new(),
            webp_codec: WebPCodec::new(),
            avif_codec: AvifCodec::new(),
            bmp_codec: BmpCodec::new(),
            tiff_codec: TiffCodec::new(),
            gif_codec: GifCodec::new(),
            ico_codec: IcoCodec::new(),
        }
    }
    
    /// 解码图像数据
    pub fn decode<P: Pixel>(
        &self,
        data: &[u8],
        format: ImageFormat,
    ) -> Result<ImageBuffer<P>> {
        let dyn_img = decode_with_image_crate(data, format)?;
        // 统一转换为 RGBA8，然后再映射到 P
        let rgba = dyn_img.to_rgba8();
        let (w, h) = rgba.dimensions();
        let mut out: Vec<P> = Vec::with_capacity((w as usize) * (h as usize));
        for px in rgba.pixels() {
            let channels = [px[0], px[1], px[2], px[3]];
            let p: P = if P::CHANNEL_COUNT == 4 {
                P::from_channels(&[channels[0], channels[1], channels[2], channels[3]])
            } else if P::CHANNEL_COUNT == 3 {
                P::from_channels(&[channels[0], channels[1], channels[2]])
            } else {
                let y = (0.299f32 * channels[0] as f32 + 0.587f32 * channels[1] as f32 + 0.114f32 * channels[2] as f32).round() as u8;
                P::from_channels(&[y])
            };
            out.push(p);
        }
        let pf = match P::CHANNEL_COUNT {
            4 => PixelFormat::Rgba8,
            3 => PixelFormat::Rgb8,
            1 => PixelFormat::Gray8,
            _ => PixelFormat::Rgba8,
        };
        ImageBuffer::from_raw(w, h, out, pf)
    }
    
    /// 编码图像数据
    pub fn encode<P: Pixel>(
        &self,
        buffer: &ImageBuffer<P>,
        format: ImageFormat,
        options: &ConversionOptions,
    ) -> Result<Vec<u8>> {
        // 将缓冲区归一化到 RGBA8 再交给 image crate 编码
        let dims = buffer.dimensions();
        let width = dims.width;
        let height = dims.height;
        let mut flat: Vec<u8> = Vec::with_capacity((width as usize) * (height as usize) * 4);
        for p in buffer.as_slice() {
            let ch = p.to_channels();
            match P::CHANNEL_COUNT {
                4 => {
                    flat.extend_from_slice(&[ch[0], ch[1], ch[2], ch[3]]);
                }
                3 => {
                    flat.extend_from_slice(&[ch[0], ch[1], ch[2], 255]);
                }
                1 => {
                    let y = ch[0];
                    flat.extend_from_slice(&[y, y, y, 255]);
                }
                _ => return Err(ImageError::InvalidParameters { details: "Unsupported channel count".into() }),
            }
        }
        let img: ImgBuffer<ImgRgba<u8>, _> = ImgBuffer::from_vec(width, height, flat)
            .ok_or_else(|| ImageError::ProcessingFailed { reason: "Invalid buffer size".into() })?;
        let dyn_img = DynamicImage::ImageRgba8(img);
        encode_with_image_crate(&dyn_img, format, options)
    }
    
    /// 获取编解码器信息
    pub fn get_codec_info(format: ImageFormat) -> CodecInfo {
        CodecInfo {
            format,
            name: format!("{:?}", format),
            version: "image-crate-backend".into(),
            supports_decode: true,
            supports_encode: true,
            performance_level: PerformanceLevel::Balanced,
            quality_features: QualityFeatures {
                supports_lossless: matches!(format, ImageFormat::Png | ImageFormat::WebP | ImageFormat::Avif | ImageFormat::Bmp | ImageFormat::Tiff),
                supports_lossy: matches!(format, ImageFormat::Jpeg | ImageFormat::WebP | ImageFormat::Avif),
                supports_progressive: matches!(format, ImageFormat::Jpeg),
                supports_transparency: format.supports_transparency(),
                supports_animation: format.supports_animation(),
                max_quality_level: 100,
            },
        }
    }
}

/// 编解码器信息
#[derive(Debug, Clone)]
pub struct CodecInfo {
    pub format: ImageFormat,
    pub name: String,
    pub version: String,
    pub supports_decode: bool,
    pub supports_encode: bool,
    pub performance_level: PerformanceLevel,
    pub quality_features: QualityFeatures,
}

/// 性能级别
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PerformanceLevel {
    Fast,
    Balanced,
    HighQuality,
}

/// 质量特性
#[derive(Debug, Clone)]
pub struct QualityFeatures {
    pub supports_lossless: bool,
    pub supports_lossy: bool,
    pub supports_progressive: bool,
    pub supports_transparency: bool,
    pub supports_animation: bool,
    pub max_quality_level: u8,
}

/// 编解码器特征 - 零成本抽象
pub trait Codec<P: Pixel>: Send + Sync {
    /// 解码图像数据
    fn decode(&self, data: &[u8]) -> Result<ImageBuffer<P>>;
    
    /// 编码图像数据
    fn encode(&self, buffer: &ImageBuffer<P>, options: &ConversionOptions) -> Result<Vec<u8>>;
    
    /// 获取编解码器信息
    fn info(&self) -> CodecInfo;
    
    /// 验证数据格式
    fn validate_format(&self, data: &[u8]) -> bool;
    
    /// 获取默认选项
    fn default_options(&self) -> ConversionOptions;
    
    /// 预处理 - 用于优化
    fn preprocess(&mut self, dimensions: ImageDimensions) -> Result<()> {
        Ok(())
    }
    
    /// 后处理 - 清理资源
    fn postprocess(&mut self) -> Result<()> {
        Ok(())
    }
}

/// JPEG 编解码器 - 展示有损压缩优化
pub struct JpegCodec {
    /// 质量查找表
    quality_table: Option<Vec<u8>>,
    /// 优化策略
    optimization_level: u8,
}

impl JpegCodec {
    pub fn new() -> Self {
        Self { quality_table: None, optimization_level: 0 }
    }
}

/// PNG 编解码器 - 展示无损压缩优化
pub struct PngCodec {
    /// 压缩策略
    compression_strategy: CompressionStrategy,
    /// 滤波器类型
    filter_type: PngFilter,
}

/// PNG 滤波器类型
#[derive(Debug, Clone, Copy)]
pub enum PngFilter {
    None,
    Sub,
    Up,
    Average,
    Paeth,
    Adaptive, // 自适应选择最佳滤波器
}

/// 压缩策略
#[derive(Debug, Clone, Copy)]
pub enum CompressionStrategy {
    Default,
    Filtered,
    HuffmanOnly,
    Rle,
    Fixed,
}

impl PngCodec {
    pub fn new() -> Self {
        Self { compression_strategy: CompressionStrategy::Default, filter_type: PngFilter::Adaptive }
    }
}

/// WebP 编解码器 - 展示现代格式特性
pub struct WebPCodec {
    /// 编码模式
    encoding_mode: WebPMode,
    /// 预处理选项
    preprocessing: WebPPreprocessing,
}

/// WebP 编码模式
#[derive(Debug, Clone, Copy)]
pub enum WebPMode {
    Lossy,
    Lossless,
    Mixed, // 混合模式，自动选择
}

/// WebP 预处理选项
#[derive(Debug, Clone)]
pub struct WebPPreprocessing {
    pub sharp_yuv: bool,
    pub auto_filter: bool,
    pub alpha_compression: bool,
}

impl WebPCodec {
    pub fn new() -> Self {
        Self { encoding_mode: WebPMode::Mixed, preprocessing: WebPPreprocessing { sharp_yuv: true, auto_filter: true, alpha_compression: true } }
    }
}

/// AVIF 编解码器 - 展示下一代格式
pub struct AvifCodec {
    /// 编码器设置
    encoder_settings: AvifEncoderSettings,
}

/// AVIF 编码器设置
#[derive(Debug, Clone)]
pub struct AvifEncoderSettings {
    pub speed: u8,          // 0-10, 0最慢但质量最好
    pub quality: u8,        // 0-100
    pub quality_alpha: u8,  // Alpha通道质量
    pub tiling: TilingMode,
}

/// 分块模式
#[derive(Debug, Clone, Copy)]
pub enum TilingMode {
    Disabled,
    Auto,
    Custom { cols: u8, rows: u8 },
}

impl AvifCodec {
    pub fn new() -> Self {
        Self { encoder_settings: AvifEncoderSettings { speed: 6, quality: 80, quality_alpha: 80, tiling: TilingMode::Auto } }
    }
}

/// BMP 编解码器 - 简单无压缩格式
pub struct BmpCodec {
    /// 是否支持压缩
    support_compression: bool,
}

impl BmpCodec {
    pub fn new() -> Self {
        Self { support_compression: false }
    }
}

/// TIFF 编解码器 - 专业格式支持
pub struct TiffCodec {
    /// 压缩类型
    compression_type: TiffCompression,
}

/// TIFF 压缩类型
#[derive(Debug, Clone, Copy)]
pub enum TiffCompression {
    None,
    Lzw,
    Deflate,
    PackBits,
    Jpeg,
}

impl TiffCodec {
    pub fn new() -> Self {
        Self { compression_type: TiffCompression::None }
    }
}

/// GIF 编解码器 - 动画支持
pub struct GifCodec {
    /// 是否支持动画
    animation_support: bool,
    /// 调色板优化
    palette_optimization: bool,
}

impl GifCodec {
    pub fn new() -> Self {
        Self { animation_support: true, palette_optimization: true }
    }
}

/// ICO 编解码器 - 图标格式
pub struct IcoCodec {
    /// 支持的图标尺寸
    supported_sizes: Vec<u32>,
}

impl IcoCodec {
    pub fn new() -> Self {
        Self { supported_sizes: vec![16, 24, 32, 48, 64, 128, 256] }
    }
}

/// 编解码器工厂
pub struct CodecFactory;

impl CodecFactory {
    /// 创建编解码器
    pub fn create_codec(_format: ImageFormat) -> Result<Box<dyn Codec<Rgba<u8>>>> {
        Err(ImageError::UnsupportedOperation { operation: "create_codec not implemented".into() })
    }
    
    /// 获取所有支持的格式
    pub fn supported_formats() -> Vec<ImageFormat> {
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
    
    /// 检查格式是否支持
    pub fn is_format_supported(format: ImageFormat) -> bool {
        Self::supported_formats().contains(&format)
    }
}

// 使用 image crate 解码
fn decode_with_image_crate(data: &[u8], format: ImageFormat) -> Result<DynamicImage> {
    let img_format = to_image_crate_format(format).ok_or_else(|| ImageError::InvalidFormat { format: format!("{:?}", format) })?;
    let reader = image::io::Reader::with_format(Cursor::new(data), img_format);
    let dyn_img = reader.decode().map_err(|e| ImageError::DecodeError { message: e.to_string() })?;
    Ok(dyn_img)
}

// 使用 image crate 编码
fn encode_with_image_crate(img: &DynamicImage, format: ImageFormat, _options: &ConversionOptions) -> Result<Vec<u8>> {
    let mut cursor = Cursor::new(Vec::new());
    let fmt = to_image_crate_format(format).ok_or_else(|| ImageError::InvalidFormat { format: format!("{:?}", format) })?;
    img.write_to(&mut cursor, fmt).map_err(|e| ImageError::EncodeError { message: e.to_string() })?;
    Ok(cursor.into_inner())
}

fn to_image_crate_format(format: ImageFormat) -> Option<ImgFmt> {
    match format {
        ImageFormat::Jpeg => Some(ImgFmt::Jpeg),
        ImageFormat::Png => Some(ImgFmt::Png),
        ImageFormat::WebP => Some(ImgFmt::WebP),
        ImageFormat::Avif => Some(ImgFmt::Avif),
        ImageFormat::Bmp => Some(ImgFmt::Bmp),
        ImageFormat::Tiff => Some(ImgFmt::Tiff),
        ImageFormat::Gif => Some(ImgFmt::Gif),
        ImageFormat::Ico => Some(ImgFmt::Ico),
    }
}