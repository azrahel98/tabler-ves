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
  <div class="max-w-7xl mx-auto px-4 py-4">
    <div class="flex flex-col gap-6">
      <div class="flex flex-col lg:flex-row gap-6">
        <div class="w-full lg:w-1/3">
          <div class="overflow-y-auto rounded-lg shadow-md bg-card border border-border">
            <Cumples />
          </div>
        </div>

        <div class="w-full lg:w-1/4 flex flex-col gap-6">
          <Sexo :hombres="info.por_sexo?.[1]?.cantidad" :mujeres="info.por_sexo?.[0]?.cantidad" />
        </div>
        <div class="w-full lg:w-1/4 flex flex-col gap-6">
          <Regimen :regimenes="info.por_regimen" />
        </div>
      </div>
      <div class="flex flex-col lg:flex-row gap-6">
        <div class="w-full lg:w-1/3">
          <div class="overflow-y-auto rounded-lg shadow-md bg-card border border-border">
            <Renuncias />
          </div>
        </div>
      </div>
    </div>
  </div>
</template>
