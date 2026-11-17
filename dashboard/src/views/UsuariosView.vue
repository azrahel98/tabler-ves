<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import Button from '@/components/ui/button/Button.vue'
import Badge from '@/components/ui/badge/Badge.vue'
import Modal from '@/components/ui/modal/Modal.vue'
import PageHeader from '@/components/ui/page-header/PageHeader.vue'
import FilterDropdown from '@/components/ui/form/FilterDropdown.vue'
import { useAuthStore } from '@/stores/auth'
import { useToastStore } from '@/stores/toast'
import {
  listarUsuarios,
  editarUsuario,
  eliminarUsuario,
  type Usuario,
  type RolUsuario,
  type EstadoUsuario,
} from '@/services/usuarios'
import { SUPER_ADMIN_EMAIL, esSuperAdmin } from '@/constants/superadmin'
import { formatDate } from '@/utils/date'
import {
  IconRefresh,
  IconSearch,
  IconAlertTriangle,
  IconUsersGroup,
  IconUserCheck,
  IconUserExclamation,
  IconUserX,
  IconCheck,
  IconBan,
  IconTrash,
  IconShieldLock,
  IconClockHour4,
} from '@tabler/icons-vue'

const authStore = useAuthStore()
const toast = useToastStore()

const usuarios = ref<Usuario[]>([])
const isLoading = ref(true)
const errorMessage = ref<string | null>(null)
const guardandoId = ref<number | null>(null)

const busqueda = ref('')
const filtroEstado = ref<'todos' | EstadoUsuario>('todos')
const filtroRol = ref<'todos' | RolUsuario>('todos')

const ESTADOS: { id: EstadoUsuario; label: string; variant: 'warning' | 'success' | 'danger'; icon: typeof IconClockHour4; tint: string }[] = [
  { id: 'PENDING', label: 'Pendientes', variant: 'warning', icon: IconUserExclamation, tint: 'bg-amber-500/10 text-amber-600 dark:text-amber-400' },
  { id: 'APPROVED', label: 'Aprobados', variant: 'success', icon: IconUserCheck, tint: 'bg-emerald-500/10 text-emerald-600 dark:text-emerald-400' },
  { id: 'REJECTED', label: 'Rechazados', variant: 'danger', icon: IconUserX, tint: 'bg-rose-500/10 text-rose-600 dark:text-rose-400' },
]

const ROLES: { id: RolUsuario; label: string; hint: string }[] = [
  { id: 'ADMIN', label: 'Administrador', hint: 'Lectura y escritura total' },
  { id: 'EDITOR', label: 'Editor', hint: 'Puede registrar y modificar' },
  { id: 'USER', label: 'Usuario', hint: 'Solo lectura' },
]

const estadoOptions = [
  { value: 'todos', label: 'Todos los estados' },
  ...ESTADOS.map((e) => ({ value: e.id, label: e.label })),
]
const rolOptions = [
  { value: 'todos', label: 'Todos los roles' },
  ...ROLES.map((r) => ({ value: r.id, label: r.label })),
]

const normalizar = (t: string | null | undefined) =>
  (t || '').normalize('NFD').replace(/[̀-ͯ]/g, '').toLowerCase()

const estadoDe = (u: Usuario) => (u.status || '').toUpperCase() as EstadoUsuario
const rolDe = (u: Usuario) => (u.role || '').toUpperCase() as RolUsuario
const configEstado = (u: Usuario) =>
  ESTADOS.find((e) => e.id === estadoDe(u)) ?? { id: 'PENDING', label: u.status || 'Sin estado', variant: 'warning' as const, icon: IconClockHour4, tint: '' }
const labelRol = (u: Usuario) => ROLES.find((r) => r.id === rolDe(u))?.label ?? (u.role || 'Sin rol')

const esYo = (u: Usuario) => u.id === authStore.user?.id
const bloqueado = (u: Usuario) => esYo(u) || esSuperAdmin(u.email)

const cargarDatos = async () => {
  isLoading.value = true
  errorMessage.value = null
  try {
    usuarios.value = await listarUsuarios()
  } catch (err: unknown) {
    errorMessage.value = err instanceof Error ? err.message : 'Error al obtener la lista de usuarios.'
  } finally {
    isLoading.value = false
  }
}

onMounted(cargarDatos)

const conteos = computed(() => {
  const c: Record<string, number> = { PENDING: 0, APPROVED: 0, REJECTED: 0 }
  for (const u of usuarios.value) c[estadoDe(u)] = (c[estadoDe(u)] || 0) + 1
  return c
})

const hayFiltros = computed(
  () => busqueda.value !== '' || filtroEstado.value !== 'todos' || filtroRol.value !== 'todos',
)
const limpiarFiltros = () => {
  busqueda.value = ''
  filtroEstado.value = 'todos'
  filtroRol.value = 'todos'
}

const ORDEN_ESTADO: Record<string, number> = { PENDING: 0, APPROVED: 1, REJECTED: 2 }

const filtrados = computed(() => {
  const q = normalizar(busqueda.value.trim())
  return usuarios.value
    .filter((u) => {
      if (filtroEstado.value !== 'todos' && estadoDe(u) !== filtroEstado.value) return false
      if (filtroRol.value !== 'todos' && rolDe(u) !== filtroRol.value) return false
      if (q && !normalizar(`${u.full_name} ${u.email}`).includes(q)) return false
      return true
    })
    .sort((a, b) => {
      const d = (ORDEN_ESTADO[estadoDe(a)] ?? 9) - (ORDEN_ESTADO[estadoDe(b)] ?? 9)
      return d !== 0 ? d : a.full_name.localeCompare(b.full_name, 'es')
    })
})

const obtenerIniciales = (nombre: string): string => {
  if (!nombre) return 'US'
  const p = nombre.trim().split(/\s+/)
  return p.length >= 2 ? (p[0][0] + p[1][0]).toUpperCase() : nombre.slice(0, 2).toUpperCase()
}

const ocultarAvatarErroneo = (event: Event) => {
  const t = event.target as HTMLElement | null
  if (t) t.style.display = 'none'
}

const actualizar = async (u: Usuario, cambios: { role?: string; status?: string }) => {
  if (bloqueado(u) || guardandoId.value) return
  const role = (cambios.role ?? u.role).toUpperCase()
  const status = (cambios.status ?? u.status).toUpperCase()
  if (role === rolDe(u) && status === estadoDe(u)) return

  const previo = { role: u.role, status: u.status }
  guardandoId.value = u.id
  u.role = role
  u.status = status
  try {
    await editarUsuario({ id: u.id, role, status })
    toast.success(
      cambios.status
        ? `${u.full_name}: ${status === 'APPROVED' ? 'acceso aprobado' : status === 'REJECTED' ? 'acceso rechazado' : 'marcado como pendiente'}`
        : `${u.full_name}: rol cambiado a ${labelRol(u)}`,
    )
  } catch (err: unknown) {
    u.role = previo.role
    u.status = previo.status
    toast.error(err instanceof Error ? err.message : 'No se pudo actualizar el usuario.')
  } finally {
    guardandoId.value = null
  }
}

const onCambiarRol = (u: Usuario, event: Event) => {
  const value = (event.target as HTMLSelectElement).value
  actualizar(u, { role: value })
}

// Eliminación con confirmación
const usuarioAEliminar = ref<Usuario | null>(null)
const isDeleting = ref(false)

const pedirEliminar = (u: Usuario) => {
  if (bloqueado(u)) return
  usuarioAEliminar.value = u
}

const confirmarEliminar = async () => {
  const u = usuarioAEliminar.value
  if (!u) return
  isDeleting.value = true
  try {
    await eliminarUsuario(u.id)
    usuarios.value = usuarios.value.filter((x) => x.id !== u.id)
    toast.success(`Usuario ${u.full_name} eliminado`)
    usuarioAEliminar.value = null
  } catch (err: unknown) {
    toast.error(err instanceof Error ? err.message : 'No se pudo eliminar el usuario.')
  } finally {
    isDeleting.value = false
  }
}
</script>

<template>
  <div class="mx-auto w-full max-w-7xl space-y-3 px-4 pb-12">
    <PageHeader title="Gestión de usuarios" subtitle="Aprobación de accesos, roles y bajas de cuentas del sistema">
      <template #actions>
        <Button variant="ghost" size="sm" class="gap-1.5 text-meta text-muted-foreground hover:text-foreground cursor-pointer"
          :disabled="isLoading" @click="cargarDatos">
          <IconRefresh class="size-3.5" :class="isLoading ? 'animate-spin' : ''" />
          <span>Actualizar</span>
        </Button>
      </template>
    </PageHeader>

    <div v-if="errorMessage" class="p-4 text-center rounded-xl border border-rose-500/20 bg-rose-500/5 space-y-3" role="alert">
      <IconAlertTriangle class="size-6 mx-auto text-rose-500" />
      <p class="text-meta font-bold text-rose-600 dark:text-rose-400">Error al cargar los usuarios</p>
      <p class="text-meta text-muted-foreground max-w-md mx-auto">{{ errorMessage }}</p>
      <Button variant="outline" size="sm" class="gap-1.5 text-meta cursor-pointer" @click="cargarDatos">
        <IconRefresh class="size-3.5" />
        <span>Reintentar</span>
      </Button>
    </div>

    <template v-else>
      <!-- KPIs -->
      <div class="grid grid-cols-2 md:grid-cols-4 gap-3">
        <button type="button"
          class="bg-card border rounded-xl p-3 flex items-center gap-2.5 text-left transition cursor-pointer hover:bg-muted/40"
          :class="filtroEstado === 'todos' ? 'border-primary/50 ring-1 ring-primary/30' : 'border-border shadow-2xs'"
          @click="filtroEstado = 'todos'">
          <span class="size-8 rounded-lg bg-primary/10 text-primary flex items-center justify-center shrink-0">
            <IconUsersGroup class="size-4" />
          </span>
          <span class="min-w-0">
            <span class="block text-micro text-muted-foreground">Total</span>
            <span class="block text-kpi font-bold font-mono tabular-nums text-foreground leading-none">{{ usuarios.length }}</span>
          </span>
        </button>
        <button v-for="e in ESTADOS" :key="e.id" type="button"
          class="bg-card border rounded-xl p-3 flex items-center gap-2.5 text-left transition cursor-pointer hover:bg-muted/40"
          :class="filtroEstado === e.id ? 'border-primary/50 ring-1 ring-primary/30' : 'border-border shadow-2xs'"
          @click="filtroEstado = filtroEstado === e.id ? 'todos' : e.id">
          <span class="size-8 rounded-lg flex items-center justify-center shrink-0" :class="e.tint">
            <component :is="e.icon" class="size-4" />
          </span>
          <span class="min-w-0">
            <span class="block text-micro text-muted-foreground">{{ e.label }}</span>
            <span class="block text-kpi font-bold font-mono tabular-nums text-foreground leading-none">{{ conteos[e.id] || 0 }}</span>
          </span>
        </button>
      </div>

      <!-- Tabla -->
      <section class="bg-card border border-border shadow-2xs rounded-xl overflow-hidden" aria-label="Lista de usuarios">
        <div class="flex flex-wrap items-center gap-2 px-3 py-2 border-b border-border">
          <label class="relative block w-full sm:w-64">
            <IconSearch class="size-3 absolute start-2.5 top-1/2 -translate-y-1/2 text-muted-foreground pointer-events-none" />
            <input v-model="busqueda" type="search" placeholder="Buscar por nombre o correo…"
              class="w-full ps-7 pe-2 h-control-sm py-0 rounded-md bg-card border border-border text-micro text-foreground placeholder:text-muted-foreground focus:border-primary focus:ring-primary" />
          </label>
          <FilterDropdown v-model="filtroEstado" :options="estadoOptions" aria-label="Filtrar por estado" />
          <FilterDropdown v-model="filtroRol" :options="rolOptions" aria-label="Filtrar por rol" />
          <Button v-if="hayFiltros" variant="ghost" size="xs"
            class="text-micro text-muted-foreground hover:text-foreground cursor-pointer" @click="limpiarFiltros">
            Limpiar filtros
          </Button>
          <span class="ms-auto text-micro text-muted-foreground tabular-nums">
            {{ filtrados.length }} de {{ usuarios.length }}
          </span>
        </div>

        <div v-if="isLoading" class="divide-y divide-border" aria-busy="true">
          <div v-for="i in 6" :key="i" class="flex items-center gap-3 px-3 py-2.5 animate-pulse">
            <div class="size-8 rounded-full bg-muted/60"></div>
            <div class="flex-1 space-y-1.5">
              <div class="h-2.5 w-40 rounded bg-muted/60"></div>
              <div class="h-2 w-56 rounded bg-muted/40"></div>
            </div>
            <div class="h-5 w-20 rounded-full bg-muted/50"></div>
            <div class="h-5 w-16 rounded-full bg-muted/50"></div>
          </div>
        </div>

        <div v-else class="overflow-x-auto">
          <table class="w-full text-left caption-bottom">
            <thead class="bg-muted/20 border-b border-border">
              <tr>
                <th class="px-3 py-2 text-micro font-semibold uppercase tracking-wider text-muted-foreground whitespace-nowrap">Usuario</th>
                <th class="px-3 py-2 text-micro font-semibold uppercase tracking-wider text-muted-foreground whitespace-nowrap">Rol</th>
                <th class="px-3 py-2 text-micro font-semibold uppercase tracking-wider text-muted-foreground whitespace-nowrap">Estado</th>
                <th class="px-3 py-2 text-micro font-semibold uppercase tracking-wider text-muted-foreground whitespace-nowrap hidden md:table-cell">Registro</th>
                <th class="px-3 py-2 text-micro font-semibold uppercase tracking-wider text-muted-foreground whitespace-nowrap text-end">Acciones</th>
              </tr>
            </thead>
            <tbody class="divide-y divide-border">
              <tr v-if="filtrados.length === 0">
                <td colspan="5" class="p-8 text-center text-meta text-muted-foreground">
                  <IconUsersGroup class="size-6 mx-auto mb-2 text-muted-foreground/50" />
                  {{ hayFiltros ? 'Ningún usuario coincide con los filtros.' : 'No hay usuarios registrados.' }}
                </td>
              </tr>
              <tr v-for="u in filtrados" :key="u.id" class="transition hover:bg-muted/30"
                :class="guardandoId === u.id ? 'opacity-60' : ''">
                <td class="px-3 py-2 align-middle">
                  <div class="flex items-center gap-2.5 min-w-0">
                    <span class="size-8 shrink-0 rounded-full bg-primary/10 border border-border overflow-hidden flex items-center justify-center font-bold text-micro text-primary">
                      <img v-if="u.picture_url" :src="u.picture_url" :alt="u.full_name" referrerpolicy="no-referrer"
                        class="size-full object-cover" @error="ocultarAvatarErroneo" />
                      <span v-else>{{ obtenerIniciales(u.full_name) }}</span>
                    </span>
                    <div class="min-w-0">
                      <p class="text-meta font-semibold text-foreground truncate flex items-center gap-1.5">
                        {{ u.full_name }}
                        <Badge v-if="esYo(u)" variant="primary" size="nano">Tú</Badge>
                        <span v-else-if="esSuperAdmin(u.email)" class="inline-flex text-primary" title="Administrador del sistema">
                          <IconShieldLock class="size-3" />
                        </span>
                      </p>
                      <p class="text-micro text-muted-foreground truncate">{{ u.email }}</p>
                    </div>
                  </div>
                </td>
                <td class="px-3 py-2 align-middle">
                  <span v-if="bloqueado(u)" class="text-meta text-foreground">{{ labelRol(u) }}</span>
                  <select v-else class="hs-input w-auto min-w-32 h-control-sm py-0 text-micro cursor-pointer"
                    :value="rolDe(u)" :disabled="guardandoId === u.id" :aria-label="`Rol de ${u.full_name}`"
                    @change="onCambiarRol(u, $event)">
                    <option v-for="r in ROLES" :key="r.id" :value="r.id">{{ r.label }}</option>
                  </select>
                </td>
                <td class="px-3 py-2 align-middle">
                  <Badge :variant="configEstado(u).variant" size="xs" dot>
                    {{ estadoDe(u) === 'PENDING' ? 'Pendiente' : estadoDe(u) === 'APPROVED' ? 'Aprobado' : estadoDe(u) === 'REJECTED' ? 'Rechazado' : u.status }}
                  </Badge>
                </td>
                <td class="px-3 py-2 align-middle hidden md:table-cell">
                  <span class="text-micro text-muted-foreground font-mono tabular-nums">{{ formatDate(u.created_at) }}</span>
                </td>
                <td class="px-3 py-2 align-middle">
                  <div v-if="!bloqueado(u)" class="flex items-center justify-end gap-1">
                    <Button v-if="estadoDe(u) !== 'APPROVED'" variant="ghost" size="xs"
                      class="gap-1 text-micro text-emerald-600 dark:text-emerald-400 hover:bg-emerald-500/10 cursor-pointer"
                      :disabled="guardandoId === u.id" title="Aprobar acceso" @click="actualizar(u, { status: 'APPROVED' })">
                      <IconCheck class="size-3.5" />
                      <span class="hidden sm:inline">Aprobar</span>
                    </Button>
                    <Button v-if="estadoDe(u) !== 'REJECTED'" variant="ghost" size="xs"
                      class="gap-1 text-micro text-amber-600 dark:text-amber-400 hover:bg-amber-500/10 cursor-pointer"
                      :disabled="guardandoId === u.id" title="Rechazar acceso" @click="actualizar(u, { status: 'REJECTED' })">
                      <IconBan class="size-3.5" />
                      <span class="hidden sm:inline">Rechazar</span>
                    </Button>
                    <Button variant="ghost" size="xs"
                      class="gap-1 text-micro text-muted-foreground hover:text-rose-600 dark:hover:text-rose-400 hover:bg-rose-500/10 cursor-pointer"
                      :disabled="guardandoId === u.id" title="Eliminar usuario" @click="pedirEliminar(u)">
                      <IconTrash class="size-3.5" />
                    </Button>
                  </div>
                  <span v-else class="block text-end text-micro text-muted-foreground">—</span>
                </td>
              </tr>
            </tbody>
          </table>
        </div>

        <div class="flex flex-wrap items-center gap-x-3 gap-y-1 px-3 py-2 border-t border-border text-micro text-muted-foreground">
          <span v-for="r in ROLES" :key="r.id"><span class="font-medium text-foreground">{{ r.label }}</span>: {{ r.hint }}</span>
          <span class="ms-auto inline-flex items-center gap-1"><IconShieldLock class="size-3" /> Solo {{ SUPER_ADMIN_EMAIL }} puede gestionar cuentas</span>
        </div>
      </section>
    </template>

    <Modal :is-open="!!usuarioAEliminar" :persistent="isDeleting" title="Eliminar usuario"
      subtitle="Esta acción no se puede deshacer" size="sm" @close="usuarioAEliminar = null">
      <div v-if="usuarioAEliminar" class="px-4 py-3 space-y-2">
        <div class="flex items-start gap-3 rounded-lg border border-rose-500/30 bg-rose-500/10 p-3">
          <span class="size-7 shrink-0 rounded-full bg-rose-500/15 text-rose-600 dark:text-rose-400 flex items-center justify-center">
            <IconAlertTriangle class="size-4" />
          </span>
          <div class="min-w-0 flex-1">
            <p class="text-meta font-semibold text-foreground">¿Eliminar a {{ usuarioAEliminar.full_name }}?</p>
            <p class="mt-0.5 text-micro text-muted-foreground">
              Se borrará la cuenta <span class="font-mono">{{ usuarioAEliminar.email }}</span>. La persona podrá volver a registrarse y quedará pendiente de aprobación.
            </p>
          </div>
        </div>
      </div>
      <template #footer>
        <Button variant="outline" size="sm" class="cursor-pointer" :disabled="isDeleting" @click="usuarioAEliminar = null">Cancelar</Button>
        <Button variant="danger" size="sm" class="gap-1.5 cursor-pointer" :loading="isDeleting" @click="confirmarEliminar">
          <IconTrash class="size-3.5" />
          {{ isDeleting ? 'Eliminando…' : 'Sí, eliminar' }}
        </Button>
      </template>
    </Modal>
  </div>
</template>
