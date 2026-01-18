<script setup lang="ts">
import { router } from '@router/router'
import { onMounted, watch } from 'vue'
import { useProfileStore } from '@store/perfil'
import Vinculos from '@comp/personal/vinculos.vue'
import Info from '@comp/personal/info.vue'
import Banco from '@comp/personal/banco.vue'
import Historial from '@comp/personal/historial.vue'

const perfil = useProfileStore()

const cargarPerfil = () => {
  const dni = router.currentRoute.value.params.dni?.toString() || ''
  perfil.fetchData(dni)
}

onMounted(cargarPerfil)

watch(
  () => router.currentRoute.value.params.dni,
  () => {
    cargarPerfil()
  }
)
</script>

<template>
  <div class="max-w-[1600px] mx-auto px-4 sm:px-6 py-6">
    <div class="grid grid-cols-1 lg:grid-cols-[230px_minmax(0,1fr)] gap-6 items-start">
      <aside class="w-full flex justify-center lg:block lg:sticky lg:top-4 z-10">
        <div class="w-full lg:max-w-none">
          <Info :user="perfil.perfil" :vinculo="perfil.vinculos" :Emergencia="perfil.contacto" />
        </div>
      </aside>

      <main class="flex flex-col 2xl:flex-row gap-6 w-full min-w-0">
        <div class="flex-1 w-full min-w-0 space-y-6 h-min">
          <Vinculos :vinculos="perfil.vinculos" />
          <Banco v-if="perfil.banco" />
        </div>

        <aside class="w-full xl:w-80 2xl:w-96 shrink-0">
          <div class="xl:sticky xl:top-4" v-if="perfil.historial.length > 0">
            <Historial :lista="perfil.historial" />
          </div>
        </aside>
      </main>
    </div>
  </div>
</template>
