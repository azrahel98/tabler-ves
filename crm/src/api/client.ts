import { ApiError, type ApiErrorResponse } from './types'

const BASE_URL = import.meta.env.VITE_API_BASE_URL || 'http://127.0.0.1:4010'

function getHeaders(customHeaders?: HeadersInit, isFormData = false): Headers {
	const headers = new Headers(customHeaders)

	if (!isFormData && !headers.has('Content-Type')) {
		headers.set('Content-Type', 'application/json')
	}

	const token = localStorage.getItem('auth_token')
	if (token && !headers.has('token')) {
		headers.set('token', token)
	}

	return headers
}

async function request<T>(endpoint: string, options: RequestInit = {}, isFormData = false): Promise<T> {
	const url = `${BASE_URL.replace(/\/$/, '')}/${endpoint.replace(/^\//, '')}`
	const headers = getHeaders(options.headers, isFormData)

	const config: RequestInit = {
		...options,
		headers,
	}

	const response = await fetch(url, config)

	if (!response.ok) {
		let errorCode = response.status
		let errorMessage = response.statusText || 'Error desconocido'

		try {
			const errorData: ApiErrorResponse = await response.json()
			if (errorData.code) {
				errorCode = errorData.code
			}
			if (errorData.error) {
				errorMessage = errorData.error
			}
		} catch {
		}

		throw new ApiError(errorCode, errorMessage)
	}

	if (response.status === 204) {
		return null as T
	}

	return (await response.json()) as T
}

async function requestBlob(endpoint: string, options: RequestInit = {}): Promise<Blob> {
	const url = `${BASE_URL.replace(/\/$/, '')}/${endpoint.replace(/^\//, '')}`
	const headers = getHeaders(options.headers, false)

	const config: RequestInit = {
		...options,
		headers,
	}

	const response = await fetch(url, config)

	if (!response.ok) {
		let errorCode = response.status
		let errorMessage = response.statusText || 'Error al descargar archivo'

		try {
			const errorData: ApiErrorResponse = await response.json()
			if (errorData.code) errorCode = errorData.code
			if (errorData.error) errorMessage = errorData.error
		} catch {
		}

		throw new ApiError(errorCode, errorMessage)
	}

	return await response.blob()
}

export const apiClient = {
	get<T>(endpoint: string, options?: RequestInit): Promise<T> {
		return request<T>(endpoint, { ...options, method: 'GET' })
	},

	post<T>(endpoint: string, body?: unknown, options?: RequestInit): Promise<T> {
		return request<T>(endpoint, {
			...options,
			method: 'POST',
			body: body !== undefined ? JSON.stringify(body) : undefined,
		})
	},

	postFormData<T>(endpoint: string, formData: FormData, options?: RequestInit): Promise<T> {
		return request<T>(endpoint, {
			...options,
			method: 'POST',
			body: formData,
		}, true)
	},

	postBlob(endpoint: string, body?: unknown, options?: RequestInit): Promise<Blob> {
		return requestBlob(endpoint, {
			...options,
			method: 'POST',
			body: body !== undefined ? JSON.stringify(body) : undefined,
		})
	},

	put<T>(endpoint: string, body?: unknown, options?: RequestInit): Promise<T> {
		return request<T>(endpoint, {
			...options,
			method: 'PUT',
			body: body !== undefined ? JSON.stringify(body) : undefined,
		})
	},

	patch<T>(endpoint: string, body?: unknown, options?: RequestInit): Promise<T> {
		return request<T>(endpoint, {
			...options,
			method: 'PATCH',
			body: body !== undefined ? JSON.stringify(body) : undefined,
		})
	},

	delete<T>(endpoint: string, options?: RequestInit): Promise<T> {
		return request<T>(endpoint, { ...options, method: 'DELETE' })
	},
}

