import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'
import path from 'path'
// https://vite.dev/config/
export default defineConfig({
  plugins: [vue()],
  define:{
    __VUE_PROD_DEVTOOLS_API__: false,
    __VUE_PROD_DEVTOOLS__: false,
  },
  resolve: {
    alias: [
      { find: '@views', replacement: path.resolve(__dirname, './src/views') },
      { find: '@router', replacement: path.resolve(__dirname, './src/router/') },
      { find: '@comp', replacement: path.resolve(__dirname, './src/components') },
      { find: '@api', replacement: path.resolve(__dirname, './src/client/') }
    ]
  },
})
