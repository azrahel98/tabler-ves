const formatFechaCompleta = (fecha: any, seconds?: boolean, day?: boolean): string => {
  try {
    const date = new Date(fecha)
    if (isNaN(date.getTime())) return fecha

    return date.toLocaleString('es-ES', {
      timeZone: 'UTC',
      weekday: day ? 'short' : undefined,
      year: 'numeric',
      month: 'short',
      day: 'numeric',
      hour: seconds ? '2-digit' : undefined,
      minute: seconds ? '2-digit' : undefined,
      second: seconds ? '2-digit' : undefined
    })
  } catch {
    return fecha
  }
}

export { formatFechaCompleta }
