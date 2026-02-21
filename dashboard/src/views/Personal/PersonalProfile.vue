<template>
  <div class="space-y-6 mx-auto w-full p-4 md:p-6 lg:p-8">
    <header-perfil v-if="perfilActual" />

    <div v-if="perfilActual" class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-6">
      <!-- Lado Izquierdo, Info Relevante -->
      <div class="col-span-1 md:col-span-2 lg:col-span-2 xl:col-span-3 space-y-6">
        <!-- Tarjetas Superiores Information & Relations -->
        <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
          <!-- Personal Information -->
          <info />

          <!-- Labor Relations & Bancario -->
          <div class="rounded-2xl border border-stroke bg-white p-6 shadow-sm dark:border-strokedark dark:bg-boxdark">
            <div class="flex items-center gap-2 text-xs font-bold uppercase tracking-wider text-black dark:text-white mb-6">
              <svg class="h-5 w-5 text-primary" fill="currentColor" viewBox="0 0 24 24">
                <path d="M20 6h-4V4c0-1.11-.89-2-2-2h-4c-1.11 0-2 .89-2 2v2H4c-1.11 0-1.99.89-1.99 2L2 19c0 1.11.89 2 2 2h16c1.11 0 2-.89 2-2V8c0-1.11-.89-2-2-2zm-6 0h-4V4h4v2z" />
              </svg>
              LABOR RELATIONS & BANK
            </div>

            <div class="space-y-5">
              <div>
                <p class="text-[10px] font-bold uppercase tracking-wider text-gray-400">DEPARTMENT</p>
                <p class="mt-1 font-medium text-black dark:text-white">{{ vinculos.length > 0 ? vinculos[0].area : 'Engineering' }}</p>
              </div>
              <div>
                <p class="text-[10px] font-bold uppercase tracking-wider text-gray-400">RÉGIMEN ASIGNADO</p>
                <p class="mt-1 font-medium text-black dark:text-white">{{ vinculos.length > 0 ? vinculos[0].regimen : 'D.L. 276' }}</p>
              </div>
              <div>
                <p class="text-[10px] font-bold uppercase tracking-wider text-gray-400">DIRECT MANAGER (Ejemplo)</p>
                <div class="mt-1 flex items-center gap-2">
                  <div class="flex h-6 w-6 items-center justify-center rounded-full bg-blue-100 text-[10px] font-bold text-blue-600">SJ</div>
                  <span class="font-medium text-black dark:text-white">Sarah Jenkins</span>
                </div>
              </div>
              <div>
                <p class="text-[10px] font-bold uppercase tracking-wider text-gray-400">HIRE DATE</p>
                <p class="mt-1 font-medium text-black dark:text-white">{{ vinculos.length > 0 ? vinculos[0].fecha_ingreso : 'January 12, 2021 (3.2 years)' }}</p>
              </div>
              <div>
                <p class="text-[10px] font-bold uppercase tracking-wider text-gray-400">BANK INFO ({{ infoBancaria ? infoBancaria.banco : '-' }})</p>
                <p class="mt-1 font-medium text-black dark:text-white">{{ infoBancaria ? infoBancaria.numero_cuenta : 'Sin datos bancarios' }}</p>
                <p class="text-xs text-gray-500">CCI: {{ infoBancaria ? infoBancaria.cci : '-' }}</p>
              </div>
            </div>
          </div>
        </div>

        <!-- Virtual Folder -->
        <div class="rounded-2xl border border-stroke bg-white shadow-sm dark:border-strokedark dark:bg-boxdark overflow-hidden">
          <div class="flex items-center justify-between border-b border-stroke p-6 dark:border-strokedark">
            <div class="flex items-center gap-2 text-xs font-bold uppercase tracking-wider text-black dark:text-white">
              <svg class="h-5 w-5 text-primary" fill="currentColor" viewBox="0 0 24 24">
                <path d="M20 6h-8l-2-2H4c-1.1 0-1.99.9-1.99 2L2 18c0 1.1.9 2 2 2h16c1.1 0 2-.9 2-2V8c0-1.1-.9-2-2-2zm0 12H4V8h16v10z" />
              </svg>
              VIRTUAL FOLDER (LEGAJO)
            </div>
            <button class="text-sm font-semibold text-primary hover:underline">Upload Document</button>
          </div>

          <div class="divide-y divide-stroke dark:divide-strokedark">
            <div v-if="legajo.length === 0" class="p-6 text-center text-gray-500">No documents uploaded yet.</div>
            <!-- Elemento iterado de legajo o de muestra -->
            <div v-for="(item, i) in legajo.length > 0 ? legajo : mockDocuments" :key="i" class="flex items-center p-6 hover:bg-gray-50 transition cursor-pointer dark:hover:bg-meta-4/30">
              <div
                class="flex h-10 w-10 shrink-0 items-center justify-center rounded-full"
                :class="{
                  'bg-blue-50 text-blue-500 dark:bg-blue-900/20 dark:text-blue-400': item.type === 'pdf' || !item.type,
                  'bg-orange-50 text-orange-500 dark:bg-orange-900/20 dark:text-orange-400': item.type === 'image',
                  'bg-green-50 text-green-500 dark:bg-green-900/20 dark:text-green-400': item.type === 'data',
                }">
                <svg class="h-5 w-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    stroke-width="2"
                    d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"></path>
                </svg>
              </div>
              <div class="ml-4 flex-1">
                <h4 class="font-bold text-black dark:text-white text-sm">{{ item.title || item.descrip }}</h4>
                <p class="text-xs text-gray-500 dark:text-gray-400">{{ item.meta || `Fecha: ${item.fecha}` }}</p>
              </div>
            </div>
          </div>
        </div>
      </div>

      <!-- Lado Derecho, Stats & Attendance -->
      <div class="col-span-1 lg:col-span-1 xl:col-span-1 space-y-6">
        <!-- Attendance Widget -->
        <div class="rounded-2xl border border-stroke bg-white p-6 shadow-sm dark:border-strokedark dark:bg-boxdark text-center">
          <div class="flex items-center justify-center gap-2 text-xs font-bold uppercase tracking-wider text-black dark:text-white mb-6">
            <svg class="h-5 w-5 text-primary" fill="currentColor" viewBox="0 0 24 24">
              <path d="M19 3h-1V1h-2v2H8V1H6v2H5c-1.11 0-1.99.9-1.99 2L3 19c0 1.1.89 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm0 16H5V8h14v11zM7 10h5v5H7z" />
            </svg>
            ATTENDANCE
          </div>

          <!-- Circular Progress -->
          <div class="relative mx-auto h-40 w-40">
            <svg class="h-full w-full" viewBox="0 0 100 100">
              <circle class="stroke-gray-100 dark:stroke-meta-4" stroke-width="8" fill="transparent" r="42" cx="50" cy="50" />
              <circle
                class="stroke-primary"
                stroke-width="8"
                stroke-linecap="round"
                fill="transparent"
                r="42"
                cx="50"
                cy="50"
                stroke-dasharray="264"
                stroke-dashoffset="15"
                transform="rotate(-90 50 50)" />
            </svg>
            <div class="absolute inset-0 flex flex-col items-center justify-center">
              <span class="text-3xl font-bold text-black dark:text-white mt-2">98%</span>
              <span class="text-[9px] font-bold uppercase tracking-widest text-gray-400">EFFICIENCY</span>
            </div>
          </div>

          <div class="mt-6">
            <h4 class="font-bold text-black dark:text-white text-lg">Good Attendance</h4>
            <p class="text-xs text-gray-500 mt-2 px-4 leading-relaxed">{{ perfilActual.nombre.split(' ')[0] }} has only missed 2 days in the last 6 months.</p>
          </div>

          <div class="mt-8 border-t border-stroke pt-6 dark:border-strokedark">
            <p class="text-[10px] font-bold uppercase tracking-wider text-gray-400 mb-4 text-left">CURRENT WEEK</p>
            <div class="flex justify-between items-center rounded-xl bg-gray-50 dark:bg-meta-4/40 p-3">
              <div v-for="day in ['M', 'T', 'W', 'T', 'F', 'S', 'S']" :key="day" class="flex flex-col items-center gap-2">
                <span class="text-[10px] font-bold text-gray-400">{{ day }}</span>
                <div v-if="['M', 'T', 'W', 'T'].includes(day)" class="h-6 w-6 rounded-full bg-[#00B884] text-white flex items-center justify-center">
                  <svg class="h-3.5 w-3.5" fill="none" stroke="currentColor" stroke-width="3" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" d="M5 13l4 4L19 7"></path>
                  </svg>
                </div>
                <div v-else-if="day === 'F'" class="h-6 w-6 rounded-full bg-[#3B82F6] text-white flex items-center justify-center border-2 border-[#3B82F6]">
                  <svg class="h-3.5 w-3.5" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z"></path>
                  </svg>
                </div>
                <div v-else class="h-6 w-6 rounded-full border border-stroke dark:border-strokedark"></div>
              </div>
            </div>
          </div>
        </div>

        <!-- Current Shift Card -->
        <div class="rounded-2xl bg-primary p-6 text-white shadow-sm pb-10">
          <div class="flex items-center gap-2 text-xs font-bold uppercase tracking-wider text-white/80 mb-6">
            <svg class="h-4 w-4" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z"></path>
            </svg>
            CURRENT SHIFT
          </div>
          <div>
            <h3 class="text-3xl font-bold tracking-tight">9:00 AM - 6:00 PM</h3>
            <p class="mt-2 text-sm text-white/80">Standard Daily Shift • 1h Lunch</p>
          </div>
        </div>
      </div>
    </div>

    <!-- Edit Modal -->
    <div v-if="showEditModal" class="fixed inset-0 z-50 flex items-center justify-center bg-black/60 p-4 backdrop-blur-sm transition-opacity">
      <div class="w-full max-w-2xl overflow-hidden rounded-2xl bg-white shadow-2xl dark:bg-boxdark transform transition-transform" @click.stop>
        <div class="border-b border-stroke bg-gray-50/50 px-6 py-4 dark:border-strokedark dark:bg-meta-4/30">
          <h3 class="text-xl font-bold text-black dark:text-white flex items-center gap-2">
            <svg class="w-5 h-5 text-primary" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path
                stroke-linecap="round"
                stroke-linejoin="round"
                stroke-width="2"
                d="M15.232 5.232l3.536 3.536m-2.036-5.036a2.5 2.5 0 113.536 3.536L6.5 21.036H3v-3.572L16.732 3.732z"></path>
            </svg>
            Editar Perfil
          </h3>
        </div>

        <form @submit.prevent="handleUpdateProfile" class="p-6">
          <div class="grid grid-cols-1 gap-5 md:grid-cols-2">
            <div class="md:col-span-2">
              <label class="mb-2 block text-sm font-semibold text-black dark:text-white">Nombre Completo</label>
              <input
                v-model="editForm.nombre"
                type="text"
                class="w-full rounded-lg border border-stroke bg-transparent py-3 px-4 font-medium text-black outline-none transition focus:border-primary active:border-primary disabled:cursor-default disabled:bg-whiter dark:border-form-strokedark dark:bg-form-input dark:text-white dark:focus:border-primary shadow-sm" />
            </div>
            <div>
              <label class="mb-2 block text-sm font-semibold text-black dark:text-white">Teléfono</label>
              <input
                v-model="editForm.telf"
                type="tel"
                class="w-full rounded-lg border border-stroke bg-transparent py-3 px-4 font-medium text-black outline-none transition focus:border-primary active:border-primary dark:border-form-strokedark dark:bg-form-input dark:text-white dark:focus:border-primary shadow-sm" />
            </div>
            <div>
              <label class="mb-2 block text-sm font-semibold text-black dark:text-white">Email</label>
              <input
                v-model="editForm.email"
                type="email"
                class="w-full rounded-lg border border-stroke bg-transparent py-3 px-4 font-medium text-black outline-none transition focus:border-primary active:border-primary dark:border-form-strokedark dark:bg-form-input dark:text-white dark:focus:border-primary shadow-sm" />
            </div>
            <div class="md:col-span-2">
              <label class="mb-2 block text-sm font-semibold text-black dark:text-white">Dirección</label>
              <input
                v-model="editForm.direccion"
                type="text"
                class="w-full rounded-lg border border-stroke bg-transparent py-3 px-4 font-medium text-black outline-none transition focus:border-primary active:border-primary dark:border-form-strokedark dark:bg-form-input dark:text-white dark:focus:border-primary shadow-sm" />
            </div>
            <div>
              <label class="mb-2 block text-sm font-semibold text-black dark:text-white">RUC</label>
              <input
                v-model="editForm.ruc"
                type="text"
                class="w-full rounded-lg border border-stroke bg-transparent py-3 px-4 font-medium text-black outline-none transition focus:border-primary active:border-primary dark:border-form-strokedark dark:bg-form-input dark:text-white dark:focus:border-primary shadow-sm" />
            </div>
            <div>
              <label class="mb-2 block text-sm font-semibold text-black dark:text-white">Fecha de Nacimiento</label>
              <input
                v-model="editForm.nacimiento"
                type="date"
                required
                class="w-full rounded-lg border border-stroke bg-transparent py-3 px-4 font-medium text-black outline-none transition focus:border-primary active:border-primary dark:border-form-strokedark dark:bg-form-input dark:text-white dark:focus:border-primary shadow-sm" />
            </div>
            <div>
              <label class="mb-2 block text-sm font-semibold text-black dark:text-white">Sexo</label>
              <select
                v-model="editForm.sexo"
                class="w-full rounded-lg border border-stroke bg-transparent py-3 px-4 font-medium text-black outline-none transition focus:border-primary active:border-primary dark:border-form-strokedark dark:bg-form-input dark:text-white dark:focus:border-primary shadow-sm appearance-none">
                <option value="M">Masculino</option>
                <option value="F">Femenino</option>
              </select>
            </div>
          </div>

          <div class="mt-8 flex items-center justify-end gap-3 border-t border-stroke pt-5 dark:border-strokedark">
            <button
              type="button"
              @click="showEditModal = false"
              class="rounded-lg border border-stroke px-6 py-2.5 font-medium text-black transition-colors hover:bg-gray-50 dark:border-strokedark dark:text-white dark:hover:bg-meta-4">
              Cancelar
            </button>
            <button type="submit" class="rounded-lg bg-primary px-6 py-2.5 font-medium text-white shadow-sm transition-colors hover:bg-opacity-90">Guardar Cambios</button>
          </div>
        </form>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
  import { onMounted, ref, reactive, watch } from 'vue'
  import { useRoute } from 'vue-router'
  import { usePersonalStore } from '../../stores/personal'
  import { storeToRefs } from 'pinia'
  import HeaderPerfil from '../../components/perfil/header.vue'
  import Info from '../../components/perfil/info.vue'

  const route = useRoute()
  const personalStore = usePersonalStore()
  const { perfilActual, vinculos, infoBancaria, legajo } = storeToRefs(personalStore)

  const mockDocuments = [
    { type: 'pdf', title: 'Employment Contract', meta: 'PDF • 1.2 MB • Uploaded Jan 12, 2021' },
    { type: 'image', title: 'ID Document (Passport)', meta: 'JPEG • 450 KB • Uploaded Jan 12, 2021' },
    { type: 'data', title: 'Tax Forms (W-4)', meta: 'PDF • 890 KB • Updated Feb 24, 2024' },
  ]

  const showEditModal = ref(false)
  const editForm = reactive({
    dni: '',
    nombre: '',
    telf: '',
    email: '',
    direccion: '',
    ruc: '',
    nacimiento: '',
    sexo: 'M',
  })

  const handleUpdateProfile = async () => {
    await personalStore.actualizarPerfil(editForm)
    showEditModal.value = false
  }

  const cargarDatosBase = async (dni: string) => {
    if (dni) {
      await personalStore.obtenerPerfil(dni)
    }
  }

  onMounted(() => {
    cargarDatosBase(route.params.dni as string)
  })

  watch(
    () => route.params.dni as string,
    (newDni) => {
      cargarDatosBase(newDni)
    }
  )
</script>
