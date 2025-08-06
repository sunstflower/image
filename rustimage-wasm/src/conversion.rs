//! 图像格式转换的 WASM 接口
//!
//! 本模块提供深模块设计的转换接口：
//! - 简单的 JavaScript API
//! - 强大的 Rust 后端实现
//! - 完整的错误处理和性能监控

use wasm_bindgen::prelude::*;
use crate::types::*;

// =============================================================================
// 深模块设计的主要转换接口
// =============================================================================

/// 图像格式转换器 - 主要的 WASM 接口
/// 
/// 这是一个深模块：为 JavaScript 提供简单的转换接口，
/// 内部管理复杂的 Rust 转换逻辑、内存管理、错误处理等
#[wasm_bindgen]
pub struct ImageConverter {
    // 私有字段：完全隐藏 Rust 实现细节
    inner: Option<rustimage_core::FormatConverter>,
}

/// 批量转换任务
#[wasm_bindgen]
pub struct BatchConversionTask {
    image_data: Vec<u8>,
    from_format: JsImageFormat,
    to_format: JsImageFormat,
    options: Option<JsConversionOptions>,
}

/// 批量转换结果
#[wasm_bindgen]
pub struct BatchConversionResult {
    results: Vec<Result<JsConvertedImage, String>>,
    successful_count: usize,
    total_count: usize,
    total_time_ms: f64,
}

// =============================================================================
// 主要转换接口实现
// =============================================================================

#[wasm_bindgen]
impl ImageConverter {
    /// 创建新的图像转换器（默认配置）
    #[wasm_bindgen(constructor)]
    pub fn new() -> Result<ImageConverter, JsValue> {
        let converter = rustimage_core::FormatConverter::with_defaults()
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        
        Ok(ImageConverter {
            inner: Some(converter),
        })
    }
    
    /// 创建高性能配置的转换器
    #[wasm_bindgen(js_name = createHighPerformance)]
    pub fn create_high_performance() -> Result<ImageConverter, JsValue> {
        let converter = rustimage_core::FormatConverter::with_high_performance()
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        
        Ok(ImageConverter {
            inner: Some(converter),
        })
    }
    
    /// 创建高质量配置的转换器
    #[wasm_bindgen(js_name = createHighQuality)]
    pub fn create_high_quality() -> Result<ImageConverter, JsValue> {
        let converter = rustimage_core::FormatConverter::with_high_quality()
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        
        Ok(ImageConverter {
            inner: Some(converter),
        })
    }
    
    /// 转换图像格式 - 主要接口
    /// 
    /// # 参数
    /// * `image_data` - 图像数据（Uint8Array）
    /// * `from_format` - 源格式
    /// * `to_format` - 目标格式
    /// * `options` - 转换选项（可选）
    /// 
    /// # 返回
    /// 转换后的图像结果
    #[wasm_bindgen(js_name = convertFormat)]
    pub fn convert_format(
        &mut self,
        image_data: &[u8],
        from_format: JsImageFormat,
        to_format: JsImageFormat,
        options: Option<JsConversionOptions>,
    ) -> Result<JsConvertedImage, JsValue> {
        let converter = self.inner.as_mut()
            .ok_or_else(|| JsValue::from_str("Converter has been destroyed"))?;
        
        // 转换类型
        let rust_from_format = from_js_image_format(from_format);
        let rust_to_format = from_js_image_format(to_format);
        let rust_options = options.as_ref().map(from_js_conversion_options);
        
        // 执行转换
        let result = converter.convert_format(
            image_data,
            rust_from_format,
            rust_to_format,
            rust_options,
        ).map_err(|e| JsValue::from_str(&e.to_string()))?;
        
        // 转换结果
        Ok(to_js_converted_image(result))
    }
    
    /// 批量转换图像格式
    /// 
    /// # 参数
    /// * `tasks` - 转换任务数组
    /// 
    /// # 返回
    /// 批量转换结果
    #[wasm_bindgen(js_name = batchConvert)]
    pub fn batch_convert(
        &mut self,
        tasks: Vec<BatchConversionTask>,
    ) -> Result<BatchConversionResult, JsValue> {
        let converter = self.inner.as_mut()
            .ok_or_else(|| JsValue::from_str("Converter has been destroyed"))?;
        
        let start_time = web_sys::js_sys::Date::now();
        let mut results = Vec::new();
        let mut successful_count = 0;
        
        for task in tasks {
            let rust_from_format = from_js_image_format(task.from_format);
            let rust_to_format = from_js_image_format(task.to_format);
            let rust_options = task.options.as_ref().map(from_js_conversion_options);
            
            match converter.convert_format(
                &task.image_data,
                rust_from_format,
                rust_to_format,
                rust_options,
            ) {
                Ok(result) => {
                    results.push(Ok(to_js_converted_image(result)));
                    successful_count += 1;
                },
                Err(e) => {
                    results.push(Err(e.to_string()));
                }
            }
        }
        
        let total_time_ms = web_sys::js_sys::Date::now() - start_time;
        
        let total_count = results.len();
        
        Ok(BatchConversionResult {
            results,
            successful_count,
            total_count,
            total_time_ms,
        })
    }
    
    /// 检测图像格式
    /// 
    /// # 参数
    /// * `image_data` - 图像数据
    /// 
    /// # 返回
    /// 检测到的图像格式
    #[wasm_bindgen(js_name = detectFormat)]
    pub fn detect_format(&self, image_data: &[u8]) -> Result<JsImageFormat, JsValue> {
        let converter = self.inner.as_ref()
            .ok_or_else(|| JsValue::from_str("Converter has been destroyed"))?;
        
        let format = converter.detect_format(image_data)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        
        Ok(to_js_image_format(format))
    }
    
    /// 获取格式信息
    /// 
    /// # 参数
    /// * `format` - 图像格式
    /// 
    /// # 返回
    /// 格式详细信息
    #[wasm_bindgen(js_name = getFormatInfo)]
    pub fn get_format_info(&self, format: JsImageFormat) -> JsFormatInfo {
        let converter = self.inner.as_ref()
            .expect("Converter should be available for format info");
        
        let rust_format = from_js_image_format(format);
        let info = converter.get_format_info(rust_format);
        
        to_js_format_info(info)
    }
    
    /// 获取支持的格式列表
    #[wasm_bindgen(js_name = getSupportedFormats)]
    pub fn get_supported_formats(&self) -> js_sys::Array {
        let converter = self.inner.as_ref()
            .expect("Converter should be available for supported formats");
        
        let formats = converter.get_supported_formats()
            .into_iter()
            .map(to_js_image_format)
            .collect::<Vec<_>>();
        
        let js_array = js_sys::Array::new_with_length(formats.len() as u32);
        for (i, format) in formats.iter().enumerate() {
            js_array.set(i as u32, JsValue::from_str(&crate::types::format_to_string(*format)));
        }
        
        js_array
    }
    
    /// 获取转换统计信息
    #[wasm_bindgen(js_name = getConversionStats)]
    pub fn get_conversion_stats(&self) -> Result<String, JsValue> {
        let converter = self.inner.as_ref()
            .ok_or_else(|| JsValue::from_str("Converter has been destroyed"))?;
        
        let stats = converter.get_conversion_statistics();
        
        // 序列化统计信息为 JSON
        serde_json::to_string(&serde_json::json!({
            "total_conversions": stats.total_conversions,
            "successful_conversions": stats.successful_conversions,
            "success_rate": stats.success_rate,
            "average_time_ms": stats.average_processing_time_ms,
            "total_bytes_processed": stats.total_bytes_processed,
            "compression_ratio": stats.compression_ratio,
            "peak_memory_usage_bytes": stats.performance_metrics.memory.peak_memory_bytes
        })).map_err(|e| JsValue::from_str(&e.to_string()))
    }
    
    /// 销毁转换器，释放资源
    #[wasm_bindgen(js_name = destroy)]
    pub fn destroy(&mut self) {
        self.inner.take();
    }
}

#[wasm_bindgen]
impl BatchConversionTask {
    /// 创建新的批量转换任务
    #[wasm_bindgen(constructor)]
    pub fn new(
        image_data: &[u8],
        from_format: JsImageFormat,
        to_format: JsImageFormat,
        options: Option<JsConversionOptions>,
    ) -> BatchConversionTask {
        BatchConversionTask {
            image_data: image_data.to_vec(),
            from_format,
            to_format,
            options,
        }
    }
    
    /// 获取图像数据
    #[wasm_bindgen(js_name = getImageData)]
    pub fn get_image_data(&self) -> Vec<u8> {
        self.image_data.clone()
    }
    
    /// 获取源格式
    #[wasm_bindgen(js_name = getFromFormat)]
    pub fn get_from_format(&self) -> JsImageFormat {
        self.from_format
    }
    
    /// 获取目标格式
    #[wasm_bindgen(js_name = getToFormat)]
    pub fn get_to_format(&self) -> JsImageFormat {
        self.to_format
    }
}

#[wasm_bindgen]
impl BatchConversionResult {
    /// 获取成功转换的数量
    #[wasm_bindgen(js_name = getSuccessfulCount)]
    pub fn get_successful_count(&self) -> usize {
        self.successful_count
    }
    
    /// 获取总任务数量
    #[wasm_bindgen(js_name = getTotalCount)]
    pub fn get_total_count(&self) -> usize {
        self.total_count
    }
    
    /// 获取失败的数量
    #[wasm_bindgen(js_name = getFailedCount)]
    pub fn get_failed_count(&self) -> usize {
        self.total_count - self.successful_count
    }
    
    /// 获取总耗时
    #[wasm_bindgen(js_name = getTotalTimeMs)]
    pub fn get_total_time_ms(&self) -> f64 {
        self.total_time_ms
    }
    
    /// 获取成功率
    #[wasm_bindgen(js_name = getSuccessRate)]
    pub fn get_success_rate(&self) -> f32 {
        if self.total_count > 0 {
            self.successful_count as f32 / self.total_count as f32
        } else {
            0.0
        }
    }
    
    /// 获取处理速度（图像/秒）
    #[wasm_bindgen(js_name = getProcessingSpeed)]
    pub fn get_processing_speed(&self) -> f64 {
        if self.total_time_ms > 0.0 {
            (self.total_count as f64 * 1000.0) / self.total_time_ms
        } else {
            0.0
        }
    }
    
    /// 获取指定索引的结果
    #[wasm_bindgen(js_name = getResult)]
    pub fn get_result(&self, index: usize) -> Option<JsConvertedImage> {
        self.results.get(index).and_then(|r| r.as_ref().ok().cloned())
    }
    
    /// 获取指定索引的错误信息
    #[wasm_bindgen(js_name = getError)]
    pub fn get_error(&self, index: usize) -> Option<String> {
        self.results.get(index).and_then(|r| r.as_ref().err().cloned())
    }
    
    /// 获取所有成功的结果
    #[wasm_bindgen(js_name = getSuccessfulResults)]
    pub fn get_successful_results(&self) -> Vec<JsConvertedImage> {
        self.results.iter()
            .filter_map(|r| r.as_ref().ok().cloned())
            .collect()
    }
}

// =============================================================================
// 便利函数 - 简化的 JavaScript 接口
// =============================================================================

/// 便利函数：快速转换图像格式
/// 
/// 这是最简单的转换接口，适合一次性转换需求
#[wasm_bindgen(js_name = convertImage)]
pub fn convert_image(
    image_data: &[u8],
    from_format: JsImageFormat,
    to_format: JsImageFormat,
    options: Option<JsConversionOptions>,
) -> Result<JsConvertedImage, JsValue> {
    let mut converter = ImageConverter::new()?;
    converter.convert_format(image_data, from_format, to_format, options)
}

/// 便利函数：检测图像格式
#[wasm_bindgen(js_name = detectImageFormat)]
pub fn detect_image_format(image_data: &[u8]) -> Result<JsImageFormat, JsValue> {
    let converter = ImageConverter::new()?;
    converter.detect_format(image_data)
}

/// 便利函数：获取格式信息
#[wasm_bindgen(js_name = getImageFormatInfo)]
pub fn get_image_format_info(format: JsImageFormat) -> JsFormatInfo {
    let converter = ImageConverter::new()
        .expect("Should be able to create converter for format info");
    converter.get_format_info(format)
}

/// 便利函数：获取所有支持的格式
#[wasm_bindgen(js_name = getSupportedImageFormats)]
pub fn get_supported_image_formats() -> js_sys::Array {
    let converter = ImageConverter::new()
        .expect("Should be able to create converter for supported formats");
    converter.get_supported_formats()
}

/// 便利函数：验证图像数据有效性
#[wasm_bindgen(js_name = validateImageData)]
pub fn validate_image_data(image_data: &[u8]) -> bool {
    if image_data.is_empty() {
        return false;
    }
    
    // 尝试检测格式来验证数据有效性
    match detect_image_format(image_data) {
        Ok(_) => true,
        Err(_) => false,
    }
}

/// 便利函数：获取图像基本信息（不进行转换）
#[wasm_bindgen(js_name = getImageInfo)]
pub fn get_image_info(image_data: &[u8]) -> Result<String, JsValue> {
    let format = detect_image_format(image_data)?;
    let format_info = get_image_format_info(format);
    
    // 返回基本信息的 JSON
    serde_json::to_string(&serde_json::json!({
        "format": format_info.get_name(),
        "mime_type": format_info.get_mime_type(),
        "extensions": format_info.get_extensions(),
        "supports_lossy": format_info.supports_lossy(),
        "supports_transparency": format_info.supports_transparency(),
        "supports_animation": format_info.supports_animation(),
        "data_size": image_data.len()
    })).map_err(|e| JsValue::from_str(&e.to_string()))
}