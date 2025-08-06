//! WASM 类型转换和工具
//!
//! 本模块负责 Rust 类型和 JavaScript 类型之间的转换
//! 遵循深模块设计：简单的转换接口，隐藏复杂的序列化逻辑

use wasm_bindgen::prelude::*;
use serde::Serialize;

// =============================================================================
// JavaScript 兼容的类型定义
// =============================================================================

/// JavaScript 兼容的图像格式枚举
#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JsImageFormat {
    Jpeg,
    Png,
    WebP,
    Avif,
    Bmp,
    Tiff,
    Gif,
    Ico,
}

/// JavaScript 兼容的转换选项
#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct JsConversionOptions {
    quality: Option<f32>,
    compression_level: Option<u8>,
    progressive: Option<bool>,
    preserve_dimensions: bool,
    preserve_color_space: bool,
    preserve_metadata: bool,
    custom_params: js_sys::Map,
}

/// JavaScript 兼容的转换结果
#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct JsConvertedImage {
    data: Vec<u8>,
    width: u32,
    height: u32,
    format: JsImageFormat,
    conversion_time_ms: f64,
    original_size: u64,
    compression_ratio: f32,
}

/// JavaScript 兼容的性能指标
#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct JsPerformanceMetrics {
    total_time_ms: f64,
    decode_time_ms: f64,
    encode_time_ms: f64,
    memory_usage_bytes: u64,
    cpu_usage_percent: f32,
    throughput_images_per_second: f64,
}

/// JavaScript 兼容的格式信息
#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct JsFormatInfo {
    name: String,
    description: String,
    extensions: Vec<String>,
    mime_type: String,
    supports_lossy: bool,
    supports_transparency: bool,
    supports_animation: bool,
}

// =============================================================================
// WASM 绑定实现
// =============================================================================

/// 从字符串创建格式
#[wasm_bindgen(js_name = formatFromString)]
pub fn format_from_string(format_str: &str) -> Result<JsImageFormat, JsValue> {
    match format_str.to_lowercase().as_str() {
        "jpeg" | "jpg" => Ok(JsImageFormat::Jpeg),
        "png" => Ok(JsImageFormat::Png),
        "webp" => Ok(JsImageFormat::WebP),
        "avif" => Ok(JsImageFormat::Avif),
        "bmp" => Ok(JsImageFormat::Bmp),
        "tiff" | "tif" => Ok(JsImageFormat::Tiff),
        "gif" => Ok(JsImageFormat::Gif),
        "ico" => Ok(JsImageFormat::Ico),
        _ => Err(JsValue::from_str(&format!("Unsupported format: {}", format_str))),
    }
}

/// 转换为字符串
#[wasm_bindgen(js_name = formatToString)]
pub fn format_to_string(format: JsImageFormat) -> String {
    match format {
        JsImageFormat::Jpeg => "jpeg".to_string(),
        JsImageFormat::Png => "png".to_string(),
        JsImageFormat::WebP => "webp".to_string(),
        JsImageFormat::Avif => "avif".to_string(),
        JsImageFormat::Bmp => "bmp".to_string(),
        JsImageFormat::Tiff => "tiff".to_string(),
        JsImageFormat::Gif => "gif".to_string(),
        JsImageFormat::Ico => "ico".to_string(),
    }
}

#[wasm_bindgen]
impl JsConversionOptions {
    /// 创建新的转换选项
    #[wasm_bindgen(constructor)]
    pub fn new() -> JsConversionOptions {
        JsConversionOptions {
            quality: None,
            compression_level: None,
            progressive: None,
            preserve_dimensions: true,
            preserve_color_space: true,
            preserve_metadata: false,
            custom_params: js_sys::Map::new(),
        }
    }
    
    /// 设置质量参数 (0.0 - 1.0)
    #[wasm_bindgen(js_name = setQuality)]
    pub fn set_quality(&mut self, quality: f32) -> Result<(), JsValue> {
        if !(0.0..=1.0).contains(&quality) {
            return Err(JsValue::from_str("Quality must be between 0.0 and 1.0"));
        }
        self.quality = Some(quality);
        Ok(())
    }
    
    /// 获取质量参数
    #[wasm_bindgen(js_name = getQuality)]
    pub fn get_quality(&self) -> Option<f32> {
        self.quality
    }
    
    /// 设置压缩级别 (0 - 9)
    #[wasm_bindgen(js_name = setCompressionLevel)]
    pub fn set_compression_level(&mut self, level: u8) -> Result<(), JsValue> {
        if level > 9 {
            return Err(JsValue::from_str("Compression level must be between 0 and 9"));
        }
        self.compression_level = Some(level);
        Ok(())
    }
    
    /// 获取压缩级别
    #[wasm_bindgen(js_name = getCompressionLevel)]
    pub fn get_compression_level(&self) -> Option<u8> {
        self.compression_level
    }
    
    /// 设置是否使用渐进式编码
    #[wasm_bindgen(js_name = setProgressive)]
    pub fn set_progressive(&mut self, progressive: bool) {
        self.progressive = Some(progressive);
    }
    
    /// 是否使用渐进式编码
    #[wasm_bindgen(js_name = isProgressive)]
    pub fn is_progressive(&self) -> Option<bool> {
        self.progressive
    }
    
    /// 设置是否保持原始尺寸
    #[wasm_bindgen(js_name = setPreserveDimensions)]
    pub fn set_preserve_dimensions(&mut self, preserve: bool) {
        self.preserve_dimensions = preserve;
    }
    
    /// 是否保持原始尺寸
    #[wasm_bindgen(js_name = preservesDimensions)]
    pub fn preserves_dimensions(&self) -> bool {
        self.preserve_dimensions
    }
    
    /// 设置是否保持色彩空间
    #[wasm_bindgen(js_name = setPreserveColorSpace)]
    pub fn set_preserve_color_space(&mut self, preserve: bool) {
        self.preserve_color_space = preserve;
    }
    
    /// 是否保持色彩空间
    #[wasm_bindgen(js_name = preservesColorSpace)]
    pub fn preserves_color_space(&self) -> bool {
        self.preserve_color_space
    }
    
    /// 设置是否保持元数据
    #[wasm_bindgen(js_name = setPreserveMetadata)]
    pub fn set_preserve_metadata(&mut self, preserve: bool) {
        self.preserve_metadata = preserve;
    }
    
    /// 是否保持元数据
    #[wasm_bindgen(js_name = preservesMetadata)]
    pub fn preserves_metadata(&self) -> bool {
        self.preserve_metadata
    }
    
    /// 设置自定义参数
    #[wasm_bindgen(js_name = setCustomParam)]
    pub fn set_custom_param(&mut self, key: &str, value: &str) {
        self.custom_params.set(&JsValue::from_str(key), &JsValue::from_str(value));
    }
    
    /// 获取自定义参数
    #[wasm_bindgen(js_name = getCustomParam)]
    pub fn get_custom_param(&self, key: &str) -> Option<String> {
        self.custom_params.get(&JsValue::from_str(key))
            .as_string()
    }
}

#[wasm_bindgen]
impl JsConvertedImage {
    /// 获取图像数据
    #[wasm_bindgen(js_name = getData)]
    pub fn get_data(&self) -> Vec<u8> {
        self.data.clone()
    }
    
    /// 获取图像宽度
    #[wasm_bindgen(js_name = getWidth)]
    pub fn get_width(&self) -> u32 {
        self.width
    }
    
    /// 获取图像高度
    #[wasm_bindgen(js_name = getHeight)]
    pub fn get_height(&self) -> u32 {
        self.height
    }
    
    /// 获取图像格式
    #[wasm_bindgen(js_name = getFormat)]
    pub fn get_format(&self) -> JsImageFormat {
        self.format
    }
    
    /// 获取转换时间（毫秒）
    #[wasm_bindgen(js_name = getConversionTimeMs)]
    pub fn get_conversion_time_ms(&self) -> f64 {
        self.conversion_time_ms
    }
    
    /// 获取原始大小（字节）
    #[wasm_bindgen(js_name = getOriginalSize)]
    pub fn get_original_size(&self) -> u64 {
        self.original_size
    }
    
    /// 获取转换后大小（字节）
    #[wasm_bindgen(js_name = getConvertedSize)]
    pub fn get_converted_size(&self) -> u64 {
        self.data.len() as u64
    }
    
    /// 获取压缩比
    #[wasm_bindgen(js_name = getCompressionRatio)]
    pub fn get_compression_ratio(&self) -> f32 {
        self.compression_ratio
    }
    
    /// 获取压缩率百分比
    #[wasm_bindgen(js_name = getCompressionPercentage)]
    pub fn get_compression_percentage(&self) -> f32 {
        if self.original_size > 0 {
            (1.0 - self.compression_ratio) * 100.0
        } else {
            0.0
        }
    }
    
    /// 获取节省的字节数
    #[wasm_bindgen(js_name = getBytesSaved)]
    pub fn get_bytes_saved(&self) -> i64 {
        self.original_size as i64 - self.data.len() as i64
    }
    
    /// 创建 Blob URL（在浏览器中）
    #[wasm_bindgen(js_name = createBlobUrl)]
    pub fn create_blob_url(&self) -> Result<String, JsValue> {
        use web_sys::*;
        
        let mime_type = match self.format {
            JsImageFormat::Jpeg => "image/jpeg",
            JsImageFormat::Png => "image/png",
            JsImageFormat::WebP => "image/webp",
            JsImageFormat::Avif => "image/avif",
            JsImageFormat::Bmp => "image/bmp",
            JsImageFormat::Tiff => "image/tiff",
            JsImageFormat::Gif => "image/gif",
            JsImageFormat::Ico => "image/x-icon",
        };
        
        let uint8_array = js_sys::Uint8Array::new_with_length(self.data.len() as u32);
        uint8_array.copy_from(&self.data);
        
        let blob_parts = js_sys::Array::new();
        blob_parts.push(&uint8_array);
        
        let blob_options = web_sys::BlobPropertyBag::new();
        blob_options.set_type(mime_type);
        
        let blob = web_sys::Blob::new_with_u8_array_sequence_and_options(&blob_parts, &blob_options)?;
        let url = web_sys::Url::create_object_url_with_blob(&blob)?;
        
        Ok(url)
    }
}

#[wasm_bindgen]
impl JsPerformanceMetrics {
    /// 获取总时间
    #[wasm_bindgen(js_name = getTotalTimeMs)]
    pub fn get_total_time_ms(&self) -> f64 {
        self.total_time_ms
    }
    
    /// 获取解码时间
    #[wasm_bindgen(js_name = getDecodeTimeMs)]
    pub fn get_decode_time_ms(&self) -> f64 {
        self.decode_time_ms
    }
    
    /// 获取编码时间
    #[wasm_bindgen(js_name = getEncodeTimeMs)]
    pub fn get_encode_time_ms(&self) -> f64 {
        self.encode_time_ms
    }
    
    /// 获取内存使用量
    #[wasm_bindgen(js_name = getMemoryUsageBytes)]
    pub fn get_memory_usage_bytes(&self) -> u64 {
        self.memory_usage_bytes
    }
    
    /// 获取 CPU 使用率
    #[wasm_bindgen(js_name = getCpuUsagePercent)]
    pub fn get_cpu_usage_percent(&self) -> f32 {
        self.cpu_usage_percent
    }
    
    /// 获取吞吐量
    #[wasm_bindgen(js_name = getThroughputImagesPerSecond)]
    pub fn get_throughput_images_per_second(&self) -> f64 {
        self.throughput_images_per_second
    }
    
    /// 转换为 JSON 字符串
    #[wasm_bindgen(js_name = toJson)]
    pub fn to_json(&self) -> Result<String, JsValue> {
        #[derive(Serialize)]
        struct MetricsJson {
            total_time_ms: f64,
            decode_time_ms: f64,
            encode_time_ms: f64,
            memory_usage_bytes: u64,
            cpu_usage_percent: f32,
            throughput_images_per_second: f64,
        }
        
        let metrics = MetricsJson {
            total_time_ms: self.total_time_ms,
            decode_time_ms: self.decode_time_ms,
            encode_time_ms: self.encode_time_ms,
            memory_usage_bytes: self.memory_usage_bytes,
            cpu_usage_percent: self.cpu_usage_percent,
            throughput_images_per_second: self.throughput_images_per_second,
        };
        
        serde_json::to_string(&metrics)
            .map_err(|e| JsValue::from_str(&e.to_string()))
    }
}

#[wasm_bindgen]
impl JsFormatInfo {
    /// 获取格式名称
    #[wasm_bindgen(js_name = getName)]
    pub fn get_name(&self) -> String {
        self.name.clone()
    }
    
    /// 获取格式描述
    #[wasm_bindgen(js_name = getDescription)]
    pub fn get_description(&self) -> String {
        self.description.clone()
    }
    
    /// 获取文件扩展名
    #[wasm_bindgen(js_name = getExtensions)]
    pub fn get_extensions(&self) -> Vec<String> {
        self.extensions.clone()
    }
    
    /// 获取 MIME 类型
    #[wasm_bindgen(js_name = getMimeType)]
    pub fn get_mime_type(&self) -> String {
        self.mime_type.clone()
    }
    
    /// 是否支持有损压缩
    #[wasm_bindgen(js_name = supportsLossy)]
    pub fn supports_lossy(&self) -> bool {
        self.supports_lossy
    }
    
    /// 是否支持透明度
    #[wasm_bindgen(js_name = supportsTransparency)]
    pub fn supports_transparency(&self) -> bool {
        self.supports_transparency
    }
    
    /// 是否支持动画
    #[wasm_bindgen(js_name = supportsAnimation)]
    pub fn supports_animation(&self) -> bool {
        self.supports_animation
    }
}

// =============================================================================
// 内部类型转换函数
// =============================================================================

/// 从 Rust ImageFormat 转换为 JavaScript JsImageFormat
pub(crate) fn to_js_image_format(format: rustimage_core::ImageFormat) -> JsImageFormat {
    match format {
        rustimage_core::ImageFormat::Jpeg => JsImageFormat::Jpeg,
        rustimage_core::ImageFormat::Png => JsImageFormat::Png,
        rustimage_core::ImageFormat::WebP => JsImageFormat::WebP,
        rustimage_core::ImageFormat::Avif => JsImageFormat::Avif,
        rustimage_core::ImageFormat::Bmp => JsImageFormat::Bmp,
        rustimage_core::ImageFormat::Tiff => JsImageFormat::Tiff,
        rustimage_core::ImageFormat::Gif => JsImageFormat::Gif,
        rustimage_core::ImageFormat::Ico => JsImageFormat::Ico,
    }
}

/// 从 JavaScript JsImageFormat 转换为 Rust ImageFormat
pub(crate) fn from_js_image_format(format: JsImageFormat) -> rustimage_core::ImageFormat {
    match format {
        JsImageFormat::Jpeg => rustimage_core::ImageFormat::Jpeg,
        JsImageFormat::Png => rustimage_core::ImageFormat::Png,
        JsImageFormat::WebP => rustimage_core::ImageFormat::WebP,
        JsImageFormat::Avif => rustimage_core::ImageFormat::Avif,
        JsImageFormat::Bmp => rustimage_core::ImageFormat::Bmp,
        JsImageFormat::Tiff => rustimage_core::ImageFormat::Tiff,
        JsImageFormat::Gif => rustimage_core::ImageFormat::Gif,
        JsImageFormat::Ico => rustimage_core::ImageFormat::Ico,
    }
}

/// 从 JavaScript JsConversionOptions 转换为 Rust ConversionOptions
pub(crate) fn from_js_conversion_options(js_options: &JsConversionOptions) -> rustimage_core::ConversionOptions {
    let mut builder = rustimage_core::ConversionOptionsBuilder::new();
    
    if let Some(quality) = js_options.quality {
        builder = builder.quality(quality);
    }
    
    if let Some(level) = js_options.compression_level {
        builder = builder.compression_level(level);
    }
    
    if let Some(progressive) = js_options.progressive {
        builder = builder.progressive(progressive);
    }
    
    builder = builder
        .preserve_dimensions(js_options.preserve_dimensions)
        .preserve_color_space(js_options.preserve_color_space)
        .preserve_metadata(js_options.preserve_metadata);
    
    // 转换自定义参数
    let keys = js_sys::Object::keys(&js_options.custom_params);
    for i in 0..keys.length() {
        if let Some(key) = keys.get(i).as_string() {
            if let Some(value) = js_options.custom_params.get(&JsValue::from_str(&key)).as_string() {
                builder = builder.custom_param(key, value);
            }
        }
    }
    
    builder.build()
}

/// 从 Rust ConvertedImage 转换为 JavaScript JsConvertedImage
pub(crate) fn to_js_converted_image(rust_image: rustimage_core::ConvertedImage) -> JsConvertedImage {
    JsConvertedImage {
        data: rust_image.data().to_vec(),
        width: rust_image.dimensions().width,
        height: rust_image.dimensions().height,
        format: to_js_image_format(rust_image.format()),
        conversion_time_ms: rust_image.conversion_time_ms(),
        original_size: rust_image.original_size(),
        compression_ratio: rust_image.compression_ratio(),
    }
}

/// 从 Rust PerformanceMetrics 转换为 JavaScript JsPerformanceMetrics
pub(crate) fn to_js_performance_metrics(rust_metrics: rustimage_core::PerformanceMetrics) -> JsPerformanceMetrics {
    JsPerformanceMetrics {
        total_time_ms: rust_metrics.timing.total_time_ms,
        decode_time_ms: rust_metrics.timing.decode_time_ms,
        encode_time_ms: rust_metrics.timing.encode_time_ms,
        memory_usage_bytes: rust_metrics.memory.peak_memory_bytes,
        cpu_usage_percent: rust_metrics.system.cpu_usage_percent,
        throughput_images_per_second: rust_metrics.throughput.images_per_second,
    }
}

/// 从 Rust FormatInfo 转换为 JavaScript JsFormatInfo
pub(crate) fn to_js_format_info(rust_info: rustimage_core::FormatInfo) -> JsFormatInfo {
    JsFormatInfo {
        name: rust_info.name.to_string(),
        description: rust_info.description.to_string(),
        extensions: rust_info.extensions.iter().map(|s| s.to_string()).collect(),
        mime_type: rust_info.mime_type.to_string(),
        supports_lossy: rust_info.capabilities.supports_lossy(),
        supports_transparency: rust_info.capabilities.supports_transparency(),
        supports_animation: rust_info.capabilities.supports_animation(),
    }
}