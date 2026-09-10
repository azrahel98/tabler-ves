import { defineStore } from 'pinia'
import { ref, computed } from 'vue'

export type ThemeMode = 'light' | 'dark'

export const useThemeStore = defineStore('theme', () => {
  const getInitialTheme = (): ThemeMode => {
    if (typeof window === 'undefined') return 'light'
    const saved = localStorage.getItem('hs_theme') as ThemeMode | null
    if (saved === 'light' || saved === 'dark') {
      return saved
    }
    return window.matchMedia('(prefers-color-scheme: dark)').matches ? 'dark' : 'light'
  }

  const theme = ref<ThemeMode>(getInitialTheme())

  const isDark = computed(() => theme.value === 'dark')

  function applyTheme(newTheme: ThemeMode) {
    theme.value = newTheme
    if (typeof document !== 'undefined') {
      if (newTheme === 'dark') {
        document.documentElement.classList.add('dark')
      } else {
        document.documentElement.classList.remove('dark')
      }
    }
    if (typeof window !== 'undefined') {
      localStorage.setItem('hs_theme', newTheme)
    }
  }

  function toggleTheme() {
    applyTheme(theme.value === 'dark' ? 'light' : 'dark')
  }

  function setTheme(newTheme: ThemeMode) {
    applyTheme(newTheme)
  }

  function initTheme() {
    applyTheme(theme.value)
  }

  initTheme()

  return {
    theme,
    isDark,
    toggleTheme,
    setTheme,
    initTheme,
  }
})
