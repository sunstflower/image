//! 错误处理 - 信息隐藏和清晰的错误传播

use thiserror::Error;

/// 统一的错误类型 - 隐藏内部复杂性
#[derive(Error, Debug)]
pub enum ImageError {
    #[error("Invalid image format: {format}")]
    InvalidFormat { format: String },
    
    #[error("Image processing failed: {reason}")]
    ProcessingFailed { reason: String },
    
    #[error("Invalid parameters: {details}")]
    InvalidParameters { details: String },
    
    #[error("Memory allocation failed: required {bytes} bytes")]
    MemoryError { bytes: u64 },
    
    #[error("I/O error: {source}")]
    IoError {
        #[from]
        source: std::io::Error,
    },
    
    #[error("Image decode error: {message}")]
    DecodeError { message: String },
    
    #[error("Image encode error: {message}")]
    EncodeError { message: String },
    
    #[error("Unsupported operation: {operation}")]
    UnsupportedOperation { operation: String },
    
    #[error("Performance monitoring error: {details}")]
    PerformanceError { details: String },
}

/// 统一的结果类型
pub type Result<T> = std::result::Result<T, ImageError>;

/// 错误上下文扩展 - 提供更好的错误信息
pub trait ErrorContext<T> {
    fn with_context<F>(self, f: F) -> Result<T>
    where
        F: FnOnce() -> String;
}

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
                ImageError::ProcessingFailed { reason } => {
                    ImageError::ProcessingFailed {
                        reason: format!("{}: {}", f(), reason),
                    }
                }
                other => other,
            }
        })
    }
}