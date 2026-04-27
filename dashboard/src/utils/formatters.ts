export const formatPhone = (phone: string | null | undefined): string => {
  if (!phone) return ''
  const limpio = phone.replace(/\D/g, '')
  return limpio.match(/.{1,3}/g)?.join(' ') ?? phone
}
