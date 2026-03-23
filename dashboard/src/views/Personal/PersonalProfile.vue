<template>
  <div class="space-y-2.5 mx-auto w-full p-4 md:p-6 lg:p-8 lg:pt-0">
    <header-perfil v-if="perfilActual" />

    <div v-if="perfilActual" class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-2">
      <div class="col-span-1 md:col-span-2 lg:col-span-2 xl:col-span-3 space-y-6">
        <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
          <info />
          <vinculo />
        </div>

        <vinculos-tabla />

        <legajo-folder />
      </div>

      <div class="col-span-1 md:col-span-2 lg:col-span-1 xl:col-span-1 space-y-6">
        <contacto />
        <banco />
        <grado />
        <historial />
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
  import Vinculo from '../../components/perfil/vinculo.vue'
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
