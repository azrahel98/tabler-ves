<script setup lang="ts">
import { ref, computed } from 'vue'
import { RouterLink } from 'vue-router'
import Card from '@/components/ui/card/Card.vue'
import Badge from '@/components/ui/badge/Badge.vue'
import Button from '@/components/ui/button/Button.vue'
import DataTable, { type DataTableColumn } from '@/components/ui/datatable/DataTable.vue'
import { useAuthStore } from '@/stores/auth'
import {
  compararAirhsp,
  generarExcelMef,
  descargarBlob,
  aplicarAirhsp,
  CAMPOS_DATOS_PERSONALES,
  ETIQUETAS_CAMPOS,
  type ComparacionItem,
  type ResumenComparacion,
  type ResultadoComparacion,
  type AplicarAirhspResponse,
} from '@/services/airhsp'
import {
  IconFileSpreadsheet,
  IconArrowsDiff,
  IconUpload,
  IconDownload,
  IconAlertTriangle,
  IconAlertCircle,
  IconCheck,
  IconX,
  IconRefresh,
  IconFilter,
  IconUserExclamation,
  IconUsers,
  IconInfoCircle,
  IconCalendar,
  IconEye,
  IconArrowRight,
  IconChevronDown,
} from '@tabler/icons-vue'

// Store de autenticación y verificación de roles
const authStore = useAuthStore()

// Rol permitido para aplicar cambios definitivos (ADMIN o EDITOR)
const puedeAplicar = computed(() => {
  const role = authStore.user?.role?.toUpperCase() || ''
  return ['ADMIN', 'EDITOR'].includes(role)
})

// Archivos seleccionados
const fileCas = ref<File | null>(null)
const fileOtros = ref<File | null>(null)
const inputCasRef = ref<HTMLInputElement | null>(null)
const inputOtrosRef = ref<HTMLInputElement | null>(null)

// Estados de la petición de comparación
const isComparing = ref(false)
const isExporting = ref(false)
const errorMessage = ref<string | null>(null)

// Estados de Vista Previa y Aplicación de Actualizaciones
const isPreviewing = ref(false)
const isApplying = ref(false)
const previewData = ref<AplicarAirhspResponse | null>(null)
const errorActualizar = ref<string | null>(null)
const mensajeExito = ref<string | null>(null)
const mostrarModalConfirmacion = ref(false)
const omitidosColapsado = ref(true)

// Datos obtenidos de comparación
const resumen = ref<ResumenComparacion | null>(null)
const comparaciones = ref<ComparacionItem[]>([])

// Filtros
const filtroResultado = ref<string>('DIFERENCIA')
const soloDatosPersonales = ref<boolean>(true)
const filtroCampo = ref<string>('todos')
const busquedaTexto = ref<string>('')

// Columnas de la tabla de comparaciones
const columns: DataTableColumn[] = [
  {
    key: 'dni',
    label: 'DNI',
    sortable: true,
    width: '110px',
  },
  {
    key: 'nombre',
    label: 'Servidor Público',
    sortable: true,
  },
  {
    key: 'regimen',
    label: 'Régimen',
    sortable: true,
    width: '120px',
  },
  {
    key: 'campo',
    label: 'Campo Comparado',
    sortable: true,
    width: '190px',
  },
  {
    key: 'valor_propio',
    label: 'Nuestro Dato (Sistema)',
    sortable: true,
  },
  {
    key: 'valor_mef',
    label: 'Dato AIRHSP (MEF)',
    sortable: true,
  },
  {
    key: 'resultado',
    label: 'Resultado',
    sortable: true,
    width: '150px',
    align: 'center',
  },
]

// Columnas para la tabla de personas con cambios a actualizar
const columnasCambios: DataTableColumn[] = [
  {
    key: 'dni',
    label: 'DNI',
    sortable: true,
    width: '110px',
  },
  {
    key: 'nombre',
    label: 'Servidor Público',
    sortable: true,
    width: '240px',
  },
  {
    key: 'campos',
    label: 'Campos y Modificaciones (Antes → Después)',
    sortable: false,
  },
  {
    key: 'estado',
    label: 'Estado',
    sortable: false,
    width: '160px',
    align: 'center',
  },
]

// Columnas para la tabla de omitidos por fecha de ingreso
const columnasOmitidos: DataTableColumn[] = [
  {
    key: 'dni',
    label: 'DNI',
    sortable: true,
    width: '110px',
  },
  {
    key: 'nombre',
    label: 'Servidor Público',
    sortable: true,
  },
  {
    key: 'fecha_ingreso_sistema',
    label: 'Fecha en Sistema',
    sortable: true,
    width: '160px',
  },
  {
    key: 'fecha_ingreso_airhsp',
    label: 'Fecha en AIRHSP',
    sortable: true,
    width: '160px',
  },
  {
    key: 'observacion',
    label: 'Observación',
    sortable: false,
    width: '160px',
    align: 'center',
  },
]

// Manejadores de selección de archivos
const handleCasChange = (e: Event) => {
  const target = e.target as HTMLInputElement
  if (target.files && target.files.length > 0) {
    fileCas.value = target.files[0]
    previewData.value = null
    errorActualizar.value = null
    mensajeExito.value = null
  }
}

const handleOtrosChange = (e: Event) => {
  const target = e.target as HTMLInputElement
  if (target.files && target.files.length > 0) {
    fileOtros.value = target.files[0]
    previewData.value = null
    errorActualizar.value = null
    mensajeExito.value = null
  }
}

const quitarArchivoCas = () => {
  fileCas.value = null
  if (inputCasRef.value) inputCasRef.value.value = ''
  previewData.value = null
  errorActualizar.value = null
  mensajeExito.value = null
}

const quitarArchivoOtros = () => {
  fileOtros.value = null
  if (inputOtrosRef.value) inputOtrosRef.value.value = ''
  previewData.value = null
  errorActualizar.value = null
  mensajeExito.value = null
}

const formatoTamano = (bytes: number): string => {
  if (bytes === 0) return '0 B'
  const k = 1024
  const sizes = ['B', 'KB', 'MB', 'GB']
  const i = Math.floor(Math.log(bytes) / Math.log(k))
  return `${(bytes / Math.pow(k, i)).toFixed(1)} ${sizes[i]}`
}

// Ejecutar comparación
const ejecutarComparacion = async () => {
  if (!fileCas.value && !fileOtros.value) return

  isComparing.value = true
  errorMessage.value = null

  try {
    const data = await compararAirhsp(fileCas.value, fileOtros.value)
    resumen.value = data.resumen
    comparaciones.value = data.comparaciones || []
  } catch (err: unknown) {
    if (err instanceof Error) {
      errorMessage.value = err.message
    } else {
      errorMessage.value = 'Ocurrió un error inesperado al procesar la comparación con AIRHSP.'
    }
  } finally {
    isComparing.value = false
  }
}

// Ejecutar vista previa de actualización
const ejecutarVistaPrevia = async () => {
  if (!fileCas.value && !fileOtros.value) return

  isPreviewing.value = true
  errorActualizar.value = null
  mensajeExito.value = null

  try {
    const data = await aplicarAirhsp(fileCas.value, fileOtros.value, false)
    previewData.value = data
  } catch (err: unknown) {
    if (err instanceof Error) {
      if (err.message.includes('403') || err.message.toLowerCase().includes('forbidden')) {
        errorActualizar.value = 'Acceso denegado (403): Se requiere rol ADMIN o EDITOR para realizar esta operación.'
      } else {
        errorActualizar.value = err.message
      }
    } else {
      errorActualizar.value = 'Ocurrió un error inesperado al obtener la vista previa de actualización.'
    }
  } finally {
    isPreviewing.value = false
  }
}

// Ejecutar aplicación definitiva de actualizaciones
const ejecutarAplicar = async () => {
  if (!fileCas.value && !fileOtros.value) return

  isApplying.value = true
  errorActualizar.value = null
  mensajeExito.value = null
  mostrarModalConfirmacion.value = false

  try {
    const data = await aplicarAirhsp(fileCas.value, fileOtros.value, true)
    previewData.value = data
    mensajeExito.value = `Se aplicaron correctamente las actualizaciones para ${data.resumen.actualizados} personas en el sistema.`

    // Volver a ejecutar automáticamente la comparación para refrescar la tabla
    await ejecutarComparacion()
  } catch (err: unknown) {
    if (err instanceof Error) {
      if (err.message.includes('403') || err.message.toLowerCase().includes('forbidden')) {
        errorActualizar.value = 'Acceso denegado (403): Se requiere rol ADMIN o EDITOR para aplicar las actualizaciones en el sistema.'
      } else {
        errorActualizar.value = err.message
      }
    } else {
      errorActualizar.value = 'Ocurrió un error inesperado al aplicar las actualizaciones de AIRHSP.'
    }
  } finally {
    isApplying.value = false
  }
}

// Descargar Excel con todas las comparaciones
const descargarExcel = async () => {
  if (!comparaciones.value.length) return

  isExporting.value = true
  try {
    const blob = await generarExcelMef(comparaciones.value)
    const fecha = resumen.value?.fecha_comparacion
      ? resumen.value.fecha_comparacion.replace(/[: ]/g, '_')
      : new Date().toISOString().slice(0, 10)
    descargarBlob(blob, `comparacion_airhsp_${fecha}.xlsx`)
  } catch (err: unknown) {
    if (err instanceof Error) {
      errorMessage.value = `Error al exportar Excel: ${err.message}`
    } else {
      errorMessage.value = 'Error al generar el archivo Excel de comparación.'
    }
  } finally {
    isExporting.value = false
  }
}

// Función para determinar si un campo es considerado dato personal
const esDatoPersonal = (campo: string): boolean => {
  return (CAMPOS_DATOS_PERSONALES as readonly string[]).includes(campo)
}

// Cálculo en el front: Personas (DNI únicos) con diferencia en datos personales
const personasConDiferenciaPersonal = computed<number>(() => {
  if (!comparaciones.value.length) return 0
  const dnis = new Set<string>()
  for (const c of comparaciones.value) {
    if (c.resultado === 'DIFERENCIA' && esDatoPersonal(c.campo)) {
      if (c.dni) {
        dnis.add(c.dni)
      }
    }
  }
  return dnis.size
})

// Lista de campos disponibles en las comparaciones obtenidas
const camposDisponibles = computed<string[]>(() => {
  const set = new Set<string>()
  for (const c of comparaciones.value) {
    if (c.campo && c.campo !== '---') {
      set.add(c.campo)
    }
  }
  return Array.from(set).sort()
})

// Filtrado de comparaciones según controles
const comparacionesFiltradas = computed<ComparacionItem[]>(() => {
  let lista = comparaciones.value

  // Filtro por resultado
  if (filtroResultado.value !== 'TODOS') {
    lista = lista.filter((c) => c.resultado === filtroResultado.value)
  }

  // Filtro "Solo datos personales"
  if (soloDatosPersonales.value) {
    lista = lista.filter((c) => esDatoPersonal(c.campo))
  }

  // Filtro por campo específico
  if (filtroCampo.value !== 'todos') {
    lista = lista.filter((c) => c.campo === filtroCampo.value)
  }

  // Búsqueda por DNI o Nombre
  if (busquedaTexto.value.trim()) {
    const q = busquedaTexto.value.toLowerCase().trim()
    lista = lista.filter(
      (c) =>
        (c.dni && c.dni.toLowerCase().includes(q)) ||
        (c.nombre && c.nombre.toLowerCase().includes(q))
    )
  }

  return lista
})

// Etiqueta legible para campos
const obtenerEtiquetaCampo = (campo: string): string => {
  return ETIQUETAS_CAMPOS[campo] || campo || '-'
}

// Configuración visual por resultado
const obtenerBadgeResultado = (resultado: ResultadoComparacion) => {
  switch (resultado) {
    case 'OK':
      return {
        texto: 'Coincide',
        variant: 'success' as const,
        customClass: '',
      }
    case 'DIFERENCIA':
      return {
        texto: 'Diferencia',
        variant: 'warning' as const,
        customClass: '',
      }
    case 'NO_EXISTE_EN_MEF':
      return {
        texto: 'Solo en Sistema',
        variant: 'danger' as const,
        customClass: '',
      }
    case 'NO_EXISTE_EN_SISTEMA':
      return {
        texto: 'Solo en AIRHSP',
        variant: 'outline' as const,
        customClass:
          'bg-purple-500/10 text-purple-700 dark:text-purple-300 border-purple-500/30 font-medium',
      }
    default:
      return {
        texto: resultado,
        variant: 'neutral' as const,
        customClass: '',
      }
  }
}

// Limpiar filtros a valores por defecto
const restablecerFiltros = () => {
  filtroResultado.value = 'DIFERENCIA'
  soloDatosPersonales.value = true
  filtroCampo.value = 'todos'
  busquedaTexto.value = ''
}
</script>

<template>
  <div class="space-y-6 max-w-7xl mx-auto pb-16">
    <!-- Encabezado -->
    <div class="flex flex-col sm:flex-row sm:items-center justify-between gap-4 border-b border-border pb-4">
      <div>
        <div class="flex items-center gap-2.5">
          <div class="p-2 rounded-xl bg-primary/10 text-primary border border-primary/20">
            <IconArrowsDiff class="size-6" :stroke-width="2" />
          </div>
          <div>
            <h1 class="text-xl sm:text-2xl font-bold tracking-tight text-foreground">
              Comparación AIRHSP
            </h1>
            <p class="text-xs sm:text-sm text-muted-foreground mt-0.5">
              AIRHSP es la fuente oficial para nombres, apellidos y fecha de nacimiento.
            </p>
          </div>
        </div>
      </div>

      <div v-if="comparaciones.length > 0" class="flex items-center gap-2">
        <Button
          variant="outline"
          size="sm"
          class="gap-1.5 text-xs font-semibold"
          :loading="isExporting"
          @click="descargarExcel"
        >
          <IconDownload class="size-4 text-emerald-600 dark:text-emerald-400" />
          <span>Exportar Excel</span>
        </Button>
      </div>
    </div>

    <!-- Sección de carga de archivos -->
    <Card class="p-4 sm:p-6">
      <div class="space-y-4">
        <div class="flex items-center justify-between">
          <div>
            <h2 class="text-sm font-semibold text-foreground">
              Carga de Planillas AIRHSP
            </h2>
            <p class="text-xs text-muted-foreground">
              Selecciona los archivos Excel (.xlsx) proporcionados por el MEF. Puedes cargar uno o ambos regímenes.
            </p>
          </div>
        </div>

        <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
          <!-- Archivo CAS -->
          <div
            class="relative border border-dashed rounded-xl p-4 transition-all"
            :class="
              fileCas
                ? 'border-primary/50 bg-primary/5'
                : 'border-border bg-card hover:border-muted-foreground/40'
            "
          >
            <input
              ref="inputCasRef"
              type="file"
              accept=".xlsx,.xls"
              class="hidden"
              @change="handleCasChange"
            />
            <div class="flex items-start justify-between gap-3">
              <div class="flex items-center gap-3">
                <div
                  class="size-10 rounded-lg flex items-center justify-center shrink-0"
                  :class="fileCas ? 'bg-primary text-primary-foreground' : 'bg-muted text-muted-foreground'"
                >
                  <IconFileSpreadsheet class="size-5" />
                </div>
                <div class="min-w-0">
                  <div class="flex items-center gap-2">
                    <span class="text-xs font-bold text-foreground">Planilla CAS</span>
                    <Badge variant="secondary" size="xs">D.L. 1057</Badge>
                  </div>
                  <p v-if="fileCas" class="text-xs text-foreground font-medium truncate mt-0.5" :title="fileCas.name">
                    {{ fileCas.name }}
                  </p>
                  <p v-else class="text-xs text-muted-foreground mt-0.5">
                    No se ha seleccionado archivo
                  </p>
                  <span v-if="fileCas" class="text-[10px] text-muted-foreground">
                    {{ formatoTamano(fileCas.size) }}
                  </span>
                </div>
              </div>

              <div class="flex items-center gap-1.5 shrink-0">
                <Button
                  v-if="fileCas"
                  variant="ghost"
                  size="xs"
                  class="text-muted-foreground hover:text-destructive"
                  title="Quitar archivo"
                  @click="quitarArchivoCas"
                >
                  <IconX class="size-3.5" />
                </Button>
                <Button
                  variant="outline"
                  size="xs"
                  class="text-xs gap-1"
                  @click="inputCasRef?.click()"
                >
                  <IconUpload class="size-3.5" />
                  <span>{{ fileCas ? 'Cambiar' : 'Seleccionar' }}</span>
                </Button>
              </div>
            </div>
          </div>

          <!-- Archivo 276 / 728 -->
          <div
            class="relative border border-dashed rounded-xl p-4 transition-all"
            :class="
              fileOtros
                ? 'border-primary/50 bg-primary/5'
                : 'border-border bg-card hover:border-muted-foreground/40'
            "
          >
            <input
              ref="inputOtrosRef"
              type="file"
              accept=".xlsx,.xls"
              class="hidden"
              @change="handleOtrosChange"
            />
            <div class="flex items-start justify-between gap-3">
              <div class="flex items-center gap-3">
                <div
                  class="size-10 rounded-lg flex items-center justify-center shrink-0"
                  :class="fileOtros ? 'bg-primary text-primary-foreground' : 'bg-muted text-muted-foreground'"
                >
                  <IconFileSpreadsheet class="size-5" />
                </div>
                <div class="min-w-0">
                  <div class="flex items-center gap-2">
                    <span class="text-xs font-bold text-foreground">Planilla 276 / 728</span>
                    <Badge variant="secondary" size="xs">Otros Regímenes</Badge>
                  </div>
                  <p v-if="fileOtros" class="text-xs text-foreground font-medium truncate mt-0.5" :title="fileOtros.name">
                    {{ fileOtros.name }}
                  </p>
                  <p v-else class="text-xs text-muted-foreground mt-0.5">
                    No se ha seleccionado archivo
                  </p>
                  <span v-if="fileOtros" class="text-[10px] text-muted-foreground">
                    {{ formatoTamano(fileOtros.size) }}
                  </span>
                </div>
              </div>

              <div class="flex items-center gap-1.5 shrink-0">
                <Button
                  v-if="fileOtros"
                  variant="ghost"
                  size="xs"
                  class="text-muted-foreground hover:text-destructive"
                  title="Quitar archivo"
                  @click="quitarArchivoOtros"
                >
                  <IconX class="size-3.5" />
                </Button>
                <Button
                  variant="outline"
                  size="xs"
                  class="text-xs gap-1"
                  @click="inputOtrosRef?.click()"
                >
                  <IconUpload class="size-3.5" />
                  <span>{{ fileOtros ? 'Cambiar' : 'Seleccionar' }}</span>
                </Button>
              </div>
            </div>
          </div>
        </div>

        <!-- Regla de negocio en una línea -->
        <div class="rounded-xl border border-primary/20 bg-primary/5 p-3 sm:p-4 text-xs flex items-start gap-3">
          <IconInfoCircle class="size-4.5 text-primary shrink-0 mt-0.5" />
          <p class="leading-relaxed text-muted-foreground text-[11px] sm:text-xs">
            <strong class="text-foreground font-semibold">Regla de actualización:</strong>
            Solo se actualiza a quien coincide en DNI y fecha de ingreso con AIRHSP. Se actualizan nombres, apellidos, fecha de nacimiento, código AIRHSP (plaza), cargo estructural, grupo ocupacional y datos bancarios. Un dato vacío en AIRHSP no borra el nuestro.
          </p>
        </div>

        <!-- Botones de acción principales -->
        <div class="flex flex-wrap items-center justify-end gap-3 pt-2">
          <Button
            variant="outline"
            size="md"
            class="gap-2 text-xs font-semibold cursor-pointer"
            :disabled="(!fileCas && !fileOtros) || isComparing || isPreviewing || isApplying"
            :loading="isComparing"
            @click="ejecutarComparacion"
          >
            <IconArrowsDiff class="size-4" />
            <span>{{ isComparing ? 'Comparando datos...' : 'Comparar' }}</span>
          </Button>

          <Button
            variant="primary"
            size="md"
            class="gap-2 text-xs font-semibold cursor-pointer"
            :disabled="(!fileCas && !fileOtros) || isComparing || isPreviewing || isApplying"
            :loading="isPreviewing"
            @click="ejecutarVistaPrevia"
          >
            <IconEye class="size-4" />
            <span>{{ isPreviewing ? 'Cargando vista previa...' : 'Vista previa de actualización' }}</span>
          </Button>
        </div>
      </div>
    </Card>

    <!-- Error de comparación del backend -->
    <div
      v-if="errorMessage"
      class="p-4 sm:p-5 text-left rounded-xl border border-destructive/20 bg-destructive/5 flex items-start gap-3.5"
    >
      <IconAlertTriangle class="size-5 text-destructive shrink-0 mt-0.5" />
      <div class="space-y-1 flex-1">
        <h3 class="text-xs sm:text-sm font-bold text-destructive">
          Error en la comparación
        </h3>
        <p class="text-xs text-muted-foreground">
          {{ errorMessage }}
        </p>
      </div>
      <Button
        variant="ghost"
        size="xs"
        class="text-muted-foreground hover:text-foreground shrink-0 cursor-pointer"
        @click="errorMessage = null"
      >
        <IconX class="size-4" />
      </Button>
    </div>

    <!-- Error de actualización del backend (incluido 403) -->
    <div
      v-if="errorActualizar"
      class="p-4 sm:p-5 text-left rounded-xl border border-destructive/20 bg-destructive/5 flex items-start gap-3.5"
    >
      <IconAlertTriangle class="size-5 text-destructive shrink-0 mt-0.5" />
      <div class="space-y-1 flex-1">
        <h3 class="text-xs sm:text-sm font-bold text-destructive">
          Error en la actualización con AIRHSP
        </h3>
        <p class="text-xs text-muted-foreground">
          {{ errorActualizar }}
        </p>
      </div>
      <Button
        variant="ghost"
        size="xs"
        class="text-muted-foreground hover:text-foreground shrink-0 cursor-pointer"
        @click="errorActualizar = null"
      >
        <IconX class="size-4" />
      </Button>
    </div>

    <!-- Mensaje de éxito tras aplicar -->
    <div
      v-if="mensajeExito"
      class="p-4 sm:p-5 text-left rounded-xl border border-emerald-500/20 bg-emerald-500/5 flex items-start gap-3.5"
    >
      <IconCheck class="size-5 text-emerald-600 dark:text-emerald-400 shrink-0 mt-0.5" />
      <div class="space-y-1 flex-1">
        <h3 class="text-xs sm:text-sm font-bold text-emerald-600 dark:text-emerald-400">
          Actualización completada exitosamente
        </h3>
        <p class="text-xs text-muted-foreground">
          {{ mensajeExito }}
        </p>
      </div>
      <Button
        variant="ghost"
        size="xs"
        class="text-muted-foreground hover:text-foreground shrink-0 cursor-pointer"
        @click="mensajeExito = null"
      >
        <IconX class="size-4" />
      </Button>
    </div>

    <!-- Sección de Vista Previa / Aplicación de Actualizaciones -->
    <template v-if="previewData">
      <Card class="p-4 sm:p-6 space-y-6">
        <!-- Encabezado de la sección -->
        <div class="flex flex-col sm:flex-row sm:items-center justify-between gap-4 border-b border-border pb-4">
          <div class="space-y-1">
            <div class="flex items-center gap-2">
              <h2 class="text-base sm:text-lg font-bold text-foreground">
                {{ previewData.aplicado ? 'Resultado de la Actualización con AIRHSP' : 'Vista Previa de Actualización con AIRHSP' }}
              </h2>
              <Badge
                v-if="previewData.aplicado"
                variant="success"
                size="xs"
              >
                Actualizado en Sistema
              </Badge>
              <Badge
                v-else
                variant="secondary"
                size="xs"
              >
                Modo Simulación
              </Badge>
            </div>
            <p class="text-xs text-muted-foreground">
              {{
                previewData.aplicado
                  ? 'Las modificaciones han sido aplicadas en la base de datos de nuestro sistema.'
                  : 'Verifica los cambios propuestos antes de aplicarlos. Solo se modificarán los registros de personas que coincidan en DNI y fecha de ingreso.'
              }}
            </p>
          </div>

          <!-- Botón Aplicar N actualizaciones (solo ADMIN o EDITOR) -->
          <div class="flex items-center gap-2 shrink-0">
            <template v-if="puedeAplicar">
              <Button
                v-if="!previewData.aplicado && previewData.resumen.con_cambios > 0"
                variant="primary"
                size="md"
                class="gap-2 text-xs font-semibold cursor-pointer shadow-sm"
                :loading="isApplying"
                :disabled="isApplying"
                @click="mostrarModalConfirmacion = true"
              >
                <IconRefresh class="size-4" />
                <span>Aplicar {{ previewData.resumen.con_cambios }} {{ previewData.resumen.con_cambios === 1 ? 'actualización' : 'actualizaciones' }}</span>
              </Button>
            </template>
            <template v-else-if="!previewData.aplicado && previewData.resumen.con_cambios > 0">
              <div class="text-[11px] text-muted-foreground flex items-center gap-1.5 p-2 rounded-lg bg-muted/40 border border-border">
                <IconInfoCircle class="size-3.5 text-amber-500" />
                <span>Se requiere rol ADMIN o EDITOR para aplicar cambios</span>
              </div>
            </template>
          </div>
        </div>

        <!-- Tarjetas de métricas del resumen de la vista previa -->
        <div class="grid grid-cols-2 sm:grid-cols-3 lg:grid-cols-6 gap-3">
          <!-- Coincidencias -->
          <Card class="p-3.5">
            <div class="flex items-center justify-between gap-2">
              <span class="text-[11px] font-medium text-muted-foreground truncate">Coincidencias</span>
              <IconUsers class="size-3.5 text-muted-foreground shrink-0" />
            </div>
            <div class="mt-2 flex items-baseline justify-between">
              <span class="text-xl sm:text-2xl font-bold tracking-tight text-foreground font-mono">
                {{ previewData.resumen.coincidencias }}
              </span>
            </div>
            <p class="text-[10px] text-muted-foreground mt-0.5 truncate">DNI y fecha coinciden</p>
          </Card>

          <!-- Personas con cambios -->
          <Card class="p-3.5" :class="previewData.resumen.con_cambios > 0 ? 'border-primary/40 bg-primary/5' : ''">
            <div class="flex items-center justify-between gap-2">
              <span class="text-[11px] font-semibold text-primary truncate">Con Cambios</span>
              <span class="size-2 rounded-full bg-primary shrink-0"></span>
            </div>
            <div class="mt-2 flex items-baseline justify-between">
              <span class="text-xl sm:text-2xl font-bold tracking-tight text-primary font-mono">
                {{ previewData.resumen.con_cambios }}
              </span>
            </div>
            <p class="text-[10px] text-muted-foreground mt-0.5 truncate">Tienen datos nuevos</p>
          </Card>

          <!-- Sin cambios -->
          <Card class="p-3.5">
            <div class="flex items-center justify-between gap-2">
              <span class="text-[11px] font-semibold text-emerald-600 dark:text-emerald-400 truncate">Sin Cambios</span>
              <IconCheck class="size-3.5 text-emerald-500 shrink-0" />
            </div>
            <div class="mt-2 flex items-baseline justify-between">
              <span class="text-xl sm:text-2xl font-bold tracking-tight text-emerald-600 dark:text-emerald-400 font-mono">
                {{ previewData.resumen.sin_cambios }}
              </span>
            </div>
            <p class="text-[10px] text-muted-foreground mt-0.5 truncate">Ya sincronizados</p>
          </Card>

          <!-- Omitidos por fecha de ingreso distinta -->
          <Card class="p-3.5" :class="previewData.resumen.omitidos_fecha_ingreso > 0 ? 'border-amber-500/30 bg-amber-500/5' : ''">
            <div class="flex items-center justify-between gap-2">
              <span class="text-[11px] font-semibold text-amber-600 dark:text-amber-400 truncate">Omitidos (Fecha)</span>
              <IconAlertTriangle class="size-3.5 text-amber-500 shrink-0" />
            </div>
            <div class="mt-2 flex items-baseline justify-between">
              <span class="text-xl sm:text-2xl font-bold tracking-tight text-amber-600 dark:text-amber-400 font-mono">
                {{ previewData.resumen.omitidos_fecha_ingreso }}
              </span>
            </div>
            <p class="text-[10px] text-muted-foreground mt-0.5 truncate">Fecha distinta</p>
          </Card>

          <!-- Errores -->
          <Card class="p-3.5" :class="previewData.resumen.errores > 0 ? 'border-destructive/30 bg-destructive/5' : ''">
            <div class="flex items-center justify-between gap-2">
              <span class="text-[11px] font-semibold text-destructive truncate">Con Errores</span>
              <span class="size-2 rounded-full bg-destructive shrink-0"></span>
            </div>
            <div class="mt-2 flex items-baseline justify-between">
              <span class="text-xl sm:text-2xl font-bold tracking-tight text-destructive font-mono">
                {{ previewData.resumen.errores }}
              </span>
            </div>
            <p class="text-[10px] text-muted-foreground mt-0.5 truncate">Errores de validación</p>
          </Card>

          <!-- Actualizados (cuando fue aplicado) -->
          <Card class="p-3.5 border-emerald-500/40 bg-emerald-500/10">
            <div class="flex items-center justify-between gap-2">
              <span class="text-[11px] font-semibold text-emerald-700 dark:text-emerald-300 truncate">Actualizados</span>
              <IconCheck class="size-3.5 text-emerald-600 dark:text-emerald-400 shrink-0" />
            </div>
            <div class="mt-2 flex items-baseline justify-between">
              <span class="text-xl sm:text-2xl font-bold tracking-tight text-emerald-700 dark:text-emerald-300 font-mono">
                {{ previewData.resumen.actualizados }}
              </span>
            </div>
            <p class="text-[10px] text-emerald-600/80 dark:text-emerald-400/80 mt-0.5 truncate">
              {{ previewData.aplicado ? 'Aplicados en sistema' : 'Pendientes de aplicar' }}
            </p>
          </Card>
        </div>

        <!-- Lista / Tabla de personas con cambios -->
        <div class="space-y-3">
          <div class="flex items-center justify-between">
            <div>
              <h3 class="text-sm font-semibold text-foreground">
                Personas con Modificaciones
              </h3>
              <p class="text-xs text-muted-foreground">
                Detalle de campos antes y después por servidor público.
              </p>
            </div>
            <Badge variant="outline" size="xs">
              {{ previewData.cambios.length }} {{ previewData.cambios.length === 1 ? 'persona' : 'personas' }}
            </Badge>
          </div>

          <DataTable
            :columns="columnasCambios"
            :data="previewData.cambios"
            :row-key="(row: any) => row.dni || row.nombre"
            default-sort-column="dni"
            default-sort-direction="asc"
            :searchable="true"
            search-placeholder="Buscar por DNI o nombre..."
            :page-size="10"
            :page-size-options="[5, 10, 20, 50]"
            empty-message="No hay personas con cambios que requieran actualización"
          >
            <!-- Slot DNI -->
            <template #cell-dni="{ row }">
              <RouterLink
                v-if="row.dni"
                :to="{ name: 'perfil', params: { dni: row.dni } }"
                class="font-mono font-semibold text-xs text-primary hover:underline underline-offset-2"
                :title="`Ver perfil de ${row.nombre}`"
              >
                {{ row.dni }}
              </RouterLink>
              <span v-else class="font-mono text-xs text-muted-foreground">-</span>
            </template>

            <!-- Slot Nombre -->
            <template #cell-nombre="{ row }">
              <div class="space-y-1">
                <p class="font-semibold text-foreground text-xs" :title="row.nombre">
                  {{ row.nombre || '-' }}
                </p>
                <div
                  v-if="row.error"
                  class="p-2 rounded-lg bg-destructive/10 border border-destructive/20 text-destructive text-[11px] flex items-start gap-1.5"
                >
                  <IconAlertCircle class="size-3.5 shrink-0 mt-0.5" />
                  <span class="leading-tight">{{ row.error }}</span>
                </div>
              </div>
            </template>

            <!-- Slot Campos: antes → después -->
            <template #cell-campos="{ row }">
              <div class="space-y-2 py-1 max-w-xl">
                <div
                  v-for="(c, cIdx) in row.campos"
                  :key="cIdx"
                  class="p-2.5 rounded-lg text-xs transition-colors"
                  :class="
                    c.error
                      ? 'bg-destructive/5 border border-destructive/25'
                      : 'bg-muted/30 border border-border/60 hover:bg-muted/50'
                  "
                >
                  <div class="flex items-center justify-between gap-2 mb-1.5">
                    <span class="font-semibold text-foreground text-[11px]">
                      {{ obtenerEtiquetaCampo(c.campo) }}
                    </span>
                    <Badge v-if="c.error" variant="danger" size="xs" class="font-medium">
                      no se aplicará
                    </Badge>
                  </div>

                  <div class="flex items-center gap-2 flex-wrap font-mono text-xs">
                    <div class="flex items-center gap-1.5 text-muted-foreground">
                      <span class="text-[10px] uppercase font-sans tracking-wider text-muted-foreground/70">Antes:</span>
                      <span class="line-through decoration-muted-foreground/50 bg-muted/60 px-1.5 py-0.5 rounded">
                        {{ c.antes || '(vacío)' }}
                      </span>
                    </div>

                    <IconArrowRight class="size-3 text-muted-foreground shrink-0" />

                    <div class="flex items-center gap-1.5">
                      <span class="text-[10px] uppercase font-sans tracking-wider text-muted-foreground/70">Después:</span>
                      <span
                        v-if="!c.error"
                        class="px-1.5 py-0.5 rounded font-semibold text-emerald-700 dark:text-emerald-300 bg-emerald-500/15 border border-emerald-500/30"
                      >
                        {{ c.despues || '(vacío)' }}
                      </span>
                      <span
                        v-else
                        class="px-1.5 py-0.5 rounded font-semibold text-destructive bg-destructive/10 border border-destructive/25"
                      >
                        {{ c.despues || '(vacío)' }}
                      </span>
                    </div>
                  </div>

                  <!-- Mensaje de error del campo en rojo -->
                  <p
                    v-if="c.error"
                    class="text-[11px] text-destructive mt-1.5 flex items-center gap-1.5 font-sans"
                  >
                    <IconAlertTriangle class="size-3 shrink-0" />
                    <span>{{ c.error }}</span>
                  </p>
                </div>
              </div>
            </template>

            <!-- Slot Estado -->
            <template #cell-estado="{ row }">
              <div>
                <Badge v-if="row.error" variant="danger" size="xs" class="gap-1">
                  <IconAlertCircle class="size-3" />
                  <span>Con error</span>
                </Badge>
                <Badge v-else-if="row.campos.some((c: any) => c.error)" variant="warning" size="xs" class="gap-1">
                  <IconAlertTriangle class="size-3" />
                  <span>Parcial</span>
                </Badge>
                <Badge v-else variant="success" size="xs" class="gap-1">
                  <IconCheck class="size-3" />
                  <span>Listo</span>
                </Badge>
              </div>
            </template>
          </DataTable>
        </div>

        <!-- Sección colapsable "Omitidos por fecha de ingreso" -->
        <div class="border border-border/80 rounded-xl overflow-hidden bg-card">
          <button
            type="button"
            class="w-full p-4 flex items-center justify-between text-left hover:bg-muted/30 transition-colors cursor-pointer select-none"
            @click="omitidosColapsado = !omitidosColapsado"
          >
            <div class="flex items-center gap-3 min-w-0">
              <div class="p-2 rounded-lg bg-amber-500/10 text-amber-600 dark:text-amber-400 border border-amber-500/20 shrink-0">
                <IconCalendar class="size-4.5" />
              </div>
              <div>
                <div class="flex items-center gap-2">
                  <h3 class="text-sm font-semibold text-foreground">
                    Omitidos por fecha de ingreso distinta
                  </h3>
                  <Badge variant="secondary" size="xs">
                    {{ previewData.omitidos.length }}
                  </Badge>
                </div>
                <p class="text-xs text-muted-foreground mt-0.5">
                  Personas cuya fecha de ingreso en nuestro sistema difiere de la registrada en AIRHSP.
                </p>
              </div>
            </div>

            <div class="flex items-center gap-2 shrink-0 ml-2">
              <span class="text-xs text-muted-foreground hidden sm:inline">
                {{ omitidosColapsado ? 'Ver detalle' : 'Ocultar' }}
              </span>
              <IconChevronDown
                class="size-4 text-muted-foreground transition-transform duration-200"
                :class="{ 'rotate-180': !omitidosColapsado }"
              />
            </div>
          </button>

          <div v-show="!omitidosColapsado" class="p-4 pt-0 border-t border-border/60">
            <DataTable
              :columns="columnasOmitidos"
              :data="previewData.omitidos"
              :row-key="(row: any) => row.dni || row.nombre"
              :searchable="true"
              search-placeholder="Buscar por DNI o nombre..."
              :page-size="10"
              :page-size-options="[10, 20, 50]"
              empty-message="No hay servidores omitidos por fecha de ingreso"
            >
              <template #cell-dni="{ row }">
                <RouterLink
                  v-if="row.dni"
                  :to="{ name: 'perfil', params: { dni: row.dni } }"
                  class="font-mono font-semibold text-xs text-primary hover:underline underline-offset-2"
                  :title="`Ver perfil de ${row.nombre}`"
                >
                  {{ row.dni }}
                </RouterLink>
                <span v-else class="font-mono text-xs text-muted-foreground">-</span>
              </template>

              <template #cell-nombre="{ row }">
                <span class="font-medium text-foreground text-xs">{{ row.nombre || '-' }}</span>
              </template>

              <template #cell-fecha_ingreso_sistema="{ row }">
                <span class="font-mono text-xs text-muted-foreground">
                  {{ row.fecha_ingreso_sistema || '(sin fecha)' }}
                </span>
              </template>

              <template #cell-fecha_ingreso_airhsp="{ row }">
                <span class="font-mono text-xs font-semibold text-amber-600 dark:text-amber-400">
                  {{ row.fecha_ingreso_airhsp || '(sin fecha)' }}
                </span>
              </template>

              <template #cell-observacion>
                <Badge variant="outline" size="xs" class="text-amber-600 dark:text-amber-400 border-amber-500/30 bg-amber-500/10">
                  Fecha discrepante
                </Badge>
              </template>
            </DataTable>
          </div>
        </div>
      </Card>
    </template>

    <!-- Resultados y Resumen -->
    <template v-if="resumen">
      <!-- Tarjeta destacada de identidad -->
      <Card class="p-4 sm:p-5 border-amber-500/30 bg-amber-500/5">
        <div class="flex flex-col sm:flex-row sm:items-center justify-between gap-4">
          <div class="flex items-start gap-3.5">
            <div class="p-2.5 rounded-xl bg-amber-500/10 text-amber-600 dark:text-amber-400 border border-amber-500/20 shrink-0">
              <IconUserExclamation class="size-6" :stroke-width="2" />
            </div>
            <div>
              <div class="flex items-center gap-2">
                <h3 class="text-sm font-bold text-foreground">
                  Datos personales con diferencia
                </h3>
                <Badge variant="warning" size="xs">Identidad Crítica</Badge>
              </div>
              <p class="text-xs text-muted-foreground mt-0.5 max-w-2xl">
                Número de personas (DNI únicos) que presentan discrepancias en nombres, apellidos o fecha de nacimiento
                frente a la fuente oficial de AIRHSP.
              </p>
            </div>
          </div>

          <div class="flex items-baseline gap-2 shrink-0 self-end sm:self-center">
            <span class="text-3xl sm:text-4xl font-extrabold text-amber-600 dark:text-amber-400 font-mono">
              {{ personasConDiferenciaPersonal }}
            </span>
            <span class="text-xs font-medium text-muted-foreground">
              {{ personasConDiferenciaPersonal === 1 ? 'servidor' : 'servidores' }}
            </span>
          </div>
        </div>
      </Card>

      <!-- Resumen (tarjetas de métricas) -->
      <div class="grid grid-cols-2 sm:grid-cols-3 lg:grid-cols-6 gap-3">
        <!-- Procesados -->
        <Card class="p-3.5">
          <div class="flex items-center justify-between gap-2">
            <span class="text-[11px] font-medium text-muted-foreground truncate">Personas Procesadas</span>
            <IconUsers class="size-3.5 text-muted-foreground shrink-0" />
          </div>
          <div class="mt-2 flex items-baseline justify-between">
            <span class="text-xl sm:text-2xl font-bold tracking-tight text-foreground font-mono">
              {{ resumen.procesados }}
            </span>
          </div>
          <p class="text-[10px] text-muted-foreground mt-0.5 truncate">Total analizados</p>
        </Card>

        <!-- Encontrados MEF -->
        <Card class="p-3.5">
          <div class="flex items-center justify-between gap-2">
            <span class="text-[11px] font-medium text-muted-foreground truncate">Encontradas AIRHSP</span>
            <span class="size-2 rounded-full bg-primary shrink-0"></span>
          </div>
          <div class="mt-2 flex items-baseline justify-between">
            <span class="text-xl sm:text-2xl font-bold tracking-tight text-primary font-mono">
              {{ resumen.encontrados_mef }}
            </span>
          </div>
          <p class="text-[10px] text-muted-foreground mt-0.5 truncate">En planilla MEF</p>
        </Card>

        <!-- Campos OK -->
        <Card class="p-3.5">
          <div class="flex items-center justify-between gap-2">
            <span class="text-[11px] font-semibold text-emerald-600 dark:text-emerald-400 truncate">Campos OK</span>
            <IconCheck class="size-3.5 text-emerald-500 shrink-0" />
          </div>
          <div class="mt-2 flex items-baseline justify-between">
            <span class="text-xl sm:text-2xl font-bold tracking-tight text-emerald-600 dark:text-emerald-400 font-mono">
              {{ resumen.ok }}
            </span>
          </div>
          <p class="text-[10px] text-muted-foreground mt-0.5 truncate">Sin diferencias</p>
        </Card>

        <!-- Campos con diferencias -->
        <Card class="p-3.5">
          <div class="flex items-center justify-between gap-2">
            <span class="text-[11px] font-semibold text-amber-600 dark:text-amber-400 truncate">Diferencias</span>
            <span class="size-2 rounded-full bg-amber-500 shrink-0"></span>
          </div>
          <div class="mt-2 flex items-baseline justify-between">
            <span class="text-xl sm:text-2xl font-bold tracking-tight text-amber-600 dark:text-amber-400 font-mono">
              {{ resumen.diferencias }}
            </span>
          </div>
          <p class="text-[10px] text-muted-foreground mt-0.5 truncate">Campos a revisar</p>
        </Card>

        <!-- Solo en sistema -->
        <Card class="p-3.5">
          <div class="flex items-center justify-between gap-2">
            <span class="text-[11px] font-semibold text-rose-600 dark:text-rose-400 truncate">Solo en Sistema</span>
            <span class="size-2 rounded-full bg-rose-500 shrink-0"></span>
          </div>
          <div class="mt-2 flex items-baseline justify-between">
            <span class="text-xl sm:text-2xl font-bold tracking-tight text-rose-600 dark:text-rose-400 font-mono">
              {{ resumen.no_encontrados }}
            </span>
          </div>
          <p class="text-[10px] text-muted-foreground mt-0.5 truncate">No hallados en AIRHSP</p>
        </Card>

        <!-- Solo en AIRHSP -->
        <Card class="p-3.5">
          <div class="flex items-center justify-between gap-2">
            <span class="text-[11px] font-semibold text-purple-600 dark:text-purple-400 truncate">Solo en AIRHSP</span>
            <span class="size-2 rounded-full bg-purple-500 shrink-0"></span>
          </div>
          <div class="mt-2 flex items-baseline justify-between">
            <span class="text-xl sm:text-2xl font-bold tracking-tight text-purple-600 dark:text-purple-400 font-mono">
              {{ resumen.no_en_sistema }}
            </span>
          </div>
          <p class="text-[10px] text-muted-foreground mt-0.5 truncate">Faltan en sistema</p>
        </Card>
      </div>

      <!-- Barra de fecha e información de la comparación -->
      <div class="flex items-center justify-between text-xs text-muted-foreground px-1">
        <div class="flex items-center gap-1.5">
          <IconCalendar class="size-3.5 text-muted-foreground" />
          <span>Fecha de comparación: <strong class="text-foreground font-mono">{{ resumen.fecha_comparacion }}</strong></span>
        </div>
        <div>
          <span>Total de comparaciones generadas: <strong class="text-foreground font-mono">{{ comparaciones.length }}</strong></span>
        </div>
      </div>

      <!-- Tabla de comparaciones -->
      <DataTable
        :columns="columns"
        :data="comparacionesFiltradas"
        row-key="num"
        default-sort-column="num"
        default-sort-direction="asc"
        :searchable="false"
        :page-size="20"
        :page-size-options="[10, 20, 50, 100]"
        empty-message="No se encontraron comparaciones que coincidan con los filtros"
      >
        <!-- Toolbar con filtros personalizados -->
        <template #toolbar>
          <div class="flex items-center gap-2.5 flex-wrap w-full lg:w-auto">
            <!-- Búsqueda por DNI o Nombre -->
            <div class="relative w-full sm:w-56">
              <input
                v-model="busquedaTexto"
                type="text"
                placeholder="Buscar DNI o nombre..."
                class="w-full bg-card border border-border text-foreground placeholder:text-muted-foreground/70 px-2.5 py-1.5 text-xs rounded-lg outline-none focus:border-primary focus:ring-1 focus:ring-primary"
              />
              <button
                v-if="busquedaTexto"
                type="button"
                class="absolute inset-y-0 right-0 pr-2 flex items-center text-muted-foreground hover:text-foreground cursor-pointer"
                @click="busquedaTexto = ''"
              >
                <IconX class="size-3" />
              </button>
            </div>

            <!-- Filtro por Resultado -->
            <div class="flex items-center gap-1.5">
              <IconFilter class="size-3.5 text-muted-foreground shrink-0" />
              <select
                v-model="filtroResultado"
                class="bg-card border border-border rounded-lg px-2 py-1.5 text-xs text-foreground outline-none focus:border-primary cursor-pointer"
              >
                <option value="DIFERENCIA">Solo Diferencias</option>
                <option value="TODOS">Todos los resultados</option>
                <option value="OK">Coincidencias (OK)</option>
                <option value="NO_EXISTE_EN_MEF">Solo en Sistema</option>
                <option value="NO_EXISTE_EN_SISTEMA">Solo en AIRHSP</option>
              </select>
            </div>

            <!-- Conmutador: Solo datos personales -->
            <label
              class="inline-flex items-center gap-2 px-2.5 py-1.5 rounded-lg border text-xs cursor-pointer select-none transition-colors"
              :class="
                soloDatosPersonales
                  ? 'bg-amber-500/10 border-amber-500/30 text-amber-700 dark:text-amber-400 font-semibold'
                  : 'bg-card border-border text-muted-foreground hover:text-foreground'
              "
            >
              <input
                v-model="soloDatosPersonales"
                type="checkbox"
                class="size-3.5 rounded border-border text-amber-500 focus:ring-amber-500/20"
              />
              <span>Solo datos personales</span>
            </label>

            <!-- Selector de Campo -->
            <select
              v-model="filtroCampo"
              class="bg-card border border-border rounded-lg px-2 py-1.5 text-xs text-foreground outline-none focus:border-primary cursor-pointer max-w-48"
            >
              <option value="todos">Todos los campos</option>
              <option v-for="c in camposDisponibles" :key="c" :value="c">
                {{ obtenerEtiquetaCampo(c) }}
              </option>
            </select>

            <!-- Contador de filas visibles -->
            <div class="text-xs text-muted-foreground font-mono ml-auto sm:ml-0">
              <span class="text-foreground font-semibold">{{ comparacionesFiltradas.length }}</span> / {{ comparaciones.length }}
            </div>

            <!-- Restablecer filtros -->
            <Button
              v-if="
                filtroResultado !== 'DIFERENCIA' ||
                !soloDatosPersonales ||
                filtroCampo !== 'todos' ||
                busquedaTexto
              "
              variant="ghost"
              size="xs"
              class="text-xs text-muted-foreground hover:text-foreground cursor-pointer"
              @click="restablecerFiltros"
            >
              <IconRefresh class="size-3" />
              <span>Limpiar filtros</span>
            </Button>
          </div>
        </template>

        <!-- Slot celda DNI con enlace al perfil -->
        <template #cell-dni="{ row }">
          <RouterLink
            v-if="row.dni && row.resultado !== 'NO_EXISTE_EN_SISTEMA'"
            :to="{ name: 'perfil', params: { dni: row.dni } }"
            class="font-mono font-semibold text-xs text-primary hover:underline underline-offset-2"
            :title="`Ver perfil de ${row.nombre}`"
          >
            {{ row.dni }}
          </RouterLink>
          <span v-else class="font-mono text-xs text-muted-foreground">
            {{ row.dni || '-' }}
          </span>
        </template>

        <!-- Slot celda Nombre -->
        <template #cell-nombre="{ row }">
          <div class="min-w-44 max-w-sm">
            <p class="font-semibold text-foreground text-xs truncate" :title="row.nombre">
              {{ row.nombre || '-' }}
            </p>
            <span
              v-if="row.codigo_registro"
              class="text-[10px] font-mono text-muted-foreground"
              :title="`Código Registro: ${row.codigo_registro}`"
            >
              Cód: {{ row.codigo_registro }}
            </span>
          </div>
        </template>

        <!-- Slot celda Régimen -->
        <template #cell-regimen="{ row }">
          <div class="space-y-0.5">
            <Badge variant="secondary" size="xs" class="font-medium truncate max-w-28">
              {{ row.regimen || 'Sin régimen' }}
            </Badge>
            <p
              v-if="row.regimen_mef && row.regimen_mef !== row.regimen"
              class="text-[10px] text-amber-600 dark:text-amber-400 font-mono truncate"
              :title="`Régimen MEF: ${row.regimen_mef}`"
            >
              MEF: {{ row.regimen_mef }}
            </p>
          </div>
        </template>

        <!-- Slot celda Campo -->
        <template #cell-campo="{ row }">
          <div class="space-y-0.5">
            <span class="text-xs font-medium text-foreground">
              {{ obtenerEtiquetaCampo(row.campo) }}
            </span>
            <div v-if="esDatoPersonal(row.campo)" class="flex items-center gap-1">
              <span class="size-1.5 rounded-full bg-amber-500"></span>
              <span class="text-[10px] text-amber-600 dark:text-amber-400 font-medium">Dato Personal</span>
            </div>
          </div>
        </template>

        <!-- Slot celda Nuestro Dato -->
        <template #cell-valor_propio="{ row }">
          <div class="text-xs font-mono break-all max-w-xs">
            <span v-if="row.valor_propio" class="text-foreground">
              {{ row.valor_propio }}
            </span>
            <span v-else class="text-muted-foreground/60 italic">
              (vacío)
            </span>
          </div>
        </template>

        <!-- Slot celda AIRHSP (resaltado si es diferencia) -->
        <template #cell-valor_mef="{ row }">
          <div class="text-xs font-mono break-all max-w-xs">
            <template v-if="row.resultado === 'DIFERENCIA'">
              <span
                class="inline-block px-1.5 py-0.5 rounded font-semibold text-amber-700 dark:text-amber-300 bg-amber-500/15 border border-amber-500/30"
              >
                {{ row.valor_mef || '(vacío en MEF)' }}
              </span>
            </template>
            <template v-else-if="row.valor_mef">
              <span class="text-foreground">
                {{ row.valor_mef }}
              </span>
            </template>
            <template v-else>
              <span class="text-muted-foreground/60 italic">
                (vacío)
              </span>
            </template>
          </div>
        </template>

        <!-- Slot celda Resultado con Badge -->
        <template #cell-resultado="{ row }">
          <Badge
            :variant="obtenerBadgeResultado(row.resultado).variant"
            size="xs"
            :class="obtenerBadgeResultado(row.resultado).customClass"
          >
            {{ obtenerBadgeResultado(row.resultado).texto }}
          </Badge>
        </template>

        <!-- Estado vacío de la tabla -->
        <template #empty>
          <div class="p-8 text-center space-y-2">
            <IconInfoCircle class="size-8 mx-auto text-muted-foreground/60" />
            <h4 class="text-xs sm:text-sm font-semibold text-foreground">
              Sin registros para mostrar
            </h4>
            <p class="text-xs text-muted-foreground max-w-sm mx-auto">
              No hay comparaciones que coincidan con los filtros aplicados.
            </p>
            <Button
              variant="outline"
              size="xs"
              class="mt-2 text-xs"
              @click="restablecerFiltros"
            >
              Restablecer filtros
            </Button>
          </div>
        </template>
      </DataTable>
    </template>

    <!-- Diálogo de confirmación para aplicar actualizaciones -->
    <div
      v-if="mostrarModalConfirmacion"
      role="dialog"
      aria-modal="true"
      aria-labelledby="modal-confirmar-actualizacion-title"
      tabindex="-1"
      class="fixed inset-0 z-50 flex items-center justify-center p-4 bg-neutral-900/65 backdrop-blur-xs focus:outline-hidden"
      @click.self="!isApplying && (mostrarModalConfirmacion = false)"
      @keydown.esc="!isApplying && (mostrarModalConfirmacion = false)"
    >
      <div
        class="w-full max-w-md bg-card border border-border rounded-2xl shadow-2xl overflow-hidden text-xs flex flex-col"
      >
        <!-- Encabezado del modal -->
        <div class="p-4 border-b border-border flex items-center justify-between bg-primary/5 shrink-0">
          <div class="flex items-center gap-2.5 min-w-0">
            <div
              class="size-8 rounded-lg bg-primary/10 text-primary border border-primary/20 flex items-center justify-center shrink-0"
            >
              <IconRefresh class="size-4.5" />
            </div>
            <div class="min-w-0">
              <h3 id="modal-confirmar-actualizacion-title" class="font-bold text-foreground text-sm tracking-tight truncate">
                Confirmar Actualización con AIRHSP
              </h3>
              <p class="text-[11px] text-muted-foreground truncate">
                Modificación en la base de datos del sistema
              </p>
            </div>
          </div>

          <button
            type="button"
            class="size-7 rounded-lg text-muted-foreground hover:text-foreground hover:bg-muted flex items-center justify-center transition-colors cursor-pointer shrink-0"
            aria-label="Cerrar modal"
            :disabled="isApplying"
            @click="mostrarModalConfirmacion = false"
          >
            <IconX class="size-4" />
          </button>
        </div>

        <!-- Cuerpo del modal -->
        <div class="p-5 space-y-4">
          <div class="p-3.5 rounded-xl border border-border bg-muted/20 space-y-2">
            <div class="flex items-center justify-between text-xs">
              <span class="text-muted-foreground">Personas a actualizar:</span>
              <span class="font-mono font-bold text-foreground text-sm">
                {{ previewData?.resumen.con_cambios ?? 0 }}
              </span>
            </div>
            <div class="flex items-center justify-between text-xs">
              <span class="text-muted-foreground">Coincidencias en planilla:</span>
              <span class="font-mono text-muted-foreground">
                {{ previewData?.resumen.coincidencias ?? 0 }}
              </span>
            </div>
            <div class="flex items-center justify-between text-xs">
              <span class="text-muted-foreground">Omitidos por fecha distinta:</span>
              <span class="font-mono text-amber-600 dark:text-amber-400">
                {{ previewData?.resumen.omitidos_fecha_ingreso ?? 0 }}
              </span>
            </div>
          </div>

          <div class="p-3.5 rounded-xl bg-amber-500/10 border border-amber-500/20 text-amber-800 dark:text-amber-300 text-[11px] flex items-start gap-2.5">
            <IconAlertTriangle class="size-4 shrink-0 mt-0.5 text-amber-600 dark:text-amber-400" />
            <div class="space-y-1">
              <p class="font-semibold text-foreground">
                ¿Estás seguro de que deseas aplicar las actualizaciones para {{ previewData?.resumen.con_cambios }} personas?
              </p>
              <p class="text-muted-foreground leading-relaxed">
                Se actualizarán nombres, apellidos, fecha de nacimiento, código AIRHSP, cargo estructural, grupo ocupacional y datos bancarios. Los campos con errores de validación no se aplicarán. Un dato vacío en AIRHSP no borrará los datos del sistema.
              </p>
            </div>
          </div>
        </div>

        <!-- Botones del modal -->
        <div class="p-4 border-t border-border bg-muted/20 flex items-center justify-end gap-2 shrink-0">
          <Button
            type="button"
            variant="outline"
            size="sm"
            class="cursor-pointer"
            :disabled="isApplying"
            @click="mostrarModalConfirmacion = false"
          >
            Cancelar
          </Button>
          <Button
            type="button"
            variant="primary"
            size="sm"
            class="cursor-pointer gap-1.5"
            :loading="isApplying"
            :disabled="isApplying"
            @click="ejecutarAplicar"
          >
            <IconCheck class="size-3.5" />
            <span>{{ isApplying ? 'Aplicando cambios...' : 'Sí, aplicar actualizaciones' }}</span>
          </Button>
        </div>
      </div>
    </div>
  </div>
</template>
