import { defineConfig } from 'vite'
import path from 'node:path'
import vue from '@vitejs/plugin-vue'
import tailwindcss from '@tailwindcss/vite'

// https://vite.dev/config/
export default defineConfig({
  plugins: [vue(), tailwindcss()],
  resolve: {
    alias: [
      { find: '@views', replacement: path.resolve(__dirname, './src/views') },
      { find: '@router', replacement: path.resolve(__dirname, './src/router/') },
      { find: '@comp', replacement: path.resolve(__dirname, './src/components') },
      { find: '@lib', replacement: path.resolve(__dirname, './utils/') },
      { find: '@api', replacement: path.resolve(__dirname, './src/api/') },
      { find: '@store', replacement: path.resolve(__dirname, './src/store/') },
      { find: '@tools', replacement: path.resolve(__dirname, './src/tools/') }
    ]
  }
})
