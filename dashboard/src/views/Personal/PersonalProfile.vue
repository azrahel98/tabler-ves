<template>
  <div class="mx-auto w-full max-w-6xl p-4 md:p-6 lg:p-8 lg:pt-0">
    <Transition name="profile" appear>
      <div v-if="perfilActual" class="flex flex-col gap-8 md:gap-10">
        <!-- Cabecera de Perfil -->
        <header-perfil />

        <!-- Ficha Principal de Datos (Estructura de 3 Columnas Alineadas) -->
        <div class="flex flex-col gap-6">
          <div class="grid grid-cols-1 md:grid-cols-3 gap-6 items-start">
            <info class="md:col-span-1" />
            <vinculos-tabla class="md:col-span-2" />
          </div>

          <div class="grid grid-cols-1 md:grid-cols-3 gap-6 items-start">
            <grado class="md:col-span-2" />
            <banco class="md:col-span-1" />
          </div>
        </div>

        <!-- Carpeta de Legajo Digital -->
        <legajo-folder />

        <!-- Bitácora de Historial de Cambios -->
        <historial />
      </div>
    </Transition>
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
