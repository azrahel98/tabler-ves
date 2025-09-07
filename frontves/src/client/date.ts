const formatFechaCompleta = (fecha: string): string => {
  try {
    const date = new Date(fecha)
    return date.toLocaleString('es-ES', {
      weekday: 'short',
      year: 'numeric',
      month: 'short',
      day: 'numeric',
      hour: '2-digit',
      minute: '2-digit'
    })
  } catch {
    return fecha
  }
}

export { formatFechaCompleta }
