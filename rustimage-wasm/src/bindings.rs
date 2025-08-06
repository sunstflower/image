//! WASM 绑定定义
//!
//! 本模块定义了与 JavaScript 交互的底层绑定
//! 提供类型安全的 WASM 接口

use wasm_bindgen::prelude::*;

// =============================================================================
// 外部 JavaScript 函数绑定
// =============================================================================

#[wasm_bindgen]
extern "C" {
    /// JavaScript 的 alert 函数
    fn alert(s: &str);
    
    /// JavaScript 的 console.time
    #[wasm_bindgen(js_namespace = console)]
    fn time(label: &str);
    
    /// JavaScript 的 console.timeEnd
    #[wasm_bindgen(js_namespace = console)]
    fn timeEnd(label: &str);
    
    /// JavaScript 的 performance.now
    #[wasm_bindgen(js_namespace = performance)]
    fn now() -> f64;
    
    /// JavaScript 的 Date.now
    #[wasm_bindgen(js_namespace = Date)]
    fn now_date() -> f64;
}

// =============================================================================
// 导出的 WASM 函数
// =============================================================================

/// 显示警告对话框
#[wasm_bindgen]
pub fn alert_message(message: &str) {
    alert(message);
}

/// 开始性能计时
#[wasm_bindgen(js_name = startTimer)]
pub fn start_timer(label: &str) {
    time(label);
}

/// 结束性能计时
#[wasm_bindgen(js_name = endTimer)]
pub fn end_timer(label: &str) {
    timeEnd(label);
}

/// 获取高精度时间戳
#[wasm_bindgen(js_name = getTimestamp)]
pub fn get_timestamp() -> f64 {
    now()
}

/// 获取当前日期时间戳
#[wasm_bindgen(js_name = getDateTimestamp)]
pub fn get_date_timestamp() -> f64 {
    now_date()
}

// =============================================================================
// 版本和模块信息
// =============================================================================

/// 获取 WASM 模块版本
#[wasm_bindgen(js_name = getVersion)]
pub fn get_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

/// 获取模块名称
#[wasm_bindgen(js_name = getModuleName)]
pub fn get_module_name() -> String {
    env!("CARGO_PKG_NAME").to_string()
}

/// 获取模块描述
#[wasm_bindgen(js_name = getDescription)]
pub fn get_description() -> String {
    env!("CARGO_PKG_DESCRIPTION").to_string()
}

/// 获取完整的模块信息
#[wasm_bindgen(js_name = getModuleInfo)]
pub fn get_module_info() -> Result<String, JsValue> {
    serde_json::to_string(&serde_json::json!({
        "name": env!("CARGO_PKG_NAME"),
        "version": env!("CARGO_PKG_VERSION"),
        "description": env!("CARGO_PKG_DESCRIPTION"),
        "authors": env!("CARGO_PKG_AUTHORS"),
        "repository": env!("CARGO_PKG_REPOSITORY"),
        "license": env!("CARGO_PKG_LICENSE"),
        "build_timestamp": compile_time::datetime_str!(),
        "rust_version": env!("CARGO_PKG_RUST_VERSION"),
    })).map_err(|e| JsValue::from_str(&e.to_string()))
}

// =============================================================================
// 模块初始化和配置
// =============================================================================

/// WASM 模块配置选项
#[wasm_bindgen]
pub struct WasmConfig {
    enable_logging: bool,
    enable_performance_monitoring: bool,
    enable_memory_optimization: bool,
    log_level: String,
}

#[wasm_bindgen]
impl WasmConfig {
    /// 创建默认配置
    #[wasm_bindgen(constructor)]
    pub fn new() -> WasmConfig {
        WasmConfig {
            enable_logging: true,
            enable_performance_monitoring: true,
            enable_memory_optimization: true,
            log_level: "info".to_string(),
        }
    }
    
    /// 设置是否启用日志
    #[wasm_bindgen(js_name = setLogging)]
    pub fn set_logging(&mut self, enabled: bool) {
        self.enable_logging = enabled;
    }
    
    /// 是否启用日志
    #[wasm_bindgen(js_name = isLoggingEnabled)]
    pub fn is_logging_enabled(&self) -> bool {
        self.enable_logging
    }
    
    /// 设置是否启用性能监控
    #[wasm_bindgen(js_name = setPerformanceMonitoring)]
    pub fn set_performance_monitoring(&mut self, enabled: bool) {
        self.enable_performance_monitoring = enabled;
    }
    
    /// 是否启用性能监控
    #[wasm_bindgen(js_name = isPerformanceMonitoringEnabled)]
    pub fn is_performance_monitoring_enabled(&self) -> bool {
        self.enable_performance_monitoring
    }
    
    /// 设置是否启用内存优化
    #[wasm_bindgen(js_name = setMemoryOptimization)]
    pub fn set_memory_optimization(&mut self, enabled: bool) {
        self.enable_memory_optimization = enabled;
    }
    
    /// 是否启用内存优化
    #[wasm_bindgen(js_name = isMemoryOptimizationEnabled)]
    pub fn is_memory_optimization_enabled(&self) -> bool {
        self.enable_memory_optimization
    }
    
    /// 设置日志级别
    #[wasm_bindgen(js_name = setLogLevel)]
    pub fn set_log_level(&mut self, level: &str) {
        self.log_level = level.to_string();
    }
    
    /// 获取日志级别
    #[wasm_bindgen(js_name = getLogLevel)]
    pub fn get_log_level(&self) -> String {
        self.log_level.clone()
    }
}

/// 初始化 WASM 模块
#[wasm_bindgen(js_name = initializeModule)]
pub fn initialize_module(config: Option<WasmConfig>) -> Result<(), JsValue> {
    let config = config.unwrap_or_else(WasmConfig::new);
    
    // 设置 panic hook
    #[cfg(feature = "console_error_panic_hook")]
    if config.enable_logging {
        console_error_panic_hook::set_once();
    }
    
    // 设置内存分配器
    #[cfg(feature = "wee_alloc")]
    if config.enable_memory_optimization {
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
    
    if config.enable_logging {
        crate::utils::log_info(&format!(
            "RustImage WASM module initialized (version: {})",
            get_version()
        ));
    }
    
    Ok(())
}

// =============================================================================
// 状态管理
// =============================================================================

/// WASM 模块状态
#[wasm_bindgen]
pub struct ModuleState {
    initialized: bool,
    active_converters: u32,
    active_monitors: u32,
    total_conversions: u64,
    total_memory_allocated: u64,
}

static mut MODULE_STATE: ModuleState = ModuleState {
    initialized: false,
    active_converters: 0,
    active_monitors: 0,
    total_conversions: 0,
    total_memory_allocated: 0,
};

#[wasm_bindgen]
impl ModuleState {
    /// 获取模块状态
    #[wasm_bindgen(js_name = getState)]
    pub fn get_state() -> ModuleState {
        unsafe { 
            ModuleState {
                initialized: MODULE_STATE.initialized,
                active_converters: MODULE_STATE.active_converters,
                active_monitors: MODULE_STATE.active_monitors,
                total_conversions: MODULE_STATE.total_conversions,
                total_memory_allocated: MODULE_STATE.total_memory_allocated,
            }
        }
    }
    
    /// 是否已初始化
    #[wasm_bindgen(js_name = isInitialized)]
    pub fn is_initialized(&self) -> bool {
        self.initialized
    }
    
    /// 获取活跃转换器数量
    #[wasm_bindgen(js_name = getActiveConverters)]
    pub fn get_active_converters(&self) -> u32 {
        self.active_converters
    }
    
    /// 获取活跃监控器数量
    #[wasm_bindgen(js_name = getActiveMonitors)]
    pub fn get_active_monitors(&self) -> u32 {
        self.active_monitors
    }
    
    /// 获取总转换次数
    #[wasm_bindgen(js_name = getTotalConversions)]
    pub fn get_total_conversions(&self) -> u64 {
        self.total_conversions
    }
    
    /// 获取总分配内存
    #[wasm_bindgen(js_name = getTotalMemoryAllocated)]
    pub fn get_total_memory_allocated(&self) -> u64 {
        self.total_memory_allocated
    }
    
    /// 获取状态的 JSON 表示
    #[wasm_bindgen(js_name = toJson)]
    pub fn to_json(&self) -> Result<String, JsValue> {
        serde_json::to_string(&serde_json::json!({
            "initialized": self.initialized,
            "active_converters": self.active_converters,
            "active_monitors": self.active_monitors,
            "total_conversions": self.total_conversions,
            "total_memory_allocated": self.total_memory_allocated,
            "memory_info": crate::utils::get_wasm_memory_info().unwrap_or_else(|_| "{}".to_string()),
            "timestamp": get_date_timestamp()
        })).map_err(|e| JsValue::from_str(&e.to_string()))
    }
}

/// 内部状态更新函数
pub(crate) fn update_state<F>(updater: F) 
where 
    F: FnOnce(&mut ModuleState)
{
    unsafe {
        updater(&mut MODULE_STATE);
    }
}

/// 标记模块为已初始化
pub(crate) fn mark_initialized() {
    update_state(|state| {
        state.initialized = true;
    });
}

/// 增加活跃转换器计数
pub(crate) fn increment_active_converters() {
    update_state(|state| {
        state.active_converters += 1;
    });
}

/// 减少活跃转换器计数
pub(crate) fn decrement_active_converters() {
    update_state(|state| {
        state.active_converters = state.active_converters.saturating_sub(1);
    });
}

/// 增加活跃监控器计数
pub(crate) fn increment_active_monitors() {
    update_state(|state| {
        state.active_monitors += 1;
    });
}

/// 减少活跃监控器计数
pub(crate) fn decrement_active_monitors() {
    update_state(|state| {
        state.active_monitors = state.active_monitors.saturating_sub(1);
    });
}

/// 增加总转换次数
pub(crate) fn increment_total_conversions() {
    update_state(|state| {
        state.total_conversions += 1;
    });
}

/// 增加内存分配统计
pub(crate) fn add_memory_allocation(bytes: u64) {
    update_state(|state| {
        state.total_memory_allocated += bytes;
    });
}

// =============================================================================
// 健康检查和诊断
// =============================================================================

/// 执行模块健康检查
#[wasm_bindgen(js_name = healthCheck)]
pub fn health_check() -> Result<String, JsValue> {
    let mut issues = Vec::new();
    let mut suggestions = Vec::new();
    
    // 检查内存使用
    if let Ok(memory_info) = crate::utils::get_wasm_memory_info() {
        if let Ok(memory_data) = serde_json::from_str::<serde_json::Value>(&memory_info) {
            if let Some(buffer_size_mb) = memory_data["buffer_size_mb"].as_f64() {
                if buffer_size_mb > 100.0 {
                    issues.push("High memory usage detected".to_string());
                    suggestions.push("Consider calling force_garbage_collection() or destroying unused converters".to_string());
                }
            }
        }
    }
    
    // 检查活跃对象数量
    let state = ModuleState::get_state();
    if state.active_converters > 10 {
        issues.push("Many active converters detected".to_string());
        suggestions.push("Consider reusing converters or calling destroy() when done".to_string());
    }
    
    if state.active_monitors > 5 {
        issues.push("Many active monitors detected".to_string());
        suggestions.push("Consider reducing the number of performance monitors".to_string());
    }
    
    // 检查环境能力
    let env_capabilities = crate::utils::detect_environment_capabilities()?;
    let env_data: serde_json::Value = serde_json::from_str(&env_capabilities)
        .map_err(|e| JsValue::from_str(&e.to_string()))?;
    
    if !env_data["in_browser"].as_bool().unwrap_or(false) {
        issues.push("Not running in browser environment".to_string());
        suggestions.push("Some features may not be available outside browser context".to_string());
    }
    
    let health_status = if issues.is_empty() { "healthy" } else { "warning" };
    
    serde_json::to_string(&serde_json::json!({
        "status": health_status,
        "timestamp": get_date_timestamp(),
        "module_info": {
            "version": get_version(),
            "initialized": state.initialized,
            "active_converters": state.active_converters,
            "active_monitors": state.active_monitors,
            "total_conversions": state.total_conversions
        },
        "issues": issues,
        "suggestions": suggestions,
        "environment": env_data
    })).map_err(|e| JsValue::from_str(&e.to_string()))
}