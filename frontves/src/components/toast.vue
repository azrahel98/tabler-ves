<template>
  <Transition name="fade">
    <div v-if="isVisible" class="toast-container position-fixed bottom-0 end-0 p-3">
      <div class="toast fade show" role="alert" aria-live="assertive" aria-atomic="true">
        <div class="d-flex justify-content-between w-100 pt-1" :class="['toast-header', headerClass]">
          <img src="../assets/logo.png" alt="Logo" width="30" class="avatar-img rounded-circle" />

          <span class="me-auto small fw-semibold">{{ store.nombre || 'Villa el Salvador' }}</span>
          <button type="button" class="ms-2 btn btn-icon btn-action btn-sm" @click="isVisible = false">
            <IconX class="icon" />
          </button>
        </div>
        <div class="toast-body fs-5">{{ message }}</div>
      </div>
    </div>
  </Transition>
</template>

<script setup lang="ts">
import { ref, computed, defineExpose } from 'vue'
import { userStore } from '../store/user'
import { IconX } from '@tabler/icons-vue'

const isVisible = ref(false)
const message = ref('')
const type = ref<'success' | 'error' | 'warning'>('success')
const store = userStore()

const showToast = (text: string, toastType: 'success' | 'error' | 'warning' = 'success') => {
  message.value = text
  type.value = toastType
  isVisible.value = true
  setTimeout(() => {
    isVisible.value = false
  }, 3000)
}

const headerClass = computed(
  () =>
    ({
      success: 'bg-success text-white',
      error: 'bg-danger-lt text-white',
      warning: 'bg-warning text-white'
    }[type.value])
)

defineExpose({ showToast })
</script>
