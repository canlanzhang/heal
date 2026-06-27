import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'
import path from 'path'

export default defineConfig({
  plugins: [vue()],

  resolve: {
    alias: {
      '@': path.resolve(__dirname, './src')
    }
  },

  server: {
    port: 5173,

    proxy: {
      '/api': {
        target: 'https://127.0.0.1:8443',

        changeOrigin: true,

        secure: false, // ⭐允许自签证书（必须）

        // ⭐关键：避免路径重复 /api/api
        //rewrite: (path) => path
      }
    }
  }
})