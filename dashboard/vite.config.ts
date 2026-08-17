import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'
import tailwindcss from '@tailwindcss/vite'
import vueDevTools from 'vite-plugin-vue-devtools'


export default defineConfig({
  plugins: [vue(), tailwindcss(), vueDevTools()],
  build: {
    rollupOptions: {
      output: {
        manualChunks(id) {
          if (id.includes('pdfjs-dist')) {
            return 'pdfjs'
          }
          if (id.includes('leaflet')) {
            return 'leaflet'
          }
          if (id.includes('chart.js') || id.includes('vue-chartjs')) {
            return 'charts'
          }
          if (id.includes('node_modules')) {
            return 'vendor'
          }
        },
      },
    },
  },
})
