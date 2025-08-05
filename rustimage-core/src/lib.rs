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