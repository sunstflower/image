//! 性能监控的 WASM 接口
//!
//! 本模块为 JavaScript 提供性能监控功能，
//! 展示 Rust + WASM 的性能优势

use wasm_bindgen::prelude::*;
use crate::types::*;

// =============================================================================
// 性能监控器 WASM 接口
// =============================================================================

/// 性能监控器 - WASM 接口
/// 
/// 提供实时性能监控和分析功能
#[wasm_bindgen]
pub struct PerformanceMonitor {
    inner: Option<rustimage_core::performance::PerformanceMonitor>,
    session_start: f64, // JavaScript timestamp
}

/// 性能监控配置
#[wasm_bindgen]
pub struct MonitorConfig {
    enabled: bool,
    detailed_monitoring: bool,
    max_history_entries: usize,
    sampling_interval_ms: u64,
    monitor_memory: bool,
    monitor_cpu: bool,
}

/// 性能报告
#[wasm_bindgen]
pub struct PerformanceReport {
    report_data: String, // JSON 格式的报告数据
    generation_time: f64,
}

// =============================================================================
// 性能监控器实现
// =============================================================================

#[wasm_bindgen]
impl PerformanceMonitor {
    /// 创建新的性能监控器
    #[wasm_bindgen(constructor)]
    pub fn new(enabled: bool) -> Result<PerformanceMonitor, JsValue> {
        let monitor = rustimage_core::performance::PerformanceMonitor::new(enabled)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        
        Ok(PerformanceMonitor {
            inner: Some(monitor),
            session_start: web_sys::js_sys::Date::now(),
        })
    }
    
    /// 使用自定义配置创建监控器
    #[wasm_bindgen(js_name = withConfig)]
    pub fn with_config(config: &MonitorConfig) -> Result<PerformanceMonitor, JsValue> {
        let rust_config = rustimage_core::performance::MonitorConfigBuilder::new()
            .enabled(config.enabled)
            .detailed_monitoring(config.detailed_monitoring)
            .max_history_entries(config.max_history_entries)
            .sampling_interval_ms(config.sampling_interval_ms)
            .monitor_memory(config.monitor_memory)
            .monitor_cpu(config.monitor_cpu)
            .build();
        
        let monitor = rustimage_core::performance::PerformanceMonitor::with_config(rust_config)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        
        Ok(PerformanceMonitor {
            inner: Some(monitor),
            session_start: web_sys::js_sys::Date::now(),
        })
    }
    
    /// 开始转换监控
    #[wasm_bindgen(js_name = startConversion)]
    pub fn start_conversion(&self, from_format: JsImageFormat, to_format: JsImageFormat) {
        if let Some(monitor) = &self.inner {
            let rust_from = from_js_image_format(from_format);
            let rust_to = from_js_image_format(to_format);
            monitor.start_conversion(&rust_from, &rust_to);
        }
    }
    
    /// 结束转换监控
    #[wasm_bindgen(js_name = endConversion)]
    pub fn end_conversion(&self, duration_ms: f64, success: bool) {
        if let Some(monitor) = &self.inner {
            let duration = std::time::Duration::from_millis(duration_ms as u64);
            monitor.end_conversion(duration, success);
        }
    }
    
    /// 开始批量转换监控
    #[wasm_bindgen(js_name = startBatchConversion)]
    pub fn start_batch_conversion(&self, batch_size: usize) {
        if let Some(monitor) = &self.inner {
            monitor.start_batch_conversion(batch_size);
        }
    }
    
    /// 结束批量转换监控
    #[wasm_bindgen(js_name = endBatchConversion)]
    pub fn end_batch_conversion(&self, duration_ms: f64, success_count: usize, total_count: usize) {
        if let Some(monitor) = &self.inner {
            let duration = std::time::Duration::from_millis(duration_ms as u64);
            monitor.end_batch_conversion(duration, success_count, total_count);
        }
    }
    
    /// 获取当前性能指标
    #[wasm_bindgen(js_name = getCurrentMetrics)]
    pub fn get_current_metrics(&self) -> Result<JsPerformanceMetrics, JsValue> {
        let monitor = self.inner.as_ref()
            .ok_or_else(|| JsValue::from_str("Monitor has been destroyed"))?;
        
        let metrics = monitor.get_current_metrics();
        Ok(to_js_performance_metrics(metrics))
    }
    
    /// 生成性能报告
    #[wasm_bindgen(js_name = generateReport)]
    pub fn generate_report(&self) -> Result<PerformanceReport, JsValue> {
        let monitor = self.inner.as_ref()
            .ok_or_else(|| JsValue::from_str("Monitor has been destroyed"))?;
        
        let report = monitor.generate_report()
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        
        // 序列化报告为 JSON
        let report_json = serde_json::to_string(&serde_json::json!({
            "report_id": report.report_id,
            "generated_at": format!("{:?}", report.generated_at),
            "analysis_period": report.analysis_period.as_secs(),
            "executive_summary": {
                "session_duration_ms": web_sys::js_sys::Date::now() - self.session_start,
                "overall_performance_score": report.executive_summary.overall_performance_score,
                "key_metrics": {
                    "success_rate": report.executive_summary.key_metrics.success_rate,
                    "average_response_time_ms": report.executive_summary.key_metrics.average_response_time_ms,
                    "throughput_ops_per_sec": report.executive_summary.key_metrics.throughput_ops_per_sec
                },
                "key_findings": report.executive_summary.key_findings
            },
            "detailed_statistics": {
                "operation_stats_count": report.detailed_statistics.operation_stats.len(),
                "system_resource_stats_available": report.detailed_statistics.system_resource_stats.peak_memory_usage_mb > 0.0
            },
            "optimization_recommendations": report.optimization_recommendations.into_iter().map(|rec| {
                serde_json::json!({
                    "recommendation_type": format!("{:?}", rec.recommendation_type),
                    "description": rec.description,
                    "expected_improvement": rec.expected_improvement,
                    "implementation_difficulty": format!("{:?}", rec.implementation_difficulty)
                })
            }).collect::<Vec<_>>()
        })).map_err(|e| JsValue::from_str(&e.to_string()))?;
        
        Ok(PerformanceReport {
            report_data: report_json,
            generation_time: web_sys::js_sys::Date::now(),
        })
    }
    
    /// 重置监控数据
    #[wasm_bindgen(js_name = reset)]
    pub fn reset(&mut self) {
        if let Some(monitor) = &mut self.inner {
            monitor.reset();
            self.session_start = web_sys::js_sys::Date::now();
        }
    }
    
    /// 获取会话持续时间
    #[wasm_bindgen(js_name = getSessionDurationMs)]
    pub fn get_session_duration_ms(&self) -> f64 {
        web_sys::js_sys::Date::now() - self.session_start
    }
    
    /// 是否启用监控
    #[wasm_bindgen(js_name = isEnabled)]
    pub fn is_enabled(&self) -> bool {
        self.inner.is_some()
    }
    
    /// 销毁监控器
    #[wasm_bindgen(js_name = destroy)]
    pub fn destroy(&mut self) {
        self.inner.take();
    }
}

#[wasm_bindgen]
impl MonitorConfig {
    /// 创建新的监控配置
    #[wasm_bindgen(constructor)]
    pub fn new() -> MonitorConfig {
        MonitorConfig {
            enabled: true,
            detailed_monitoring: true,
            max_history_entries: 1000,
            sampling_interval_ms: 100,
            monitor_memory: true,
            monitor_cpu: true,
        }
    }
    
    /// 创建禁用的配置
    #[wasm_bindgen(js_name = disabled)]
    pub fn disabled() -> MonitorConfig {
        MonitorConfig {
            enabled: false,
            detailed_monitoring: false,
            max_history_entries: 0,
            sampling_interval_ms: 0,
            monitor_memory: false,
            monitor_cpu: false,
        }
    }
    
    /// 设置是否启用监控
    #[wasm_bindgen(js_name = setEnabled)]
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }
    
    /// 是否启用监控
    #[wasm_bindgen(js_name = isEnabled)]
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
    
    /// 设置是否启用详细监控
    #[wasm_bindgen(js_name = setDetailedMonitoring)]
    pub fn set_detailed_monitoring(&mut self, enabled: bool) {
        self.detailed_monitoring = enabled;
    }
    
    /// 是否启用详细监控
    #[wasm_bindgen(js_name = isDetailedMonitoringEnabled)]
    pub fn is_detailed_monitoring_enabled(&self) -> bool {
        self.detailed_monitoring
    }
    
    /// 设置最大历史记录数
    #[wasm_bindgen(js_name = setMaxHistoryEntries)]
    pub fn set_max_history_entries(&mut self, count: usize) {
        self.max_history_entries = count;
    }
    
    /// 获取最大历史记录数
    #[wasm_bindgen(js_name = getMaxHistoryEntries)]
    pub fn get_max_history_entries(&self) -> usize {
        self.max_history_entries
    }
    
    /// 设置采样间隔
    #[wasm_bindgen(js_name = setSamplingIntervalMs)]
    pub fn set_sampling_interval_ms(&mut self, ms: u64) {
        self.sampling_interval_ms = ms;
    }
    
    /// 获取采样间隔
    #[wasm_bindgen(js_name = getSamplingIntervalMs)]
    pub fn get_sampling_interval_ms(&self) -> u64 {
        self.sampling_interval_ms
    }
    
    /// 设置是否监控内存
    #[wasm_bindgen(js_name = setMonitorMemory)]
    pub fn set_monitor_memory(&mut self, enabled: bool) {
        self.monitor_memory = enabled;
    }
    
    /// 是否监控内存
    #[wasm_bindgen(js_name = isMonitorMemoryEnabled)]
    pub fn is_monitor_memory_enabled(&self) -> bool {
        self.monitor_memory
    }
    
    /// 设置是否监控 CPU
    #[wasm_bindgen(js_name = setMonitorCpu)]
    pub fn set_monitor_cpu(&mut self, enabled: bool) {
        self.monitor_cpu = enabled;
    }
    
    /// 是否监控 CPU
    #[wasm_bindgen(js_name = isMonitorCpuEnabled)]
    pub fn is_monitor_cpu_enabled(&self) -> bool {
        self.monitor_cpu
    }
}

#[wasm_bindgen]
impl PerformanceReport {
    /// 获取报告数据（JSON 格式）
    #[wasm_bindgen(js_name = getData)]
    pub fn get_data(&self) -> String {
        self.report_data.clone()
    }
    
    /// 获取报告生成时间
    #[wasm_bindgen(js_name = getGenerationTime)]
    pub fn get_generation_time(&self) -> f64 {
        self.generation_time
    }
    
    /// 获取报告年龄（毫秒）
    #[wasm_bindgen(js_name = getAgeMs)]
    pub fn get_age_ms(&self) -> f64 {
        web_sys::js_sys::Date::now() - self.generation_time
    }
    
    /// 导出为 Blob URL（用于下载）
    #[wasm_bindgen(js_name = exportAsBlobUrl)]
    pub fn export_as_blob_url(&self) -> Result<String, JsValue> {
        use web_sys::*;
        
        let uint8_array = js_sys::Uint8Array::new_with_length(self.report_data.len() as u32);
        uint8_array.copy_from(self.report_data.as_bytes());
        
        let blob_parts = js_sys::Array::new();
        blob_parts.push(&uint8_array);
        
        let blob_options = web_sys::BlobPropertyBag::new();
        blob_options.set_type("application/json");
        
        let blob = web_sys::Blob::new_with_u8_array_sequence_and_options(&blob_parts, &blob_options)?;
        let url = web_sys::Url::create_object_url_with_blob(&blob)?;
        
        Ok(url)
    }
}

// =============================================================================
// 便利函数和实用工具
// =============================================================================

/// 创建性能基准测试
#[wasm_bindgen(js_name = createPerformanceBenchmark)]
pub fn create_performance_benchmark() -> PerformanceBenchmark {
    PerformanceBenchmark::new()
}

/// 性能基准测试工具
#[wasm_bindgen]
pub struct PerformanceBenchmark {
    tests: Vec<BenchmarkTest>,
    start_time: f64,
}

/// 单个基准测试
#[wasm_bindgen]
pub struct BenchmarkTest {
    name: String,
    duration_ms: f64,
    memory_usage_bytes: u64,
    success: bool,
    details: String,
}

#[wasm_bindgen]
impl PerformanceBenchmark {
    /// 创建新的基准测试
    #[wasm_bindgen(constructor)]
    pub fn new() -> PerformanceBenchmark {
        PerformanceBenchmark {
            tests: Vec::new(),
            start_time: web_sys::js_sys::Date::now(),
        }
    }
    
    /// 运行转换基准测试
    #[wasm_bindgen(js_name = runConversionBenchmark)]
    pub fn run_conversion_benchmark(
        &mut self,
        test_name: &str,
        image_data: &[u8],
        from_format: JsImageFormat,
        to_format: JsImageFormat,
    ) -> Result<(), JsValue> {
        let start_time = web_sys::js_sys::Date::now();
        let initial_memory = get_memory_usage();
        
        let result = crate::conversion::convert_image(
            image_data,
            from_format,
            to_format,
            None,
        );
        
        let end_time = web_sys::js_sys::Date::now();
        let final_memory = get_memory_usage();
        
        let test = BenchmarkTest {
            name: test_name.to_string(),
            duration_ms: end_time - start_time,
            memory_usage_bytes: final_memory.saturating_sub(initial_memory),
            success: result.is_ok(),
            details: match result {
                Ok(converted) => format!(
                    "Converted {} bytes to {} bytes ({}% compression)",
                    image_data.len(),
                    converted.get_converted_size(),
                    converted.get_compression_percentage()
                ),
                Err(e) => format!("Error: {}", e.as_string().unwrap_or_else(|| "Unknown error".to_string())),
            },
        };
        
        self.tests.push(test);
        Ok(())
    }
    
    /// 获取基准测试结果
    #[wasm_bindgen(js_name = getResults)]
    pub fn get_results(&self) -> String {
        let total_duration = web_sys::js_sys::Date::now() - self.start_time;
        let successful_tests = self.tests.iter().filter(|t| t.success).count();
        
        serde_json::to_string(&serde_json::json!({
            "summary": {
                "total_tests": self.tests.len(),
                "successful_tests": successful_tests,
                "success_rate": if self.tests.is_empty() { 0.0 } else { successful_tests as f64 / self.tests.len() as f64 },
                "total_duration_ms": total_duration,
                "average_test_duration_ms": if self.tests.is_empty() { 0.0 } else { 
                    self.tests.iter().map(|t| t.duration_ms).sum::<f64>() / self.tests.len() as f64 
                }
            },
            "tests": self.tests.iter().map(|test| {
                serde_json::json!({
                    "name": test.name,
                    "duration_ms": test.duration_ms,
                    "memory_usage_bytes": test.memory_usage_bytes,
                    "success": test.success,
                    "details": test.details
                })
            }).collect::<Vec<_>>()
        })).unwrap_or_else(|_| "{}".to_string())
    }
    
    /// 清除所有测试结果
    #[wasm_bindgen(js_name = clear)]
    pub fn clear(&mut self) {
        self.tests.clear();
        self.start_time = web_sys::js_sys::Date::now();
    }
}

// =============================================================================
// 内部工具函数
// =============================================================================

/// 获取当前内存使用量（近似值）
fn get_memory_usage() -> u64 {
    // 在 WASM 环境中，我们使用 performance.memory（如果可用）
    if let Some(window) = web_sys::window() {
        if let Some(performance) = window.performance() {
            // 尝试访问 memory 属性（Chrome/Edge 支持）
            if let Ok(memory) = js_sys::Reflect::get(&performance, &JsValue::from_str("memory")) {
                if !memory.is_undefined() {
                    if let Ok(used_js_heap) = js_sys::Reflect::get(&memory, &JsValue::from_str("usedJSHeapSize")) {
                        if let Some(used) = used_js_heap.as_f64() {
                            return used as u64;
                        }
                    }
                }
            }
        }
    }
    
    // fallback: 返回 0 表示无法获取内存信息
    0
}