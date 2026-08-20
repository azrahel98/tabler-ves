export interface CargaUtilJwt {
  id: number
  exp: number
  role: string
  email: string
  full_name: string
  picture_url?: string
}

export function decodificarToken(token: string): CargaUtilJwt | null {
  try {
    const partes = token.split('.')
    if (partes.length !== 3) {
      return null
    }
    const base64 = partes[1].replace(/-/g, '+').replace(/_/g, '/')
    const jsonPayload = decodeURIComponent(
      atob(base64)
        .split('')
        .map((c) => `%${`00${c.charCodeAt(0).toString(16)}`.slice(-2)}`)
        .join('')
    )
    return JSON.parse(jsonPayload)
  } catch {
    return null
  }
}

export function tokenExpirado(token: string | null): boolean {
  if (!token) return true
  const carga = decodificarToken(token)
  if (!carga || !carga.exp) return true
  return carga.exp * 1000 <= Date.now()
}
