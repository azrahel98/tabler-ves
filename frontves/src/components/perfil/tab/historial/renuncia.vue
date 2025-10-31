<template>
  <div class="card-body p-0 m-0 pb-3">
    <div class="credit-card mt-3">
      <div class="d-flex justify-content-between">
        <h3>{{ item?.documento }}</h3>
        <span>{{ item?.descripcion }}</span>
      </div>
      <div class="card-chip mt-3"></div>
      <div class="mt-4">
        <span class="card-number fs-5 mb-0">{{ item?.fecha }}</span>
        <p class="card-number m-0 fs-3">{{ item?.nombre }}</p>
      </div>
    </div>
    <div class="form"></div>
  </div>
</template>

<script setup lang="ts">
defineProps({
  item: { type: Object, required: true }
})
</script>
