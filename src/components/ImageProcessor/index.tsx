/**
 * 图像处理器组件 - 深模块的 React 体现
 * 
 * 这是用户交互的主要入口，封装了所有图像处理相关的复杂逻辑
 * 对外暴露极简的 props 接口
 */

import React from 'react';
import type { ImageProcessorProps } from '../../types';

/**
 * 主图像处理组件
 * 
 * 深模块设计：
 * - 简单的 props 接口
 * - 内部封装文件上传、处理、预览、错误处理等复杂逻辑
 * - 自动管理状态和生命周期
 */
export const ImageProcessor: React.FC<ImageProcessorProps> = ({
  onImageProcessed,
  onError,
  performanceMode,
  className,
}) => {
  // TODO: 实现组件逻辑
  return (
    <div className={className}>
      <h2>Image Processor - Interface Declaration</h2>
      <p>Performance Mode: {performanceMode}</p>
      {/* 组件实现将在后续添加 */}
    </div>
  );
};

// 组件的默认 props
ImageProcessor.defaultProps = {
  performanceMode: 'realtime',
  className: '',
};