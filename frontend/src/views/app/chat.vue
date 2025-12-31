<script setup lang="ts">
import { ref, nextTick } from 'vue'
// Asegúrate de que tu archivo @tools/openia devuelva la estructura correcta
import { consultar, type Mensaje, type Cita } from '@tools/openia'
import Popover from '@comp/ui/poppover.vue'

// --- Tipos para la UI (Visualización) ---
interface Message {
  id: string
  role: 'user' | 'assistant'
  content: string
  citas?: Cita[]
  timestamp: Date
}

// Estado del chat
const messages = ref<Message[]>([
  {
    id: 'welcome',
    role: 'assistant',
    content: '<p>Hola. Soy <strong>Raul VES AI</strong>. ¿En qué puedo ayudarte hoy con tus datos o reportes?</p>',
    timestamp: new Date(),
    citas: []
  }
])

// --- MEMORIA DEL CHAT (Contexto para la IA) ---
let historialConversacion: Mensaje[] = []

const userInput = ref('')
const isLoading = ref(false)
const chatContainer = ref<HTMLElement | null>(null)

const scrollToBottom = async () => {
  await nextTick()
  if (chatContainer.value) {
    chatContainer.value.scrollTop = chatContainer.value.scrollHeight
  }
}

const generateId = () => Math.random().toString(36).substring(2, 9)

const sendMessage = async () => {
  if (!userInput.value.trim() || isLoading.value) return

  const currentQuery = userInput.value

  // 1. Agregar mensaje del usuario a la UI
  const userMsg: Message = {
    id: generateId(),
    role: 'user',
    content: currentQuery,
    timestamp: new Date()
  }

  messages.value.push(userMsg)
  userInput.value = ''
  isLoading.value = true
  await scrollToBottom()

  try {
    // 2. LLAMADA REAL A LA API
    const respuesta = await consultar(currentQuery, historialConversacion)

    historialConversacion = respuesta.historialActualizado

    // 4. Agregar respuesta del bot a la UI
    // IMPORTANTE: Aquí respuesta.texto ya debe venir con HTML desde tu función consultar
    messages.value.push({
      id: generateId(),
      role: 'assistant',
      content: respuesta.texto,
      citas: respuesta.citas,
      timestamp: new Date()
    })
  } catch (error) {
    console.error(error)
    messages.value.push({
      id: generateId(),
      role: 'assistant',
      content: '<p class="text-red-500">Lo siento, tuve un problema conectando con la base de conocimiento.</p>',
      timestamp: new Date()
    })
  } finally {
    isLoading.value = false
    await scrollToBottom()
  }
}

const formatTime = (date: Date) => {
  return new Intl.DateTimeFormat('es-ES', { hour: '2-digit', minute: '2-digit' }).format(date)
}
</script>

<template>
  <div class="flex flex-col h-full w-full bg-white rounded-2xl border border-gray-200 shadow-sm overflow-hidden font-sans">
    <div class="flex items-center justify-between px-6 py-4 border-b border-gray-100 bg-white/80 backdrop-blur-sm sticky top-0 z-10">
      <div class="flex items-center gap-3">
        <div class="flex items-center justify-center w-10 h-10 rounded-full bg-primary/10 text-primary">
          <svg
            xmlns="http://www.w3.org/2000/svg"
            width="20"
            height="20"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"
          >
            <path d="M12 2a2 2 0 0 1 2 2v2a2 2 0 0 1-2 2 2 2 0 0 1-2-2V4a2 2 0 0 1 2-2z" />
            <path d="m4.93 10.93 1.41 1.41" />
            <path d="M2 12h2" />
            <path d="m4.93 13.07 1.41-1.41" />
            <path d="M12 22v-2" />
            <path d="m17.66 10.93-1.41 1.41" />
            <path d="M20 12h2" />
            <path d="m17.66 13.07-1.41-1.41" />
            <rect x="8" y="8" width="8" height="8" rx="2" />
          </svg>
        </div>
        <div>
          <h3 class="text-gray-900 font-bold text-base tracking-tight leading-none">Raul - MVES</h3>
          <p class="text-gray-500 text-xs font-medium mt-1">Potenciado por AI</p>
        </div>
      </div>
      <button class="text-gray-400 hover:text-primary transition-colors p-2 rounded-full hover:bg-gray-50">
        <svg
          xmlns="http://www.w3.org/2000/svg"
          width="20"
          height="20"
          viewBox="0 0 24 24"
          fill="none"
          stroke="currentColor"
          stroke-width="2"
          stroke-linecap="round"
          stroke-linejoin="round"
        >
          <circle cx="12" cy="12" r="1" />
          <circle cx="19" cy="12" r="1" />
          <circle cx="5" cy="12" r="1" />
        </svg>
      </button>
    </div>

    <div ref="chatContainer" class="flex-1 overflow-y-auto p-4 md:p-6 space-y-6 bg-gray-50/50 scroll-smooth">
      <div v-for="msg in messages" :key="msg.id" class="flex flex-col gap-1.5" :class="msg.role === 'user' ? 'items-end' : 'items-start'">
        <span v-if="msg.role === 'assistant'" class="text-[10px] font-semibold text-gray-400 uppercase tracking-widest ml-1">Nexus AI • {{ formatTime(msg.timestamp) }}</span>
        <span v-else class="text-[10px] font-semibold text-gray-400 uppercase tracking-widest mr-1">Tú • {{ formatTime(msg.timestamp) }}</span>

        <div
          class="relative max-w-[90%] md:max-w-[80%] rounded-2xl px-5 py-3.5 text-sm leading-relaxed shadow-sm transition-all"
          :class="[msg.role === 'user' ? 'bg-primary text-white rounded-tr-none' : 'bg-white text-gray-900 border border-gray-100 rounded-tl-none']"
        >
          <div v-if="msg.role === 'assistant'" class="ai-content" v-html="msg.content"></div>

          <p v-else>{{ msg.content }}</p>
        </div>

        <div v-if="msg.citas && msg.citas.length > 0" class="mt-2 max-w-[90%] md:max-w-[80%] w-full pl-2">
          <p class="text-[10px] font-bold text-gray-400 uppercase tracking-wider flex items-center gap-2 mb-2">
            <span class="w-1 h-1 rounded-full bg-gray-400"></span>
            Fuentes consultadas
          </p>

          <div class="flex flex-wrap gap-2">
            <Popover v-for="(cita, idx) in msg.citas" :key="idx" placement="top" width="w-80">
              <template #trigger>
                <button
                  class="flex items-center gap-2 bg-white border border-gray-200 hover:border-primary/50 text-gray-600 hover:text-primary px-3 py-1.5 rounded-full text-xs font-medium transition-all shadow-sm active:scale-95"
                >
                  <svg
                    xmlns="http://www.w3.org/2000/svg"
                    width="12"
                    height="12"
                    viewBox="0 0 24 24"
                    fill="none"
                    stroke="currentColor"
                    stroke-width="2"
                    stroke-linecap="round"
                    stroke-linejoin="round"
                  >
                    <path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"></path>
                    <polyline points="14 2 14 8 20 8"></polyline>
                  </svg>
                  <span class="truncate max-w-[150px]">{{ cita.fuente }}</span>
                </button>
              </template>

              <div class="p-4 bg-white text-sm">
                <div class="flex items-center gap-2 mb-2 pb-2 border-b border-gray-100">
                  <span class="bg-primary/10 text-primary text-[10px] px-2 py-0.5 rounded font-bold uppercase"> Fuente {{ idx + 1 }} </span>
                  <span class="font-semibold text-gray-900 truncate">{{ cita.fuente }}</span>
                </div>

                <p class="text-gray-600 italic leading-relaxed text-xs bg-gray-50 p-3 rounded-lg border border-gray-100">"{{ cita.contextoOriginal }}"</p>
              </div>
            </Popover>
          </div>
        </div>
      </div>

      <div v-if="isLoading" class="flex items-start gap-3 animate-pulse">
        <div class="flex items-center justify-center w-8 h-8 rounded-full bg-white border border-gray-100 shadow-sm shrink-0">
          <div class="w-4 h-4 rounded-full bg-primary/20"></div>
        </div>
        <div class="bg-white border border-gray-100 px-4 py-3 rounded-xl rounded-tl-none shadow-sm flex items-center gap-1.5 h-10">
          <div class="w-1.5 h-1.5 bg-gray-400 rounded-full animate-bounce" style="animation-delay: 0ms"></div>
          <div class="w-1.5 h-1.5 bg-gray-400 rounded-full animate-bounce" style="animation-delay: 150ms"></div>
          <div class="w-1.5 h-1.5 bg-gray-400 rounded-full animate-bounce" style="animation-delay: 300ms"></div>
        </div>
      </div>
    </div>

    <div class="p-4 bg-white border-t border-gray-100">
      <div class="flex gap-3 relative">
        <input
          v-model="userInput"
          @keyup.enter="sendMessage"
          type="text"
          placeholder="Escribe tu pregunta sobre los reportes..."
          class="flex-1 bg-gray-50 text-gray-900 placeholder:text-gray-400 text-sm rounded-xl px-4 py-3 border border-gray-200 focus:outline-none focus:border-primary focus:ring-2 focus:ring-primary/10 transition-all shadow-sm"
        />
        <button
          @click="sendMessage"
          :disabled="!userInput.trim() || isLoading"
          class="absolute right-2 top-2 bottom-2 bg-primary hover:bg-primary/90 disabled:opacity-50 disabled:cursor-not-allowed text-white rounded-lg px-3 transition-all flex items-center justify-center shadow-md active:scale-95"
        >
          <svg
            xmlns="http://www.w3.org/2000/svg"
            width="18"
            height="18"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"
          >
            <line x1="22" y1="2" x2="11" y2="13" />
            <polygon points="22 2 15 22 11 13 2 9 22 2" />
          </svg>
        </button>
      </div>
      <div class="mt-2 text-center">
        <p class="text-[10px] text-gray-400 font-medium">Raul VES AI puede cometer errores. Verifica la info.</p>
      </div>
    </div>
  </div>
</template>

<style scoped>
/* ESTILOS PARA EL CONTENIDO HTML DEL CHAT */
/* Usamos :deep() porque el HTML se inyecta dinámicamente */

:deep(.ai-content h3) {
  /* Títulos */
  font-weight: 700;
  font-size: 1rem;
  margin-bottom: 0.5rem;
  margin-top: 0.5rem;
  color: #1a1a1a;
}

:deep(.ai-content ul) {
  /* Listas desordenadas */
  list-style-type: disc;
  padding-left: 1.25rem;
  margin-bottom: 0.75rem;
}

:deep(.ai-content ol) {
  /* Listas ordenadas */
  list-style-type: decimal;
  padding-left: 1.25rem;
  margin-bottom: 0.75rem;
}

:deep(.ai-content li) {
  /* Elementos de lista */
  margin-bottom: 0.25rem;
}

:deep(.ai-content p) {
  /* Párrafos */
  margin-bottom: 0.75rem;
}

:deep(.ai-content strong) {
  /* Negritas */
  font-weight: 700;
  color: #000;
}

:deep(.ai-content blockquote) {
  /* Citas / Bloques destacados (si la IA decide usarlos) */
  border-left-width: 4px;
  border-color: #e5e7eb;
  padding-left: 1rem;
  font-style: italic;
  color: #4b5563;
}

/* Eliminar margen del último elemento para que cuadre bien en la burbuja */
:deep(.ai-content > *:last-child) {
  margin-bottom: 0;
}
</style>
