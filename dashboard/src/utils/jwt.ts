import { jwtDecode } from 'jwt-decode'

interface JwtPayload {
  exp: number
  lvl: number
  nombre: string
}

export function decodificar(token: string): JwtPayload | null {
  try {
    return jwtDecode<JwtPayload>(token)
  } catch {
    return null
  }
}

export function tokenExpirado(token: string): boolean {
  const decoded = decodificar(token)
  if (!decoded) return true
  return decoded.exp * 1000 < Date.now()
}
