/* 
// wasm-module.d.ts
// 这个文件是用于定义 wasm 模块的类型，是 wasm-pack 生成的，用于 typescript 的类型检查
*/    
// 仅声明全局 window 上的 rustimage_wasm，避免从 /public 进行模块导入
declare global {
  interface Window {
    rustimage_wasm?: any
  }
}

export {}
