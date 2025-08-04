import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'
import path from 'path'

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [react()],
  
  // 路径别名
  resolve: {
    alias: {
      '@': path.resolve(__dirname, './src'),
      '@/components': path.resolve(__dirname, './src/components'),
      '@/hooks': path.resolve(__dirname, './src/hooks'),
      '@/stores': path.resolve(__dirname, './src/stores'),
      '@/types': path.resolve(__dirname, './src/types'),
      '@/utils': path.resolve(__dirname, './src/utils'),
    },
  },

  // 开发服务器配置
  server: {
    port: 5173,
    host: true,
    cors: true,
  },

  // 构建配置
  build: {
    outDir: 'dist',
    sourcemap: true,
    rollupOptions: {
      output: {
        manualChunks: {
          vendor: ['react', 'react-dom'],
          ui: ['framer-motion', 'recharts'],
          utils: ['@tanstack/react-query', 'zustand'],
        },
      },
    },
  },

  // WebAssembly 支持
  assetsInclude: ['**/*.wasm'],
  
  // 优化配置
  optimizeDeps: {
    exclude: ['@rustimage/wasm'], // 排除 WASM 模块的预构建
  },

  // 实验性功能
  experimental: {
    renderBuiltUrl(filename, { hostType }) {
      if (filename.endsWith('.wasm')) {
        return `/${filename}`
      }
      return { relative: true }
    },
  },

  // 环境变量
  define: {
    __DEV__: JSON.stringify(process.env.NODE_ENV === 'development'),
    __VERSION__: JSON.stringify(process.env.npm_package_version),
  },
})