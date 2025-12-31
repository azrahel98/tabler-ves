function generarSiglas(nombreArea: string): string {
  const palabrasAExcluir: string[] = ['de', 'la', 'el', 'los', 'las', 'y', 'e', 'o', 'u', 'a', 'del', 'para', 'con', 'en', 'por', 'al', 'sobre', 'un', 'una', ',']

  const nombreLimpio = nombreArea
    .replace(/[,;]/g, '')
    .toUpperCase()
    .normalize('NFD')
    .replace(/[\u0300-\u036f]/g, '')

  const palabras = nombreLimpio.split(/\s+/).filter((word) => word.length > 0)

  let siglas: string = ''

  for (const palabra of palabras) {
    const palabraMinuscula = palabra.toLowerCase()

    if (!palabrasAExcluir.includes(palabraMinuscula)) {
      siglas += palabra.charAt(0)
    }
  }

  return siglas
}

const nombreabrv = (nombrefull: string) => {
  if (!nombrefull) return ''
  const palabras = nombrefull.trim().split(/\s+/)
  const iniciales = palabras
    .slice(0, 2)
    .map((palabra) => palabra.charAt(0).toUpperCase())
    .join('')
  return iniciales
}

export { generarSiglas, nombreabrv }
