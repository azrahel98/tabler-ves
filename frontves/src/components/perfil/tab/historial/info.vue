<template>
  <form @submit.prevent="" class="p-2 small">
    <div class="row">
      <div
        v-for="(x, y) in Object.entries(perfil)"
        :key="y"
        :class="[(x[1] !== null && x[0] === 'direccion') || (x[0] === 'telf' && x[1] != null) || (x[0] === 'email' && x[1] != null) ? 'col-auto' : 'd-none']"
      >
        <div v-if="x[0] === 'direccion' && x[1] != null">
          <label class="form-label fs-5">
            <MapPin class="icon text-warning icon-sm" />
            Dirección
          </label>

          <div>
            <strong class="small fw-normal">{{ x[1] }}</strong>
          </div>
        </div>
        <div v-else-if="x[0] === 'telf' && x[1] != null">
          <label class="form-label fs-5">
            <Phone class="icon text-warning icon-sm" />
            Teléfono
          </label>

          <div>
            <strong class="small fw-normal">{{ x[1] }}</strong>
          </div>
        </div>
        <div v-else-if="x[0] === 'email' && x[1] != null">
          <label class="form-label fs-5">
            <MailCheck class="icon icon-sm me-1 text-warning" />
            Correo
          </label>

          <div>
            <strong class="small fw-normal">{{ x[1] }}</strong>
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
