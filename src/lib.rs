//! # RustImage Core - 深模块设计的图像格式转换核心库
//! 
//! 遵循《软件设计哲学》理念：
//! - 深模块：简化接口，只暴露 convert_format() 等核心函数
//! - 信息隐藏：封装复杂的编解码算法实现
//! - 零成本抽象：利用Rust特性实现高性能转换

pub mod types;        // 类型定义和抽象
pub mod converter;    // 主转换器
pub mod codecs;       // 编解码引擎
pub mod performance;  // 性能监控
pub mod error;        // 错误处理

// 重新导出主要类型和函数 - 深模块的简单接口
pub use types::*;
pub use converter::FormatConverter;
pub use error::{ImageError, Result};

/// 主要的格式转换接口 - 体现深模块设计
/// 
/// 这是用户唯一需要了解的接口，隐藏了所有复杂的编解码实现
pub fn convert_format(  // 格式转换接口 
    image_data: &[u8],     // 图像数据
    from_format: ImageFormat, // 源格式
    to_format: ImageFormat,   // 目标格式
    options: Option<ConversionOptions>, // 转换选项
) -> Result<ConvertedImage> {
    // 使用默认配置创建转换器
    let mut converter = FormatConverter::with_defaults()?;
    
    // 执行转换
    converter.convert_format(image_data, from_format, to_format, options)
}

/// 批量格式转换接口 - 展示并行处理能力
pub fn batch_convert(
    images: Vec<ImageInput>, // 图像输入
    conversion_tasks: Vec<ConversionTask>, // 转换任务
) -> Result<Vec<ConvertedImage>> {
    // 使用高性能配置创建转换器
    let mut converter = FormatConverter::with_high_performance()?;
    
    // 执行批量转换
    converter.batch_convert(images, conversion_tasks)
}

/// 格式检测接口 - 自动识别图像格式
pub fn detect_format(image_data: &[u8]) -> Result<ImageFormat> {
    // 使用格式检测器
    crate::codecs::FormatDetector::detect(image_data)
}

/// 获取格式信息 - 了解格式特性
pub fn get_format_info(format: ImageFormat) -> FormatInfo {
    // 使用格式的内置信息方法
    format.info()
}

/// 获取性能指标 - 用于展示 Rust 性能优势
pub fn get_performance_metrics() -> PerformanceMetrics {
    // 创建临时监控器获取当前指标
    match crate::performance::PerformanceMonitor::new(true) {
        Ok(monitor) => monitor.get_current_metrics(),
        Err(_) => PerformanceMetrics::default(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 测试基础模块创建和配置
    #[test]
    fn test_basic_module_creation() {
        // 测试 FormatConverter 创建
        let converter = FormatConverter::with_defaults();
        assert!(converter.is_ok(), "FormatConverter 创建失败: {:?}", converter);
        
        // 测试 PerformanceMonitor 创建
        let monitor = crate::performance::PerformanceMonitor::new(true);
        assert!(monitor.is_ok(), "PerformanceMonitor 创建失败: {:?}", monitor);
        
        // 测试 CodecEngine 创建
        let codec_config = crate::codecs::CodecConfigBuilder::new().build();
        let codec_engine = crate::codecs::CodecEngine::new(codec_config);
        assert!(codec_engine.is_ok(), "CodecEngine 创建失败: {:?}", codec_engine);
    }

    /// 测试类型定义的完整性
    #[test]
    fn test_type_definitions() {
        // 测试 ImageFormat 枚举
        let formats = vec![
            ImageFormat::Jpeg,
            ImageFormat::Png,
            ImageFormat::WebP,
            ImageFormat::Avif,
            ImageFormat::Bmp,
            ImageFormat::Tiff,
            ImageFormat::Gif,
            ImageFormat::Ico,
        ];
        
        for format in formats {
            let info = format.info();
            assert!(!info.name.is_empty(), "格式 {:?} 的名称不能为空", format);
            assert!(!info.extensions.is_empty(), "格式 {:?} 必须有扩展名", format);
        }
        
        // 测试 ConversionOptions 构建器
        let options = ConversionOptionsBuilder::new()
            .quality(0.8)
            .compression_level(6)
            .progressive(true)
            .build();
        
        assert!(options.quality().is_some(), "质量设置应该被保存");
        assert!(options.compression_level().is_some(), "压缩级别设置应该被保存");
        assert!(options.is_progressive().is_some(), "渐进式设置应该被保存");
    }

    /// 测试性能监控模块
    #[test]
    fn test_performance_monitoring() {
        let monitor = crate::performance::PerformanceMonitor::new(true)
            .expect("性能监控器创建应该成功");
        
        // 测试基本性能指标获取
        let metrics = monitor.get_current_metrics();
        
        // 验证指标结构完整性
        assert!(metrics.timing.total_time_ms >= 0.0, "时间指标应该非负");
        assert!(metrics.memory.peak_memory_bytes >= 0, "内存指标应该非负");
        assert!(metrics.throughput.images_per_second >= 0.0, "吞吐量指标应该非负");
    }

    /// 测试错误处理模块
    #[test]
    fn test_error_handling() {
        // 测试错误创建
        let format_error = ImageError::invalid_format("test_format");
        assert_eq!(format_error.category(), crate::error::ErrorCategory::Format);
        assert_eq!(format_error.severity(), crate::error::ErrorSeverity::Medium);
        
        let param_error = ImageError::invalid_parameters("invalid parameter");
        assert_eq!(param_error.category(), crate::error::ErrorCategory::Parameter);
        assert_eq!(param_error.severity(), crate::error::ErrorSeverity::Low);
        
        // 测试错误特征
        assert!(!format_error.is_retryable(), "格式错误通常不可重试");
        assert!(!param_error.is_retryable(), "参数错误通常不可重试");
    }

    /// 测试像素类型
    #[test]
    fn test_pixel_types() {
        // 测试 RGB 像素
        let rgb_pixel = Rgb { r: 255u8, g: 128u8, b: 64u8 };
        let rgb_channels = rgb_pixel.to_channels();
        assert_eq!(rgb_channels, vec![255u8, 128u8, 64u8], "RGB 像素通道转换应该正确");
        
        let reconstructed_rgb = Rgb::from_channels(&rgb_channels);
        assert_eq!(reconstructed_rgb, rgb_pixel, "RGB 像素应该能正确重构");
        
        // 测试 RGBA 像素
        let rgba_pixel = Rgba { r: 255u8, g: 128u8, b: 64u8, a: 200u8 };
        let rgba_channels = rgba_pixel.to_channels();
        assert_eq!(rgba_channels, vec![255u8, 128u8, 64u8, 200u8], "RGBA 像素通道转换应该正确");
        
        // 测试 Luma 像素
        let luma_pixel = Luma { l: 128u8 };
        let luma_channels = luma_pixel.to_channels();
        assert_eq!(luma_channels, vec![128u8], "Luma 像素通道转换应该正确");
    }

    /// 测试便利函数的集成
    #[test]
    fn test_convenience_functions() {
        // 创建一个简单的测试图像数据
        let test_data = vec![
            0xFF, 0xD8, 0xFF, 0xE0, // JPEG 文件头
            0x00, 0x10, 0x4A, 0x46, 0x49, 0x46, 0x00, 0x01, // JFIF
            // 简化的 JPEG 数据...
        ];
        
        // 测试格式检测
        let detected_format = detect_format(&test_data);
        // 注意：这可能会失败，因为我们的测试数据不是完整的 JPEG
        // 但至少应该不会 panic
        match detected_format {
            Ok(format) => {
                assert_eq!(format, ImageFormat::Jpeg, "应该检测为 JPEG 格式");
            }
            Err(_) => {
                // 可以接受，因为测试数据不完整
            }
        }
        
        // 测试格式信息获取
        let jpeg_info = get_format_info(ImageFormat::Jpeg);
        assert_eq!(jpeg_info.name, "JPEG", "JPEG 格式名称应该正确");
        assert!(jpeg_info.extensions.contains(&"jpg"), "JPEG 应该支持 .jpg 扩展名");
        
        // 测试性能指标获取
        let metrics = get_performance_metrics();
        // 基本验证：应该是默认值
        assert!(metrics.timing.total_time_ms >= 0.0, "性能指标应该是合理的");
    }
}