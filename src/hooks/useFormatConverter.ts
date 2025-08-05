/**
 * 格式转换 Hook - 核心业务逻辑的封装
 * 
 * 深模块设计：隐藏 WASM 调用、格式转换、错误处理等复杂性
 * 暴露简单的 convertFormat 和 batchConvert 接口
 */

import { useState, useCallback, useRef } from 'react';
import { useWasm } from './useWasm';
import type { 
  UseFormatConverter, 
  ConvertedImage, 
  ImageFormat, 
  ConversionOptions,
  ImageInput,
  ConversionTask,
  ConversionError,
  FormatInfo
} from '../types';
import { FORMAT_INFO, DEFAULT_FORMAT_OPTIONS } from '../types';

interface ConversionState {
  isConverting: boolean;
  progress: number;
  currentOperation: string;
  error: string | null;
}

/**
 * 格式转换业务逻辑 Hook
 */
export function useFormatConverter(): UseFormatConverter {
  const { wasmModule, isReady } = useWasm();
  const [state, setState] = useState<ConversionState>({
    isConverting: false,
    progress: 0,
    currentOperation: '',
    error: null,
  });
  
  const cancelRef = useRef<(() => void) | null>(null);

  /**
   * 转换单个图像格式 - 主要的深模块接口
   */
  const convertFormat = useCallback(async (
    input: ImageInput,
    targetFormat: ImageFormat,
    options: ConversionOptions = {}
  ): Promise<ConvertedImage> => {
    if (!wasmModule || !isReady) {
      throw new Error('WASM module not ready');
    }

    // 合并默认选项
    const finalOptions = { ...DEFAULT_FORMAT_OPTIONS[targetFormat], ...options };

    // 重置状态
    setState({
      isConverting: true,
      progress: 0,
      currentOperation: `Converting ${input.format} to ${targetFormat}`,
      error: null,
    });

    try {
      // 步骤 1: 输入验证
      setState(prev => ({ ...prev, progress: 10 }));
      validateConversion(input.format, targetFormat);
      
      // 步骤 2: 格式转换
      setState(prev => ({ ...prev, progress: 30, currentOperation: 'Converting format...' }));
      const startTime = performance.now();
      
      const result = wasmModule.convert_format(
        input.data,
        input.format,
        targetFormat,
        finalOptions.quality,
        finalOptions.compressionLevel
      );
      
      const endTime = performance.now();
      
      // 步骤 3: 结果处理
      setState(prev => ({ ...prev, progress: 90, currentOperation: 'Finalizing...' }));
      
      const convertedImage: ConvertedImage = {
        id: generateImageId(),
        originalFile: input.file,
        originalFormat: input.format,
        targetFormat,
        convertedData: result.data,
        dimensions: {
          width: result.width,
          height: result.height,
        },
        conversionTime: endTime - startTime,
        originalSize: input.data.length,
        convertedSize: result.convertedSize,
        compressionRatio: result.compressionRatio,
        appliedOptions: finalOptions,
        convertedAt: new Date(),
      };

      setState({
        isConverting: false,
        progress: 100,
        currentOperation: 'Conversion completed',
        error: null,
      });

      return convertedImage;

    } catch (error) {
      const errorMessage = error instanceof Error ? error.message : 'Unknown error';
      setState({
        isConverting: false,
        progress: 0,
        currentOperation: '',
        error: errorMessage,
      });
      throw new ConversionError('conversion', errorMessage);
    }
  }, [wasmModule, isReady]);

  /**
   * 批量格式转换
   */
  const batchConvert = useCallback(async (
    inputs: ImageInput[],
    tasks: ConversionTask[]
  ): Promise<ConvertedImage[]> => {
    if (!wasmModule || !isReady) {
      throw new Error('WASM module not ready');
    }

    setState({
      isConverting: true,
      progress: 0,
      currentOperation: `Converting ${inputs.length} images`,
      error: null,
    });

    try {
      const results: ConvertedImage[] = [];
      const totalOperations = inputs.length * tasks.length;
      let completedOperations = 0;

      for (let i = 0; i < inputs.length; i++) {
        const input = inputs[i];
        
        for (let j = 0; j < tasks.length; j++) {
          const task = tasks[j];
          
          // 检查是否取消
          if (cancelRef.current) {
            throw new Error('Operation cancelled');
          }

          setState(prev => ({
            ...prev,
            progress: (completedOperations / totalOperations) * 100,
            currentOperation: `Converting ${input.file.name} to ${task.toFormat}`,
          }));

          const result = await convertFormat(input, task.toFormat, task.options);
          results.push(result);
          
          completedOperations++;
        }
      }

      setState({
        isConverting: false,
        progress: 100,
        currentOperation: 'Batch conversion completed',
        error: null,
      });

      return results;

    } catch (error) {
      const errorMessage = error instanceof Error ? error.message : 'Unknown error';
      setState({
        isConverting: false,
        progress: 0,
        currentOperation: '',
        error: errorMessage,
      });
      throw new ConversionError('conversion', errorMessage);
    }
  }, [wasmModule, isReady, convertFormat]);

  /**
   * 检测图像格式
   */
  const detectFormat = useCallback(async (file: File): Promise<ImageFormat> => {
    if (!wasmModule || !isReady) {
      throw new Error('WASM module not ready');
    }

    try {
      const arrayBuffer = await file.arrayBuffer();
      const uint8Array = new Uint8Array(arrayBuffer);
      const detectedFormat = wasmModule.detect_format(uint8Array);
      return detectedFormat as ImageFormat;
    } catch (error) {
      // 如果检测失败，尝试从文件扩展名推测
      const extension = file.name.split('.').pop()?.toLowerCase();
      return guessFormatFromExtension(extension || '');
    }
  }, [wasmModule, isReady]);

  /**
   * 获取支持的格式列表
   */
  const getSupportedFormats = useCallback((): ImageFormat[] => {
    if (!wasmModule || !isReady) {
      return Object.keys(FORMAT_INFO) as ImageFormat[];
    }
    
    try {
      const formats = wasmModule.get_supported_formats();
      return formats as ImageFormat[];
    } catch (error) {
      console.warn('Failed to get supported formats from WASM:', error);
      return Object.keys(FORMAT_INFO) as ImageFormat[];
    }
  }, [wasmModule, isReady]);

  /**
   * 获取格式信息
   */
  const getFormatInfo = useCallback((format: ImageFormat): FormatInfo | null => {
    return FORMAT_INFO[format] || null;
  }, []);

  /**
   * 取消当前操作
   */
  const cancel = useCallback(() => {
    cancelRef.current = () => {
      setState({
        isConverting: false,
        progress: 0,
        currentOperation: '',
        error: 'Operation cancelled',
      });
    };
  }, []);

  return {
    convertFormat,
    batchConvert,
    detectFormat,
    getSupportedFormats,
    getFormatInfo,
    isConverting: state.isConverting,
    progress: state.progress,
    error: state.error,
    cancel,
  };
}

// 辅助函数 - 隐藏实现细节

/**
 * 验证格式转换是否支持
 */
function validateConversion(fromFormat: ImageFormat, toFormat: ImageFormat): void {
  if (fromFormat === toFormat) {
    throw new Error('Source and target formats are the same');
  }
  
  const supportedFormats = Object.keys(FORMAT_INFO) as ImageFormat[];
  
  if (!supportedFormats.includes(fromFormat)) {
    throw new Error(`Unsupported source format: ${fromFormat}`);
  }
  
  if (!supportedFormats.includes(toFormat)) {
    throw new Error(`Unsupported target format: ${toFormat}`);
  }
}

/**
 * 从文件扩展名推测格式
 */
function guessFormatFromExtension(extension: string): ImageFormat {
  const extensionMap: Record<string, ImageFormat> = {
    'jpg': 'jpeg',
    'jpeg': 'jpeg',
    'png': 'png',
    'webp': 'webp',
    'avif': 'avif',
    'bmp': 'bmp',
    'tiff': 'tiff',
    'tif': 'tiff',
    'gif': 'gif',
    'ico': 'ico',
  };
  
  return extensionMap[extension] || 'png'; // 默认为 PNG
}

/**
 * 生成图像 ID
 */
function generateImageId(): string {
  return `img_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`;
}

/**
 * 格式转换错误类
 */
class ConversionError extends Error {
  constructor(
    public type: 'format' | 'conversion' | 'memory' | 'network' | 'unknown',
    message: string,
    public details?: string
  ) {
    super(message);
    this.name = 'ConversionError';
  }
}