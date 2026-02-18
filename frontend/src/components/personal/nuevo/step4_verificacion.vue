<template>
  <div class="max-w-4xl mx-auto space-y-3">
    <div class="bg-blue-50/50 border border-blue-100/50 rounded-lg p-3 flex items-start gap-3">
      <div class="p-1.5 bg-blue-100/50 rounded-md text-blue-600">
        <Info class="w-4 h-4" />
      </div>
      <div>
        <h3 class="text-xs font-bold text-blue-700 mb-0.5">Verificación de Datos</h3>
        <p class="text-[10px] text-blue-600/80 leading-relaxed">
          Por favor revisa cuidadosamente la información ingresada antes de proceder con el registro. Una vez creado, podrás editar la información desde el perfil.
        </p>
      </div>
    </div>

    <div class="border border-slate-200 rounded-lg overflow-hidden">
      <div class="bg-slate-50/50 px-3 py-2 border-b border-slate-200 flex items-center justify-between">
        <h4 class="text-[10px] font-bold text-slate-400 uppercase tracking-wider">Información de la Plaza</h4>
        <button @click="store.currentStep = 1" class="text-[10px] font-medium text-primary hover:underline">Editar</button>
      </div>
      <div class="p-3 grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 gap-3 text-xs">
        <div>
          <span class="block text-[9px] text-slate-400 uppercase tracking-wide mb-0.5">Código</span>
          <span class="font-semibold text-slate-700 font-mono">{{ store.plaza.codigo || '-' }}</span>
        </div>
        <div>
          <span class="block text-[9px] text-slate-400 uppercase tracking-wide mb-0.5">Cargo Estructural</span>
          <span class="font-semibold text-slate-700">{{ store.plaza.cargo_estructural.nombre || '-' }}</span>
        </div>
        <div>
          <span class="block text-[9px] text-slate-400 uppercase tracking-wide mb-0.5">Grupo Ocupacional</span>
          <span class="font-semibold text-slate-700">{{ store.plaza.grupo_ocupacional.nombre || '-' }}</span>
        </div>
        <div>
          <span class="block text-[9px] text-slate-400 uppercase tracking-wide mb-0.5">Reemplazo</span>
          <span class="font-semibold text-slate-700">{{ store.plaza.nombre || '-' }}</span>
        </div>
        <div>
          <span class="block text-[9px] text-slate-400 uppercase tracking-wide mb-0.5">Cargo</span>
          <span class="font-semibold text-slate-700">{{ store.plaza.cargo || '-' }}</span>
        </div>
        <div>
          <span class="block text-[9px] text-slate-400 uppercase tracking-wide mb-0.5">Condicion</span>
          <span class="font-semibold text-slate-700">{{ store.plaza.condicion || '-' }}</span>
        </div>
      </div>
    </div>

    <div class="border border-slate-200 rounded-lg overflow-hidden">
      <div class="bg-slate-50/50 px-3 py-2 border-b border-slate-200 flex items-center justify-between">
        <h4 class="text-[10px] font-bold text-slate-400 uppercase tracking-wider">Datos Personales</h4>
        <button @click="store.currentStep = 2" class="text-[10px] font-medium text-primary hover:underline">Editar</button>
      </div>
      <div class="p-3 grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 gap-3 text-xs">
        <div class="col-span-1 sm:col-span-2 md:col-span-1">
          <span class="block text-[9px] text-slate-400 uppercase tracking-wide mb-0.5">Nombre Completo</span>
          <span class="font-semibold text-slate-700">{{ store.personal.nombre || '-' }}</span>
        </div>
        <div>
          <span class="block text-[9px] text-slate-400 uppercase tracking-wide mb-0.5">DNI</span>
          <span class="font-mono font-semibold text-slate-700">{{ store.personal.dni || '-' }}</span>
        </div>
        <div>
          <span class="block text-[9px] text-slate-400 uppercase tracking-wide mb-0.5">Correo</span>
          <span class="text-slate-600">{{ store.personal.correo || '-' }}</span>
        </div>
        <div>
          <span class="block text-[9px] text-slate-400 uppercase tracking-wide mb-0.5">Teléfono</span>
          <span class="text-slate-600">{{ store.personal.telefono || '-' }}</span>
        </div>

        <div>
          <span class="block text-[9px] text-slate-400 uppercase tracking-wide mb-0.5">Fecha Nacimiento</span>
          <span class="text-slate-600">{{ store.personal.fecha_nacimiento || '-' }}</span>
        </div>
        <div>
          <span class="block text-[9px] text-slate-400 uppercase tracking-wide mb-0.5">Dirección</span>
          <span class="text-slate-600 truncate">{{ store.personal.direccion || '-' }}</span>
        </div>
      </div>
    </div>

    <div class="border border-slate-200 rounded-lg overflow-hidden">
      <div class="bg-slate-50/50 px-3 py-2 border-b border-slate-200 flex items-center justify-between">
        <h4 class="text-[10px] font-bold text-slate-400 uppercase tracking-wider">Documento de Ingreso</h4>
        <button @click="store.currentStep = 3" class="text-[10px] font-medium text-primary hover:underline">Editar</button>
      </div>
      <div class="p-3 grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 gap-3 text-xs">
        <div class="col-span-1 sm:col-span-2">
          <span class="block text-[9px] text-slate-400 uppercase tracking-wide mb-0.5">Documento</span>
          <span class="font-semibold text-slate-700">{{ store.documento.tipo_documento.nombre }} - {{ store.documento.tipo_documento.siglas }}</span>
        </div>
        <div>
          <span class="block text-[9px] text-slate-400 uppercase tracking-wide mb-0.5">Fecha Ingreso</span>
          <span class="text-slate-600">{{ store.documento.fecha_emision }}</span>
        </div>
        <div>
          <span class="block text-[9px] text-slate-400 uppercase tracking-wide mb-0.5">Observación</span>
          <span class="text-slate-600 truncate">{{ store.documento.observacion }}</span>
        </div>
        <div>
          <span class="block text-[9px] text-slate-400 uppercase tracking-wide mb-0.5">Régimen</span>
          <span class="font-semibold text-slate-700">D.L. {{ store.plaza.regimen.nombre }}</span>
        </div>
        <div>
          <span class="block text-[9px] text-slate-400 uppercase tracking-wide mb-0.5">Sueldo</span>
          <span class="font-mono font-semibold text-slate-700">S/. {{ store.plaza.sueldo || '0.00' }}</span>
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
