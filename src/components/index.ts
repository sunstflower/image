/**
 * React 组件入口 - 导出所有组件接口
 * 
 * 遵循深模块设计原则，每个组件都有简单的 props 接口
 * 隐藏内部复杂的状态管理和业务逻辑
 */

// 主要组件
export { ImageProcessor } from './ImageProcessor';
export { PerformanceMonitor } from './PerformanceMonitor';

// UI 组件
export { Button } from './UI/Button';
export { Card } from './UI/Card';
export { Slider } from './UI/Slider';
export { LoadingSpinner } from './UI/LoadingSpinner';

// 重新导出组件 Props 类型
export type {
  ImageProcessorProps,
  ImagePreviewProps,
  FilterControlsProps,
  PerformanceMonitorProps,
  ProcessingProgressProps,
} from '../types';