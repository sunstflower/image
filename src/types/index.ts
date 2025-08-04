/**
 * TypeScript 类型定义 - 深模块设计在前端的体现
 * 
 * 对应 Rust 核心模块的类型，提供类型安全的 JavaScript 接口
 */

// WebAssembly 模块类型声明
export interface RustImageWasm {
  process_image(
    imageData: Uint8Array,
    filterType: string,
    intensity: number,
    radius?: number
  ): WasmProcessedImage;
  
  batch_process_images(
    images: WasmImageData[],
    operations: WasmFilterOperation[]
  ): WasmProcessedImage[];
  
  get_performance_metrics(): WasmPerformanceMetrics;
  get_supported_filters(): string[];
  warmup(): void;
}

// WASM 返回类型
export interface WasmProcessedImage {
  readonly data: Uint8Array;
  readonly width: number;
  readonly height: number;
  readonly processingTimeMs: number;
  readonly memoryUsage: number;
}

export interface WasmPerformanceMetrics {
  readonly totalTimeMs: number;
  readonly peakMemoryBytes: number;
  readonly cpuUsage: number;
  readonly pixelsProcessed: number;
  readonly pixelsPerSecond: number;
  readonly threadsUsed: number;
  readonly parallelEfficiency: number;
  readonly simdUtilized: boolean;
}

export interface WasmImageData {
  data: Uint8Array;
  width: number;
  height: number;
}

export interface WasmFilterOperation {
  filterType: FilterType;
  intensity: number;
  radius?: number;
  colorParams?: ColorParams;
}

// 应用层类型定义
export interface ProcessedImage {
  id: string;
  originalFile: File;
  processedData: Uint8Array;
  dimensions: ImageDimensions;
  format: ImageFormat;
  processingTime: number;
  memoryUsage: number;
  appliedFilters: AppliedFilter[];
}

export interface ImageDimensions {
  width: number;
  height: number;
}

export type ImageFormat = 'png' | 'jpeg' | 'webp' | 'bmp';

export type FilterType = 
  | 'gaussian_blur'
  | 'edge_detection' 
  | 'sharpen'
  | 'color_adjust'
  | 'noise_reduction'
  | 'super_resolution';

export interface FilterParams {
  intensity: number;
  radius?: number;
  colorParams?: ColorParams;
  customParams?: Record<string, number>;
}

export interface ColorParams {
  brightness: number;
  contrast: number;
  saturation: number;
  hue: number;
}

export interface AppliedFilter {
  type: FilterType;
  params: FilterParams;
  appliedAt: Date;
  processingTime: number;
}

// 性能监控类型
export interface PerformanceMetrics {
  totalTime: number;
  peakMemory: number;
  cpuUsage: number;
  pixelsProcessed: number;
  pixelsPerSecond: number;
  threadInfo: {
    threadsUsed: number;
    parallelEfficiency: number;
    simdUtilized: boolean;
  };
}

export interface PerformanceSnapshot {
  timestamp: Date;
  metrics: PerformanceMetrics;
  operation: string;
}

export interface PerformanceComparison {
  rustMetrics: PerformanceSnapshot[];
  jsMetrics: PerformanceSnapshot[];
  speedImprovement: number;
  memoryEfficiency: number;
  stabilityComparison: number;
}

// 应用状态类型
export interface ImageProcessingState {
  // 当前图像
  currentImage: ProcessedImage | null;
  // 处理历史
  history: ProcessedImage[];
  // 当前滤镜参数
  currentFilter: {
    type: FilterType;
    params: FilterParams;
  };
  // 处理状态
  isProcessing: boolean;
  // 错误状态
  error: string | null;
  // 性能数据
  performanceMetrics: PerformanceMetrics | null;
}

export interface UIState {
  // 当前视图模式
  viewMode: 'single' | 'comparison' | 'batch';
  // 侧边栏状态
  sidebarOpen: boolean;
  // 性能面板状态
  performancePanelOpen: boolean;
  // 主题
  theme: 'light' | 'dark';
  // 布局设置
  layout: LayoutSettings;
}

export interface LayoutSettings {
  showPreview: boolean;
  showControls: boolean;
  showPerformance: boolean;
  previewSize: 'small' | 'medium' | 'large';
}

// 组件 Props 类型
export interface ImageProcessorProps {
  onImageProcessed: (image: ProcessedImage) => void;
  onError: (error: string) => void;
  performanceMode: 'realtime' | 'batch' | 'comparison';
  className?: string;
}

export interface ImagePreviewProps {
  original: ProcessedImage | null;
  processed: ProcessedImage | null;
  showComparison: boolean;
  onZoom?: (level: number) => void;
  className?: string;
}

export interface FilterControlsProps {
  filterType: FilterType;
  params: FilterParams;
  onFilterChange: (type: FilterType) => void;
  onParamsChange: (params: FilterParams) => void;
  disabled?: boolean;
  className?: string;
}

export interface PerformanceMonitorProps {
  metrics: PerformanceMetrics | null;
  history: PerformanceSnapshot[];
  showComparison: boolean;
  onReset: () => void;
  className?: string;
}

export interface ProcessingProgressProps {
  isProcessing: boolean;
  progress: number;
  operation: string;
  estimatedTime?: number;
  className?: string;
}

// Hook 返回类型
export interface UseWasm {
  wasmModule: RustImageWasm | null;
  isLoading: boolean;
  error: string | null;
  isReady: boolean;
}

export interface UseImageProcessor {
  processImage: (file: File, filterType: FilterType, params?: FilterParams) => Promise<ProcessedImage>;
  batchProcess: (files: File[], operations: Array<{ filterType: FilterType; params?: FilterParams }>) => Promise<ProcessedImage[]>;
  isProcessing: boolean;
  progress: number;
  error: string | null;
  cancel: () => void;
}

export interface UsePerformanceMonitor {
  metrics: PerformanceMetrics | null;
  history: PerformanceSnapshot[];
  comparison: PerformanceComparison | null;
  startMeasurement: (operation: string) => void;
  endMeasurement: (operation: string) => void;
  reset: () => void;
  generateReport: () => PerformanceReport;
}

export interface UseFileUpload {
  uploadedFiles: File[];
  isDragging: boolean;
  uploadFile: (file: File) => void;
  uploadFiles: (files: File[]) => void;
  removeFile: (index: number) => void;
  clearFiles: () => void;
  getRootProps: () => Record<string, any>;
  getInputProps: () => Record<string, any>;
}

// 错误类型
export interface ImageProcessingError {
  type: 'format' | 'processing' | 'memory' | 'network' | 'unknown';
  message: string;
  details?: string;
  timestamp: Date;
}

// 配置类型
export interface AppConfig {
  // WebAssembly 模块路径
  wasmModulePath: string;
  // 默认滤镜参数
  defaultFilterParams: Record<FilterType, FilterParams>;
  // 性能监控配置
  performanceConfig: {
    enableDetailedMonitoring: boolean;
    maxHistoryEntries: number;
    samplingInterval: number;
  };
  // UI 配置
  uiConfig: {
    defaultTheme: 'light' | 'dark';
    animationDuration: number;
    maxImageSize: number;
  };
}

// 工具类型
export type DeepPartial<T> = {
  [P in keyof T]?: T[P] extends object ? DeepPartial<T[P]> : T[P];
};

export type RequiredKeys<T, K extends keyof T> = T & Required<Pick<T, K>>;

export type OptionalKeys<T, K extends keyof T> = Omit<T, K> & Partial<Pick<T, K>>;

// 性能报告类型
export interface PerformanceReport {
  summary: {
    totalProcessingTime: number;
    averageProcessingTime: number;
    fastestProcessingTime: number;
    slowestProcessingTime: number;
    totalPixelsProcessed: number;
    averagePixelsPerSecond: number;
  };
  operationDetails: Array<{
    operationName: string;
    executionCount: number;
    totalTime: number;
    averageTime: number;
    memoryStats: {
      peakUsage: number;
      averageUsage: number;
      allocationCount: number;
    };
  }>;
  trends: {
    performanceTrend: number;
    memoryTrend: number;
    stabilityScore: number;
  };
  recommendations: Array<{
    type: 'memory' | 'algorithm' | 'parallel' | 'simd' | 'cache';
    description: string;
    expectedImprovement: number;
    implementationDifficulty: number;
  }>;
}