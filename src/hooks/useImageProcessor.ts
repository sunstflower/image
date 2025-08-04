/**
 * 图像处理 Hook - 核心业务逻辑的封装
 * 
 * 深模块设计：隐藏 WASM 调用、文件处理、错误处理等复杂性
 * 暴露简单的 processImage 和 batchProcess 接口
 */

import { useState, useCallback, useRef } from 'react';
import { useWasm } from './useWasm';
import type { 
  UseImageProcessor, 
  ProcessedImage, 
  FilterType, 
  FilterParams,
  ImageProcessingError 
} from '../types';

interface ProcessingState {
  isProcessing: boolean;
  progress: number;
  currentOperation: string;
  error: string | null;
}

/**
 * 图像处理业务逻辑 Hook
 */
export function useImageProcessor(): UseImageProcessor {
  const { wasmModule, isReady } = useWasm();
  const [state, setState] = useState<ProcessingState>({
    isProcessing: false,
    progress: 0,
    currentOperation: '',
    error: null,
  });
  
  const cancelRef = useRef<(() => void) | null>(null);

  /**
   * 处理单张图像 - 主要的深模块接口
   */
  const processImage = useCallback(async (
    file: File,
    filterType: FilterType,
    params: FilterParams = { intensity: 0.5 }
  ): Promise<ProcessedImage> => {
    if (!wasmModule || !isReady) {
      throw new Error('WASM module not ready');
    }

    // 重置状态
    setState({
      isProcessing: true,
      progress: 0,
      currentOperation: `Applying ${filterType} filter`,
      error: null,
    });

    try {
      // 步骤 1: 文件验证和读取
      setState(prev => ({ ...prev, progress: 10 }));
      const imageData = await readFileAsUint8Array(file);
      
      // 步骤 2: 格式验证
      setState(prev => ({ ...prev, progress: 20 }));
      validateImageFormat(file);
      
      // 步骤 3: WASM 处理
      setState(prev => ({ ...prev, progress: 30, currentOperation: 'Processing image...' }));
      const startTime = performance.now();
      
      const result = wasmModule.process_image(
        imageData,
        filterType,
        params.intensity,
        params.radius
      );
      
      const endTime = performance.now();
      
      // 步骤 4: 结果转换
      setState(prev => ({ ...prev, progress: 90, currentOperation: 'Finalizing...' }));
      const processedImage: ProcessedImage = {
        id: generateImageId(),
        originalFile: file,
        processedData: result.data,
        dimensions: {
          width: result.width,
          height: result.height,
        },
        format: getImageFormat(file),
        processingTime: endTime - startTime,
        memoryUsage: result.memoryUsage,
        appliedFilters: [{
          type: filterType,
          params,
          appliedAt: new Date(),
          processingTime: result.processingTimeMs,
        }],
      };

      setState({
        isProcessing: false,
        progress: 100,
        currentOperation: 'Completed',
        error: null,
      });

      return processedImage;

    } catch (error) {
      const errorMessage = error instanceof Error ? error.message : 'Unknown error';
      setState({
        isProcessing: false,
        progress: 0,
        currentOperation: '',
        error: errorMessage,
      });
      throw new ImageProcessingError('processing', errorMessage);
    }
  }, [wasmModule, isReady]);

  /**
   * 批量处理图像
   */
  const batchProcess = useCallback(async (
    files: File[],
    operations: Array<{ filterType: FilterType; params?: FilterParams }>
  ): Promise<ProcessedImage[]> => {
    if (!wasmModule || !isReady) {
      throw new Error('WASM module not ready');
    }

    setState({
      isProcessing: true,
      progress: 0,
      currentOperation: `Processing ${files.length} images`,
      error: null,
    });

    try {
      const results: ProcessedImage[] = [];
      const totalOperations = files.length * operations.length;
      let completedOperations = 0;

      for (let i = 0; i < files.length; i++) {
        const file = files[i];
        
        for (let j = 0; j < operations.length; j++) {
          const operation = operations[j];
          
          // 检查是否取消
          if (cancelRef.current) {
            throw new Error('Operation cancelled');
          }

          setState(prev => ({
            ...prev,
            progress: (completedOperations / totalOperations) * 100,
            currentOperation: `Processing ${file.name} with ${operation.filterType}`,
          }));

          const result = await processImage(file, operation.filterType, operation.params);
          results.push(result);
          
          completedOperations++;
        }
      }

      setState({
        isProcessing: false,
        progress: 100,
        currentOperation: 'Batch processing completed',
        error: null,
      });

      return results;

    } catch (error) {
      const errorMessage = error instanceof Error ? error.message : 'Unknown error';
      setState({
        isProcessing: false,
        progress: 0,
        currentOperation: '',
        error: errorMessage,
      });
      throw new ImageProcessingError('processing', errorMessage);
    }
  }, [wasmModule, isReady, processImage]);

  /**
   * 取消当前操作
   */
  const cancel = useCallback(() => {
    cancelRef.current = () => {
      setState({
        isProcessing: false,
        progress: 0,
        currentOperation: '',
        error: 'Operation cancelled',
      });
    };
  }, []);

  return {
    processImage,
    batchProcess,
    isProcessing: state.isProcessing,
    progress: state.progress,
    error: state.error,
    cancel,
  };
}

// 辅助函数 - 隐藏实现细节

/**
 * 读取文件为 Uint8Array
 */
async function readFileAsUint8Array(file: File): Promise<Uint8Array> {
  return new Promise((resolve, reject) => {
    const reader = new FileReader();
    
    reader.onload = (event) => {
      if (event.target?.result instanceof ArrayBuffer) {
        resolve(new Uint8Array(event.target.result));
      } else {
        reject(new Error('Failed to read file as ArrayBuffer'));
      }
    };
    
    reader.onerror = () => {
      reject(new Error('File reading failed'));
    };
    
    reader.readAsArrayBuffer(file);
  });
}

/**
 * 验证图像格式
 */
function validateImageFormat(file: File): void {
  const supportedTypes = ['image/jpeg', 'image/png', 'image/webp', 'image/bmp'];
  
  if (!supportedTypes.includes(file.type)) {
    throw new Error(`Unsupported image format: ${file.type}`);
  }
  
  // 文件大小限制 (10MB)
  const maxSize = 10 * 1024 * 1024;
  if (file.size > maxSize) {
    throw new Error(`Image too large: ${(file.size / 1024 / 1024).toFixed(1)}MB (max: 10MB)`);
  }
}

/**
 * 获取图像格式
 */
function getImageFormat(file: File): 'png' | 'jpeg' | 'webp' | 'bmp' {
  switch (file.type) {
    case 'image/png': return 'png';
    case 'image/jpeg': return 'jpeg';
    case 'image/webp': return 'webp';
    case 'image/bmp': return 'bmp';
    default: return 'png'; // 默认
  }
}

/**
 * 生成图像 ID
 */
function generateImageId(): string {
  return `img_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`;
}

/**
 * 图像处理错误类
 */
class ImageProcessingError extends Error {
  constructor(
    public type: 'format' | 'processing' | 'memory' | 'network' | 'unknown',
    message: string,
    public details?: string
  ) {
    super(message);
    this.name = 'ImageProcessingError';
  }
}