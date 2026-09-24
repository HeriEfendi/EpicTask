import { defineConfig } from 'vite';
import vue from '@vitejs/plugin-vue';
import path from 'path';

export default defineConfig({
  plugins: [vue()],
  resolve: {
    alias: {
      '@': path.resolve(__dirname, './src'),
    },
  },
  clearScreen: false,
  server: {
    port: 1420,
    strictPort: true,
    host: true,
    proxy: {
      '/api': {
        target: 'http://127.0.0.1:8088',
        changeOrigin: true,
      },
      '/ws': {
        target: 'ws://127.0.0.1:8088',
        ws: true,
      },
    },
    watch: {
      ignored: [
        '**/src-tauri/**',
        '**/target/**',
        '**/dist/**',
        '**/*.log',
        '**/*.sql',
        '**/.git/**',
      ],
    },
  },
});
