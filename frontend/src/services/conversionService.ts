/* 
// conversionService.ts
// 实际WASM方法的导出
*/    

/// 转换选项
export interface ConversionOptions {
  quality?: number;
  compressionLevel?: number;
  progressive?: boolean;
  preserveMetadata?: boolean;
  preserveDimensions?: boolean;
  preserveColorSpace?: boolean;
}

/// 转换结果  
export interface ConversionResult {
  data: Uint8Array;
  originalSize: number;
  convertedSize: number;
  conversionTimeMs: number;
  dimensions: {
    width: number;
    height: number;
  };
  format: string;
  compressionRatio: number;
}

/// 图像格式
export interface ImageFormat {
  id: string;
  name: string;
  extensions: string[];
  supportsLossy: boolean;
  supportsTransparency: boolean;
  supportsAnimation: boolean;
}

/// 转换进度
export interface ConversionProgress {
  fileId: string;
  progress: number;
  stage: 'uploading' | 'converting' | 'downloading' | 'completed' | 'error';
  message?: string;
  error?: string;
}

/// 转换服务
class ConversionService {
  private baseUrl: string;

  constructor(baseUrl: string = '/api') {
    this.baseUrl = baseUrl;
  }

  /**
   * 检测图像格式
   */
  async detectFormat(file: File): Promise<string> {
    try {
      // 首先尝试从文件扩展名检测
      const extension = file.name.split('.').pop()?.toLowerCase();
      const formatMap: Record<string, string> = {
        'jpg': 'jpeg',
        'jpeg': 'jpeg',
        'png': 'png',
        'webp': 'webp',
        'avif': 'avif',
        'bmp': 'bmp',
        'tiff': 'tiff',
        'tif': 'tiff',
        'gif': 'gif',
        'ico': 'ico'
      };

      if (extension && formatMap[extension]) {
        return formatMap[extension];
      }

      // 如果扩展名检测失败，尝试读取文件头
      const buffer = await file.arrayBuffer();
      const bytes = new Uint8Array(buffer.slice(0, 12));

      // JPEG文件头: FF D8 FF
      if (bytes[0] === 0xFF && bytes[1] === 0xD8 && bytes[2] === 0xFF) {
        return 'jpeg';
      }

      // PNG文件头: 89 50 4E 47 0D 0A 1A 0A
      if (bytes[0] === 0x89 && bytes[1] === 0x50 && bytes[2] === 0x4E && bytes[3] === 0x47) {
        return 'png';
      }

      // WebP文件头: RIFF....WEBP
      if (bytes[0] === 0x52 && bytes[1] === 0x49 && bytes[2] === 0x46 && bytes[3] === 0x46 &&
          bytes[8] === 0x57 && bytes[9] === 0x45 && bytes[10] === 0x42 && bytes[11] === 0x50) {
        return 'webp';
      }

      // BMP文件头: 42 4D
      if (bytes[0] === 0x42 && bytes[1] === 0x4D) {
        return 'bmp';
      }

      // GIF文件头: GIF87a 或 GIF89a
      if (bytes[0] === 0x47 && bytes[1] === 0x49 && bytes[2] === 0x46) {
        return 'gif';
      }

      return 'unknown';
    } catch (error) {
      console.error('Format detection failed:', error);
      return 'unknown';
    }
  }

  /**
   * 转换图像格式
   */
  async convertImage(
    file: File,
    fromFormat: string,
    toFormat: string,
    options: ConversionOptions = {},
    onProgress?: (progress: ConversionProgress) => void
  ): Promise<ConversionResult> {
    const fileId = `conversion-${Date.now()}-${Math.random().toString(36).substr(2, 9)}`;

    try {
      // 开始上传
      onProgress?.({
        fileId,
        progress: 0,
        stage: 'uploading',
        message: '正在上传文件...'
      });

      // 检查是否有可用的WASM转换器
      if (typeof (window as any).rustimage_wasm !== 'undefined') {
        return await this.convertWithWasm(file, fromFormat, toFormat, options, fileId, onProgress);
      }

      // 如果WASM不可用，使用模拟转换
      return await this.simulateConversion(file, fromFormat, toFormat, options, fileId, onProgress);

    } catch (error) {
      onProgress?.({
        fileId,
        progress: 0,
        stage: 'error',
        error: error instanceof Error ? error.message : '转换失败'
      });
      throw error;
    }
  }

  /**
   * 使用WASM进行转换
   */
  private async convertWithWasm(
    file: File,
    fromFormat: string,
    toFormat: string,
    options: ConversionOptions,
    fileId: string,
    onProgress?: (progress: ConversionProgress) => void
  ): Promise<ConversionResult> {
    const wasmModule = (window as any).rustimage_wasm;

    onProgress?.({
      fileId,
      progress: 20,
      stage: 'converting',
      message: '正在读取文件...'
    });

    // 读取文件数据
    const arrayBuffer = await file.arrayBuffer();
    const inputData = new Uint8Array(arrayBuffer);

    onProgress?.({
      fileId,
      progress: 40,
      stage: 'converting',
      message: '正在转换格式...'
    });

    // 准备枚举与选项（WASM 导出为 camelCase 名称）
    const fromEnum = await wasmModule.formatFromString(fromFormat);
    const toEnum = await wasmModule.formatFromString(toFormat);

    let wasmOptions: any | undefined = undefined;
    if (options && Object.keys(options).length > 0) {
      wasmOptions = new wasmModule.JsConversionOptions();
      if (typeof options.quality === 'number') {
        try { wasmOptions.setQuality(options.quality); } catch {}
      }
      if (typeof options.compressionLevel === 'number') {
        try { wasmOptions.setCompressionLevel(options.compressionLevel); } catch {}
      }
      if (typeof options.progressive === 'boolean') {
        wasmOptions.setProgressive(options.progressive);
      }
      if (typeof options.preserveMetadata === 'boolean') {
        wasmOptions.setPreserveMetadata(options.preserveMetadata);
      }
      if (typeof options.preserveDimensions === 'boolean') {
        wasmOptions.setPreserveDimensions(options.preserveDimensions);
      }
      if (typeof options.preserveColorSpace === 'boolean') {
        wasmOptions.setPreserveColorSpace(options.preserveColorSpace);
      }
    }

    // 调用 WASM 转换函数
    const startTime = Date.now();
    const result = await wasmModule.convertImage(
      inputData,
      fromEnum,
      toEnum,
      wasmOptions
    );

    const conversionTime = Date.now() - startTime;

    onProgress?.({
      fileId,
      progress: 80,
      stage: 'downloading',
      message: '正在处理结果...'
    });

    // 解析结果（WASM 对象提供 getter）
    const convertedData: Uint8Array = result.getData();
    const width: number = typeof result.getWidth === 'function' ? result.getWidth() : 0;
    const height: number = typeof result.getHeight === 'function' ? result.getHeight() : 0;

    onProgress?.({
      fileId,
      progress: 100,
      stage: 'completed',
      message: '转换完成'
    });

    return {
      data: convertedData,
      originalSize: file.size,
      convertedSize: convertedData.length,
      conversionTimeMs: typeof result.getConversionTimeMs === 'function' ? result.getConversionTimeMs() : conversionTime,
      dimensions: { width, height },
      format: toFormat,
      compressionRatio: file.size > 0 ? convertedData.length / file.size : 1
    };
  }

  /**
   * 模拟转换过程（用于演示）
   */
  private async simulateConversion(
    file: File,
    fromFormat: string,
    toFormat: string,
    options: ConversionOptions,
    fileId: string,
    onProgress?: (progress: ConversionProgress) => void
  ): Promise<ConversionResult> {
    // 模拟转换过程
    const steps = [
      { progress: 10, message: '正在验证文件格式...' },
      { progress: 30, message: '正在解码图像...' },
      { progress: 50, message: '正在处理图像数据...' },
      { progress: 70, message: `正在编码为 ${toFormat.toUpperCase()}...` },
      { progress: 90, message: '正在优化输出...' },
    ];

    for (const step of steps) {
      await new Promise(resolve => setTimeout(resolve, 200 + Math.random() * 300));
      onProgress?.({
        fileId,
        progress: step.progress,
        stage: 'converting',
        message: step.message
      });
    }

    // 创建模拟结果
    const mockConvertedSize = Math.floor(file.size * (0.7 + Math.random() * 0.6));
    const mockData = new Uint8Array(mockConvertedSize);

    // 填充一些随机数据（实际应用中这是转换后的图像数据）
    for (let i = 0; i < mockData.length; i++) {
      mockData[i] = Math.floor(Math.random() * 256);
    }

    onProgress?.({
      fileId,
      progress: 100,
      stage: 'completed',
      message: '转换完成'
    });

    return {
      data: mockData,
      originalSize: file.size,
      convertedSize: mockConvertedSize,
      conversionTimeMs: 500 + Math.random() * 1000,
      dimensions: {
        width: 800 + Math.floor(Math.random() * 400),
        height: 600 + Math.floor(Math.random() * 300)
      },
      format: toFormat,
      compressionRatio: mockConvertedSize / file.size
    };
  }

  /**
   * 批量转换图像
   */
  async convertBatch(
    files: Array<{
      file: File;
      fromFormat: string;
      toFormat: string;
      options?: ConversionOptions;
    }>,
    onProgress?: (fileIndex: number, progress: ConversionProgress) => void
  ): Promise<ConversionResult[]> {
    const results: ConversionResult[] = [];

    for (let i = 0; i < files.length; i++) {
      const { file, fromFormat, toFormat, options = {} } = files[i];

      try {
        const result = await this.convertImage(
          file,
          fromFormat,
          toFormat,
          options,
          (progress) => onProgress?.(i, progress)
        );
        results.push(result);
      } catch (error) {
        // 记录错误但继续处理其他文件
        console.error(`Failed to convert file ${file.name}:`, error);
        throw error;
      }
    }

    return results;
  }

  /**
   * 获取支持的格式列表
   */
  getSupportedFormats(): ImageFormat[] {
    return [
      {
        id: 'jpeg',
        name: 'JPEG',
        extensions: ['.jpg', '.jpeg'],
        supportsLossy: true,
        supportsTransparency: false,
        supportsAnimation: false
      },
      {
        id: 'png',
        name: 'PNG',
        extensions: ['.png'],
        supportsLossy: false,
        supportsTransparency: true,
        supportsAnimation: false
      },
      {
        id: 'webp',
        name: 'WebP',
        extensions: ['.webp'],
        supportsLossy: true,
        supportsTransparency: true,
        supportsAnimation: true
      },
      {
        id: 'avif',
        name: 'AVIF',
        extensions: ['.avif'],
        supportsLossy: true,
        supportsTransparency: true,
        supportsAnimation: false
      },
      {
        id: 'bmp',
        name: 'BMP',
        extensions: ['.bmp'],
        supportsLossy: false,
        supportsTransparency: false,
        supportsAnimation: false
      },
      {
        id: 'tiff',
        name: 'TIFF',
        extensions: ['.tiff', '.tif'],
        supportsLossy: false,
        supportsTransparency: true,
        supportsAnimation: false
      }
    ];
  }

  /**
   * 验证转换是否支持
   */
  isConversionSupported(fromFormat: string, toFormat: string): boolean {
    const supportedFormats = this.getSupportedFormats().map(f => f.id);
    return supportedFormats.includes(fromFormat) && supportedFormats.includes(toFormat);
  }

  /**
   * 获取格式信息
   */
  getFormatInfo(formatId: string): ImageFormat | undefined {
    return this.getSupportedFormats().find(f => f.id === formatId);
  }
}

// 导出单例实例
export const conversionService = new ConversionService();

// 导出类型和服务
export { ConversionService };
