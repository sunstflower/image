/**
 * 图像处理状态管理 - Zustand Store
 */

import { create } from 'zustand';
import type { ImageProcessingState, ProcessedImage, FilterType, FilterParams } from '../types';

interface ImageStoreActions {
  // 图像操作
  setCurrentImage: (image: ProcessedImage | null) => void;
  addToHistory: (image: ProcessedImage) => void;
  clearHistory: () => void;
  
  // 滤镜操作
  setCurrentFilter: (type: FilterType, params: FilterParams) => void;
  updateFilterParams: (params: Partial<FilterParams>) => void;
  
  // 处理状态
  setProcessing: (isProcessing: boolean) => void;
  setError: (error: string | null) => void;
  
  // 重置
  reset: () => void;
}

type ImageStore = ImageProcessingState & ImageStoreActions;

/**
 * 图像处理状态管理 Hook
 * 
 * 深模块设计：隐藏状态更新逻辑，暴露简单的操作接口
 */
export const useImageStore = create<ImageStore>((set, get) => ({
  // 初始状态
  currentImage: null,
  history: [],
  currentFilter: {
    type: 'gaussian_blur',
    params: { intensity: 0.5 },
  },
  isProcessing: false,
  error: null,
  performanceMetrics: null,

  // Actions
  setCurrentImage: (image) => {
    set({ currentImage: image });
  },

  addToHistory: (image) => {
    set((state) => ({
      history: [...state.history.slice(-19), image], // 保持最近 20 张
    }));
  },

  clearHistory: () => {
    set({ history: [] });
  },

  setCurrentFilter: (type, params) => {
    set({ currentFilter: { type, params } });
  },

  updateFilterParams: (params) => {
    set((state) => ({
      currentFilter: {
        ...state.currentFilter,
        params: { ...state.currentFilter.params, ...params },
      },
    }));
  },

  setProcessing: (isProcessing) => {
    set({ isProcessing });
  },

  setError: (error) => {
    set({ error });
  },

  reset: () => {
    set({
      currentImage: null,
      history: [],
      currentFilter: {
        type: 'gaussian_blur',
        params: { intensity: 0.5 },
      },
      isProcessing: false,
      error: null,
      performanceMetrics: null,
    });
  },
}));