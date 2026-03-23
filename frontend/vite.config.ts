import react from '@vitejs/plugin-react'
import { defineConfig } from 'vite'

// dev proxy to the rust api so the browser doesn't whine about CORS
export default defineConfig({
  plugins: [react()],
  server: {
    proxy: {
      '/api': {
        target: 'http://127.0.0.1:3000',
        changeOrigin: true,
      },
    },
  },
})
