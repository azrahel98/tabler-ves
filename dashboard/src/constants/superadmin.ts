/** Único correo con acceso a la gestión de usuarios (debe coincidir con SUPER_ADMIN_EMAIL del api-server). */
export const SUPER_ADMIN_EMAIL = 'azraelraul@gmail.com'

export function esSuperAdmin(email: string | null | undefined): boolean {
  return (email || '').trim().toLowerCase() === SUPER_ADMIN_EMAIL
}
