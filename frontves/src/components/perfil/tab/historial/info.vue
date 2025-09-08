<template>
  <form @submit.prevent="" class="p-3">
    <div class="row">
      <div
        v-for="(x, y) in Object.entries(perfil)"
        :key="y"
        :class="[(x[1] !== null && x[0] === 'direccion') || (x[0] === 'telf' && x[1] != null) || (x[0] === 'email' && x[1] != null) ? 'col-md-6' : 'd-none']"
      >
        <div v-if="x[0] === 'direccion' && x[1] != null">
          <label class="form-label">
            <MapPin class="icon text-warning" />
            Dirección
          </label>

          <div>
            <strong class="text-hint fw-normal">{{ x[1] }}</strong>
          </div>
        </div>
        <div v-else-if="x[0] === 'telf' && x[1] != null">
          <label class="form-label">
            <Phone class="icon text-warning" />
            Teléfono
          </label>

          <div>
            <strong class="text-hint fw-normal">{{ x[1] }}</strong>
          </div>
        </div>
        <div v-else-if="x[0] === 'email' && x[1] != null">
          <label class="form-label text-secondary fw-medium">
            <MailCheck class="icon icon-sm me-1 text-warning" />
            Correo
          </label>

          <div>
            <strong class="text-hint fw-normal">{{ x[1] }}</strong>
          </div>
        </div>
        <div v-else></div>
      </div>
    </div>
  </form>
</template>

<script setup lang="ts">
import { MailCheck, MapPin, Phone } from 'lucide-vue-next'
defineProps({
  perfil: { type: Object, required: true }
})
</script>
