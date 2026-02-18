<script setup lang="ts">
import Cumples from '@comp/dash/cumples.vue'

import Sexo from '@comp/dash/sexo.vue'
import Regimen from '@comp/dash/regimen.vue'

import { onMounted, ref } from 'vue'
import { api } from '@api/axios'
import Renuncias from '@comp/dash/renuncias.vue'

const info = ref<any>({})

onMounted(async () => {
  info.value = (await api.post('/dash/info')).data
})
</script>

<template>
  <div class="max-w-[1400px] mx-auto px-2 py-2">
    <div class="flex flex-wrap gap-2">
      <!-- Sexo -->
      <div class="w-full md:w-[48%] lg:w-[20%]">
        <Sexo :hombres="info.por_sexo?.[1]?.cantidad" :mujeres="info.por_sexo?.[0]?.cantidad" />
      </div>

      <!-- Regimen -->
      <div class="w-full md:w-[48%] lg:w-[20%]">
        <Regimen :regimenes="info.por_regimen" />
      </div>

      <!-- Cumples -->
      <div class="w-full md:w-[48%] lg:w-[32%]">
        <Cumples />
      </div>

      <!-- Renuncias -->
      <div class="w-full md:w-[48%] lg:w-[32%]">
        <Renuncias />
      </div>
    </div>
  </div>
</template>
