//! WebAssembly 绑定模块 - 深模块设计在 Web 环境中的体现

use wasm_bindgen::prelude::*;
use rustimage_core::{
    process_image as core_process_image,
    batch_process as core_batch_process,
    get_performance_metrics as core_get_performance_metrics,
    FilterType, FilterParams, ProcessedImage, PerformanceMetrics,
    ImageError, Result as CoreResult,
};
use web_sys::console;

// 设置 panic hook 以便在浏览器中调试
#[cfg(feature = "console_error_panic_hook")]
pub use console_error_panic_hook::set_once as set_panic_hook;

// 可选的内存分配器
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

/// 初始化 WASM 模块
#[wasm_bindgen(start)]
pub fn init() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
    
    console::log_1(&"RustImage WASM module initialized".into());
}

/// WebAssembly 友好的错误类型
#[wasm_bindgen]
pub struct WasmError {
    message: String,
}

#[wasm_bindgen]
impl WasmError {
    #[wasm_bindgen(getter)]
    pub fn message(&self) -> String {
        self.message.clone()
    }
}

impl From<ImageError> for WasmError {
    fn from(error: ImageError) -> Self {
        Self {
            message: error.to_string(),
        }
    }
}

/// WebAssembly 友好的处理结果
#[wasm_bindgen]
pub struct WasmProcessedImage {
    /// 处理后的图像数据
    #[wasm_bindgen(skip)]
    pub data: Vec<u8>,
    /// 图像宽度
    width: u32,
    /// 图像高度
    height: u32,
    /// 处理时间（毫秒）
    processing_time_ms: f64,
    /// 内存使用量
    memory_usage: u64,
}

#[wasm_bindgen]
impl WasmProcessedImage {
    /// 获取图像数据
    #[wasm_bindgen(getter)]
    pub fn data(&self) -> Vec<u8> {
        self.data.clone()
    }
    
    /// 获取宽度
    #[wasm_bindgen(getter)]
    pub fn width(&self) -> u32 {
        self.width
    }
    
    /// 获取高度
    #[wasm_bindgen(getter)]
    pub fn height(&self) -> u32 {
        self.height
    }
    
    /// 获取处理时间
    #[wasm_bindgen(getter = processingTimeMs)]
    pub fn processing_time_ms(&self) -> f64 {
        self.processing_time_ms
    }
    
    /// 获取内存使用量
    #[wasm_bindgen(getter = memoryUsage)]  
    pub fn memory_usage(&self) -> u64 {
        self.memory_usage
    }
}

impl From<ProcessedImage> for WasmProcessedImage {
    fn from(image: ProcessedImage) -> Self {
        Self {
            data: image.data,
            width: image.dimensions.width,
            height: image.dimensions.height,
            processing_time_ms: image.processing_time_ms,
            memory_usage: image.memory_usage,
        }
    }
}

/// WebAssembly 友好的性能指标
#[wasm_bindgen]
pub struct WasmPerformanceMetrics {
    /// 总处理时间
    total_time_ms: f64,
    /// 峰值内存使用
    peak_memory_bytes: u64,
    /// CPU 使用率
    cpu_usage: f32,
    /// 处理的像素数
    pixels_processed: u64,
    /// 每秒处理像素数
    pixels_per_second: f64,
    /// 使用的线程数
    threads_used: u32,
    /// 并行效率
    parallel_efficiency: f32,
    /// 是否使用了 SIMD
    simd_utilized: bool,
}

#[wasm_bindgen]
impl WasmPerformanceMetrics {
    #[wasm_bindgen(getter = totalTimeMs)]
    pub fn total_time_ms(&self) -> f64 {
        self.total_time_ms
    }
    
    #[wasm_bindgen(getter = peakMemoryBytes)]
    pub fn peak_memory_bytes(&self) -> u64 {
        self.peak_memory_bytes
    }
    
    #[wasm_bindgen(getter = cpuUsage)]
    pub fn cpu_usage(&self) -> f32 {
        self.cpu_usage
    }
    
    #[wasm_bindgen(getter = pixelsProcessed)]
    pub fn pixels_processed(&self) -> u64 {
        self.pixels_processed
    }
    
    #[wasm_bindgen(getter = pixelsPerSecond)]
    pub fn pixels_per_second(&self) -> f64 {
        self.pixels_per_second
    }
    
    #[wasm_bindgen(getter = threadsUsed)]
    pub fn threads_used(&self) -> u32 {
        self.threads_used
    }
    
    #[wasm_bindgen(getter = parallelEfficiency)]
    pub fn parallel_efficiency(&self) -> f32 {
        self.parallel_efficiency
    }
    
    #[wasm_bindgen(getter = simdUtilized)]
    pub fn simd_utilized(&self) -> bool {
        self.simd_utilized
    }
}

impl From<PerformanceMetrics> for WasmPerformanceMetrics {
    fn from(metrics: PerformanceMetrics) -> Self {
        Self {
            total_time_ms: metrics.total_time_ms,
            peak_memory_bytes: metrics.peak_memory_bytes,
            cpu_usage: metrics.cpu_usage,
            pixels_processed: metrics.pixels_processed,
            pixels_per_second: metrics.pixels_per_second,
            threads_used: metrics.thread_info.threads_used as u32,
            parallel_efficiency: metrics.thread_info.parallel_efficiency,
            simd_utilized: metrics.thread_info.simd_utilized,
        }
    }
}

/// 主要的图像处理接口 - 深模块的 WebAssembly 体现
#[wasm_bindgen]
pub fn process_image(
    image_data: &[u8],
    filter_type: &str,
    intensity: f32,
    radius: Option<f32>,
) -> Result<WasmProcessedImage, WasmError> {
    // 解析滤镜类型
    let filter = parse_filter_type(filter_type)?;
    
    // 构建参数
    let mut params = FilterParams::default();
    params.intensity = intensity;
    params.radius = radius;
    
    // 调用核心处理函数
    let result = core_process_image(image_data, filter, Some(params))
        .map_err(WasmError::from)?;
    
    Ok(WasmProcessedImage::from(result))
}

/// 批处理接口
#[wasm_bindgen]
pub fn batch_process_images(
    images_data: js_sys::Array,
    operations_data: js_sys::Array,
) -> Result<js_sys::Array, WasmError> {
    // 解析输入数据
    let images = parse_images_array(images_data)?;
    let operations = parse_operations_array(operations_data)?;
    
    // 调用核心批处理函数
    let results = core_batch_process(images, operations)
        .map_err(WasmError::from)?;
    
    // 转换结果为 JavaScript 数组
    let js_results = js_sys::Array::new();
    for result in results {
        let wasm_result = WasmProcessedImage::from(result);
        js_results.push(&JsValue::from(wasm_result));
    }
    
    Ok(js_results)
}

/// 获取性能指标
#[wasm_bindgen]
pub fn get_performance_metrics() -> WasmPerformanceMetrics {
    let metrics = core_get_performance_metrics();
    WasmPerformanceMetrics::from(metrics)
}

/// 获取支持的滤镜列表
#[wasm_bindgen]
pub fn get_supported_filters() -> js_sys::Array {
    let filters = js_sys::Array::new();
    filters.push(&JsValue::from_str("gaussian_blur"));
    filters.push(&JsValue::from_str("edge_detection"));
    filters.push(&JsValue::from_str("sharpen"));
    filters.push(&JsValue::from_str("color_adjust"));
    filters.push(&JsValue::from_str("noise_reduction"));
    filters.push(&JsValue::from_str("super_resolution"));
    filters
}

/// 预热 WebAssembly 模块
#[wasm_bindgen]
pub fn warmup() -> Result<(), WasmError> {
    console::log_1(&"Warming up RustImage WASM module...".into());
    // 执行预热逻辑
    Ok(())
}

// 辅助函数 - 隐藏实现细节

fn parse_filter_type(filter_str: &str) -> Result<FilterType, WasmError> {
    match filter_str {
        "gaussian_blur" => Ok(FilterType::GaussianBlur),
        "edge_detection" => Ok(FilterType::EdgeDetection),
        "sharpen" => Ok(FilterType::Sharpen),
        "color_adjust" => Ok(FilterType::ColorAdjust),
        "noise_reduction" => Ok(FilterType::NoiseReduction),
        "super_resolution" => Ok(FilterType::SuperResolution),
        _ => Err(WasmError {
            message: format!("Unsupported filter type: {}", filter_str),
        }),
    }
}

fn parse_images_array(_images: js_sys::Array) -> Result<Vec<&[u8]>, WasmError> {
    // 解析 JavaScript 数组中的图像数据
    todo!("Implementation will be added later")
}

fn parse_operations_array(_operations: js_sys::Array) -> Result<Vec<rustimage_core::FilterOperation>, WasmError> {
    // 解析 JavaScript 数组中的操作数据
    todo!("Implementation will be added later")
}

/// JavaScript 性能监控接口
#[wasm_bindgen]
pub struct PerformanceMonitor {
    #[wasm_bindgen(skip)]
    inner: rustimage_core::performance::PerformanceMonitor,
}

#[wasm_bindgen]
impl PerformanceMonitor {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        let config = rustimage_core::performance::MonitorConfig::default();
        let inner = rustimage_core::performance::PerformanceMonitor::new(config);
        Self { inner }
    }
    
    /// 开始性能测量
    #[wasm_bindgen(js_name = startMeasurement)]
    pub fn start_measurement(&mut self, operation: &str) {
        // 开始测量
        todo!("Implementation will be added later")
    }
    
    /// 结束性能测量
    #[wasm_bindgen(js_name = endMeasurement)]
    pub fn end_measurement(&mut self, operation: &str) -> f64 {
        // 结束测量并返回耗时
        todo!("Implementation will be added later")
    }
    
    /// 获取性能报告
    #[wasm_bindgen(js_name = getReport)]
    pub fn get_report(&self) -> JsValue {
        // 生成并返回性能报告
        todo!("Implementation will be added later")
    }
}