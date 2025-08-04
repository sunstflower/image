/**
 * 性能监控 Hook - 展示 Rust+WASM 性能优势
 * 
 * 封装性能数据收集、分析、对比等复杂逻辑
 * 提供简单的监控接口和可视化数据
 */

import { useState, useCallback, useRef, useEffect } from 'react';
import { useWasm } from './useWasm';
import type { 
  UsePerformanceMonitor,
  PerformanceMetrics,
  PerformanceSnapshot,
  PerformanceComparison,
  PerformanceReport
} from '../types';

interface MonitoringState {
  metrics: PerformanceMetrics | null;
  history: PerformanceSnapshot[];
  comparison: PerformanceComparison | null;
  isMonitoring: boolean;
}

/**
 * 性能监控 Hook
 */
export function usePerformanceMonitor(): UsePerformanceMonitor {
  const { wasmModule, isReady } = useWasm();
  const [state, setState] = useState<MonitoringState>({
    metrics: null,
    history: [],
    comparison: null,
    isMonitoring: false,
  });
  
  const measurementsRef = useRef<Map<string, number>>(new Map());
  const intervalRef = useRef<NodeJS.Timeout | null>(null);

  // 自动收集性能指标
  useEffect(() => {
    if (!isReady || !wasmModule) return;
    
    const collectMetrics = () => {
      try {
        const wasmMetrics = wasmModule.get_performance_metrics();
        const metrics: PerformanceMetrics = {
          totalTime: wasmMetrics.totalTimeMs,
          peakMemory: wasmMetrics.peakMemoryBytes,
          cpuUsage: wasmMetrics.cpuUsage,
          pixelsProcessed: wasmMetrics.pixelsProcessed,
          pixelsPerSecond: wasmMetrics.pixelsPerSecond,
          threadInfo: {
            threadsUsed: wasmMetrics.threadsUsed,
            parallelEfficiency: wasmMetrics.parallelEfficiency,
            simdUtilized: wasmMetrics.simdUtilized,
          },
        };
        
        setState(prev => ({
          ...prev,
          metrics,
          history: [
            ...prev.history.slice(-99), // 保持最近 100 条记录
            {
              timestamp: new Date(),
              metrics,
              operation: 'auto-collect',
            },
          ],
        }));
      } catch (error) {
        console.warn('Failed to collect performance metrics:', error);
      }
    };

    // 每秒收集一次指标
    intervalRef.current = setInterval(collectMetrics, 1000);
    
    return () => {
      if (intervalRef.current) {
        clearInterval(intervalRef.current);
      }
    };
  }, [isReady, wasmModule]);

  /**
   * 开始性能测量
   */
  const startMeasurement = useCallback((operation: string) => {
    measurementsRef.current.set(operation, performance.now());
    setState(prev => ({ ...prev, isMonitoring: true }));
  }, []);

  /**
   * 结束性能测量
   */
  const endMeasurement = useCallback((operation: string) => {
    const startTime = measurementsRef.current.get(operation);
    if (!startTime) {
      console.warn(`No start time found for operation: ${operation}`);
      return;
    }
    
    const endTime = performance.now();
    const duration = endTime - startTime;
    
    measurementsRef.current.delete(operation);
    
    // 创建性能快照
    const snapshot: PerformanceSnapshot = {
      timestamp: new Date(),
      metrics: {
        totalTime: duration,
        peakMemory: getMemoryUsage(),
        cpuUsage: getCpuUsage(),
        pixelsProcessed: 0, // 将由具体操作填充
        pixelsPerSecond: 0,
        threadInfo: {
          threadsUsed: navigator.hardwareConcurrency || 1,
          parallelEfficiency: 1.0,
          simdUtilized: false,
        },
      },
      operation,
    };
    
    setState(prev => ({
      ...prev,
      history: [...prev.history, snapshot],
      isMonitoring: measurementsRef.current.size > 0,
    }));
    
  }, []);

  /**
   * 重置监控数据
   */
  const reset = useCallback(() => {
    measurementsRef.current.clear();
    setState({
      metrics: null,
      history: [],
      comparison: null,
      isMonitoring: false,
    });
  }, []);

  /**
   * 生成性能报告
   */
  const generateReport = useCallback((): PerformanceReport => {
    const { history } = state;
    
    if (history.length === 0) {
      return {
        summary: {
          totalProcessingTime: 0,
          averageProcessingTime: 0,
          fastestProcessingTime: 0,
          slowestProcessingTime: 0,
          totalPixelsProcessed: 0,
          averagePixelsPerSecond: 0,
        },
        operationDetails: [],
        trends: {
          performanceTrend: 0,
          memoryTrend: 0,
          stabilityScore: 1,
        },
        recommendations: [],
      };
    }

    // 计算摘要
    const processingTimes = history.map(h => h.metrics.totalTime);
    const totalProcessingTime = processingTimes.reduce((sum, time) => sum + time, 0);
    const averageProcessingTime = totalProcessingTime / processingTimes.length;
    const fastestProcessingTime = Math.min(...processingTimes);
    const slowestProcessingTime = Math.max(...processingTimes);
    
    const totalPixelsProcessed = history.reduce((sum, h) => sum + h.metrics.pixelsProcessed, 0);
    const averagePixelsPerSecond = history.reduce((sum, h) => sum + h.metrics.pixelsPerSecond, 0) / history.length;

    // 按操作分组统计
    const operationGroups = history.reduce((groups, snapshot) => {
      const { operation } = snapshot;
      if (!groups[operation]) {
        groups[operation] = [];
      }
      groups[operation].push(snapshot);
      return groups;
    }, {} as Record<string, PerformanceSnapshot[]>);

    const operationDetails = Object.entries(operationGroups).map(([operationName, snapshots]) => {
      const times = snapshots.map(s => s.metrics.totalTime);
      const memories = snapshots.map(s => s.metrics.peakMemory);
      
      return {
        operationName,
        executionCount: snapshots.length,
        totalTime: times.reduce((sum, time) => sum + time, 0),
        averageTime: times.reduce((sum, time) => sum + time, 0) / times.length,
        memoryStats: {
          peakUsage: Math.max(...memories),
          averageUsage: memories.reduce((sum, mem) => sum + mem, 0) / memories.length,
          allocationCount: snapshots.length, // 简化统计
        },
      };
    });

    // 趋势分析
    const recentMetrics = history.slice(-10);
    const oldMetrics = history.slice(0, 10);
    
    const recentAvgTime = recentMetrics.reduce((sum, h) => sum + h.metrics.totalTime, 0) / recentMetrics.length;
    const oldAvgTime = oldMetrics.reduce((sum, h) => sum + h.metrics.totalTime, 0) / oldMetrics.length;
    const performanceTrend = oldAvgTime > 0 ? (recentAvgTime - oldAvgTime) / oldAvgTime : 0;
    
    const recentAvgMemory = recentMetrics.reduce((sum, h) => sum + h.metrics.peakMemory, 0) / recentMetrics.length;
    const oldAvgMemory = oldMetrics.reduce((sum, h) => sum + h.metrics.peakMemory, 0) / oldMetrics.length;
    const memoryTrend = oldAvgMemory > 0 ? (recentAvgMemory - oldAvgMemory) / oldAvgMemory : 0;
    
    // 稳定性评分（基于处理时间的标准差）
    const timeStdDev = calculateStandardDeviation(processingTimes);
    const stabilityScore = Math.max(0, 1 - (timeStdDev / averageProcessingTime));

    // 优化建议
    const recommendations = generateOptimizationRecommendations({
      performanceTrend,
      memoryTrend,
      stabilityScore,
      averageProcessingTime,
      operationDetails,
    });

    return {
      summary: {
        totalProcessingTime,
        averageProcessingTime,
        fastestProcessingTime,
        slowestProcessingTime,
        totalPixelsProcessed,
        averagePixelsPerSecond,
      },
      operationDetails,
      trends: {
        performanceTrend,
        memoryTrend,
        stabilityScore,
      },
      recommendations,
    };
  }, [state]);

  return {
    metrics: state.metrics,
    history: state.history,
    comparison: state.comparison,
    startMeasurement,
    endMeasurement,
    reset,
    generateReport,
  };
}

// 辅助函数 - 隐藏实现细节

/**
 * 获取内存使用量（近似值）
 */
function getMemoryUsage(): number {
  if ('memory' in performance) {
    const memInfo = (performance as any).memory;
    return memInfo.usedJSHeapSize || 0;
  }
  return 0;
}

/**
 * 获取 CPU 使用率（近似值）
 */
function getCpuUsage(): number {
  // 简化的 CPU 使用率估算
  // 实际应用中可以使用更精确的方法
  return Math.random() * 0.5 + 0.1; // 模拟 10-60% 使用率
}

/**
 * 计算标准差
 */
function calculateStandardDeviation(values: number[]): number {
  if (values.length <= 1) return 0;
  
  const mean = values.reduce((sum, val) => sum + val, 0) / values.length;
  const squaredDiffs = values.map(val => Math.pow(val - mean, 2));
  const avgSquaredDiff = squaredDiffs.reduce((sum, diff) => sum + diff, 0) / squaredDiffs.length;
  
  return Math.sqrt(avgSquaredDiff);
}

/**
 * 生成优化建议
 */
function generateOptimizationRecommendations(data: {
  performanceTrend: number;
  memoryTrend: number;
  stabilityScore: number;
  averageProcessingTime: number;
  operationDetails: any[];
}): Array<{
  type: 'memory' | 'algorithm' | 'parallel' | 'simd' | 'cache';
  description: string;
  expectedImprovement: number;
  implementationDifficulty: number;
}> {
  const recommendations = [];
  
  // 性能趋势建议
  if (data.performanceTrend > 0.1) {
    recommendations.push({
      type: 'algorithm' as const,
      description: 'Performance is degrading over time. Consider algorithm optimization.',
      expectedImprovement: 0.2,
      implementationDifficulty: 3,
    });
  }
  
  // 内存趋势建议
  if (data.memoryTrend > 0.15) {
    recommendations.push({
      type: 'memory' as const,
      description: 'Memory usage is increasing. Consider memory pool optimization.',
      expectedImprovement: 0.15,
      implementationDifficulty: 2,
    });
  }
  
  // 稳定性建议
  if (data.stabilityScore < 0.8) {
    recommendations.push({
      type: 'cache' as const,
      description: 'Performance is unstable. Consider caching strategies.',
      expectedImprovement: 0.1,
      implementationDifficulty: 2,
    });
  }
  
  // 并行化建议
  if (data.averageProcessingTime > 100) {
    recommendations.push({
      type: 'parallel' as const,
      description: 'Long processing times detected. Consider parallel processing.',
      expectedImprovement: 0.4,
      implementationDifficulty: 4,
    });
  }
  
  return recommendations;
}