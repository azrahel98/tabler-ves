<template>
  <div class="space-y-2.5 mx-auto w-full max-w-6xl p-4 md:p-6 lg:p-8 lg:pt-0">
    <header-perfil v-if="perfilActual" />

    <div v-if="perfilActual" class="space-y-6">
      <!-- Sección Superior: Ficha de Información Personal (Izquierda) junto a Grados Académicos e Información Bancaria (Derecha) -->
      <div class="grid grid-cols-1 lg:grid-cols-[300px_1fr] gap-6 items-start">
        <!-- Columna Izquierda: Ficha de Información Personal (ancho compacto) -->
        <div class="w-full lg:w-[300px] shrink-0">
          <info />
        </div>
        <!-- Columna Derecha: Grados Académicos e Información Bancaria (alto mínimo y flujo natural libre de superposiciones) -->
        <div class="flex flex-col gap-6 min-w-0 w-full">
          <grado />
          <banco class="max-w-[320px] w-full" />
        </div>
      </div>

      <!-- Tabla de Vínculos Laborales (Siempre ocupa todo el ancho de forma espaciosa) -->
      <div class="w-full min-w-0">
        <vinculos-tabla />
      </div>

      <!-- Sección de Legajo Digital (Archivos y Carpetas) -->
      <legajo-folder />

      <!-- Sección Final: Historial de Cambios / Auditoría (Al final de todo) -->
      <historial />
    </div>
  </div>
</template>

<script setup lang="ts">
  import { onMounted, onUnmounted, watch } from 'vue'
  import { useRoute } from 'vue-router'
  import { usePersonalStore } from '../../stores/personal'
  import { storeToRefs } from 'pinia'
  import HeaderPerfil from '../../components/perfil/header.vue'
  import Info from '../../components/perfil/info.vue'
  import Banco from '../../components/perfil/banco.vue'
  import Grado from '../../components/perfil/grado.vue'
  import LegajoFolder from '../../components/perfil/legajo/folder.vue'
  import VinculosTabla from '../../components/perfil/vinculos-tabla.vue'
  import Historial from '../../components/perfil/historial.vue'

  const route = useRoute()
  const personalStore = usePersonalStore()
  const { perfilActual } = storeToRefs(personalStore)

  onMounted(async () => {
    await personalStore.obtenerPerfil(route.params.dni as string)
  })

  watch(
    () => route.params.dni as string,
    async (newDni) => {
      await personalStore.obtenerPerfil(newDni)
    }
  )

  onUnmounted(() => {
    personalStore.limpiarDatos()
  })
</script>
