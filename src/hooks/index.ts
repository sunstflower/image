/**
 * React Hooks 接口声明 - 深模块设计在 React 中的应用
 * 
 * 这些 hooks 封装了复杂的 WebAssembly 交互、状态管理和性能监控逻辑
 * 对组件暴露简单易用的接口
 */

export { useWasm } from './useWasm';
export { useImageProcessor } from './useImageProcessor';
export { usePerformanceMonitor } from './usePerformanceMonitor';
export { useFileUpload } from './useFileUpload';

// 重新导出类型
export type {
  UseWasm,
  UseImageProcessor,
  UsePerformanceMonitor,
  UseFileUpload,
} from '../types';