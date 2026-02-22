<template>
  <div class="space-y-6 mx-auto w-full p-4 md:p-6 lg:p-8">
    <header-perfil v-if="perfilActual" />

    <div v-if="perfilActual" class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-6">
      <div class="col-span-1 md:col-span-2 lg:col-span-2 xl:col-span-3 space-y-6">
        <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
          <info />
          <vinculo />
        </div>

        <legajo-folder />
      </div>

      <div class="col-span-1 md:col-span-2 lg:col-span-1 xl:col-span-1 space-y-6">
        <banco />
        <grado />
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
  import { onMounted, watch } from 'vue'
  import { useRoute } from 'vue-router'
  import { usePersonalStore } from '../../stores/personal'
  import { storeToRefs } from 'pinia'
  import HeaderPerfil from '../../components/perfil/header.vue'
  import Info from '../../components/perfil/info.vue'
  import Vinculo from '../../components/perfil/vinculo.vue'
  import Banco from '../../components/perfil/banco.vue'
  import Grado from '../../components/perfil/grado.vue'
  import LegajoFolder from '../../components/perfil/legajo/folder.vue'

  const route = useRoute()
  const personalStore = usePersonalStore()
  const { perfilActual } = storeToRefs(personalStore)

  onMounted(async () => {
    console.log('DNI', route.params.dni)
    await personalStore.obtenerPerfil(route.params.dni as string)
  })

  watch(
    () => route.params.dni as string,
    async (newDni) => {
      console.log('DNI CAMBIADO', newDni)
      await personalStore.obtenerPerfil(newDni)
    }
  )
</script>
