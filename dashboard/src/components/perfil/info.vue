<template>
  <div class="rounded-2xl border border-stroke bg-white p-6 shadow-sm dark:border-strokedark dark:bg-boxdark">
    <div class="flex items-center gap-2 text-xs font-bold uppercase tracking-wider text-black dark:text-white mb-6">
      <svg class="h-5 w-5 text-primary" fill="currentColor" viewBox="0 0 24 24">
        <path d="M12 12c2.21 0 4-1.79 4-4s-1.79-4-4-4-4 1.79-4 4 1.79 4 4 4zm0 2c-2.67 0-8 1.34-8 4v2h16v-2c0-2.66-5.33-4-8-4z" />
      </svg>
      PERSONAL INFORMATION
    </div>

    <div class="space-y-5">
      <div>
        <p class="text-[10px] font-medium uppercase tracking-wider text-gray-400">Telefono Celular</p>
        <p class="mt-1 font-medium text-black dark:text-white">{{ perfilActual.telf || '999999999' }}</p>
      </div>
      <div>
        <p class="text-[10px] font-medium uppercase tracking-wider text-gray-400">Correo electronico</p>
        <p class="mt-1 font-medium text-black dark:text-white">{{ perfilActual.email || 'juan.perez@company.com' }}</p>
      </div>
      <div>
        <p class="text-[10px] font-medium uppercase tracking-wider text-gray-400">Direccion</p>
        <p class="mt-1 font-medium text-black dark:text-white">{{ perfilActual.direccion || 'xx 12 xx lima xx' }}</p>
      </div>
      <div>
        <p class="text-[10px] font-medium uppercase tracking-wider text-gray-400">DNI / RUC</p>
        <p class="mt-1 font-medium text-black dark:text-white">
          {{ perfilActual.dni }} <span v-if="perfilActual.ruc" class="text-gray-400">| {{ perfilActual.ruc }}</span>
        </p>
      </div>
      <div class="grid grid-cols-2 gap-4">
        <div>
          <p class="text-[10px] font-medium uppercase tracking-wider text-gray-400">FECHA DE NACIMIENTO</p>
          <p class="mt-1 font-medium text-black dark:text-white">{{ formatUTC(perfilActual.nacimiento) }}</p>
        </div>
        <div>
          <p class="text-[10px] font-medium uppercase tracking-wider text-gray-400">SEXO</p>
          <p class="mt-1 font-medium text-black dark:text-white">{{ perfilActual.sexo === 'M' ? 'Masculino' : perfilActual.sexo === 'F' ? 'Femenino' : '-' }}</p>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
  import { addMinutes, format } from 'date-fns'
  import { usePersonalStore } from '../../stores/personal'
  import { es } from 'date-fns/locale'

  const { perfilActual } = usePersonalStore()

  const formatUTC = (dateString: string) => {
    if (!dateString) return '-'
    const date = new Date(dateString)

    const dateCorrected = addMinutes(date, date.getTimezoneOffset())

    return format(dateCorrected, "d 'de' MMMM 'del' yyyy", { locale: es })
  }
</script>
