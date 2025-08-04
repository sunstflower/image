//! # RustImage Core - 深模块设计的图像处理核心库
//! 
//! 遵循《软件设计哲学》理念：
//! - 深模块：简单接口，复杂实现
//! - 信息隐藏：封装算法细节
//! - 零成本抽象：编译时优化

pub mod types;
pub mod processor;
pub mod filters;
pub mod performance;
pub mod error;

// 重新导出主要类型和函数 - 深模块的简单接口
pub use types::*;
pub use processor::ImageProcessor;
pub use error::{ImageError, Result};

/// 主要的对外接口 - 体现深模块设计
/// 
/// 这是用户唯一需要了解的接口，隐藏了所有复杂的内部实现
pub fn process_image(
    image_data: &[u8],
    filter_type: FilterType,
    params: Option<FilterParams>,
) -> Result<ProcessedImage> {
    todo!("Implementation will be added later")
}

/// 批处理接口 - 展示并行处理能力
pub fn batch_process(
    images: Vec<&[u8]>,
    operations: Vec<FilterOperation>,
) -> Result<Vec<ProcessedImage>> {
    todo!("Implementation will be added later")
}

/// 获取性能指标 - 用于展示 Rust 性能优势
pub fn get_performance_metrics() -> PerformanceMetrics {
    todo!("Implementation will be added later")
}