// Define la estructura de un mensaje para el historial
interface Parte {
  text: string
}

interface Mensaje {
  role: 'user' | 'model'
  parts: Parte[]
}

interface Cita {
  textoGenerado: string
  fuente: string
  contextoOriginal: string
}

interface Respuesta {
  texto: string
  citas: Cita[]
  historialActualizado: Mensaje[] // Agregamos esto para devolver el historial
}

const apiKey = 'AIzaSyDac7ZP4NU6lSGjePotCkwMn9qNddFR28M'

// Ahora la función acepta un historial opcional (por defecto vacío)
const consultar = async (usuario: string, historialPrevio: Mensaje[] = []): Promise<Respuesta> => {
  // Nota: Verifique si 'gemini-2.5-flash' existe.
  // Actualmente los estables son 'gemini-1.5-flash' o 'gemini-2.0-flash-exp'.
  const modelName = 'gemini-2.5-flash'
  const url = `https://generativelanguage.googleapis.com/v1beta/models/${modelName}:generateContent?key=${apiKey}`
  const storeName = 'fileSearchStores/servirv2-cjoyzm8a5xxb'

  const instruccionesSistema = {
    parts: [
      {
        text: `
          Eres un asistente experto en normativa de SERVIR y administración pública.
          
          REGLAS DE FORMATO DE SALIDA:
          1. TU RESPUESTA DEBE SER SIEMPRE CÓDIGO HTML VÁLIDO.
          2. NO envíes el HTML dentro de bloques de markdown (como \`\`\`html). Envía el HTML crudo directamente.
          3. Usa clases de Tailwind CSS para dar estilo. Diseña la respuesta para que sea visualmente atractiva:
             - Usa <h3 class="text-lg font-bold text-primary mb-2"> para títulos.
             - Usa <ul class="list-disc pl-5 space-y-1 text-gray-700"> para listas.
             - Si citas una ley o artículo, usa un contenedor tipo tarjeta: <div class="bg-gray-50 border-l-4 border-primary p-3 my-2 rounded-r">.
             - Resalta palabras clave con <span class="font-semibold text-gray-900">.
          4. Sé conciso y directo.
        `
      }
    ]
  }
  // 1. Preparamos el nuevo mensaje del usuario
  const nuevoMensajeUsuario: Mensaje = {
    role: 'user',
    parts: [{ text: usuario }]
  }

  const historialRecortado = historialPrevio.slice(-5)
  const contents = [...historialRecortado, nuevoMensajeUsuario]

  const body = {
    systemInstruction: instruccionesSistema,
    contents: contents, // Enviamos TODO el historial
    tools: [
      {
        file_search: {
          file_search_store_names: [storeName]
        }
      }
    ]
  }

  const resultado: Respuesta = {
    texto: '',
    citas: [],
    historialActualizado: []
  }

  try {
    const response = await fetch(url, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(body)
    })

    if (!response.ok) {
      const errorText = await response.text()
      throw new Error(`Error en la solicitud HTTP: ${response.status} - ${errorText}`)
    }

    const data = await response.json()

    // Obtener la respuesta del modelo
    const textoModelo = data.candidates?.[0]?.content?.parts?.[0]?.text || 'No se generó respuesta.'
    const textoLimpio = textoModelo
      .replace(/^```html/, '')
      .replace(/```$/, '')
      .trim()
    resultado.texto = textoLimpio
    resultado.citas = await procesarGrounding(data)

    // 3. Agregamos la respuesta del modelo al historial para la próxima vez
    const nuevoMensajeModelo: Mensaje = {
      role: 'model',
      parts: [{ text: textoModelo }]
    }

    // El nuevo historial contiene todo lo previo + usuario actual + respuesta modelo
    resultado.historialActualizado = [...contents, nuevoMensajeModelo]

    return resultado
  } catch (error) {
    console.error('Ha ocurrido un error durante la consulta:', error)
    throw error
  }
}

async function procesarGrounding(data: any): Promise<Cita[]> {
  const candidate = data.candidates?.[0]
  const listaDeCitas: Cita[] = []

  if (candidate?.groundingMetadata?.groundingSupports) {
    const supports = candidate.groundingMetadata.groundingSupports
    const chunks = candidate.groundingMetadata.groundingChunks

    for (const support of supports) {
      const textoSegmento = support.segment.text
      for (const chunkIndex of support.groundingChunkIndices) {
        const chunk = chunks[chunkIndex]

        const rawId = chunk.retrievedContext?.title
        // 1. Aseguramos el formato "files/XXXXX"
        // Si ya viene con "files/", lo dejamos así. Si no, se lo agregamos.
        const fileIdCorrecto = rawId && rawId.startsWith('files/') ? rawId : `files/${rawId}`

        // 2. Buscamos el nombre real en la API
        const nombreReal = await obtenerPorId(fileIdCorrecto)

        listaDeCitas.push({
          textoGenerado: textoSegmento,
          fuente: nombreReal,
          contextoOriginal: chunk.retrievedContext?.text || ''
        })
      }
    }
  }
  return listaDeCitas
}

import mapaArchivos from '../../utils/mapa_archivos.json'

async function obtenerPorId(idDelArchivo: string): Promise<any> {
  const baseDeDatos = mapaArchivos as Record<string, string>

  const nombre = baseDeDatos[idDelArchivo]

  return nombre || 'Fuente desconocida'
}

export { consultar, type Mensaje, type Cita }
