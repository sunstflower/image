//! 错误处理系统 - 深模块设计的统一错误管理
//!
//! 本模块遵循《软件设计哲学》的核心理念：
//! - **深模块设计**：简单的错误接口，隐藏复杂的错误处理逻辑
//! - **信息隐藏**：统一错误类型，封装底层库的具体错误细节
//! - **分层架构**：错误分类 -> 错误上下文 -> 错误链 -> 底层错误
//! - **零成本抽象**：编译时错误转换优化

use thiserror::Error;
use std::error::Error;

// =============================================================================
// 公共错误API - 深模块的统一错误接口
// =============================================================================

/// 统一的图像处理错误类型 - 对外的简单错误接口
/// 
/// 这是一个"深"的错误枚举：提供清晰的错误分类，
/// 内部隐藏了复杂的错误转换、错误链、错误上下文等逻辑
#[derive(Error, Debug)]
pub enum ImageError {
    // =========================================================================
    // 格式相关错误 - 图像格式处理错误
    // =========================================================================
    
    /// 无效的图像格式
    #[error("Invalid image format: {format}")]
    InvalidFormat { 
        /// 格式描述信息
        format: String 
    },
    
    /// 不支持的图像格式
    #[error("Unsupported image format: {format} (supported formats: {supported:?})")]
    UnsupportedFormat { 
        /// 请求的格式
        format: String,
        /// 支持的格式列表
        supported: Vec<String>,
    },
    
    /// 格式检测失败
    #[error("Format detection failed: {reason}")]
    FormatDetectionFailed { 
        /// 失败原因
        reason: String 
    },
    
    // =========================================================================
    // 编解码错误 - 编解码器处理错误
    // =========================================================================
    
    /// 图像解码失败
    #[error("Image decode failed for format {format}: {message}")]
    DecodeError { 
        /// 图像格式
        format: String,
        /// 错误消息
        message: String,
        /// 底层错误（可选）
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },
    
    /// 图像编码失败
    #[error("Image encode failed for format {format}: {message}")]
    EncodeError { 
        /// 图像格式
        format: String,
        /// 错误消息
        message: String,
        /// 底层错误（可选）
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },
    
    /// 编解码器初始化失败
    #[error("Codec initialization failed for {format}: {details}")]
    CodecInitializationFailed {
        /// 格式名称
        format: String,
        /// 详细信息
        details: String,
    },
    
    // =========================================================================
    // 参数和配置错误 - 用户输入错误
    // =========================================================================
    
    /// 无效参数
    #[error("Invalid parameters: {details}")]
    InvalidParameters { 
        /// 参数详细信息
        details: String 
    },
    
    /// 无效的图像尺寸
    #[error("Invalid image dimensions: {width}×{height} (reason: {reason})")]
    InvalidDimensions {
        /// 图像宽度
        width: u32,
        /// 图像高度
        height: u32,
        /// 无效原因
        reason: String,
    },
    
    /// 无效的像素格式
    #[error("Invalid pixel format: expected {expected}, got {actual}")]
    InvalidPixelFormat {
        /// 期望的格式
        expected: String,
        /// 实际的格式
        actual: String,
    },
    
    /// 配置错误
    #[error("Configuration error: {setting} = {value} ({reason})")]
    ConfigurationError {
        /// 配置项名称
        setting: String,
        /// 配置值
        value: String,
        /// 错误原因
        reason: String,
    },
    
    // =========================================================================
    // 资源和系统错误 - 系统资源相关错误
    // =========================================================================
    
    /// 内存分配失败
    #[error("Memory allocation failed: requested {requested} bytes, available {available} bytes")]
    MemoryError { 
        /// 请求的字节数
        requested: u64,
        /// 可用的字节数
        available: u64,
    },
    
    /// 资源耗尽
    #[error("Resource exhausted: {resource}")]
    ResourceExhausted { 
        /// 资源描述
        resource: String 
    },
    
    /// 系统限制超出
    #[error("System limit exceeded: {limit_type} (current: {current}, limit: {limit})")]
    SystemLimitExceeded {
        /// 限制类型
        limit_type: String,
        /// 当前值
        current: u64,
        /// 限制值
        limit: u64,
    },
    
    // =========================================================================
    // 操作相关错误 - 操作执行错误
    // =========================================================================
    
    /// 不支持的操作
    #[error("Unsupported operation: {operation}")]
    UnsupportedOperation { 
        /// 操作描述
        operation: String 
    },
    
    /// 操作被取消
    #[error("Operation cancelled: {operation} (reason: {reason})")]
    OperationCancelled {
        /// 操作名称
        operation: String,
        /// 取消原因
        reason: String,
    },
    
    /// 操作超时
    #[error("Operation timeout: {operation} exceeded {timeout_ms}ms")]
    OperationTimeout {
        /// 操作名称
        operation: String,
        /// 超时时间（毫秒）
        timeout_ms: u64,
    },
    
    // =========================================================================
    // 批处理错误 - 批量操作错误
    // =========================================================================
    
    /// 批处理失败
    #[error("Batch processing failed: {successful_count} successful")]
    BatchProcessingFailed {
        /// 成功处理的数量
        successful_count: usize,
        /// 失败处理的数量
        failed_count: usize,
        /// 第一个错误（用于错误链）
        #[source]
        first_error: Box<ImageError>,
    },
    
    /// 并行处理错误
    #[error("Parallel processing error: {details}")]
    ParallelProcessingError {
        /// 错误详情
        details: String,
        /// 失败的任务索引
        failed_task_indices: Vec<usize>,
    },
    
    // =========================================================================
    // 性能和监控错误 - 性能监控相关错误
    // =========================================================================
    
    /// 性能监控错误
    #[error("Performance monitoring error: {details}")]
    PerformanceError { 
        /// 错误详情
        details: String 
    },
    
    /// 质量评估失败
    #[error("Quality assessment failed: {metric} calculation error - {reason}")]
    QualityAssessmentFailed {
        /// 质量指标名称
        metric: String,
        /// 失败原因
        reason: String,
    },
    
    // =========================================================================
    // I/O 和外部依赖错误 - 外部系统错误
    // =========================================================================
    
    /// I/O 错误
    #[error("I/O error: {operation}")]
    IoError {
        /// 操作描述
        operation: String,
        /// 底层I/O错误
        #[source]
        source: std::io::Error,
    },
    
    /// 外部库错误
    #[error("External library error in {library}: {message}")]
    ExternalLibraryError {
        /// 库名称
        library: String,
        /// 错误消息
        message: String,
        /// 底层错误（可选）
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },
    
    // =========================================================================
    // WASM 相关错误 - WebAssembly特定错误
    // =========================================================================
    
    /// WASM绑定错误
    #[error("WebAssembly binding error: {details}")]
    WasmBindingError {
        /// 错误详情
        details: String,
    },
    
    /// JavaScript互操作错误
    #[error("JavaScript interop error: {operation} failed - {reason}")]
    JsInteropError {
        /// 操作名称
        operation: String,
        /// 失败原因
        reason: String,
    },
}

/// 统一的结果类型 - 深模块的标准返回类型
pub type Result<T> = std::result::Result<T, ImageError>;

// =============================================================================
// 错误分类和优先级 - 错误管理的内部逻辑
// =============================================================================

/// 错误严重程度 - 用于错误分类和处理策略
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ErrorSeverity {
    /// 低级别：可以恢复的错误
    Low,
    /// 中级别：需要用户注意的错误
    Medium,
    /// 高级别：严重错误，可能影响系统稳定性
    High,
    /// 严重级别：系统级错误，需要立即处理
    Critical,
}

/// 错误类别 - 用于错误统计和分析
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ErrorCategory {
    /// 格式相关
    Format,
    /// 编解码相关
    Codec,
    /// 参数相关
    Parameter,
    /// 资源相关
    Resource,
    /// 操作相关
    Operation,
    /// 批处理相关
    Batch,
    /// 性能相关
    Performance,
    /// I/O相关
    Io,
    /// 系统相关
    System,
    /// WASM相关
    Wasm,
}

// =============================================================================
// 错误上下文和增强 - 提供更好的错误信息
// =============================================================================

/// 错误上下文特征 - 为结果添加上下文信息
pub trait ErrorContext<T> {
    /// 添加上下文信息
    fn with_context<F>(self, f: F) -> Result<T>
    where
        F: FnOnce() -> String;
    
    /// 添加格式转换上下文
    fn with_format_context(self, from: &str, to: &str) -> Result<T>;
    
    /// 添加操作上下文
    fn with_operation_context(self, operation: &str) -> Result<T>;
    
    /// 添加文件上下文
    fn with_file_context(self, filename: &str) -> Result<T>;
}

/// 错误链特征 - 用于构建错误链
pub trait ErrorChain {
    /// 获取错误链中的所有错误消息
    fn error_chain(&self) -> Vec<String>;
    
    /// 获取根错误原因
    fn root_cause(&self) -> &dyn std::error::Error;
    
    /// 检查错误链中是否包含特定类型的错误
    fn contains_error_type<E: std::error::Error + 'static>(&self) -> bool;
}

// =============================================================================
// 错误统计和分析 - 错误监控支持
// =============================================================================

/// 错误统计信息 - 用于错误分析和监控
#[derive(Debug, Clone, Default)]
pub struct ErrorStatistics {
    /// 按类别统计的错误次数
    pub error_counts_by_category: std::collections::HashMap<ErrorCategory, u64>,
    /// 按严重程度统计的错误次数
    pub error_counts_by_severity: std::collections::HashMap<ErrorSeverity, u64>,
    /// 总错误次数
    pub total_errors: u64,
    /// 最常见的错误类型
    pub most_common_errors: Vec<(String, u64)>,
    /// 错误趋势（最近的错误时间戳）
    pub recent_error_timestamps: Vec<std::time::SystemTime>,
}

/// 错误收集器 - 收集和分析错误信息
#[derive(Debug, Default)]
pub struct ErrorCollector {
    /// 统计信息
    statistics: std::sync::Arc<std::sync::Mutex<ErrorStatistics>>,
    /// 是否启用收集
    enabled: bool,
}

// =============================================================================
// 公共实现 - 错误处理的核心逻辑
// =============================================================================

impl ImageError {
    /// 获取错误的严重程度
    pub fn severity(&self) -> ErrorSeverity {
        match self {
            // 低级别错误
            ImageError::InvalidParameters { .. } 
            | ImageError::ConfigurationError { .. }
            | ImageError::PerformanceError { .. } => ErrorSeverity::Low,
            
            // 中级别错误
            ImageError::InvalidFormat { .. }
            | ImageError::UnsupportedFormat { .. }
            | ImageError::DecodeError { .. }
            | ImageError::EncodeError { .. }
            | ImageError::InvalidDimensions { .. }
            | ImageError::InvalidPixelFormat { .. }
            | ImageError::UnsupportedOperation { .. }
            | ImageError::OperationCancelled { .. }
            | ImageError::FormatDetectionFailed { .. }
            | ImageError::QualityAssessmentFailed { .. }
            | ImageError::WasmBindingError { .. }
            | ImageError::JsInteropError { .. } => ErrorSeverity::Medium,
            
            // 高级别错误  
            ImageError::MemoryError { .. }
            | ImageError::ResourceExhausted { .. }
            | ImageError::BatchProcessingFailed { .. }
            | ImageError::ParallelProcessingError { .. }
            | ImageError::OperationTimeout { .. }
            | ImageError::CodecInitializationFailed { .. }
            | ImageError::IoError { .. }
            | ImageError::ExternalLibraryError { .. } => ErrorSeverity::High,
            
            // 严重级别错误
            ImageError::SystemLimitExceeded { .. } => ErrorSeverity::Critical,
        }
    }
    
    /// 获取错误的类别
    pub fn category(&self) -> ErrorCategory {
        match self {
            ImageError::InvalidFormat { .. }
            | ImageError::UnsupportedFormat { .. }
            | ImageError::FormatDetectionFailed { .. } => ErrorCategory::Format,
            
            ImageError::DecodeError { .. }
            | ImageError::EncodeError { .. }
            | ImageError::CodecInitializationFailed { .. } => ErrorCategory::Codec,
            
            ImageError::InvalidParameters { .. }
            | ImageError::InvalidDimensions { .. }
            | ImageError::InvalidPixelFormat { .. }
            | ImageError::ConfigurationError { .. } => ErrorCategory::Parameter,
            
            ImageError::MemoryError { .. }
            | ImageError::ResourceExhausted { .. }
            | ImageError::SystemLimitExceeded { .. } => ErrorCategory::Resource,
            
            ImageError::UnsupportedOperation { .. }
            | ImageError::OperationCancelled { .. }
            | ImageError::OperationTimeout { .. } => ErrorCategory::Operation,
            
            ImageError::BatchProcessingFailed { .. }
            | ImageError::ParallelProcessingError { .. } => ErrorCategory::Batch,
            
            ImageError::PerformanceError { .. }
            | ImageError::QualityAssessmentFailed { .. } => ErrorCategory::Performance,
            
            ImageError::IoError { .. } => ErrorCategory::Io,
            
            ImageError::ExternalLibraryError { .. } => ErrorCategory::System,
            
            ImageError::WasmBindingError { .. }
            | ImageError::JsInteropError { .. } => ErrorCategory::Wasm,
        }
    }
    
    /// 检查错误是否可以重试
    pub fn is_retryable(&self) -> bool {
        match self {
            // 可重试的错误
            ImageError::OperationTimeout { .. }
            | ImageError::MemoryError { .. }
            | ImageError::ResourceExhausted { .. }
            | ImageError::IoError { .. }
            | ImageError::ExternalLibraryError { .. } => true,
            
            // 不可重试的错误
            ImageError::InvalidFormat { .. }
            | ImageError::UnsupportedFormat { .. }
            | ImageError::InvalidParameters { .. }
            | ImageError::InvalidDimensions { .. }
            | ImageError::InvalidPixelFormat { .. }
            | ImageError::UnsupportedOperation { .. }
            | ImageError::ConfigurationError { .. }
            | ImageError::OperationCancelled { .. } => false,
            
            // 部分可重试的错误（取决于具体情况）
            _ => false,
        }
    }
    
    /// 获取错误的建议解决方案
    pub fn suggested_solution(&self) -> Option<String> {
        match self {
            ImageError::InvalidFormat { .. } => {
                Some("请检查图像文件是否损坏，或尝试使用支持的图像格式".to_string())
            }
            ImageError::UnsupportedFormat { supported, .. } => {
                Some(format!("请使用支持的格式之一: {}", supported.join(", ")))
            }
            ImageError::MemoryError { requested, available } => {
                Some(format!(
                    "请减少图像尺寸或释放内存。当前需要 {}MB，可用 {}MB",
                    requested / 1024 / 1024,
                    available / 1024 / 1024
                ))
            }
            ImageError::InvalidParameters { .. } => {
                Some("请检查输入参数是否正确，参考文档了解正确的参数范围".to_string())
            }
            ImageError::OperationTimeout { .. } => {
                Some("操作超时，请尝试减少图像尺寸或增加超时时间".to_string())
            }
            _ => None,
        }
    }
    
    /// 创建格式错误
    pub fn invalid_format<S: Into<String>>(format: S) -> Self {
        Self::InvalidFormat {
            format: format.into(),
        }
    }
    
    /// 创建参数错误
    pub fn invalid_parameters<S: Into<String>>(details: S) -> Self {
        Self::InvalidParameters {
            details: details.into(),
        }
    }
    
    /// 创建不支持操作错误
    pub fn unsupported_operation<S: Into<String>>(operation: S) -> Self {
        Self::UnsupportedOperation {
            operation: operation.into(),
        }
    }
    
    /// 创建资源耗尽错误
    pub fn resource_exhausted<S: Into<String>>(resource: S) -> Self {
        Self::ResourceExhausted {
            resource: resource.into(),
        }
    }
}

// =============================================================================
// 错误上下文实现 - 增强错误信息
// =============================================================================

impl<T, E> ErrorContext<T> for std::result::Result<T, E>
where
    E: Into<ImageError>,
{
    fn with_context<F>(self, f: F) -> Result<T>
    where
        F: FnOnce() -> String,
    {
        self.map_err(|e| {
            let base_error = e.into();
            match base_error {
                ImageError::DecodeError { format, message, source } => {
                    ImageError::DecodeError {
                        format,
                        message: format!("{}: {}", f(), message),
                        source,
                    }
                }
                ImageError::EncodeError { format, message, source } => {
                    ImageError::EncodeError {
                        format,
                        message: format!("{}: {}", f(), message),
                        source,
                    }
                }
                ImageError::InvalidParameters { details } => {
                    ImageError::InvalidParameters {
                        details: format!("{}: {}", f(), details),
                    }
                }
                other => other,
            }
        })
    }
    
    fn with_format_context(self, from: &str, to: &str) -> Result<T> {
        self.with_context(|| format!("Converting from {} to {}", from, to))
    }
    
    fn with_operation_context(self, operation: &str) -> Result<T> {
        self.with_context(|| format!("During operation: {}", operation))
    }
    
    fn with_file_context(self, filename: &str) -> Result<T> {
        self.with_context(|| format!("Processing file: {}", filename))
    }
}

// =============================================================================
// 错误链实现 - 错误追踪支持
// =============================================================================

impl ErrorChain for ImageError {
    fn error_chain(&self) -> Vec<String> {
        let mut chain = vec![self.to_string()];
        let mut current = self.source();
        
        while let Some(err) = current {
            chain.push(err.to_string());
            current = err.source();
        }
        
        chain
    }
    
    fn root_cause(&self) -> &dyn std::error::Error {
        let mut current = self as &dyn std::error::Error;
        while let Some(source) = current.source() {
            current = source;
        }
        current
    }
    
    fn contains_error_type<E: std::error::Error + 'static>(&self) -> bool {
        let mut current = Some(self as &dyn std::error::Error);
        while let Some(err) = current {
            if err.downcast_ref::<E>().is_some() {
                return true;
            }
            current = err.source();
        }
        false
    }
}

// =============================================================================
// 错误收集器实现 - 错误监控支持
// =============================================================================

impl ErrorCollector {
    /// 创建新的错误收集器
    pub fn new(enabled: bool) -> Self {
        Self {
            statistics: std::sync::Arc::new(std::sync::Mutex::new(ErrorStatistics::default())),
            enabled,
        }
    }
    
    /// 记录错误
    pub fn record_error(&self, error: &ImageError) {
        if !self.enabled {
            return;
        }
        
        if let Ok(mut stats) = self.statistics.lock() {
            stats.total_errors += 1;
            
            // 按类别统计
            let category = error.category();
            *stats.error_counts_by_category.entry(category).or_insert(0) += 1;
            
            // 按严重程度统计
            let severity = error.severity();
            *stats.error_counts_by_severity.entry(severity).or_insert(0) += 1;
            
            // 记录时间戳
            stats.recent_error_timestamps.push(std::time::SystemTime::now());
            
            // 只保留最近100个时间戳
            if stats.recent_error_timestamps.len() > 100 {
                stats.recent_error_timestamps.remove(0);
            }
        }
    }
    
    /// 获取统计信息
    pub fn get_statistics(&self) -> ErrorStatistics {
        self.statistics.lock().unwrap().clone()
    }
    
    /// 重置统计信息
    pub fn reset(&self) {
        if let Ok(mut stats) = self.statistics.lock() {
            *stats = ErrorStatistics::default();
        }
    }
}

// =============================================================================
// 外部错误类型转换 - 统一错误接口
// =============================================================================

impl From<std::io::Error> for ImageError {
    fn from(err: std::io::Error) -> Self {
        Self::IoError {
            operation: "File I/O operation".to_string(),
            source: err,
        }
    }
}

impl From<std::fmt::Error> for ImageError {
    fn from(err: std::fmt::Error) -> Self {
        Self::ExternalLibraryError {
            library: "std::fmt".to_string(),
            message: err.to_string(),
            source: Some(Box::new(err)),
        }
    }
}

// 为WASM绑定提供的简化错误类型
#[cfg(target_arch = "wasm32")]
impl From<ImageError> for wasm_bindgen::JsValue {
    fn from(err: ImageError) -> Self {
        wasm_bindgen::JsValue::from_str(&err.to_string())
    }
}

// =============================================================================
// 便利宏 - 简化错误创建
// =============================================================================

/// 创建格式错误的便利宏
#[macro_export]
macro_rules! format_error {
    ($format:expr) => {
        $crate::error::ImageError::invalid_format($format)
    };
}

/// 创建参数错误的便利宏
#[macro_export]
macro_rules! param_error {
    ($details:expr) => {
        $crate::error::ImageError::invalid_parameters($details)
    };
    ($fmt:expr, $($arg:tt)*) => {
        $crate::error::ImageError::invalid_parameters(format!($fmt, $($arg)*))
    };
}

/// 创建不支持操作错误的便利宏
#[macro_export]
macro_rules! unsupported_error {
    ($operation:expr) => {
        $crate::error::ImageError::unsupported_operation($operation)
    };
    ($fmt:expr, $($arg:tt)*) => {
        $crate::error::ImageError::unsupported_operation(format!($fmt, $($arg)*))
    };
}

/// 确保结果带有上下文的便利宏
#[macro_export]
macro_rules! ensure_with_context {
    ($cond:expr, $ctx:expr, $err:expr) => {
        if !($cond) {
            return Err($err).with_context(|| $ctx.to_string());
        }
    };
}

// =============================================================================
// 测试工具 - 用于单元测试的错误验证
// =============================================================================

#[cfg(test)]
impl ImageError {
    /// 检查是否为特定的格式错误
    pub fn is_invalid_format(&self, expected_format: &str) -> bool {
        match self {
            ImageError::InvalidFormat { format } => format == expected_format,
            _ => false,
        }
    }
    
    /// 检查是否为特定的参数错误
    pub fn is_invalid_parameters(&self, expected_details: &str) -> bool {
        match self {
            ImageError::InvalidParameters { details } => details.contains(expected_details),
            _ => false,
        }
    }
    
    /// 检查是否为特定的操作错误
    pub fn is_unsupported_operation(&self, expected_operation: &str) -> bool {
        match self {
            ImageError::UnsupportedOperation { operation } => operation.contains(expected_operation),
            _ => false,
        }
    }
}