import type { Directive, DirectiveBinding } from 'vue'
import { fetchAuthBlobUrl, getApiBaseUrl } from '@/services/api'

interface AuthSrcElement extends HTMLImageElement {
  _authSrcRequestId?: number
}

let requestIdCounter = 0

function shouldAuthenticate(url: string): boolean {
  if (!url) return false
  if (url.startsWith('data:') || url.startsWith('blob:')) return false
  const baseUrl = getApiBaseUrl()
  if (url.startsWith(baseUrl) || url.startsWith('/')) return true
  return false
}

async function updateSource(el: AuthSrcElement, binding: DirectiveBinding<string | null | undefined>) {
  const url = binding.value
  if (!url) {
    el.removeAttribute('src')
    return
  }

  if (!shouldAuthenticate(url)) {
    el.src = url
    return
  }

  const currentId = ++requestIdCounter
  el._authSrcRequestId = currentId

  try {
    const blobUrl = await fetchAuthBlobUrl(url)
    if (el._authSrcRequestId === currentId) {
      el.src = blobUrl
    }
  } catch {
    if (el._authSrcRequestId === currentId) {
      el.removeAttribute('src')
      el.dispatchEvent(new Event('error'))
    }
  }
}

export const vAuthSrc: Directive<AuthSrcElement, string | null | undefined> = {
  mounted(el, binding) {
    updateSource(el, binding)
  },
  updated(el, binding) {
    if (binding.value !== binding.oldValue) {
      updateSource(el, binding)
    }
  },
  unmounted(el) {
    el._authSrcRequestId = undefined
  },
}
