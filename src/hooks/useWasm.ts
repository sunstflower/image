/**
 * WebAssembly 集成 Hook - 深模块的体现
 * 
 * 隐藏 WASM 模块加载、初始化、错误处理等复杂性
 * 对外提供简单的 isReady 状态和 wasmModule 实例
 */

import { useState, useEffect, useRef } from 'react';
import type { RustImageWasm, UseWasm } from '../types';

interface WasmLoadingState {
  isLoading: boolean;
  error: string | null;
  wasmModule: RustImageWasm | null;
}

/**
 * WebAssembly 模块加载和管理 Hook
 * 
 * @param wasmPath - WASM 模块路径
 * @returns WASM 模块状态和实例
 */
export function useWasm(wasmPath?: string): UseWasm {
  const [state, setState] = useState<WasmLoadingState>({
    isLoading: true,
    error: null,
    wasmModule: null,
  });
  
  const loadingRef = useRef(false);
  const moduleRef = useRef<RustImageWasm | null>(null);

  useEffect(() => {
    // 防止重复加载
    if (loadingRef.current) return;
    loadingRef.current = true;

    const loadWasm = async () => {
      try {
        setState(prev => ({ ...prev, isLoading: true, error: null }));

        // 动态导入 WASM 模块
        const wasmModule = await loadWasmModule(wasmPath);
        
        // 初始化模块
        await initializeWasmModule(wasmModule);
        
        // 预热模块
        await warmupWasmModule(wasmModule);
        
        moduleRef.current = wasmModule;
        setState({
          isLoading: false,
          error: null,
          wasmModule,
        });
        
      } catch (error) {
        const errorMessage = error instanceof Error ? error.message : 'Unknown error';
        setState({
          isLoading: false,
          error: `Failed to load WASM module: ${errorMessage}`,
          wasmModule: null,
        });
      }
    };

    loadWasm();

    // 清理函数
    return () => {
      if (moduleRef.current) {
        cleanupWasmModule(moduleRef.current);
        moduleRef.current = null;
      }
    };
  }, [wasmPath]);

  return {
    wasmModule: state.wasmModule,
    isLoading: state.isLoading,
    error: state.error,
    isReady: !state.isLoading && !state.error && !!state.wasmModule,
  };
}

// 辅助函数 - 隐藏实现细节

/**
 * 加载 WASM 模块
 */
async function loadWasmModule(wasmPath?: string): Promise<RustImageWasm> {
  // 默认路径
  const modulePath = wasmPath || '/wasm/rustimage_wasm.js';
  
  // 动态导入
  const wasmModule = await import(/* webpackIgnore: true */ modulePath);
  
  // 初始化模块
  await wasmModule.default();
  
  return wasmModule as RustImageWasm;
}

/**
 * 初始化 WASM 模块
 */
async function initializeWasmModule(wasmModule: RustImageWasm): Promise<void> {
  // 设置 panic hook
  if ('init_panic_hook' in wasmModule && typeof wasmModule.init_panic_hook === 'function') {
    wasmModule.init_panic_hook();
  }
  
  // 其他初始化逻辑
  console.log('WASM module initialized successfully');
}

/**
 * 预热 WASM 模块
 */
async function warmupWasmModule(wasmModule: RustImageWasm): Promise<void> {
  try {
    // 调用预热函数
    wasmModule.warmup();
    
    // 预加载支持的滤镜列表
    const supportedFilters = wasmModule.get_supported_filters();
    console.log('Supported filters:', supportedFilters);
    
  } catch (error) {
    console.warn('WASM module warmup failed:', error);
    // 预热失败不应该阻止模块使用
  }
}

/**
 * 清理 WASM 模块
 */
function cleanupWasmModule(wasmModule: RustImageWasm): void {
  try {
    // 清理资源
    if ('cleanup' in wasmModule && typeof wasmModule.cleanup === 'function') {
      wasmModule.cleanup();
    }
    console.log('WASM module cleaned up');
  } catch (error) {
    console.warn('WASM module cleanup failed:', error);
  }
}