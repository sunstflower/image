/**
 * TypeScript 类型定义 - 专注于图像格式转换的深模块设计
 * 
 * 对应 Rust 核心模块的类型，提供类型安全的格式转换接口
 */

// WebAssembly 模块类型声明
export interface RustImageWasm {
  convert_format(
    imageData: Uint8Array,
    fromFormat: string,
    toFormat: string,
    quality?: number,
    compressionLevel?: number
  ): WasmConvertedImage;
  
  batch_convert(
    images: WasmImageInput[],
    conversions: WasmConversionTask[]
  ): WasmConvertedImage[];
  
  detect_format(imageData: Uint8Array): string;
  get_format_info(format: string): WasmFormatInfo;
  get_supported_formats(): string[];
  get_performance_metrics(): WasmPerformanceMetrics;
  warmup(): void;
}

// WASM 返回类型
export interface WasmConvertedImage {
  readonly data: Uint8Array;
  readonly width: number;
  readonly height: number;
  readonly format: string;
  readonly conversionTimeMs: number;
  readonly originalSize: number;
  readonly convertedSize: number;
  readonly compressionRatio: number;
}

export interface WasmImageInput {
  data: Uint8Array;
  format: string;
  filename?: string;
}

export interface WasmConversionTask {
  fromFormat: string;
  toFormat: string;
  quality?: number;
  compressionLevel?: number;
  progressive?: boolean;
}

export interface WasmFormatInfo {
  readonly name: string;
  readonly description: string;
  readonly extensions: string[];
  readonly mimeType: string;
  readonly supportsTransparency: boolean;
  readonly supportsAnimation: boolean;
  readonly isLossy: boolean;
}

export interface WasmPerformanceMetrics {
  readonly totalTimeMs: number;
  readonly peakMemoryBytes: number;
  readonly cpuUsage: number;
  readonly imagesProcessed: number;
  readonly imagesPerSecond: number;
  readonly totalDataBytes: number;
  readonly throughputMbps: number;
  readonly threadsUsed: number;
  readonly parallelEfficiency: number;
  readonly simdUtilized: boolean;
}

// 应用层类型定义
export type ImageFormat = 
  | 'jpeg'
  | 'png' 
  | 'webp'
  | 'avif'
  | 'bmp'
  | 'tiff'
  | 'gif'
  | 'ico';

export interface ConversionOptions {
  /** 质量参数 [0.0, 1.0] (适用于有损格式) */
  quality?: number;
  /** 压缩级别 [0, 9] (适用于无损格式) */
  compressionLevel?: number;
  /** 是否启用渐进式编码 */
  progressive?: boolean;
  /** 是否保持原图尺寸 */
  preserveDimensions?: boolean;
  /** 是否保持色彩空间 */
  preserveColorSpace?: boolean;
  /** 是否保持元数据 */
  preserveMetadata?: boolean;
}

export interface ConversionTask {
  fromFormat: ImageFormat;
  toFormat: ImageFormat;
  options?: ConversionOptions;
}

export interface ImageInput {
  file: File;
  format: ImageFormat;
  data: Uint8Array;
}

export interface ConvertedImage {
  id: string;
  originalFile: File;
  originalFormat: ImageFormat;
  targetFormat: ImageFormat;
  convertedData: Uint8Array;
  dimensions: ImageDimensions;
  conversionTime: number;
  originalSize: number;
  convertedSize: number;
  compressionRatio: number;
  qualityMetrics?: QualityMetrics;
  appliedOptions: ConversionOptions;
  convertedAt: Date;
}

export interface ImageDimensions {
  width: number;
  height: number;
}

export interface QualityMetrics {
  /** 峰值信噪比 (PSNR) */
  psnr: number;
  /** 结构相似性指数 (SSIM) */
  ssim: number;
  /** 感知哈希相似度 */
  perceptualSimilarity: number;
}

export interface FormatInfo {
  name: string;
  description: string;
  extensions: string[];
  mimeType: string;
  supportsTransparency: boolean;
  supportsAnimation: boolean;
  isLossy: boolean;
  maxDimensions?: ImageDimensions;
  colorDepths: number[];
}

// 性能监控类型
export interface PerformanceMetrics {
  totalTime: number;
  peakMemory: number;
  cpuUsage: number;
  imagesProcessed: number;
  imagesPerSecond: number;
  totalDataBytes: number;
  throughputMbps: number;
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
export interface ConversionState {
  // 当前转换任务
  currentTask: ConversionTask | null;
  // 输入图像列表
  inputImages: ImageInput[];
  // 转换结果
  convertedImages: ConvertedImage[];
  // 转换历史
  history: ConvertedImage[];
  // 当前转换状态
  isConverting: boolean;
  // 转换进度 (0-100)
  progress: number;
  // 错误状态
  error: string | null;
  // 性能数据
  performanceMetrics: PerformanceMetrics | null;
}

export interface UIState {
  // 当前视图模式
  viewMode: 'single' | 'batch' | 'comparison';
  // 侧边栏状态
  sidebarOpen: boolean;
  // 性能面板状态
  performancePanelOpen: boolean;
  // 主题
  theme: 'light' | 'dark';
  // 布局设置
  layout: LayoutSettings;
  // 选中的格式
  selectedFromFormat: ImageFormat | null;
  selectedToFormat: ImageFormat | null;
}

export interface LayoutSettings {
  showPreview: boolean;
  showControls: boolean;
  showPerformance: boolean;
  showQualityMetrics: boolean;
  previewSize: 'small' | 'medium' | 'large';
}

// 组件 Props 类型
export interface FormatConverterProps {
  onConversionComplete: (images: ConvertedImage[]) => void;
  onError: (error: string) => void;
  performanceMode: 'realtime' | 'batch' | 'comparison';
  className?: string;
}

export interface FormatSelectorProps {
  availableFormats: ImageFormat[];
  selectedFormat: ImageFormat | null;
  onFormatSelect: (format: ImageFormat) => void;
  label: string;
  disabled?: boolean;
  className?: string;
}

export interface ConversionOptionsProps {
  format: ImageFormat;
  options: ConversionOptions;
  onOptionsChange: (options: ConversionOptions) => void;
  disabled?: boolean;
  className?: string;
}

export interface ImagePreviewProps {
  original: ImageInput | null;
  converted: ConvertedImage | null;
  showComparison: boolean;
  onZoom?: (level: number) => void;
  className?: string;
}

export interface ConversionProgressProps {
  isConverting: boolean;
  progress: number;
  currentOperation: string;
  estimatedTime?: number;
  className?: string;
}

export interface QualityMetricsProps {
  metrics: QualityMetrics | null;
  originalSize: number;
  convertedSize: number;
  compressionRatio: number;
  className?: string;
}

export interface PerformanceMonitorProps {
  metrics: PerformanceMetrics | null;
  history: PerformanceSnapshot[];
  showComparison: boolean;
  onReset: () => void;
  className?: string;
}

// Hook 返回类型
export interface UseWasm {
  wasmModule: RustImageWasm | null;
  isLoading: boolean;
  error: string | null;
  isReady: boolean;
}

export interface UseFormatConverter {
  convertFormat: (
    input: ImageInput, 
    targetFormat: ImageFormat, 
    options?: ConversionOptions
  ) => Promise<ConvertedImage>;
  
  batchConvert: (
    inputs: ImageInput[], 
    tasks: ConversionTask[]
  ) => Promise<ConvertedImage[]>;
  
  detectFormat: (file: File) => Promise<ImageFormat>;
  getSupportedFormats: () => ImageFormat[];
  getFormatInfo: (format: ImageFormat) => FormatInfo | null;
  
  isConverting: boolean;
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
export interface ConversionError {
  type: 'format' | 'conversion' | 'memory' | 'network' | 'unknown';
  message: string;
  details?: string;
  timestamp: Date;
}

// 配置类型
export interface AppConfig {
  // WebAssembly 模块路径
  wasmModulePath: string;
  // 默认转换选项
  defaultConversionOptions: Record<ImageFormat, ConversionOptions>;
  // 支持的格式列表
  supportedFormats: ImageFormat[];
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
    maxFileSize: number;
    maxBatchSize: number;
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
    totalConversionTime: number;
    averageConversionTime: number;
    fastestConversionTime: number;
    slowestConversionTime: number;
    totalImagesProcessed: number;
    averageImagesPerSecond: number;
    totalDataProcessed: number;
    averageThroughput: number;
  };
  formatDetails: Array<{
    format: ImageFormat;
    conversions: number;
    totalTime: number;
    averageTime: number;
    averageCompressionRatio: number;
  }>;
  trends: {
    performanceTrend: number;
    memoryTrend: number;
    stabilityScore: number;
  };
  recommendations: Array<{
    type: 'format' | 'settings' | 'performance' | 'quality';
    description: string;
    expectedImprovement: number;
    implementationDifficulty: number;
  }>;
}

// 格式特定的默认选项
export const DEFAULT_FORMAT_OPTIONS: Record<ImageFormat, ConversionOptions> = {
  jpeg: {
    quality: 0.8,
    progressive: false,
    preserveDimensions: true,
    preserveColorSpace: true,
    preserveMetadata: false,
  },
  png: {
    compressionLevel: 6,
    preserveDimensions: true,
    preserveColorSpace: true,
    preserveMetadata: false,
  },
  webp: {
    quality: 0.8,
    preserveDimensions: true,
    preserveColorSpace: true,
    preserveMetadata: false,
  },
  avif: {
    quality: 0.7,
    preserveDimensions: true,
    preserveColorSpace: true,
    preserveMetadata: false,
  },
  bmp: {
    preserveDimensions: true,
    preserveColorSpace: true,
    preserveMetadata: false,
  },
  tiff: {
    compressionLevel: 6,
    preserveDimensions: true,
    preserveColorSpace: true,
    preserveMetadata: true,
  },
  gif: {
    preserveDimensions: true,
    preserveColorSpace: false,
    preserveMetadata: false,
  },
  ico: {
    preserveDimensions: false,
    preserveColorSpace: true,
    preserveMetadata: false,
  },
};

// 格式信息常量
export const FORMAT_INFO: Record<ImageFormat, FormatInfo> = {
  jpeg: {
    name: 'JPEG',
    description: '有损压缩格式，适合照片',
    extensions: ['jpg', 'jpeg'],
    mimeType: 'image/jpeg',
    supportsTransparency: false,
    supportsAnimation: false,
    isLossy: true,
    colorDepths: [8],
  },
  png: {
    name: 'PNG',
    description: '无损压缩格式，支持透明度',
    extensions: ['png'],
    mimeType: 'image/png',
    supportsTransparency: true,
    supportsAnimation: false,
    isLossy: false,
    colorDepths: [8, 16],
  },
  webp: {
    name: 'WebP',
    description: '现代格式，高压缩比',
    extensions: ['webp'],
    mimeType: 'image/webp',
    supportsTransparency: true,
    supportsAnimation: true,
    isLossy: true,
    colorDepths: [8],
  },
  avif: {
    name: 'AVIF',
    description: '新一代格式，最高压缩比',
    extensions: ['avif'],
    mimeType: 'image/avif',
    supportsTransparency: true,
    supportsAnimation: true,
    isLossy: true,
    colorDepths: [8, 10, 12],
  },
  bmp: {
    name: 'BMP',
    description: '无压缩格式，兼容性好',
    extensions: ['bmp'],
    mimeType: 'image/bmp',
    supportsTransparency: false,
    supportsAnimation: false,
    isLossy: false,
    colorDepths: [8, 16, 24, 32],
  },
  tiff: {
    name: 'TIFF',
    description: '专业格式，支持多层',
    extensions: ['tiff', 'tif'],
    mimeType: 'image/tiff',
    supportsTransparency: true,
    supportsAnimation: false,
    isLossy: false,
    colorDepths: [8, 16, 32],
  },
  gif: {
    name: 'GIF',
    description: '支持动画的格式',
    extensions: ['gif'],
    mimeType: 'image/gif',
    supportsTransparency: true,
    supportsAnimation: true,
    isLossy: false,
    colorDepths: [8],
  },
  ico: {
    name: 'ICO',
    description: '图标格式',
    extensions: ['ico'],
    mimeType: 'image/x-icon',
    supportsTransparency: true,
    supportsAnimation: false,
    isLossy: false,
    colorDepths: [8, 16, 24, 32],
  },
};