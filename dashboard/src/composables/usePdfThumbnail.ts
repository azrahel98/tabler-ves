import { ref, onMounted, onUnmounted } from 'vue'
import * as pdfjsLib from 'pdfjs-dist'
import pdfjsWorker from 'pdfjs-dist/build/pdf.worker.min.mjs?url'

pdfjsLib.GlobalWorkerOptions.workerSrc = pdfjsWorker

export function usePdfThumbnail(url: string | null, containerRef: () => HTMLElement | null) {
  const thumbnailUrl = ref<string | null>(null)
  const cargando = ref(false)
  const error = ref(false)

  let observer: IntersectionObserver | null = null
  let generado = false

  const generar = async () => {
    if (generado || !url) return
    generado = true
    cargando.value = true
    error.value = false

    try {
      const loadingTask = pdfjsLib.getDocument({ url, cMapPacked: true })
      const pdf = await loadingTask.promise
      const page = await pdf.getPage(1)

      const viewport = page.getViewport({ scale: 1 })
      const scale = 200 / viewport.width
      const scaled = page.getViewport({ scale })

      const canvas = document.createElement('canvas')
      canvas.width = scaled.width
      canvas.height = scaled.height

      const ctx = canvas.getContext('2d')!
      await page.render({ canvasContext: ctx, viewport: scaled }).promise

      thumbnailUrl.value = canvas.toDataURL('image/jpeg', 0.8)
    } catch {
      error.value = true
    } finally {
      cargando.value = false
    }
  }

  onMounted(() => {
    const el = containerRef()
    if (!el) {
      generar()
      return
    }

    observer = new IntersectionObserver(
      (entries) => {
        if (entries[0].isIntersecting) {
          generar()
          observer?.disconnect()
        }
      },
      { rootMargin: '100px' }
    )
    observer.observe(el)
  })

  onUnmounted(() => {
    observer?.disconnect()
  })

  return { thumbnailUrl, cargando, error }
}
