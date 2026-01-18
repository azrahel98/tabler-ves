<template>
  <div class="bg-white rounded-2xl px-6 py-5 shadow-lg border border-border w-full">
    <header class="flex flex-col lg::flex-row items-center gap-4 border-b border-border pb-4 mb-4">
      <div class="flex items-center gap-2">
        <div class="relative w-16 h-16 cursor-pointer group z-10 shrink-0" @click="openFilePicker" title="Cambiar foto de perfil">
          <div class="w-16 h-16 rounded-full bg-primary flex items-center justify-center overflow-hidden transition-shadow group-hover:shadow-lg">
            <img v-if="user.avatar" :src="currentImage" alt="Foto de perfil" class="w-full h-full object-cover" />
            <span v-else class="text-2xl font-semibold text-white tracking-tight">
              {{ nombreabrv(user.nombre) }}
            </span>
          </div>
          <div
            v-if="!previewImage"
            class="absolute inset-0 w-full h-full rounded-full bg-black/50 opacity-0 group-hover:opacity-100 flex items-center justify-center transition-opacity z-0 pointer-events-none"
          >
            <camera-icon class="w-6 h-6 text-teal" />
          </div>
        </div>

        <div v-if="previewImage" class="relative w-16 h-16 shrink-0 bg animate-fade-in-right">
          <div class="w-16 h-16 rounded-full bg-gray-100 flex items-center justify-center overflow-hidden border-2 border-dashed border-teal shadow-md relative z-10">
            <img :src="previewImage" alt="Preview" class="w-full h-full object-cover" />
          </div>
        </div>
        <div v-if="previewImage" class="flex flex-col gap-2 animate-fade-in-right">
          <button
            @click="saveAvatar"
            :disabled="isUploading"
            class="p-2 rounded-full bg-teal-200 text-green-700 shadow hover:bg-teal/90 disabled:opacity-50 disabled:cursor-not-allowed transition-all hover:scale-105 active:scale-95"
            title="Confirmar cambio"
          >
            <check-icon class="w-4 h-4" />
          </button>

          <button
            @click="cancelPreview"
            :disabled="isUploading"
            class="p-2 rounded-full bg-red-100 text-red-500 shadow hover:bg-red-200 disabled:opacity-50 transition-all hover:scale-105 active:scale-95"
            title="Cancelar"
          >
            <x-icon class="w-4 h-4" />
          </button>
        </div>
      </div>

      <input ref="fileInputRef" type="file" accept="image/*" class="hidden" @change="handleFileChange" />

      <div class="flex-1 min-w-0 text-center lg:text-center w-full lg:w-auto">
        <h5 class="text-lg font-bold text-text-primary tracking-tight leading-tight">
          {{ user.nombre }}
        </h5>
        <p class="text-sm mt-1 font-normal text-text-muted">{{ user.dni }}</p>
      </div>
    </header>

    <section class="space-y-2.5 mb-3">
      <div class="flex justify-between items-center">
        <h6 class="text-sm font-semibold tracking-tight mb-2 uppercase">Información Personal</h6>
        <Editar_info v-if="app.canEdit" />
      </div>
      <div class="flex items-center gap-3">
        <div class="p-1.5 rounded-lg bg-purple-light/20 text-primary shrink-0">
          <mail class="w-5 h-5" />
        </div>
        <div>
          <p class="text-xs text-text-muted font-medium">Correo</p>
          <p class="text-sm text-text-primary font-normal break-all">
            {{ user.email }}
          </p>
        </div>
      </div>

      <div class="flex items-center gap-3">
        <div class="p-1.5 rounded-lg bg-teal/20 text-teal">
          <phone class="w-5 h-5" />
        </div>
        <div>
          <p class="text-xs text-text-muted font-medium">Teléfono</p>
          <p class="text-sm text-text-primary font-normal">{{ user.telf }}</p>
        </div>
      </div>

      <div class="flex items-center gap-3">
        <div class="p-1.5 rounded-lg bg-blue/20 text-blue">
          <map-pin-icon class="w-5 h-5" />
        </div>
        <div>
          <p class="text-xs text-text-muted font-medium">Dirección</p>
          <p class="text-sm text-text-primary font-normal">{{ user.direccion }}</p>
        </div>
      </div>

      <div class="flex items-center gap-3">
        <div class="p-1.5 rounded-lg bg-blue/20 text-blue">
          <cake-icon class="w-5 h-5" />
        </div>
        <div>
          <p class="text-xs text-text-muted font-medium">Nacimiento</p>
          <p class="text-sm text-text-primary font-normal">{{ formatFechaCompleta(user.nacimiento) }}</p>
        </div>
      </div>

      <div class="flex items-center gap-3">
        <div class="p-1.5 rounded-lg bg-blue/20 text-blue">
          <cake-icon class="w-5 h-5" />
        </div>
        <div>
          <p class="text-xs text-text-muted font-medium">Edad</p>
          <p class="text-sm text-text-primary font-normal">{{ getYear(new Date()) - getYear(new Date(user.nacimiento)) }} años</p>
        </div>
      </div>
    </section>

    <div class="border-b border-border mb-4" v-if="Emergencia" />

    <section class="space-y-4" v-if="Emergencia">
      <h3 class="text-sm font-semibold text-text-primary tracking-tight mb-2 uppercase">Contacto de Emergencia</h3>

      <div class="flex items-center gap-3">
        <div class="p-1.5 rounded-lg bg-destructive/20 text-destructive">
          <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z"></path>
          </svg>
        </div>
        <div>
          <p class="text-xs text-text-muted font-medium">Nombre &amp; Relación</p>
          <p class="text-sm text-text-primary font-normal">
            {{ Emergencia.nombre }}
            <span class="text-text-muted">({{ Emergencia.relacion }})</span>
          </p>
        </div>
      </div>

      <div class="flex items-center gap-3">
        <div class="p-1.5 rounded-lg bg-destructive/20 text-destructive">
          <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
            <path
              stroke-linecap="round"
              stroke-linejoin="round"
              stroke-width="2"
              d="M3 5a2 2 0 012-2h3.28a1 1 0 01.948.684l1.498 4.493a1 1 0 01-.502 1.21l-2.257 1.13a11.042 11.042 0 005.516 5.516l1.13-2.257a1 1 0 011.21-.502l4.493 1.498a1 1 0 01.684.949V19a2 2 0 01-2 2h-11a2 2 0 01-2-2z"
            ></path>
          </svg>
        </div>
        <div>
          <p class="text-xs text-text-muted font-medium">Teléfono de Emergencia</p>
          <p class="text-sm text-text-primary font-normal">{{ Emergencia.telefono }}</p>
        </div>
      </div>
    </section>
  </div>
</template>

<script lang="ts" setup>
import { ref, watch } from 'vue'
import { nombreabrv } from '@tools/abrv'
import { getYear } from 'date-fns'
import { formatFechaCompleta } from '@tools/date'
import { CakeIcon, CameraIcon, Mail, Phone, MapPinIcon, CheckIcon, XIcon } from 'lucide-vue-next'
import Editar_info from './modal/editar_info.vue'
import { userStore } from '@store/user'
import { api, SERVER } from '@api/axios'

const app = userStore()

const props = defineProps({
  user: { type: Object, required: true },
  vinculo: { type: Object, required: true },
  Emergencia: { type: Object, required: true }
})

const emit = defineEmits(['file-selected'])

const fileInputRef = ref<HTMLInputElement | null>(null)
const currentImage = ref(props.user.avatar)
const previewImage = ref<string | null>(null)
const pendingFile = ref<File | null>(null)
const isUploading = ref(false)

watch(
  () => props.user.avatar,
  (newUrl) => {
    currentImage.value = `${SERVER}${newUrl}`
  }
)

const openFilePicker = () => fileInputRef.value?.click()

const handleFileChange = (event: Event) => {
  const target = event.target as HTMLInputElement
  const file = target.files?.[0]

  if (file) {
    pendingFile.value = file
    const reader = new FileReader()
    reader.onload = (e) => {
      previewImage.value = e.target?.result as string
    }
    reader.readAsDataURL(file)
  }
}

const cancelPreview = () => {
  previewImage.value = null
  pendingFile.value = null
  if (fileInputRef.value) fileInputRef.value.value = ''
}

const saveAvatar = async () => {
  if (!pendingFile.value) return

  try {
    isUploading.value = true
    const formData = new FormData()
    formData.append('avatar', pendingFile.value)

    await api.post(`/avatar/upload/${props.user.dni}`, formData, {
      headers: {
        'Content-Type': 'multipart/form-data'
      }
    })

    currentImage.value = previewImage.value
    cancelPreview()
  } catch (error) {
    console.error(error)
  } finally {
    isUploading.value = false
  }
}
</script>

<style scoped>
.animate-fade-in-right {
  animation: fadeInRight 0.3s ease-out forwards;
}

@keyframes fadeInRight {
  from {
    opacity: 0;
    transform: translateX(-10px);
  }
  to {
    opacity: 1;
    transform: translateX(0);
  }
}
</style>
