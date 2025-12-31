<template>
  <div class="max-w-4xl mx-auto space-y-3">
    <div class="bg-info/10 rounded-xl p-0 flex items-start gap-4">
      <div class="p-2 bg-info/20 rounded-lg text-info">
        <Info class="w-5 h-5" />
      </div>
      <div>
        <h3 class="text-sm font-bold text-info mb-0.5">Verificación de Datos</h3>
        <p class="text-xs text-info/80 leading-relaxed">
          Por favor revisa cuidadosamente la información ingresada antes de proceder con el registro. Una vez creado, podrás editar la información desde el perfil del usuario.
        </p>
      </div>
    </div>

    <div class="border border-border rounded-2xl overflow-hidden">
      <div class="bg-muted/50 px-4 py-3 border-b border-border flex items-center justify-between">
        <h4 class="text-xs font-bold text-muted-foreground uppercase tracking-wider">Información de la Plaza</h4>
        <button @click="store.currentStep = 1" class="text-xs font-medium text-primary hover:underline">Editar</button>
      </div>
      <div class="p-4 grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 gap-4 text-sm">
        <div>
          <span class="block text-xs text-muted-foreground mb-0.5">Código</span>
          <span class="font-semibold text-foreground">{{ store.plaza.codigo || '-' }}</span>
        </div>
        <div>
          <span class="block text-xs text-muted-foreground mb-0.5">Cargo Estructural</span>
          <span class="font-semibold text-foreground">{{ store.plaza.cargo_estructural.nombre || '-' }}</span>
        </div>
        <div>
          <span class="block text-xs text-muted-foreground mb-0.5">Grupo Ocupacional</span>
          <span class="font-semibold text-foreground">{{ store.plaza.grupo_ocupacional.nombre || '-' }}</span>
        </div>
        <div>
          <span class="block text-xs text-muted-foreground mb-0.5">Reemplazo</span>
          <span class="font-semibold text-foreground">{{ store.plaza.nombre || '-' }}</span>
        </div>
        <div>
          <span class="block text-xs text-muted-foreground mb-0.5">Cargo</span>
          <span class="font-semibold text-foreground">{{ store.plaza.cargo || '-' }}</span>
        </div>
        <div>
          <span class="block text-xs text-muted-foreground mb-0.5">Condicion</span>
          <span class="font-semibold text-foreground">{{ store.plaza.condicion || '-' }}</span>
        </div>
      </div>
    </div>

    <div class="border border-border rounded-2xl overflow-hidden">
      <div class="bg-muted/50 px-4 py-3 border-b border-border flex items-center justify-between">
        <h4 class="text-xs font-bold text-muted-foreground uppercase tracking-wider">Datos Personales</h4>
        <button @click="store.currentStep = 2" class="text-xs font-medium text-primary hover:underline">Editar</button>
      </div>
      <div class="p-4 grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 gap-4 text-sm">
        <div class="col-span-1 sm:col-span-2 md:col-span-1">
          <span class="block text-xs text-muted-foreground mb-0.5">Nombre Completo</span>
          <span class="font-semibold text-foreground">{{ store.personal.nombre || '-' }}</span>
        </div>
        <div>
          <span class="block text-xs text-muted-foreground mb-0.5">DNI</span>
          <span class="font-mono font-semibold text-foreground">{{ store.personal.dni || '-' }}</span>
        </div>
        <div>
          <span class="block text-xs text-muted-foreground mb-0.5">Correo</span>
          <span class="text-foreground">{{ store.personal.correo || '-' }}</span>
        </div>
        <div>
          <span class="block text-xs text-muted-foreground mb-0.5">Teléfono</span>
          <span class="text-foreground">{{ store.personal.telefono || '-' }}</span>
        </div>

        <div>
          <span class="block text-xs text-muted-foreground mb-0.5">Fecha Nacimiento</span>
          <span class="text-foreground">{{ store.personal.fecha_nacimiento || '-' }}</span>
        </div>
        <div>
          <span class="block text-xs text-muted-foreground mb-0.5">Dirección</span>
          <span class="text-foreground">{{ store.personal.direccion || '-' }}</span>
        </div>
      </div>
    </div>

    <div class="border border-border rounded-2xl overflow-hidden">
      <div class="bg-muted/50 px-4 py-3 border-b border-border flex items-center justify-between">
        <h4 class="text-xs font-bold text-muted-foreground uppercase tracking-wider">Documento de Ingreso</h4>
        <button @click="store.currentStep = 3" class="text-xs font-medium text-primary hover:underline">Editar</button>
      </div>
      <div class="p-4 grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 gap-4 text-sm">
        <div class="col-span-1 sm:col-span-2">
          <span class="block text-xs text-muted-foreground mb-0.5">Documento</span>
          <span class="font-semibold text-foreground">{{ store.documento.tipo_documento.nombre }} - {{ store.documento.tipo_documento.siglas }}</span>
        </div>
        <div>
          <span class="block text-xs text-muted-foreground mb-0.5">Fecha Ingreso</span>
          <span class="text-foreground">{{ store.documento.fecha_emision }}</span>
        </div>
        <div>
          <span class="block text-xs text-muted-foreground mb-0.5">Observación</span>
          <span class="text-foreground">{{ store.documento.observacion }}</span>
        </div>
        <div>
          <span class="block text-xs text-muted-foreground mb-0.5">Régimen</span>
          <span class="font-semibold text-foreground">D.L. {{ store.plaza.regimen.nombre }}</span>
        </div>
        <div>
          <span class="block text-xs text-muted-foreground mb-0.5">Sueldo</span>
          <span class="font-mono font-semibold text-foreground">S/. {{ store.plaza.sueldo || '0.00' }}</span>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { useNuevoUsuarioStore } from '@store/nuevo_usuario'
import { Info } from 'lucide-vue-next'

const store = useNuevoUsuarioStore()
</script>
