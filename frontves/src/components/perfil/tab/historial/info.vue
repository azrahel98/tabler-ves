<template>
  <form @submit.prevent="" class="p-3">
    <div class="row g-3">
      <div class="col-md-12">
        <label class="form-label">
          <MapPin class="icon text-blue" />
          Dirección
        </label>

        <div>
          <strong class="text-hint fw-normal">{{ perfil.direccion }}</strong>
        </div>
      </div>

      <div class="col-md-6">
        <label class="form-label">
          <Phone class="icon text-blue" />
          Teléfono
        </label>

        <div>
          <strong class="text-hint fw-normal">{{ perfil.telf }}</strong>
        </div>
      </div>

      <div class="col-md-6">
        <label class="form-label">
          <Mail class="icon text-blue" />
          Correo Electrónico
        </label>

        <div>
          <strong class="text-hint fw-normal">{{ perfil.email }}</strong>
        </div>
      </div>

      <div class="col-md-6">
        <label class="form-label">
          <FileText class="icon icon-sm me-1 text-secondary" />
          RUC
        </label>

        <div>
          <strong class="text-hint fw-normal">{{ perfil.ruc }}</strong>
        </div>
      </div>
    </div>
  </form>
</template>

<script setup lang="ts">
import { FileText, Mail, MapPin, Phone } from 'lucide-vue-next'
defineProps({
  perfil: { type: Object, required: true }
})
</script>
