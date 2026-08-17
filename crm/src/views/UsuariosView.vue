<script setup lang="ts">
import { ref, onMounted, computed } from 'vue'
import { storeToRefs } from 'pinia'
import { useUsuariosStore } from '@/stores/usuarios'
import { useAuthStore } from '@/stores/auth'
import type { User } from '@/api/types'
import { formatearFecha } from '@/utils/fechas'
import TarjetaResumen from '@/components/comun/TarjetaResumen.vue'
import {
  IconUsers,
  IconUserCheck,
  IconClock,
  IconUserOff,
  IconShield,
  IconSearch,
  IconX,
  IconRefresh,
  IconCheck,
  IconTrash,
  IconAlertTriangle,
  IconLoader2,
  IconChevronDown,
} from '@tabler/icons-vue'

const usuariosStore = useUsuariosStore()
const authStore = useAuthStore()

const {
  cargando,
  procesandoId,
  error,
  terminoBusqueda,
  filtroEstado,
  filtroRol,
  totalUsuarios,
  totalAprobados,
  totalPendientes,
  totalAdministradores,
  usuariosFiltrados,
} = storeToRefs(usuariosStore)

const { cargarUsuarios, cambiarRol, cambiarEstado, removerUsuario } = usuariosStore

const idUsuarioActual = computed(() => authStore.currentUser?.id ?? 0)

const modalEliminarAbierto = ref(false)
const usuarioAEliminar = ref<User | null>(null)
const eliminando = ref(false)
const errorEliminar = ref<string | null>(null)

onMounted(async () => {
  await cargarUsuarios()
})

function abrirModalEliminar(usuario: User) {
  usuarioAEliminar.value = usuario
  errorEliminar.value = null
  modalEliminarAbierto.value = true
}

function cerrarModalEliminar() {
  if (eliminando.value) return
  modalEliminarAbierto.value = false
  usuarioAEliminar.value = null
  errorEliminar.value = null
}

async function confirmarEliminarUsuario() {
  if (!usuarioAEliminar.value) return
  eliminando.value = true
  errorEliminar.value = null
  try {
    await removerUsuario(usuarioAEliminar.value.id)
    cerrarModalEliminar()
  } catch (e: any) {
    errorEliminar.value = e?.error || 'No se pudo eliminar el usuario'
  } finally {
    eliminando.value = false
  }
}

function obtenerIniciales(nombre: string): string {
  if (!nombre) return 'U'
  const partes = nombre.trim().split(/\s+/)
  if (partes.length === 1) return partes[0].charAt(0).toUpperCase()
  return (partes[0].charAt(0) + partes[1].charAt(0)).toUpperCase()
}
</script>

<template>
  <div class="px-4 py-5 md:px-6 md:py-6 space-y-5 max-w-[1600px] mx-auto">
    <!-- Header de la Vista -->
    <div class="flex flex-wrap items-center justify-between gap-4">
      <div>
        <div class="flex items-center gap-2.5">
          <div
            class="flex h-8 w-8 items-center justify-center rounded-xl bg-blue-50 text-blue-600 dark:bg-blue-900/20 dark:text-blue-400">
            <IconUsers class="h-4.5 w-4.5" />
          </div>
          <div>
            <h1 class="text-sm sm:text-base font-bold uppercase tracking-wider text-gray-900 dark:text-white">
              Gestión de Usuarios
            </h1>
            <p class="text-2xs font-medium text-gray-400">
              Administra roles, estados de aprobación y accesos de usuarios al CRM
            </p>
          </div>
        </div>
      </div>
    </div>

    <!-- Banner de Alerta de Error -->
    <div
      v-if="error"
      class="flex items-center justify-between rounded-xl border border-red-200 bg-red-50 px-4 py-3 dark:border-red-900/50 dark:bg-red-950/20">
      <div class="flex items-center gap-2.5">
        <IconAlertTriangle class="h-4 w-4 shrink-0 text-red-500" />
        <span class="text-xs text-red-700 dark:text-red-400">{{ error }}</span>
      </div>
      <button
        type="button"
        @click="cargarUsuarios()"
        class="flex items-center gap-1 rounded-lg px-2.5 py-1 text-2xs font-medium text-red-600 hover:bg-red-100 dark:text-red-400 dark:hover:bg-red-900/30 transition-colors cursor-pointer">
        <IconRefresh class="h-3.5 w-3.5" />
        Reintentar
      </button>
    </div>

    <!-- Fila de Tarjetas de Métricas -->
    <div class="grid grid-cols-2 gap-3 sm:grid-cols-2 lg:grid-cols-4">
      <TarjetaResumen
        titulo="Total Usuarios"
        :valor="totalUsuarios"
        subtitulo="Registrados en sistema"
        color="blue"
        :icono="IconUsers"
        interactivo
        :activo="filtroEstado === 'TODOS'"
        @click="filtroEstado = 'TODOS'" />

      <TarjetaResumen
        titulo="Por Aprobar"
        :valor="totalPendientes"
        :subtitulo="totalPendientes > 0 ? 'Acción requerida' : 'Al día'"
        color="amber"
        :icono="IconClock"
        claseValor="text-amber-700 dark:text-amber-400"
        :claseSubtitulo="totalPendientes > 0 ? 'font-semibold text-amber-600 dark:text-amber-400' : 'text-gray-400 dark:text-gray-500'"
        interactivo
        :activo="filtroEstado === 'PENDING'"
        @click="filtroEstado = 'PENDING'" />

      <TarjetaResumen
        titulo="Aprobados"
        :valor="totalAprobados"
        subtitulo="Con acceso activo"
        color="emerald"
        :icono="IconUserCheck"
        claseValor="text-emerald-700 dark:text-emerald-400"
        interactivo
        :activo="filtroEstado === 'APPROVED'"
        @click="filtroEstado = 'APPROVED'" />

      <TarjetaResumen
        titulo="Administradores"
        :valor="totalAdministradores"
        subtitulo="Privilegios totales"
        color="purple"
        :icono="IconShield"
        claseValor="text-purple-700 dark:text-purple-400"
        interactivo
        :activo="filtroRol === 'ADMIN'"
        @click="filtroRol = 'ADMIN'" />
    </div>

    <!-- Contenedor Principal: Filtros y Tabla -->
    <div
      class="rounded-xl border border-gray-200 bg-white shadow-2xs dark:border-gray-700 dark:bg-gray-800 overflow-hidden">
      <!-- Barra de Filtros y Búsqueda -->
      <div
        class="p-3.5 sm:p-4 border-b border-gray-100 dark:border-gray-700/80 flex flex-col md:flex-row items-stretch md:items-center justify-between gap-3">
        <!-- Buscador -->
        <div class="relative flex-1 max-w-md">
          <div class="pointer-events-none absolute inset-y-0 left-0 flex items-center pl-3">
            <IconSearch class="h-4 w-4 text-gray-400" />
          </div>
          <input
            v-model="terminoBusqueda"
            type="text"
            placeholder="Buscar usuario por nombre o correo..."
            class="block w-full rounded-xl border border-gray-200 bg-gray-50/80 py-2 pl-9 pr-8 text-xs text-gray-900 placeholder:text-gray-400 focus:border-blue-500 focus:bg-white focus:outline-none focus:ring-2 focus:ring-blue-500/20 dark:border-gray-700 dark:bg-gray-900/70 dark:text-gray-100 dark:placeholder:text-gray-500 dark:focus:border-blue-400 transition-all" />
          <button
            v-if="terminoBusqueda"
            type="button"
            @click="terminoBusqueda = ''"
            class="absolute inset-y-0 right-0 flex items-center pr-2.5 text-gray-400 hover:text-gray-600 dark:hover:text-gray-200 cursor-pointer">
            <IconX class="h-3.5 w-3.5" />
          </button>
        </div>

        <!-- Filtros Rápidos -->
        <div class="flex flex-wrap items-center gap-2">
          <!-- Filtro Estado -->
          <div
            class="flex items-center rounded-xl bg-gray-100 p-0.5 dark:bg-gray-900/80 border border-gray-200/60 dark:border-gray-700/60 text-2xs font-semibold">
            <button
              type="button"
              @click="filtroEstado = 'TODOS'"
              :class="[
                'rounded-lg px-2.5 py-1 transition-colors cursor-pointer',
                filtroEstado === 'TODOS'
                  ? 'bg-white text-gray-900 shadow-2xs dark:bg-gray-800 dark:text-white'
                  : 'text-gray-500 hover:text-gray-800 dark:text-gray-400 dark:hover:text-gray-200',
              ]">
              Todos
            </button>
            <button
              type="button"
              @click="filtroEstado = 'APPROVED'"
              :class="[
                'rounded-lg px-2.5 py-1 transition-colors cursor-pointer',
                filtroEstado === 'APPROVED'
                  ? 'bg-white text-emerald-600 shadow-2xs dark:bg-gray-800 dark:text-emerald-400'
                  : 'text-gray-500 hover:text-gray-800 dark:text-gray-400 dark:hover:text-gray-200',
              ]">
              Aprobados
            </button>
            <button
              type="button"
              @click="filtroEstado = 'PENDING'"
              :class="[
                'rounded-lg px-2.5 py-1 transition-colors cursor-pointer',
                filtroEstado === 'PENDING'
                  ? 'bg-white text-amber-600 shadow-2xs dark:bg-gray-800 dark:text-amber-400'
                  : 'text-gray-500 hover:text-gray-800 dark:text-gray-400 dark:hover:text-gray-200',
              ]">
              Pendientes
            </button>
            <button
              type="button"
              @click="filtroEstado = 'REJECTED'"
              :class="[
                'rounded-lg px-2.5 py-1 transition-colors cursor-pointer',
                filtroEstado === 'REJECTED'
                  ? 'bg-white text-rose-600 shadow-2xs dark:bg-gray-800 dark:text-rose-400'
                  : 'text-gray-500 hover:text-gray-800 dark:text-gray-400 dark:hover:text-gray-200',
              ]">
              Rechazados
            </button>
          </div>

          <!-- Filtro Rol -->
          <div
            class="flex items-center rounded-xl bg-gray-100 p-0.5 dark:bg-gray-900/80 border border-gray-200/60 dark:border-gray-700/60 text-2xs font-semibold">
            <button
              type="button"
              @click="filtroRol = 'TODOS'"
              :class="[
                'rounded-lg px-2.5 py-1 transition-colors cursor-pointer',
                filtroRol === 'TODOS'
                  ? 'bg-white text-gray-900 shadow-2xs dark:bg-gray-800 dark:text-white'
                  : 'text-gray-500 hover:text-gray-800 dark:text-gray-400 dark:hover:text-gray-200',
              ]">
              Roles: Todos
            </button>
            <button
              type="button"
              @click="filtroRol = 'ADMIN'"
              :class="[
                'rounded-lg px-2.5 py-1 transition-colors cursor-pointer',
                filtroRol === 'ADMIN'
                  ? 'bg-white text-purple-600 shadow-2xs dark:bg-gray-800 dark:text-purple-400'
                  : 'text-gray-500 hover:text-gray-800 dark:text-gray-400 dark:hover:text-gray-200',
              ]">
              Admin
            </button>
            <button
              type="button"
              @click="filtroRol = 'USER'"
              :class="[
                'rounded-lg px-2.5 py-1 transition-colors cursor-pointer',
                filtroRol === 'USER'
                  ? 'bg-white text-blue-600 shadow-2xs dark:bg-gray-800 dark:text-blue-400'
                  : 'text-gray-500 hover:text-gray-800 dark:text-gray-400 dark:hover:text-gray-200',
              ]">
              Usuario
            </button>
          </div>
        </div>
      </div>

      <!-- Estado de Carga (Skeleton) -->
      <div v-if="cargando && usuariosFiltrados.length === 0" class="p-6 space-y-4">
        <div v-for="i in 5" :key="i" class="flex items-center justify-between gap-4 animate-pulse">
          <div class="flex items-center gap-3">
            <div class="h-9 w-9 rounded-full bg-gray-200 dark:bg-gray-700" />
            <div class="space-y-1.5">
              <div class="h-3.5 w-36 rounded bg-gray-200 dark:bg-gray-700" />
              <div class="h-2.5 w-48 rounded bg-gray-200 dark:bg-gray-700" />
            </div>
          </div>
          <div class="h-6 w-20 rounded bg-gray-200 dark:bg-gray-700" />
          <div class="h-6 w-24 rounded bg-gray-200 dark:bg-gray-700" />
          <div class="h-7 w-20 rounded bg-gray-200 dark:bg-gray-700" />
        </div>
      </div>

      <!-- Estado Vacío -->
      <div v-else-if="usuariosFiltrados.length === 0" class="py-14 text-center px-4">
        <div
          class="mx-auto flex h-12 w-12 items-center justify-center rounded-full bg-gray-100 text-gray-400 dark:bg-gray-750 dark:text-gray-500 mb-3">
          <IconUserOff class="h-6 w-6" />
        </div>
        <h3 class="text-xs font-bold uppercase tracking-wider text-gray-800 dark:text-gray-200">
          No se encontraron usuarios
        </h3>
        <p class="text-2xs text-gray-400 mt-1 max-w-sm mx-auto">
          Intenta cambiar los filtros o el término de búsqueda para ver más resultados.
        </p>
      </div>

      <!-- Tabla de Usuarios -->
      <div v-else class="overflow-x-auto">
        <table class="w-full text-left border-collapse">
          <thead>
            <tr
              class="border-b border-gray-100 bg-gray-50/70 text-2xs font-bold uppercase tracking-wider text-gray-400 dark:border-gray-700/80 dark:bg-gray-900/40 dark:text-gray-500">
              <th class="py-3 pl-4 pr-3 sm:pl-6">Usuario</th>
              <th class="py-3 px-3">Rol</th>
              <th class="py-3 px-3">Estado de Acceso</th>
              <th class="py-3 px-3 hidden lg:table-cell">Fecha Registro</th>
              <th class="py-3 pl-3 pr-4 sm:pr-6 text-right">Acciones</th>
            </tr>
          </thead>
          <tbody class="divide-y divide-gray-100 dark:divide-gray-700/60 text-xs">
            <tr
              v-for="usuario in usuariosFiltrados"
              :key="usuario.id"
              :class="[
                'transition-colors hover:bg-gray-50/80 dark:hover:bg-gray-750/50',
                procesandoId === usuario.id ? 'opacity-60 pointer-events-none' : '',
              ]">
              <!-- Columna Usuario -->
              <td class="py-3.5 pl-4 pr-3 sm:pl-6">
                <div class="flex items-center gap-3">
                  <div
                    class="relative h-9 w-9 shrink-0 overflow-hidden rounded-full border border-gray-200 bg-gray-100 dark:border-gray-700 dark:bg-gray-700">
                    <img
                      v-if="usuario.picture_url"
                      :src="usuario.picture_url"
                      :alt="usuario.full_name"
                      class="h-full w-full object-cover" />
                    <div
                      v-else
                      class="flex h-full w-full items-center justify-center font-bold text-2xs text-gray-600 dark:text-gray-300">
                      {{ obtenerIniciales(usuario.full_name) }}
                    </div>
                  </div>

                  <div class="min-w-0">
                    <div class="flex items-center gap-1.5">
                      <p class="font-bold text-gray-900 dark:text-white truncate">
                        {{ usuario.full_name }}
                      </p>
                      <span
                        v-if="usuario.id === idUsuarioActual"
                        class="rounded-md bg-blue-50 px-1.5 py-0.2 text-[9px] font-bold text-blue-600 dark:bg-blue-900/40 dark:text-blue-300">
                        Tú
                      </span>
                    </div>
                    <p class="text-2xs text-gray-400 dark:text-gray-400 truncate">
                      {{ usuario.email }}
                    </p>
                  </div>
                </div>
              </td>

              <!-- Columna Rol -->
              <td class="py-3.5 px-3">
                <div class="inline-flex items-center">
                  <div class="relative inline-block">
                    <select
                      :value="usuario.role"
                      @change="(e) => cambiarRol(usuario.id, (e.target as HTMLSelectElement).value as 'ADMIN' | 'USER')"
                      :disabled="usuario.id === idUsuarioActual || procesandoId === usuario.id"
                      :class="[
                        'appearance-none rounded-lg border py-1 pl-2.5 pr-7 text-2xs font-bold uppercase tracking-wider transition-colors cursor-pointer focus:outline-none focus:ring-1',
                        usuario.role === 'ADMIN'
                          ? 'border-purple-200 bg-purple-50 text-purple-700 focus:ring-purple-400 dark:border-purple-900/60 dark:bg-purple-950/30 dark:text-purple-300'
                          : 'border-slate-200 bg-slate-50 text-slate-700 focus:ring-slate-400 dark:border-slate-700 dark:bg-slate-900/50 dark:text-slate-300',
                        usuario.id === idUsuarioActual ? 'cursor-not-allowed opacity-80' : '',
                      ]">
                      <option value="USER">USUARIO</option>
                      <option value="ADMIN">ADMIN</option>
                    </select>
                    <IconChevronDown
                      class="pointer-events-none absolute right-2 top-1/2 h-3 w-3 -translate-y-1/2 text-gray-400" />
                  </div>
                </div>
              </td>

              <!-- Columna Estado de Acceso -->
              <td class="py-3.5 px-3">
                <div class="flex items-center gap-2">
                  <span
                    :class="[
                      'inline-flex items-center gap-1.5 rounded-lg px-2.5 py-1 text-2xs font-bold uppercase tracking-wider border',
                      usuario.status === 'APPROVED'
                        ? 'border-emerald-200 bg-emerald-50 text-emerald-700 dark:border-emerald-900/60 dark:bg-emerald-950/30 dark:text-emerald-300'
                        : usuario.status === 'PENDING'
                          ? 'border-amber-200 bg-amber-50 text-amber-700 dark:border-amber-900/60 dark:bg-amber-950/30 dark:text-amber-300'
                          : 'border-rose-200 bg-rose-50 text-rose-700 dark:border-rose-900/60 dark:bg-rose-950/30 dark:text-rose-300',
                    ]">
                    <span
                      :class="[
                        'h-1.5 w-1.5 rounded-full',
                        usuario.status === 'APPROVED'
                          ? 'bg-emerald-500'
                          : usuario.status === 'PENDING'
                            ? 'bg-amber-500'
                            : 'bg-rose-500',
                      ]" />
                    {{
                      usuario.status === 'APPROVED'
                        ? 'Aprobado'
                        : usuario.status === 'PENDING'
                          ? 'Pendiente'
                          : 'Rechazado'
                    }}
                  </span>

                  <!-- Botones de Acción Rápida para el Estado -->
                  <div class="flex items-center gap-1">
                    <button
                      v-if="usuario.status !== 'APPROVED'"
                      type="button"
                      @click="cambiarEstado(usuario.id, 'APPROVED')"
                      :disabled="procesandoId === usuario.id"
                      title="Aceptar y conceder acceso"
                      class="inline-flex h-6.5 w-6.5 items-center justify-center rounded-md bg-emerald-50 text-emerald-600 hover:bg-emerald-100 dark:bg-emerald-950/40 dark:text-emerald-400 dark:hover:bg-emerald-900/60 transition-colors cursor-pointer">
                      <IconCheck class="h-3.5 w-3.5 stroke-[2.5]" />
                    </button>

                    <button
                      v-if="usuario.status !== 'REJECTED' && usuario.id !== idUsuarioActual"
                      type="button"
                      @click="cambiarEstado(usuario.id, 'REJECTED')"
                      :disabled="procesandoId === usuario.id"
                      title="Denegar acceso al sistema"
                      class="inline-flex h-6.5 w-6.5 items-center justify-center rounded-md bg-rose-50 text-rose-600 hover:bg-rose-100 dark:bg-rose-950/40 dark:text-rose-400 dark:hover:bg-rose-900/60 transition-colors cursor-pointer">
                      <IconX class="h-3.5 w-3.5 stroke-[2.5]" />
                    </button>

                    <button
                      v-if="usuario.status !== 'PENDING' && usuario.id !== idUsuarioActual"
                      type="button"
                      @click="cambiarEstado(usuario.id, 'PENDING')"
                      :disabled="procesandoId === usuario.id"
                      title="Poner en estado pendiente"
                      class="inline-flex h-6.5 w-6.5 items-center justify-center rounded-md bg-amber-50 text-amber-600 hover:bg-amber-100 dark:bg-amber-950/40 dark:text-amber-400 dark:hover:bg-amber-900/60 transition-colors cursor-pointer">
                      <IconClock class="h-3.5 w-3.5" />
                    </button>
                  </div>
                </div>
              </td>

              <!-- Columna Fecha de Registro -->
              <td class="py-3.5 px-3 hidden lg:table-cell text-2xs text-gray-400 font-mono">
                {{ usuario.created_at ? formatearFecha(usuario.created_at) : 'N/D' }}
              </td>

              <!-- Columna Acciones -->
              <td class="py-3.5 pl-3 pr-4 sm:pr-6 text-right">
                <div class="flex items-center justify-end gap-1.5">
                  <button
                    type="button"
                    @click="abrirModalEliminar(usuario)"
                    :disabled="usuario.id === idUsuarioActual || procesandoId === usuario.id"
                    :class="[
                      'inline-flex h-7.5 w-7.5 items-center justify-center rounded-lg text-gray-400 transition-colors',
                      usuario.id === idUsuarioActual
                        ? 'opacity-30 cursor-not-allowed'
                        : 'hover:bg-red-50 hover:text-red-600 dark:hover:bg-red-950/30 dark:hover:text-red-400 cursor-pointer',
                    ]"
                    :title="
                      usuario.id === idUsuarioActual ? 'No puedes eliminar tu propia cuenta' : 'Eliminar usuario'
                    ">
                    <IconTrash class="h-4 w-4" />
                  </button>
                </div>
              </td>
            </tr>
          </tbody>
        </table>
      </div>
    </div>

    <!-- Modal de Confirmación para Eliminar Usuario -->
    <div
      v-if="modalEliminarAbierto"
      class="fixed inset-0 z-50 flex items-center justify-center p-4 bg-gray-900/60 backdrop-blur-xs animate-in fade-in duration-150">
      <div
        class="w-full max-w-md rounded-2xl border border-gray-200 bg-white p-5 shadow-xl dark:border-gray-700 dark:bg-gray-800 space-y-4">
        <div class="flex items-center gap-3">
          <div
            class="flex h-10 w-10 shrink-0 items-center justify-center rounded-xl bg-red-50 text-red-600 dark:bg-red-900/30 dark:text-red-400">
            <IconTrash class="h-5 w-5" />
          </div>
          <div>
            <h3 class="text-sm font-bold text-gray-900 dark:text-white">¿Eliminar usuario?</h3>
            <p class="text-2xs text-gray-400">Esta acción no se puede deshacer</p>
          </div>
        </div>

        <p class="text-xs text-gray-600 dark:text-gray-300 leading-relaxed">
          Estás a punto de eliminar la cuenta de
          <strong class="text-gray-900 dark:text-white">{{ usuarioAEliminar?.full_name }}</strong>
          (<span class="font-mono">{{ usuarioAEliminar?.email }}</span
          >). El usuario perderá el acceso al CRM inmediatamente.
        </p>

        <div
          v-if="errorEliminar"
          class="rounded-lg bg-red-50 p-2.5 text-2xs text-red-600 dark:bg-red-950/30 dark:text-red-400">
          {{ errorEliminar }}
        </div>

        <div class="flex items-center justify-end gap-2 pt-2 border-t border-gray-100 dark:border-gray-700">
          <button
            type="button"
            @click="cerrarModalEliminar()"
            :disabled="eliminando"
            class="rounded-xl border border-gray-200 px-3.5 py-2 text-2xs font-semibold text-gray-700 hover:bg-gray-50 dark:border-gray-700 dark:text-gray-300 dark:hover:bg-gray-750 transition-colors cursor-pointer">
            Cancelar
          </button>

          <button
            type="button"
            @click="confirmarEliminarUsuario()"
            :disabled="eliminando"
            class="inline-flex items-center gap-1.5 rounded-xl bg-red-600 px-4 py-2 text-2xs font-semibold text-white shadow-xs hover:bg-red-700 dark:bg-red-500 dark:hover:bg-red-600 transition-colors cursor-pointer disabled:opacity-50">
            <IconLoader2 v-if="eliminando" class="h-3.5 w-3.5 animate-spin" />
            {{ eliminando ? 'Eliminando...' : 'Sí, eliminar usuario' }}
          </button>
        </div>
      </div>
    </div>
  </div>
</template>
