<template>
  <div :class="datos == null ? 'col-auto' : 'col-md-6 col-sm-12 col-lg-4 col-12'" style="height: min-content">
    <div class="card">
      <div class="card-header px-2 py-2 d-flex align-items-center flex-wrap gap-5" :class="datos == null ? 'justify-content-center row-gap-4' : 'justify-content-between'">
        <div class="d-flex align-items-center">
          <div class="px-2">
            <h5 class="card-title mb-0 p-0 m-0">Información Bancaria</h5>
            <p class="text-muted mb-0 small">Datos de cuenta bancaria</p>
          </div>
        </div>
        <button
          v-if="!store.isUser"
          class="btn btn-facebook btn-sm d-flex flex-wrap align-items-center row-gap-0 column-gap-2"
          data-bs-toggle="modal"
          data-bs-target="#add_info_bancaria"
          :title="datos ? 'Editar información' : 'Agregar información'"
        >
          <IconPlus class="icon icon-sm" />
          <!-- <span class="d-none d-sm-inline">{{ datos ? 'Editar' : 'Agregar' }}</span> -->
        </button>
      </div>

      <div class="card-body" v-if="datos != null">
        <div class="credit-card">
          <div class="credit-card-front bg-secondary-subtle">
            <div class="d-flex justify-content-between align-items-start">
              <div class="text-black align-bottom small fw-medium">
                <div class="h3 fw-bold mb-0">{{ datos.banco || 'BANCO NO ESPECIFICADO' }}</div>
              </div>
              <IconBuildingBank class="icon icon-lg text-black" />
            </div>

            <div class="mb-3">
              <div class="text-black small fw-medium mb-1">NÚMERO DE CUENTA</div>
              <div class="text-secondary fw-bold mb-0 font-monospace">{{ datos.numero_cuenta }}</div>
            </div>

            <div class="d-flex justify-content-between align-items-end">
              <div>
                <div class="small fw-medium mb-1 text-black">TIPO DE CUENTA</div>
                <div class="fw-bold text-secondary">{{ datos.tipo_cuenta || 'NO ESPECIFICADO' }}</div>
              </div>
              <div class="text-end">
                <div class="text-black small fw-medium mb-1">CCI</div>
                <div class="fw-bold font-monospace small text-secondary">{{ formatCCI(datos.cci) }}</div>
              </div>
            </div>
          </div>
        </div>
      </div>

      <addinfo v-if="datos == null && !store.isUser" />
      <addinfo v-else-if="datos != null && !store.isUser" :doc="datos" :is-edit="true" />
    </div>
  </div>
</template>

<script setup lang="ts">
import { api } from '@api/axios'
import { router } from '@router/router'
import { IconBuildingBank, IconPlus } from '@tabler/icons-vue'
import addinfo from '@comp/perfil/modal/agregar_banco.vue'
import { onMounted, ref } from 'vue'
import { userStore } from '@store/user'

const datos = ref<any>({})
const store = userStore()

const formatCCI = (cci: string) => {
  if (!cci) return '••• ••• ••• •••••••••'
  return cci.replace(/(.{3})/g, '$1 ').trim()
}

const emit = defineEmits<{
  check: [value: boolean]
}>()

onMounted(async () => {
  try {
    datos.value = await (await api.post('/personal/banco_por_dni', { dni: router.currentRoute.value.params.dni })).data
    if (!datos.value) {
      emit('check', false)
    }
  } catch (error) {
    datos.value = null
  }
})
</script>

<style scoped>
.gap-2 {
  gap: 0.5rem;
}

.credit-card {
  perspective: 1000px;
  margin-bottom: 1rem;
}

.credit-card-front {
  border-radius: 16px;
  padding: 2rem;
  position: relative;
  overflow: hidden;
  box-shadow: 0 1px 2px rgba(0, 0, 0, 0.2);
  min-height: 200px;
  display: flex;
  flex-direction: column;
  justify-content: space-between;
}

.credit-card-front::before {
  content: '';
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: linear-gradient(135deg, rgba(255, 255, 255, 0.1) 0%, rgba(255, 255, 255, 0.05) 100%);
  pointer-events: none;
}

.card-decoration {
  position: absolute;
  top: -50px;
  right: -50px;
  width: 150px;
  height: 150px;
  background: radial-gradient(circle, rgba(255, 255, 255, 0.1) 0%, transparent 70%);
  border-radius: 50%;
}

.card-decoration::after {
  content: '';
  position: absolute;
  top: 80px;
  left: 80px;
  width: 100px;
  height: 100px;
  background: radial-gradient(circle, rgba(255, 255, 255, 0.05) 0%, transparent 70%);
  border-radius: 50%;
}

@media (max-width: 768px) {
  .credit-card-front {
    padding: 1.5rem;
    min-height: 180px;
  }

  .credit-card-front .h3 {
    font-size: 1.25rem;
  }

  .credit-card-front .h4 {
    font-size: 1.1rem;
  }
}
</style>
