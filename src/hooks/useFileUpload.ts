/**
 * 文件上传 Hook - 封装文件拖拽、验证、上传逻辑
 * 
 * 深模块设计：隐藏复杂的文件处理、拖拽事件、验证逻辑
 * 暴露简单的上传接口和状态
 */

import { useState, useCallback, useRef } from 'react';
import { useDropzone } from 'react-dropzone';
import type { UseFileUpload } from '../types';

interface FileUploadState {
  uploadedFiles: File[];
  isDragging: boolean;
  error: string | null;
}

/**
 * 文件上传和拖拽 Hook
 */
export function useFileUpload(options?: {
  maxFiles?: number;
  maxSize?: number;
  acceptedTypes?: string[];
}): UseFileUpload {
  const [state, setState] = useState<FileUploadState>({
    uploadedFiles: [],
    isDragging: false,
    error: null,
  });
  
  const optionsRef = useRef({
    maxFiles: options?.maxFiles || 10,
    maxSize: options?.maxSize || 10 * 1024 * 1024, // 10MB
    acceptedTypes: options?.acceptedTypes || [
      'image/jpeg',
      'image/png', 
      'image/webp',
      'image/bmp'
    ],
  });

  /**
   * 上传单个文件
   */
  const uploadFile = useCallback((file: File) => {
    const validation = validateFile(file, optionsRef.current);
    if (!validation.isValid) {
      setState(prev => ({ ...prev, error: validation.error }));
      return;
    }

    setState(prev => {
      if (prev.uploadedFiles.length >= optionsRef.current.maxFiles) {
        return {
          ...prev,
          error: `Maximum ${optionsRef.current.maxFiles} files allowed`,
        };
      }
      
      // 检查重复文件
      const isDuplicate = prev.uploadedFiles.some(
        existingFile => existingFile.name === file.name && existingFile.size === file.size
      );
      
      if (isDuplicate) {
        return {
          ...prev,
          error: `File "${file.name}" already exists`,
        };
      }

      return {
        ...prev,
        uploadedFiles: [...prev.uploadedFiles, file],
        error: null,
      };
    });
  }, []);

  /**
   * 上传多个文件
   */
  const uploadFiles = useCallback((files: File[]) => {
    const validFiles: File[] = [];
    let errorMessage = '';

    for (const file of files) {
      const validation = validateFile(file, optionsRef.current);
      if (validation.isValid) {
        validFiles.push(file);
      } else {
        errorMessage = validation.error;
        break;
      }
    }

    if (errorMessage) {
      setState(prev => ({ ...prev, error: errorMessage }));
      return;
    }

    setState(prev => {
      const newFiles = validFiles.filter(file => 
        !prev.uploadedFiles.some(existing => 
          existing.name === file.name && existing.size === file.size
        )
      );

      const totalFiles = prev.uploadedFiles.length + newFiles.length;
      if (totalFiles > optionsRef.current.maxFiles) {
        return {
          ...prev,
          error: `Cannot upload ${newFiles.length} files. Maximum ${optionsRef.current.maxFiles} files allowed`,
        };
      }

      return {
        ...prev,
        uploadedFiles: [...prev.uploadedFiles, ...newFiles],
        error: null,
      };
    });
  }, []);

  /**
   * 移除文件
   */
  const removeFile = useCallback((index: number) => {
    setState(prev => ({
      ...prev,
      uploadedFiles: prev.uploadedFiles.filter((_, i) => i !== index),
      error: null,
    }));
  }, []);

  /**
   * 清空所有文件
   */
  const clearFiles = useCallback(() => {
    setState({
      uploadedFiles: [],
      isDragging: false,
      error: null,
    });
  }, []);

  // 使用 react-dropzone
  const {
    getRootProps,
    getInputProps,
    isDragActive,
    isDragReject,
  } = useDropzone({
    accept: optionsRef.current.acceptedTypes.reduce((acc, type) => {
      acc[type] = [];
      return acc;
    }, {} as Record<string, string[]>),
    maxFiles: optionsRef.current.maxFiles,
    maxSize: optionsRef.current.maxSize,
    onDrop: (acceptedFiles, rejectedFiles) => {
      if (rejectedFiles.length > 0) {
        const rejection = rejectedFiles[0];
        let errorMessage = 'File rejected';
        
        if (rejection.errors[0]?.code === 'file-too-large') {
          errorMessage = `File too large. Maximum size: ${formatFileSize(optionsRef.current.maxSize)}`;
        } else if (rejection.errors[0]?.code === 'file-invalid-type') {
          errorMessage = 'Invalid file type. Only images are allowed.';
        }
        
        setState(prev => ({ ...prev, error: errorMessage }));
        return;
      }

      if (acceptedFiles.length > 0) {
        uploadFiles(acceptedFiles);
      }
    },
    onDragEnter: () => {
      setState(prev => ({ ...prev, isDragging: true, error: null }));
    },
    onDragLeave: () => {
      setState(prev => ({ ...prev, isDragging: false }));
    },
  });

  return {
    uploadedFiles: state.uploadedFiles,
    isDragging: isDragActive || state.isDragging,
    uploadFile,
    uploadFiles,
    removeFile,
    clearFiles,
    getRootProps,
    getInputProps,
  };
}

// 辅助函数 - 隐藏实现细节

/**
 * 验证文件
 */
function validateFile(
  file: File,
  options: {
    maxSize: number;
    acceptedTypes: string[];
  }
): { isValid: boolean; error: string } {
  // 文件类型验证
  if (!options.acceptedTypes.includes(file.type)) {
    return {
      isValid: false,
      error: `Invalid file type: ${file.type}. Only images are allowed.`,
    };
  }

  // 文件大小验证
  if (file.size > options.maxSize) {
    return {
      isValid: false,
      error: `File too large: ${formatFileSize(file.size)}. Maximum size: ${formatFileSize(options.maxSize)}`,
    };
  }

  // 文件名验证
  if (file.name.length > 255) {
    return {
      isValid: false,
      error: 'File name too long (maximum 255 characters)',
    };
  }

  // 空文件验证
  if (file.size === 0) {
    return {
      isValid: false,
      error: 'Empty file not allowed',
    };
  }

  return { isValid: true, error: '' };
}

/**
 * 格式化文件大小
 */
function formatFileSize(bytes: number): string {
  if (bytes === 0) return '0 Bytes';
  
  const k = 1024;
  const sizes = ['Bytes', 'KB', 'MB', 'GB'];
  const i = Math.floor(Math.log(bytes) / Math.log(k));
  
  return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i];
}