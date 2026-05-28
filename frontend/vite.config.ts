import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'

// https://vite.dev/config/
export default defineConfig({
  plugins: [vue()],
  server: {
    proxy: {
      '/api': {
        target: 'https://127.0.0.1:8443', // 你的 Axum 地址
        changeOrigin: true,
        secure: false,
      }
    }
  }  
})
