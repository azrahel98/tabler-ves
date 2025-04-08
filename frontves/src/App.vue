<template>
  <div>
    <router-view></router-view>
    <ToastNoti ref="toastRef" />
  </div>
</template>

<script setup lang="ts">
import { onMounted, provide, ref } from 'vue'
import ToastNoti from '@comp/toast.vue'

const toastRef = ref<InstanceType<typeof ToastNoti> | null>(null)

onMounted(() => {
  if (toastRef.value) {
    provide('toast', toastRef.value)
  }
})
</script>
