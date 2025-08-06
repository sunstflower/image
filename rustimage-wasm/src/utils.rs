//! WASM 实用工具函数
//!
//! 提供 WASM 环境下的通用工具和辅助函数

use wasm_bindgen::prelude::*;

// =============================================================================
// 日志和调试工具
// =============================================================================

/// 输出日志到浏览器控制台
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
    
    #[wasm_bindgen(js_namespace = console)]
    fn warn(s: &str);
    
    #[wasm_bindgen(js_namespace = console)]
    fn error(s: &str);
    
    #[wasm_bindgen(js_namespace = console)]
    fn info(s: &str);
    
    #[wasm_bindgen(js_namespace = console)]
    fn debug(s: &str);
}

/// 日志级别
#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
}

/// 输出日志消息（便利函数）
pub fn log_message_to_console(message: &str) {
    log(&format!("[RustImage WASM] {}", message));
}

/// 输出警告消息
pub fn log_warn(message: &str) {
    warn(&format!("[RustImage WASM] WARNING: {}", message));
}

/// 输出错误消息
pub fn log_error(message: &str) {
    error(&format!("[RustImage WASM] ERROR: {}", message));
}

/// 输出信息消息
pub fn log_info(message: &str) {
    info(&format!("[RustImage WASM] INFO: {}", message));
}

/// 输出调试消息
pub fn log_debug(message: &str) {
    debug(&format!("[RustImage WASM] DEBUG: {}", message));
}

/// 输出调试日志
#[wasm_bindgen(js_name = logDebugMessage)]
pub fn log_debug_message(message: &str) {
    log_debug(message);
}

/// 输出信息日志
#[wasm_bindgen(js_name = logInfoMessage)]
pub fn log_info_message(message: &str) {
    log_info(message);
}

/// 输出警告日志
#[wasm_bindgen(js_name = logWarnMessage)]
pub fn log_warn_message(message: &str) {
    log_warn(message);
}

/// 输出错误日志
#[wasm_bindgen(js_name = logErrorMessage)]
pub fn log_error_message(message: &str) {
    log_error(message);
}

// =============================================================================
// 内存管理工具
// =============================================================================

/// 获取 WASM 内存信息
#[wasm_bindgen(js_name = getWasmMemoryInfo)]
pub fn get_wasm_memory_info() -> Result<String, JsValue> {
    use js_sys::WebAssembly;
    
    // 获取当前 WASM 内存实例
    let memory = wasm_bindgen::memory();
    let memory_obj = memory.unchecked_ref::<WebAssembly::Memory>();
    let buffer = memory_obj.buffer();
    let array_buffer: js_sys::ArrayBuffer = buffer.unchecked_into();
    let length = array_buffer.byte_length() as u32;
    
    serde_json::to_string(&serde_json::json!({
        "buffer_size_bytes": length,
        "buffer_size_mb": length as f64 / (1024.0 * 1024.0),
        "pages": length / 65536, // WASM page size is 64KB
    })).map_err(|e| JsValue::from_str(&e.to_string()))
}

/// 手动触发垃圾回收（如果支持）
#[wasm_bindgen(js_name = forceGarbageCollection)]
pub fn force_garbage_collection() {
    if let Some(window) = web_sys::window() {
        if let Ok(gc) = js_sys::Reflect::get(&window, &JsValue::from_str("gc")) {
            if gc.is_function() {
                let gc_fn: js_sys::Function = gc.unchecked_into();
                let _ = gc_fn.call0(&JsValue::NULL);
            }
        }
    }
}

// =============================================================================
// 类型转换工具
// =============================================================================

/// 将 Uint8Array 转换为 Vec<u8>
#[wasm_bindgen(js_name = uint8ArrayToVec)]
pub fn uint8_array_to_vec(array: &js_sys::Uint8Array) -> Vec<u8> {
    array.to_vec()
}

/// 将 Vec<u8> 转换为 Uint8Array
#[wasm_bindgen(js_name = vecToUint8Array)]
pub fn vec_to_uint8_array(vec: &[u8]) -> js_sys::Uint8Array {
    js_sys::Uint8Array::from(vec)
}

/// 将 ArrayBuffer 转换为 Vec<u8>
#[wasm_bindgen(js_name = arrayBufferToVec)]
pub fn array_buffer_to_vec(buffer: &js_sys::ArrayBuffer) -> Vec<u8> {
    let uint8_array = js_sys::Uint8Array::new(buffer);
    uint8_array.to_vec()
}

/// 将 Vec<u8> 转换为 ArrayBuffer
#[wasm_bindgen(js_name = vecToArrayBuffer)]
pub fn vec_to_array_buffer(vec: &[u8]) -> js_sys::ArrayBuffer {
    let uint8_array = js_sys::Uint8Array::from(vec);
    uint8_array.buffer()
}

// =============================================================================
// 文件处理工具
// =============================================================================

/// 从文件扩展名检测可能的图像格式
#[wasm_bindgen(js_name = detectFormatFromExtension)]
pub fn detect_format_from_extension(filename: &str) -> Option<crate::types::JsImageFormat> {
    let extension = filename.split('.').last()?.to_lowercase();
    
    match extension.as_str() {
        "jpg" | "jpeg" => Some(crate::types::JsImageFormat::Jpeg),
        "png" => Some(crate::types::JsImageFormat::Png),
        "webp" => Some(crate::types::JsImageFormat::WebP),
        "avif" => Some(crate::types::JsImageFormat::Avif),
        "bmp" => Some(crate::types::JsImageFormat::Bmp),
        "tiff" | "tif" => Some(crate::types::JsImageFormat::Tiff),
        "gif" => Some(crate::types::JsImageFormat::Gif),
        "ico" => Some(crate::types::JsImageFormat::Ico),
        _ => None,
    }
}

/// 根据 MIME 类型检测图像格式
#[wasm_bindgen(js_name = detectFormatFromMimeType)]
pub fn detect_format_from_mime_type(mime_type: &str) -> Option<crate::types::JsImageFormat> {
    match mime_type {
        "image/jpeg" => Some(crate::types::JsImageFormat::Jpeg),
        "image/png" => Some(crate::types::JsImageFormat::Png),
        "image/webp" => Some(crate::types::JsImageFormat::WebP),
        "image/avif" => Some(crate::types::JsImageFormat::Avif),
        "image/bmp" => Some(crate::types::JsImageFormat::Bmp),
        "image/tiff" => Some(crate::types::JsImageFormat::Tiff),
        "image/gif" => Some(crate::types::JsImageFormat::Gif),
        "image/x-icon" | "image/vnd.microsoft.icon" => Some(crate::types::JsImageFormat::Ico),
        _ => None,
    }
}

/// 生成建议的文件名
#[wasm_bindgen(js_name = generateSuggestedFilename)]
pub fn generate_suggested_filename(
    original_filename: Option<String>,
    target_format: crate::types::JsImageFormat,
) -> String {
    let base_name = original_filename
        .as_ref()
        .and_then(|name| name.split('.').next())
        .unwrap_or("converted_image");
    
    let extension = match target_format {
        crate::types::JsImageFormat::Jpeg => "jpg",
        crate::types::JsImageFormat::Png => "png",
        crate::types::JsImageFormat::WebP => "webp",
        crate::types::JsImageFormat::Avif => "avif",
        crate::types::JsImageFormat::Bmp => "bmp",
        crate::types::JsImageFormat::Tiff => "tiff",
        crate::types::JsImageFormat::Gif => "gif",
        crate::types::JsImageFormat::Ico => "ico",
    };
    
    format!("{}.{}", base_name, extension)
}

// =============================================================================
// 验证和检查工具
// =============================================================================

/// 检查数据是否可能是有效的图像数据
#[wasm_bindgen(js_name = isLikelyImageData)]
pub fn is_likely_image_data(data: &[u8]) -> bool {
    if data.len() < 8 {
        return false;
    }
    
    // 检查常见的图像文件头
    match &data[0..8] {
        // JPEG
        [0xFF, 0xD8, 0xFF, ..] => true,
        // PNG
        [0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A] => true,
        // WebP
        b if b.starts_with(b"RIFF") && data.len() >= 12 && &data[8..12] == b"WEBP" => true,
        // BMP
        [0x42, 0x4D, ..] => true,
        // TIFF (little endian)
        [0x49, 0x49, 0x2A, 0x00, ..] => true,
        // TIFF (big endian)
        [0x4D, 0x4D, 0x00, 0x2A, ..] => true,
        // GIF
        b if b.starts_with(b"GIF87a") || b.starts_with(b"GIF89a") => true,
        // ICO
        [0x00, 0x00, 0x01, 0x00, ..] => true,
        _ => false,
    }
}

/// 估算图像数据的压缩率
#[wasm_bindgen(js_name = estimateCompressionRatio)]
pub fn estimate_compression_ratio(width: u32, height: u32, actual_size: u32) -> f32 {
    let uncompressed_size = width * height * 4; // 假设 RGBA
    if uncompressed_size > 0 {
        actual_size as f32 / uncompressed_size as f32
    } else {
        1.0
    }
}

/// 计算理论上的文件大小范围
#[wasm_bindgen(js_name = calculateSizeRange)]
pub fn calculate_size_range(
    width: u32,
    height: u32,
    format: crate::types::JsImageFormat,
) -> Result<String, JsValue> {
    let pixel_count = width as u64 * height as u64;
    
    let (min_ratio, max_ratio, description) = match format {
        crate::types::JsImageFormat::Jpeg => (0.05, 0.3, "JPEG (lossy compression)"),
        crate::types::JsImageFormat::Png => (0.2, 1.0, "PNG (lossless compression)"),
        crate::types::JsImageFormat::WebP => (0.03, 0.8, "WebP (lossy/lossless)"),
        crate::types::JsImageFormat::Avif => (0.02, 0.5, "AVIF (next-gen compression)"),
        crate::types::JsImageFormat::Bmp => (3.0, 4.0, "BMP (uncompressed)"),
        crate::types::JsImageFormat::Tiff => (0.3, 4.0, "TIFF (various compression)"),
        crate::types::JsImageFormat::Gif => (0.1, 1.0, "GIF (LZW compression)"),
        crate::types::JsImageFormat::Ico => (0.1, 1.0, "ICO (multiple sizes)"),
    };
    
    let base_size = pixel_count * 3; // RGB baseline
    let min_size = (base_size as f64 * min_ratio) as u64;
    let max_size = (base_size as f64 * max_ratio) as u64;
    
    serde_json::to_string(&serde_json::json!({
        "format": description,
        "dimensions": format!("{}×{}", width, height),
        "pixel_count": pixel_count,
        "estimated_size_range": {
            "min_bytes": min_size,
            "max_bytes": max_size,
            "min_mb": min_size as f64 / (1024.0 * 1024.0),
            "max_mb": max_size as f64 / (1024.0 * 1024.0)
        },
        "compression_ratio_range": {
            "min": min_ratio,
            "max": max_ratio
        }
    })).map_err(|e| JsValue::from_str(&e.to_string()))
}

// =============================================================================
// 环境检测工具
// =============================================================================

/// 检测当前运行环境的能力
#[wasm_bindgen(js_name = detectEnvironmentCapabilities)]
pub fn detect_environment_capabilities() -> Result<String, JsValue> {
    let mut capabilities = serde_json::Map::new();
    
    // 检测是否在浏览器环境中
    let in_browser = web_sys::window().is_some();
    capabilities.insert("in_browser".to_string(), serde_json::Value::Bool(in_browser));
    
    if let Some(window) = web_sys::window() {
        
        // 检测 Web Workers 支持
        let supports_workers = js_sys::Reflect::has(&window, &JsValue::from_str("Worker"))
            .unwrap_or(false);
        capabilities.insert("supports_workers".to_string(), serde_json::Value::Bool(supports_workers));
        
        // 检测 OffscreenCanvas 支持
        let supports_offscreen_canvas = js_sys::Reflect::has(&window, &JsValue::from_str("OffscreenCanvas"))
            .unwrap_or(false);
        capabilities.insert("supports_offscreen_canvas".to_string(), serde_json::Value::Bool(supports_offscreen_canvas));
        
        // 检测 ImageBitmap 支持
        let supports_image_bitmap = js_sys::Reflect::has(&window, &JsValue::from_str("createImageBitmap"))
            .unwrap_or(false);
        capabilities.insert("supports_image_bitmap".to_string(), serde_json::Value::Bool(supports_image_bitmap));
        
        // 检测 Performance API
        let has_performance = window.performance().is_some();
        capabilities.insert("has_performance_api".to_string(), serde_json::Value::Bool(has_performance));
        
        if let Some(performance) = window.performance() {
            // 检测 memory API（Chrome/Edge）
            let has_memory = js_sys::Reflect::has(&performance, &JsValue::from_str("memory"))
                .unwrap_or(false);
            capabilities.insert("has_memory_api".to_string(), serde_json::Value::Bool(has_memory));
        }
    }
    
    // 检测 WASM 特性
    let memory = wasm_bindgen::memory();
    let memory_obj = memory.unchecked_ref::<js_sys::WebAssembly::Memory>();
    let buffer = memory_obj.buffer();
    let array_buffer: js_sys::ArrayBuffer = buffer.unchecked_into();
    let pages = array_buffer.byte_length() / 65536;
    capabilities.insert("wasm_memory_pages".to_string(), 
        serde_json::Value::Number(serde_json::Number::from(pages as u32))
    );
    
    serde_json::to_string(&capabilities)
        .map_err(|e| JsValue::from_str(&e.to_string()))
}

/// 获取用户代理信息
#[wasm_bindgen(js_name = getUserAgent)]
pub fn get_user_agent() -> Option<String> {
    web_sys::window()?
        .navigator()
        .user_agent()
        .ok()
}

/// 检测是否在移动设备上运行
#[wasm_bindgen(js_name = isMobileDevice)]
pub fn is_mobile_device() -> bool {
    if let Some(user_agent) = get_user_agent() {
        let ua = user_agent.to_lowercase();
        ua.contains("mobile") || 
        ua.contains("android") || 
        ua.contains("iphone") || 
        ua.contains("ipad") ||
        ua.contains("blackberry") ||
        ua.contains("windows phone")
    } else {
        false
    }
}

// =============================================================================
// 时间和性能工具
// =============================================================================

/// 获取高精度时间戳
#[wasm_bindgen(js_name = getHighResTimestamp)]
pub fn get_high_res_timestamp() -> f64 {
    if let Some(window) = web_sys::window() {
        if let Some(performance) = window.performance() {
            return performance.now();
        }
    }
    
    // fallback
    js_sys::Date::now()
}

/// 创建性能计时器
#[wasm_bindgen(js_name = createTimer)]
pub fn create_timer() -> PerformanceTimer {
    PerformanceTimer::new()
}

/// 性能计时器
#[wasm_bindgen]
pub struct PerformanceTimer {
    start_time: f64,
    marks: Vec<(String, f64)>,
}

#[wasm_bindgen]
impl PerformanceTimer {
    /// 创建新的计时器
    #[wasm_bindgen(constructor)]
    pub fn new() -> PerformanceTimer {
        PerformanceTimer {
            start_time: get_high_res_timestamp(),
            marks: Vec::new(),
        }
    }
    
    /// 添加时间标记
    #[wasm_bindgen(js_name = mark)]
    pub fn mark(&mut self, name: &str) {
        let current_time = get_high_res_timestamp();
        self.marks.push((name.to_string(), current_time - self.start_time));
    }
    
    /// 获取经过的时间
    #[wasm_bindgen(js_name = getElapsedMs)]
    pub fn get_elapsed_ms(&self) -> f64 {
        get_high_res_timestamp() - self.start_time
    }
    
    /// 重置计时器
    #[wasm_bindgen(js_name = reset)]
    pub fn reset(&mut self) {
        self.start_time = get_high_res_timestamp();
        self.marks.clear();
    }
    
    /// 获取所有标记的报告
    #[wasm_bindgen(js_name = getReport)]
    pub fn get_report(&self) -> String {
        let total_time = self.get_elapsed_ms();
        
        serde_json::to_string(&serde_json::json!({
            "total_time_ms": total_time,
            "marks": self.marks.iter().map(|(name, time)| {
                serde_json::json!({
                    "name": name,
                    "time_ms": time,
                    "percentage": if total_time > 0.0 { (time / total_time) * 100.0 } else { 0.0 }
                })
            }).collect::<Vec<_>>()
        })).unwrap_or_else(|_| "{}".to_string())
    }
}

// =============================================================================
// 错误处理工具
// =============================================================================

/// 创建标准化的错误对象
#[wasm_bindgen(js_name = createError)]
pub fn create_error(message: &str, error_type: &str, details: Option<String>) -> JsValue {
    let error_obj = js_sys::Object::new();
    js_sys::Reflect::set(&error_obj, &JsValue::from_str("message"), &JsValue::from_str(message)).unwrap();
    js_sys::Reflect::set(&error_obj, &JsValue::from_str("type"), &JsValue::from_str(error_type)).unwrap();
    js_sys::Reflect::set(&error_obj, &JsValue::from_str("timestamp"), &JsValue::from_f64(js_sys::Date::now())).unwrap();
    
    if let Some(details) = details {
        js_sys::Reflect::set(&error_obj, &JsValue::from_str("details"), &JsValue::from_str(&details)).unwrap();
    }
    
    error_obj.into()
}

/// 安全地解析 JSON
#[wasm_bindgen(js_name = safeJsonParse)]
pub fn safe_json_parse(json_str: &str) -> Result<JsValue, JsValue> {
    js_sys::JSON::parse(json_str)
        .map_err(|e| create_error("JSON parse error", "ParseError", Some(format!("{:?}", e))))
}