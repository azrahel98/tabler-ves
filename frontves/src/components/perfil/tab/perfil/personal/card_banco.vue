<template>
  <div class="card">
    <div class="card-header px-2 py-2 d-flex align-items-center flex-wrap gap-5 justify-content-between">
      <div class="d-flex align-items-center">
        <div class="px-2">
          <h4 class="mb-0 p-0 m-0">Información Bancaria</h4>
          <p class="text-muted mb-0 small">Datos de cuenta bancaria</p>
        </div>
      </div>
      <button
        v-if="!store.isUser"
        class="btn btn-facebook btn-sm d-flex flex-wrap align-items-center row-gap-0 column-gap-2"
        data-bs-toggle="modal"
        data-bs-target="#add_info_bancaria"
        :title="banco ? 'Editar información' : 'Agregar información'"
      >
        <IconPlus class="icon icon-sm" />
      </button>
    </div>
    <div class="card-body" v-if="banco != null">
      <div class="credit-card mt-3">
        <div class="d-flex justify-content-between">
          <h3>{{ banco?.banco }}</h3>
          <span>{{ banco?.tipo_cuenta }}</span>
        </div>
        <div class="card-chip mt-3"></div>
        <div class="mt-4">
          <span class="card-number fs-5 mb-0">{{ banco?.numero_cuenta }}</span>
          <p class="card-number m-0 fs-3">{{ banco?.cci }}</p>
        </div>
      </div>
      <div class="form"></div>
    </div>
    <addinfo v-if="banco == null && !store.isUser" />
    <addinfo v-else-if="banco != null && !store.isUser" :doc="banco" :is-edit="true" />
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { api } from '@api/axios'
import { router } from '@router/router'
import { userStore } from '@store/user'
import addinfo from '@comp/perfil/modal/agregar_banco.vue'
import { IconPlus } from '@tabler/icons-vue'

const banco = ref<any>(null)
const store = userStore()

const emit = defineEmits<{
  check: [value: boolean]
}>()

onMounted(async () => {
  try {
    const response = await (await api.post('/personal/banco_por_dni', { dni: router.currentRoute.value.params.dni })).data

    banco.value = response
    if (!banco.value) {
      emit('check', false)
    }
  } catch (error) {
    banco.value = null
  }
})
</script>

<style lang="scss" scoped>
.card-body {
  display: flex;
  justify-content: center;
  .credit-card {
    display: grid;
    max-width: 23vw;
    grid-template-rows: repeat(3, min-height);
    width: 90%;
    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
    border-radius: 16px;
    padding: 1.5rem;
    color: white;
    position: relative;
    overflow: hidden;

    .card-chip {
      width: 40px;
      height: 30px;
      background: linear-gradient(45deg, #ffd700, #ffed4e);
      border-radius: 6px;
      margin-bottom: 1rem;
      position: relative;
      box-shadow: inset 0 2px 4px rgba(0, 0, 0, 0.2);
    }

    .card-chip::before {
      content: '';
      position: absolute;
      top: 50%;
      left: 50%;
      transform: translate(-50%, -50%);
      width: 60%;
      height: 60%;
      background: #e6c200;
      border-radius: 2px;
    }
    .card-number {
      font-size: 1.5rem;
      font-weight: 200;
      letter-spacing: 4px;
      margin-bottom: 1.5rem;
    }
  }
  .credit-card::before {
    content: '';
    position: absolute;
    top: -50%;
    right: -50%;
    width: 100%;
    height: 100%;
    background: radial-gradient(circle, rgba(255, 255, 255, 0.008) 00%, transparent 100%);
  }
}
</style>
