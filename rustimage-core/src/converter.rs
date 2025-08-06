//! 格式转换器 - 深模块设计的核心实现
//!
//! 本模块遵循《软件设计哲学》的核心理念：
//! - **深模块设计**：极简的转换接口，隐藏复杂的多格式转换逻辑
//! - **信息隐藏**：封装编解码引擎、性能监控、质量评估等实现细节
//! - **分层架构**：Converter -> CodecEngine -> 具体Codec -> 底层算法
//! - **零成本抽象**：编译时优化和类型特化

use crate::{
    error::{ImageError, Result},
    types::*,
    codecs::{CodecEngine, CodecConfigBuilder},
    performance::PerformanceMonitor,
};
use rayon::prelude::*;
use std::sync::{Arc, Mutex};
use std::time::Instant;

// 外部依赖的简化实现
fn num_cpus_get() -> usize {
    // 简化实现，实际项目中应该使用 num_cpus crate
    std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(4)
}

// =============================================================================
// 公共API - 深模块的极简接口
// =============================================================================

/// 格式转换器 - 对外的统一转换入口
/// 
/// 这是一个"深模块"：提供简单的转换接口，
/// 内部管理复杂的编解码、性能监控、质量评估等逻辑
pub struct FormatConverter {
    // 私有字段：完全隐藏实现细节
    codec_engine: CodecEngine,                    // 编解码引擎
    performance_monitor: PerformanceMonitor,      // 性能监控器
    config: ConverterConfig,                      // 转换器配置
    conversion_stats: Arc<Mutex<ConversionStats>>, // 转换统计
}

/// 转换器配置 - 使用构建器模式简化复杂配置
#[derive(Debug, Clone)]
pub struct ConverterConfig {
    // 核心配置
    /// 是否启用并行处理
    pub enable_parallel: bool,
    /// 线程池大小
    pub thread_pool_size: Option<usize>,
    /// 是否启用SIMD优化
    pub enable_simd: bool,
    /// 内存限制（字节）
    pub memory_limit: Option<u64>,
    
    // 监控和质量
    /// 是否启用性能监控
    pub enable_performance_monitoring: bool,
    /// 是否启用质量评估
    pub enable_quality_assessment: bool,
    /// 是否启用详细日志
    pub enable_detailed_logging: bool,
    
    // 转换策略
    /// 默认质量策略
    pub default_quality_strategy: QualityStrategy,
    /// 批处理大小
    pub batch_size: usize,
    /// 是否自动优化格式选择
    pub auto_format_optimization: bool,
}

/// 质量策略
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QualityStrategy {
    /// 保持原始质量
    PreserveOriginal,
    /// 优化文件大小
    OptimizeSize,
    /// 平衡质量和大小
    Balanced,
    /// 最高质量
    MaxQuality,
}

/// 转换器配置构建器
#[derive(Debug, Clone)]
pub struct ConverterConfigBuilder {
    config: ConverterConfig,
}

// =============================================================================
// 内部类型 - 信息隐藏的体现
// =============================================================================

/// 转换统计 - 私有：用于内部监控和优化
#[derive(Debug, Default)]
struct ConversionStats {  
    /// 总转换次数
    total_conversions: u64,
    /// 成功转换次数
    successful_conversions: u64,
    /// 失败转换次数
    failed_conversions: u64,
    /// 总处理时间（毫秒）
    total_processing_time_ms: f64,
    /// 总输入大小（字节）
    total_input_bytes: u64,
    /// 总输出大小（字节）
    total_output_bytes: u64,
    /// 格式转换次数统计
    format_usage: std::collections::HashMap<(ImageFormat, ImageFormat), u64>,
}

/// 转换上下文 - 私有：单次转换的内部状态
struct ConversionContext { 
    /// 开始时间
    start_time: Instant,
    /// 输入格式
    from_format: ImageFormat,
    /// 输出格式  
    to_format: ImageFormat,
    /// 输入大小
    input_size: u64,
    /// 转换选项
    options: ConversionOptions,
    /// 是否启用监控
    enable_monitoring: bool,
}

/// 批处理上下文 - 私有：批量转换的协调器
struct BatchContext {
    /// 总任务数
    total_tasks: usize,
    /// 完成任务数
    completed_tasks: Arc<Mutex<usize>>,
    /// 开始时间
    start_time: Instant,
    /// 是否启用并行
    parallel_enabled: bool,
}

// =============================================================================
// 公共实现 - 深模块接口的核心实现
// =============================================================================

impl FormatConverter {
    /// 创建新的格式转换器 - 主要构造函数
    pub fn new(config: ConverterConfig) -> Result<Self> {
        // 1. 创建编解码引擎配置
        let mut codec_config_builder = CodecConfigBuilder::new()
            .parallel(config.enable_parallel)
            .simd(config.enable_simd);
        
        if let Some(size) = config.thread_pool_size {
            codec_config_builder = codec_config_builder.thread_pool_size(size);
        }
        
        if let Some(limit) = config.memory_limit {
            codec_config_builder = codec_config_builder.memory_limit(limit);
        }
        
        let codec_config = codec_config_builder.build();
        
        // 2. 创建编解码引擎
        let codec_engine = CodecEngine::new(codec_config)?;
        
        // 3. 创建性能监控器
        let performance_monitor = PerformanceMonitor::new(
            config.enable_performance_monitoring
        )?;
        
        // 4. 初始化统计
        let conversion_stats = Arc::new(Mutex::new(ConversionStats::default()));
        
        Ok(Self {
            codec_engine,
            performance_monitor,
            config,
            conversion_stats,
        })
    }
    
    /// 使用默认配置创建转换器
    pub fn with_defaults() -> Result<Self> {
        Self::new(ConverterConfig::default())
    }
    
    /// 使用高性能配置创建转换器
    pub fn with_high_performance() -> Result<Self> {
        let config = ConverterConfigBuilder::new()
            .enable_parallel(true)
            .enable_simd(true)
            .thread_pool_size(num_cpus_get())
            .enable_performance_monitoring(true)
            .quality_strategy(QualityStrategy::Balanced)
            .batch_size(64)
            .build();
        
        Self::new(config)
    }
    
    /// 使用高质量配置创建转换器
    pub fn with_high_quality() -> Result<Self> {
        let config = ConverterConfigBuilder::new()
            .enable_parallel(false) // 质量优先，禁用并行避免竞争
            .enable_simd(true)
            .enable_quality_assessment(true)
            .quality_strategy(QualityStrategy::MaxQuality)
            .build();
        
        Self::new(config)
    }
    
    /// 转换图像格式 - 深模块的主要接口
    pub fn convert_format(
        &mut self,
        image_data: &[u8],
        from_format: ImageFormat,
        to_format: ImageFormat,
        options: Option<ConversionOptions>,
    ) -> Result<ConvertedImage> {
        // 1. 创建转换上下文
        let context = ConversionContext {
            start_time: Instant::now(),
            from_format,
            to_format,
            input_size: image_data.len() as u64,
            options: options.unwrap_or_else(|| self.get_default_options(from_format, to_format)),
            enable_monitoring: self.config.enable_performance_monitoring,
        };
        
        // 2. 验证转换请求
        self.validate_conversion_request(&context)?;
        
        // 3. 执行转换
        let result = self.execute_conversion(image_data, &context);
        
        // 4. 更新统计
        self.update_conversion_stats(&context, &result);
        
        result
    }
    
    /// 批量转换图像格式
    pub fn batch_convert(
        &mut self,
        images: Vec<ImageInput>,
        conversion_tasks: Vec<ConversionTask>,
    ) -> Result<Vec<ConvertedImage>> {
        if images.len() != conversion_tasks.len() {
            return Err(ImageError::InvalidParameters {
                details: format!(
                    "Images count ({}) does not match tasks count ({})",
                    images.len(),
                    conversion_tasks.len()
                ),
            });
        }
        
        if images.is_empty() {
            return Ok(Vec::new());
        }
        
        // 执行批量转换
        let results = if self.config.enable_parallel {
            self.execute_parallel_batch(images, conversion_tasks)?
        } else {
            self.execute_sequential_batch(images, conversion_tasks)?
        };
        
        // 聚合结果
        self.aggregate_batch_results(results)
    }
    
    /// 检测图像格式
    pub fn detect_format(&self, image_data: &[u8]) -> Result<ImageFormat> {
        self.codec_engine.detect_format(image_data)
    }
    
    /// 获取格式信息
    pub fn get_format_info(&self, format: ImageFormat) -> FormatInfo {
        format.info()
    }
    
    /// 获取支持的格式转换
    pub fn get_supported_formats(&self) -> Vec<ImageFormat> {
        self.codec_engine.supported_formats()
    }
    
    /// 获取转换统计
    pub fn get_conversion_statistics(&self) -> ConversionStatistics {
        let stats = self.conversion_stats.lock().unwrap();
        let performance_metrics = self.performance_monitor.get_current_metrics();
        
        ConversionStatistics {
            total_conversions: stats.total_conversions,
            successful_conversions: stats.successful_conversions,
            success_rate: if stats.total_conversions > 0 {
                stats.successful_conversions as f64 / stats.total_conversions as f64
            } else {
                0.0
            },
            average_processing_time_ms: if stats.successful_conversions > 0 {
                stats.total_processing_time_ms / stats.successful_conversions as f64
            } else {
                0.0
            },
            total_bytes_processed: stats.total_input_bytes,
            compression_ratio: if stats.total_input_bytes > 0 {
                stats.total_output_bytes as f64 / stats.total_input_bytes as f64
            } else {
                1.0
            },
            most_used_conversion: self.get_most_used_conversion(&stats),
            performance_metrics,
        }
    }
}

// =============================================================================
// 私有实现方法 - 信息隐藏的核心
// =============================================================================

impl FormatConverter {
    /// 验证转换请求 - 私有方法
    fn validate_conversion_request(&self, context: &ConversionContext) -> Result<()> {
        if !self.codec_engine.supports_conversion(context.from_format, context.to_format) {
            return Err(ImageError::UnsupportedOperation {
                operation: format!(
                    "Conversion from {:?} to {:?} is not supported",
                    context.from_format, context.to_format
                ),
            });
        }
        
        if context.input_size == 0 {
            return Err(ImageError::InvalidParameters {
                details: "Empty input data".to_string(),
            });
        }
        
        Ok(())
    }
    
    /// 执行单次转换 - 核心转换逻辑
    fn execute_conversion(
        &mut self,
        image_data: &[u8],
        context: &ConversionContext,
    ) -> Result<ConvertedImage> {
        let start_time = Instant::now();
        
        // 开始性能监控
        if context.enable_monitoring {
            self.performance_monitor.start_conversion(&context.from_format, &context.to_format);
        }
        
        let result = (|| -> Result<ConvertedImage> {
            // 1. 解码输入图像
            let image_buffer = self.codec_engine.decode::<Rgba8>(image_data, context.from_format)?;
            
            // 2. 编码为目标格式
            let output_data = self.codec_engine.encode(
                &image_buffer,
                context.to_format,
                &context.options,
            )?;
            
            // 3. 计算指标
            let conversion_time_ms = context.start_time.elapsed().as_secs_f64() * 1000.0;
            
            // 4. 构建结果
            Ok(ConvertedImage::new(
                output_data,
                image_buffer.dimensions(),
                context.to_format,
                conversion_time_ms,
                context.input_size,
            ))
        })();
        
        // 结束性能监控
        if context.enable_monitoring {
            let duration = start_time.elapsed();
            let success = result.is_ok();
            self.performance_monitor.end_conversion(duration, success);
        }
        
        result
    }
    
    /// 执行并行批处理
    fn execute_parallel_batch(
        &mut self,
        images: Vec<ImageInput>,
        tasks: Vec<ConversionTask>,
    ) -> Result<Vec<Result<ConvertedImage>>> {
        let results: Vec<Result<ConvertedImage>> = images
            .into_par_iter()
            .zip(tasks.into_par_iter())
            .map(|(image, task)| {
                let mut local_converter = FormatConverter::new(self.config.clone())?;
                local_converter.convert_format(
                    image.data(),
                    task.from_format,
                    task.to_format,
                    task.options,
                )
            })
            .collect();
        
        Ok(results)
    }
    
    /// 执行顺序批处理
    fn execute_sequential_batch(
        &mut self,
        images: Vec<ImageInput>,
        tasks: Vec<ConversionTask>,
    ) -> Result<Vec<Result<ConvertedImage>>> {
        let mut results = Vec::with_capacity(images.len());
        
        for (image, task) in images.into_iter().zip(tasks.into_iter()) {
            let result = self.convert_format(
                image.data(),
                task.from_format,
                task.to_format,
                task.options,  
            );
            results.push(result);
        }
        
        Ok(results)
    }
    
    /// 聚合批处理结果
    fn aggregate_batch_results(
        &self,
        results: Vec<Result<ConvertedImage>>,
    ) -> Result<Vec<ConvertedImage>> {
        let mut converted_images = Vec::new();
        let mut errors = Vec::new();
        
        for (index, result) in results.into_iter().enumerate() {
            match result {
                Ok(image) => converted_images.push(image),
                Err(error) => errors.push((index, error)),
            }
        }
        
        if !errors.is_empty() {
            return Err(ImageError::BatchProcessingFailed {
                successful_count: converted_images.len(),
                failed_count: errors.len(),
                first_error: Box::new(errors.into_iter().next().unwrap().1),
            });
        }
        
        Ok(converted_images)
    }
    
    /// 获取默认选项
    fn get_default_options(
        &self,
        _from_format: ImageFormat,
        _to_format: ImageFormat,
    ) -> ConversionOptions {
        match self.config.default_quality_strategy {
            QualityStrategy::PreserveOriginal => ConversionOptionsBuilder::new().quality(0.95).build(),
            QualityStrategy::OptimizeSize => ConversionOptionsBuilder::new().quality(0.75).compression_level(9).build(),
            QualityStrategy::Balanced => ConversionOptionsBuilder::new().quality(0.85).compression_level(6).build(),
            QualityStrategy::MaxQuality => ConversionOptionsBuilder::new().quality(1.0).compression_level(0).build(),
        }
    }
    
    fn update_conversion_stats(&self, context: &ConversionContext, result: &Result<ConvertedImage>) {
        let mut stats = self.conversion_stats.lock().unwrap();
        
        stats.total_conversions += 1;
        stats.total_input_bytes += context.input_size;
        
        match result {
            Ok(converted) => {
                stats.successful_conversions += 1;
                stats.total_processing_time_ms += context.start_time.elapsed().as_secs_f64() * 1000.0;
                stats.total_output_bytes += converted.data().len() as u64;
            }
            Err(_) => {
                stats.failed_conversions += 1;
            }
        }
        
        let format_pair = (context.from_format, context.to_format);
        *stats.format_usage.entry(format_pair).or_insert(0) += 1;
    }
    
    fn get_most_used_conversion(&self, stats: &ConversionStats) -> Option<(ImageFormat, ImageFormat)> {
        stats.format_usage
            .iter()
            .max_by_key(|(_, count)| *count)
            .map(|(formats, _)| *formats)
    }
}

// =============================================================================
// 配置构建器实现
// =============================================================================

impl ConverterConfigBuilder {
    pub fn new() -> Self {
        Self {
            config: ConverterConfig::default(),
        }
    }
    
    pub fn enable_parallel(mut self, enabled: bool) -> Self {
        self.config.enable_parallel = enabled;
        self
    }
    
    pub fn thread_pool_size(mut self, size: usize) -> Self {
        self.config.thread_pool_size = Some(size);
        self
    }
    
    pub fn enable_simd(mut self, enabled: bool) -> Self {
        self.config.enable_simd = enabled;
        self
    }
    
    pub fn memory_limit(mut self, limit: u64) -> Self {
        self.config.memory_limit = Some(limit);
        self
    }
    
    pub fn enable_performance_monitoring(mut self, enabled: bool) -> Self {
        self.config.enable_performance_monitoring = enabled;
        self
    }
    
    pub fn enable_quality_assessment(mut self, enabled: bool) -> Self {
        self.config.enable_quality_assessment = enabled;
        self
    }
    
    pub fn quality_strategy(mut self, strategy: QualityStrategy) -> Self {
        self.config.default_quality_strategy = strategy;
        self
    }
    
    pub fn batch_size(mut self, size: usize) -> Self {
        self.config.batch_size = size;
        self
    }
    
    pub fn build(self) -> ConverterConfig {
        self.config
    }
}

impl Default for ConverterConfigBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for ConverterConfig {
    fn default() -> Self {
        Self {
            enable_parallel: true,
            thread_pool_size: None,
            enable_simd: true,
            memory_limit: None,
            enable_performance_monitoring: false,
            enable_quality_assessment: false,
            enable_detailed_logging: false,
            default_quality_strategy: QualityStrategy::Balanced,
            batch_size: 32,
            auto_format_optimization: false,
        }
    }
}

// =============================================================================
// 辅助类型定义
// =============================================================================

/// 优化目标
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OptimizationTarget {
    Speed,
    Size,
    Quality,
    Balanced,
}

/// 转换统计信息
#[derive(Debug, Clone)]
pub struct ConversionStatistics {
    pub total_conversions: u64,
    pub successful_conversions: u64,
    pub success_rate: f64,
    pub average_processing_time_ms: f64,
    pub total_bytes_processed: u64,
    pub compression_ratio: f64,
    pub most_used_conversion: Option<(ImageFormat, ImageFormat)>,
    pub performance_metrics: PerformanceMetrics,
}