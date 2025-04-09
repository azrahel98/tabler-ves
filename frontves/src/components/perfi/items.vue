<template>
  <div class="d-flex gap-4">
    <div class="avatar avatar-rounded avatar-sm">
      <slot />
    </div>
    <div class="d-flex flex-column">
      <span class="text-muted fw-medium fs-5" v-if="valor">{{ texto }}</span>
      <span v-else class="text-danger fw-medium fs-5">{{ texto }}</span>
      <span class="fw-medium small" v-if="valor">{{ valor }}</span>
      <span class="status-dot status-dot-animated bg-warning d-block h-1 w-1" v-else>{{ valor }}</span>
    </div>
  </div>
</template>
<script setup lang="ts">
defineProps({
  texto: { type: String, required: true },
  valor: { type: String }
})
</script>

<style lang="css" scoped>
span {
  padding: 0;
  margin: 0;
  font-size: 0.89rem;
}
</style>
