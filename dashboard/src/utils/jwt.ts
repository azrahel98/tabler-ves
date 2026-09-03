export interface JwtPayload {
  exp?: number
  iat?: number
  sub?: string
  [key: string]: any
}

export function parseJwt(token: string): JwtPayload | null {
  try {
    const parts = token.split('.')
    if (parts.length !== 3) {
      return null
    }
    const base64Url = parts[1]
    const base64 = base64Url.replace(/-/g, '+').replace(/_/g, '/')
    const jsonPayload = decodeURIComponent(
      atob(base64)
        .split('')
        .map((c) => '%' + ('00' + c.charCodeAt(0).toString(16)).slice(-2))
        .join(''),
    )
    return JSON.parse(jsonPayload)
  } catch {
    return null
  }
}

export function isTokenValid(token: string | null): boolean {
  if (!token || typeof token !== 'string') {
    return false
  }
  const payload = parseJwt(token)
  if (!payload) {
    return false
  }
  if (typeof payload.exp === 'number') {
    const now = Math.floor(Date.now() / 1000)
    return payload.exp > now
  }
  return true
}
