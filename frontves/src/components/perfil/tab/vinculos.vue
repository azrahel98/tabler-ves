<template>
  <div class="card">
    <div class="vinculos-container">
      <div class="d-flex justify-content-between align-items-center p-3 border-bottom">
        <h3 class="card-title mb-0">Vínculos Laborales</h3>
      </div>
      <div class="timeline-scroll-container">
        <div class="p-3" style="max-height: 40vh; overflow-y: auto">
          <ul class="timeline" v-if="!isloading && vinculos && vinculos.length > 0">
            <Timeline :x="x" v-for="x in (vinculos as Array<any>)" :key="x.Id" :click_collapse="click_collapse" />
          </ul>
          <div v-else-if="isloading" class="d-flex flex-column gap-4">
            <div class="card placeholder-glow" v-for="i in 4" :key="i">
              <div class="placeholder col-9 mb-3 pt-2"></div>
            </div>
          </div>
          <div v-else class="empty">
            <div class="empty-icon">
              <IconBriefcase class="icon-lg" />
            </div>
            <p class="empty-title">No hay vínculos registrados</p>
            <p class="empty-subtitle text-muted">No se encontraron vínculos laborales para esta persona</p>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script lang="ts" setup>
import Timeline from '@comp/perfil/tab/vinculos/timeline.vue'
import { IconBriefcase } from '@tabler/icons-vue'

defineProps({
  isloading: { type: Boolean, default: false },
  vinculos: { type: Array, required: true }
})

const click_collapse = (x: number) => {
  document.getElementById(`collapse#${x}`)?.classList.toggle('toggle-collapse')
}
</script>
