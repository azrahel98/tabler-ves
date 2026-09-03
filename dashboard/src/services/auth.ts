export interface GooglePayload {
  sub: string
  email: string
  name: string
  picture?: string
}

export function decodeGoogleCredential(credential: string): GooglePayload {
  const base64Url = credential.split('.')[1]
  const base64 = base64Url.replace(/-/g, '+').replace(/_/g, '/')
  const jsonPayload = decodeURIComponent(
    atob(base64)
      .split('')
      .map((c) => '%' + ('00' + c.charCodeAt(0).toString(16)).slice(-2))
      .join(''),
  )
  return JSON.parse(jsonPayload)
}

export interface AuthUser {
  id: number
  google_sub: string
  email: string
  full_name: string
  picture_url?: string | null
  role: string
  status: 'APPROVED' | 'PENDING' | 'REJECTED' | string
  created_at?: string
  updated_at?: string
}

export interface LoginResponse {
  token: string
  user: AuthUser
}

export interface RegisterResponse {
  id: number
  message: string
}

export interface ApiErrorResponse {
  code: number
  error: string
}

import { api, getApiBaseUrl } from './api'

export { getApiBaseUrl }

export async function loginWithGoogleApi(
  google_sub: string,
  email: string,
): Promise<LoginResponse> {
  return await api<LoginResponse>('/login/', {
    method: 'POST',
    body: { google_sub, email },
  })
}

export async function registerWithGoogleApi(payload: {
  google_sub: string
  email: string
  full_name: string
  picture_url?: string | null
}): Promise<RegisterResponse> {
  return await api<RegisterResponse>('/login/register', {
    method: 'POST',
    body: payload,
  })
}
