<template>
  <main>
    <div class="p-4 pt-1 mx-auto max-w-(--breakpoint-2xl) md:p-6">
      <div class="mb-6 flex flex-wrap items-center justify-between gap-4">
        <div>
          <h1 class="text-lg font-bold text-gray-900 dark:text-white tracking-tight">Gestión de Usuarios</h1>
          <p class="mt-1 text-sm text-gray-500 dark:text-gray-400">Administra los usuarios del sistema</p>
        </div>
        <button
          @click="abrirCrear"
          class="inline-flex items-center gap-2 rounded-xl bg-primary px-4 py-2 text-sm font-medium text-white hover:bg-primary-container transition-colors shadow-sm">
          <UserPlus class="h-4 w-4" />
          Nuevo Usuario
        </button>
      </div>

      <!-- Error de acceso -->
      <div v-if="errorAcceso" class="flex flex-col items-center justify-center py-20 text-center">
        <ShieldOff class="h-16 w-16 text-red-300 dark:text-red-700 mb-4" />
        <p class="text-lg font-semibold text-gray-600 dark:text-gray-400">Acceso denegado</p>
        <p class="text-sm text-gray-400 dark:text-gray-500 mt-1">Solo los administradores pueden gestionar usuarios.</p>
      </div>

      <!-- Tabla de usuarios -->
      <div v-else class="rounded-2xl border border-gray-100 bg-card dark:border-white/6 dark:bg-white/3 overflow-hidden">
        <div v-if="cargando" class="flex flex-col items-center gap-3 py-16">
          <Loader2 class="h-8 w-8 animate-spin text-primary" />
          <span class="text-sm text-gray-500 dark:text-gray-400">Cargando usuarios…</span>
        </div>

        <table v-else class="w-full">
          <thead>
            <tr class="border-b border-gray-100 dark:border-white/6">
              <th class="px-5 py-3 text-left text-xs font-semibold uppercase tracking-wider text-gray-400">Nombre</th>
              <th class="px-5 py-3 text-left text-xs font-semibold uppercase tracking-wider text-gray-400">Usuario</th>
              <th class="px-5 py-3 text-left text-xs font-semibold uppercase tracking-wider text-gray-400">Nivel</th>
              <th class="px-5 py-3"></th>
            </tr>
          </thead>
          <tbody class="divide-y divide-gray-100 dark:divide-white/5">
            <tr
              v-for="u in usuarios"
              :key="u.id"
              class="hover:bg-primary/5 dark:hover:bg-white/5 transition-colors">
              <td class="px-5 py-3.5">
                <span class="text-sm font-medium text-gray-800 dark:text-white">{{ u.nombre }}</span>
              </td>
              <td class="px-5 py-3.5">
                <span class="text-sm text-gray-500 dark:text-gray-400 font-mono">{{ u.nickname }}</span>
              </td>
              <td class="px-5 py-3.5">
                <span
                  class="inline-flex items-center rounded-full px-2 py-0.5 text-xs font-medium"
                  :class="u.nivel === 1
                    ? 'bg-primary/10 text-primary dark:bg-primary/15 dark:text-brand-300'
                    : 'bg-gray-100 text-gray-600 dark:bg-white/5 dark:text-gray-400'">
                  {{ u.nivel === 1 ? 'Administrador' : 'Usuario' }}
                </span>
              </td>
              <td class="px-5 py-3.5 text-right">
                <div class="flex items-center justify-end gap-1">
                  <button
                    @click="abrirEditar(u)"
                    class="rounded-lg p-1.5 text-gray-400 hover:bg-primary/10 hover:text-primary dark:hover:bg-primary/20 dark:hover:text-brand-300 transition-colors"
                    title="Editar">
                    <Pencil class="h-4 w-4" />
                  </button>
                  <button
                    @click="abrirResetPass(u)"
                    class="rounded-lg p-1.5 text-gray-400 hover:bg-blue-50 hover:text-blue-500 dark:hover:bg-blue-900/20 dark:hover:text-blue-400 transition-colors"
                    title="Restablecer contraseña">
                    <KeyRound class="h-4 w-4" />
                  </button>
                  <button
                    @click="confirmarEliminar(u)"
                    :disabled="u.id === miId"
                    class="rounded-lg p-1.5 text-gray-400 hover:bg-red-50 hover:text-red-500 dark:hover:bg-red-900/20 dark:hover:text-red-400 transition-colors disabled:opacity-30 disabled:cursor-not-allowed"
                    title="Eliminar">
                    <Trash2 class="h-4 w-4" />
                  </button>
                </div>
              </td>
            </tr>
          </tbody>
        </table>
      </div>
    </div>

    <!-- Modal crear / editar -->
    <Modal :isOpen="modalAbierto" :title="editando ? 'Editar Usuario' : 'Nuevo Usuario'" @close="cerrarModal">
      <form @submit.prevent="guardar" class="space-y-4">
        <div>
          <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">Nombre completo</label>
          <input
            v-model="form.nombre"
            type="text"
            class="h-11 w-full rounded-lg border border-gray-100 bg-transparent px-3 py-2.5 text-sm dark:border-white/6 dark:bg-white/5 dark:text-white/90"
            placeholder="Juan Pérez" />
        </div>
        <div>
          <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">Nickname</label>
          <input
            v-model="form.nickname"
            type="text"
            class="h-11 w-full rounded-lg border border-gray-100 bg-transparent px-3 py-2.5 text-sm dark:border-white/6 dark:bg-white/5 dark:text-white/90"
            placeholder="jpérez" />
        </div>
        <div v-if="!editando">
          <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">Contraseña</label>
          <input
            v-model="form.pass"
            type="password"
            class="h-11 w-full rounded-lg border border-gray-100 bg-transparent px-3 py-2.5 text-sm dark:border-white/6 dark:bg-white/5 dark:text-white/90"
            placeholder="••••••••" />
        </div>
        <div>
          <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">Nivel</label>
          <select
            v-model="form.nivel"
            class="h-11 w-full rounded-lg border border-gray-100 bg-transparent px-3 py-2.5 text-sm dark:border-white/6 dark:bg-white/5 dark:text-white/90">
            <option :value="2">Usuario</option>
            <option :value="1">Administrador</option>
          </select>
        </div>
        <p v-if="errorModal" class="text-sm text-red-500">{{ errorModal }}</p>
      </form>
      <template #footer>
        <button
          type="button"
          @click="guardar"
          :disabled="guardando"
          class="inline-flex w-full justify-center items-center gap-2 rounded-lg bg-primary px-4 py-2 text-sm font-medium text-white hover:bg-primary-container transition sm:ml-3 sm:w-auto disabled:opacity-60">
          <Loader2 v-if="guardando" class="h-4 w-4 animate-spin" />
          {{ editando ? 'Guardar cambios' : 'Crear usuario' }}
        </button>
        <button
          type="button"
          @click="cerrarModal"
          class="mt-3 inline-flex w-full justify-center rounded-lg bg-card px-4 py-2 text-sm font-medium text-gray-700 shadow-sm ring-1 ring-inset ring-gray-200 hover:bg-primary/5 transition dark:bg-white/5 dark:text-gray-300 dark:ring-white/10 dark:hover:bg-white/10 sm:mt-0 sm:w-auto">
          Cancelar
        </button>
      </template>
    </Modal>

    <!-- Modal reset contraseña -->
    <Modal :isOpen="modalResetAbierto" title="Restablecer Contraseña" @close="cerrarReset">
      <div class="space-y-4">
        <p class="text-sm text-gray-600 dark:text-gray-400">
          Ingresa la nueva contraseña para <strong>{{ usuarioSeleccionado?.nombre }}</strong>.
        </p>
        <div>
          <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">Nueva contraseña</label>
          <input
            v-model="nuevaPass"
            type="password"
            class="h-11 w-full rounded-lg border border-gray-100 bg-transparent px-3 py-2.5 text-sm dark:border-white/6 dark:bg-white/5 dark:text-white/90"
            placeholder="••••••••" />
        </div>
        <p v-if="errorReset" class="text-sm text-red-500">{{ errorReset }}</p>
      </div>
      <template #footer>
        <button
          type="button"
          @click="resetPass"
          :disabled="guardando"
          class="inline-flex w-full justify-center items-center gap-2 rounded-lg bg-blue-600 px-4 py-2 text-sm font-medium text-white hover:bg-blue-700 transition sm:ml-3 sm:w-auto disabled:opacity-60">
          <Loader2 v-if="guardando" class="h-4 w-4 animate-spin" />
          Restablecer
        </button>
        <button
          type="button"
          @click="cerrarReset"
          class="mt-3 inline-flex w-full justify-center rounded-lg bg-card px-4 py-2 text-sm font-medium text-gray-700 shadow-sm ring-1 ring-inset ring-gray-200 hover:bg-primary/5 transition dark:bg-white/5 dark:text-gray-300 dark:ring-white/10 dark:hover:bg-white/10 sm:mt-0 sm:w-auto">
          Cancelar
        </button>
      </template>
    </Modal>
  </main>
</template>

<script setup lang="ts">
  import { ref, onMounted, computed } from 'vue'
  import { UserPlus, Pencil, Trash2, KeyRound, Loader2, ShieldOff } from 'lucide-vue-next'
  import Modal from '../components/ui/Modal.vue'
  import api from '../services/api'
  import { useAutenticacionStore } from '../stores/auth'
  import { storeToRefs } from 'pinia'

  interface Usuario {
    id: number
    nombre: string
    nickname: string
    nivel: number
  }

  const authStore = useAutenticacionStore()
  const { esAdmin } = storeToRefs(authStore)
  const miId = computed(() => authStore.usuario?.id ?? 0)

  const usuarios = ref<Usuario[]>([])
  const cargando = ref(false)
  const errorAcceso = ref(false)

  const modalAbierto = ref(false)
  const editando = ref(false)
  const guardando = ref(false)
  const errorModal = ref('')
  const usuarioSeleccionado = ref<Usuario | null>(null)

  const form = ref({ nombre: '', nickname: '', pass: '', nivel: 2 })

  const modalResetAbierto = ref(false)
  const nuevaPass = ref('')
  const errorReset = ref('')

  async function cargarUsuarios() {
    cargando.value = true
    errorAcceso.value = false
    try {
      const res = await api.post('/usuarios/listar')
      usuarios.value = res.data
    } catch (e: any) {
      if (e?.response?.status === 401) errorAcceso.value = true
    } finally {
      cargando.value = false
    }
  }

  function abrirCrear() {
    editando.value = false
    form.value = { nombre: '', nickname: '', pass: '', nivel: 2 }
    errorModal.value = ''
    modalAbierto.value = true
  }

  function abrirEditar(u: Usuario) {
    editando.value = true
    usuarioSeleccionado.value = u
    form.value = { nombre: u.nombre, nickname: u.nickname, pass: '', nivel: u.nivel }
    errorModal.value = ''
    modalAbierto.value = true
  }

  function cerrarModal() {
    if (guardando.value) return
    modalAbierto.value = false
  }

  async function guardar() {
    errorModal.value = ''
    if (!form.value.nombre.trim() || !form.value.nickname.trim()) {
      errorModal.value = 'Nombre y nickname son requeridos.'
      return
    }
    if (!editando.value && form.value.pass.length < 4) {
      errorModal.value = 'La contraseña debe tener al menos 4 caracteres.'
      return
    }
    guardando.value = true
    try {
      if (editando.value) {
        await api.post('/usuarios/editar', {
          id: usuarioSeleccionado.value!.id,
          nombre: form.value.nombre,
          nickname: form.value.nickname,
          nivel: form.value.nivel,
        })
      } else {
        await api.post('/usuarios/crear', form.value)
      }
      await cargarUsuarios()
      cerrarModal()
    } catch (e: any) {
      errorModal.value = e?.response?.data?.error ?? 'Error al guardar.'
    } finally {
      guardando.value = false
    }
  }

  async function confirmarEliminar(u: Usuario) {
    if (!confirm(`¿Eliminar al usuario "${u.nombre}"?`)) return
    try {
      await api.post('/usuarios/eliminar', { id: u.id })
      await cargarUsuarios()
    } catch (e: any) {
      alert(e?.response?.data?.error ?? 'Error al eliminar.')
    }
  }

  function abrirResetPass(u: Usuario) {
    usuarioSeleccionado.value = u
    nuevaPass.value = ''
    errorReset.value = ''
    modalResetAbierto.value = true
  }

  function cerrarReset() {
    if (guardando.value) return
    modalResetAbierto.value = false
  }

  async function resetPass() {
    errorReset.value = ''
    if (nuevaPass.value.length < 4) {
      errorReset.value = 'La contraseña debe tener al menos 4 caracteres.'
      return
    }
    guardando.value = true
    try {
      await api.post('/usuarios/reset_pass', {
        id: usuarioSeleccionado.value!.id,
        nueva_pass: nuevaPass.value,
      })
      cerrarReset()
    } catch (e: any) {
      errorReset.value = e?.response?.data?.error ?? 'Error al restablecer.'
    } finally {
      guardando.value = false
    }
  }

  onMounted(() => {
    if (esAdmin.value) cargarUsuarios()
    else errorAcceso.value = true
  })
</script>
