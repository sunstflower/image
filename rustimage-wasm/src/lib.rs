//! # RustImage WASM - WebAssembly 绑定层
//!
//! 本模块遵循《软件设计哲学》的深模块设计理念：
//! - **深模块设计**：提供简单的 JavaScript 接口，隐藏复杂的 Rust 实现
//! - **信息隐藏**：封装 WASM 绑定细节，暴露直观的 Web API
//! - **零成本抽象**：高效的内存管理和类型转换
//! - **分层架构**：JS 接口 -> WASM 绑定 -> Rust 核心

mod bindings;     // WASM 绑定定义
mod conversion;   // 格式转换接口
mod performance;  // 性能监控接口  
mod types;        // 类型转换和工具
mod utils;        // WASM 实用工具

// 重新导出公共接口
pub use bindings::*;
pub use conversion::*;
pub use performance::*;
pub use types::*;
pub use utils::*;

use wasm_bindgen::prelude::*;

// 设置 panic hook 用于更好的错误调试
#[cfg(feature = "console_error_panic_hook")]
pub fn set_panic_hook() {
    console_error_panic_hook::set_once();
}

// WASM 模块初始化
#[wasm_bindgen(start)]
pub fn main() {
    #[cfg(feature = "console_error_panic_hook")]
    set_panic_hook();
    
    #[cfg(feature = "wee_alloc")]
    static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    
    utils::log_message_to_console("RustImage WASM module initialized successfully");
}