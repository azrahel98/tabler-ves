<template>
  <div class="space-y-2.5 mx-auto w-full p-4 md:p-6 lg:p-8 lg:pt-0">
    <header-perfil v-if="perfilActual" />

    <div v-if="perfilActual" class="space-y-6">
      <div class="grid grid-cols-1 lg:grid-cols-4 gap-6">
        <div class="lg:col-span-1 space-y-4">
          <info />
          <banco />
          <historial />
        </div>
        <div class="lg:col-span-3 space-y-4">
          <vinculos-tabla />
          <div class="grid grid-cols-1 md:grid-cols-6 gap-6 items-start justify-between w-full">
            <contacto class="col-span-2" />
            <grado class="col-span-2" />
          </div>
          <legajo-folder />
        </div>
      </div>
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
  import Contacto from '../../components/perfil/contacto.vue'
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
