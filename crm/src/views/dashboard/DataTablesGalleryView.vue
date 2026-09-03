<script setup lang="ts">
import { ref, computed } from 'vue'
import DataTable, { type DataTableColumn } from '@/components/ui/datatable/DataTable.vue'
import Badge from '@/components/ui/badge/Badge.vue'
import Button from '@/components/ui/button/Button.vue'
import Card from '@/components/ui/card/Card.vue'
import { formatDate } from '@/utils/date'
import {
  IconTrash,
  IconEdit,
  IconMail,
  IconFilter,
} from '@tabler/icons-vue'

interface Employee {
  id: number
  name: string
  position: string
  office: string
  age: number
  startDate: string
  salary: number
  status: 'Activo' | 'Remoto' | 'Permiso' | 'Inactivo'
}

const sampleEmployees = ref<Employee[]>([
  { id: 1, name: 'Martín Morales', position: 'Gerente Comercial', office: 'Lima', age: 38, startDate: formatDate('2019/04/12'), salary: 14500, status: 'Activo' },
  { id: 2, name: 'Sofía Valdivia', position: 'Desarrolladora Senior', office: 'Arequipa', age: 31, startDate: formatDate('2021/08/20'), salary: 12800, status: 'Remoto' },
  { id: 3, name: 'Diego Paredes', position: 'Diseñador UI/UX', office: 'Cusco', age: 29, startDate: formatDate('2022/01/15'), salary: 9200, status: 'Activo' },
  { id: 4, name: 'Valeria Mendoza', position: 'Directora de Cuentas', office: 'Lima', age: 35, startDate: formatDate('2018/11/02'), salary: 18000, status: 'Activo' },
  { id: 5, name: 'Rodrigo Flores', position: 'Especialista DevOps', office: 'Trujillo', age: 33, startDate: formatDate('2020/06/18'), salary: 13500, status: 'Remoto' },
  { id: 6, name: 'Camila Rojas', position: 'Analista QA', office: 'Lima', age: 26, startDate: formatDate('2023/03/10'), salary: 7800, status: 'Permiso' },
  { id: 7, name: 'Mateo Castillo', position: 'Arquitecto Cloud', office: 'Arequipa', age: 41, startDate: formatDate('2017/09/25'), salary: 19500, status: 'Activo' },
  { id: 8, name: 'Lucía Benítez', position: 'Líder de Producto', office: 'Lima', age: 34, startDate: formatDate('2020/02/14'), salary: 16000, status: 'Activo' },
  { id: 9, name: 'Álvaro Gutiérrez', position: 'Ingeniero de Datos', office: 'Chiclayo', age: 30, startDate: formatDate('2022/07/01'), salary: 11500, status: 'Remoto' },
  { id: 10, name: 'Elena Quispe', position: 'Especialista en Soporte', office: 'Cusco', age: 28, startDate: formatDate('2021/10/18'), salary: 6500, status: 'Inactivo' },
  { id: 11, name: 'Fernando Soria', position: 'Consultor CRM', office: 'Lima', age: 37, startDate: formatDate('2019/12/05'), salary: 15200, status: 'Activo' },
  { id: 12, name: 'Gabriela Paz', position: 'Coordinadora de Marketing', office: 'Trujillo', age: 27, startDate: formatDate('2023/05/22'), salary: 8400, status: 'Activo' },
])

const baseColumns: DataTableColumn[] = [
  { key: 'name', label: 'Nombre', sortable: true },
  { key: 'position', label: 'Puesto', sortable: true },
  { key: 'office', label: 'Sede', sortable: true },
  { key: 'age', label: 'Edad', sortable: true, align: 'right', width: '80px' },
  { key: 'startDate', label: 'Ingreso', sortable: true },
  { key: 'salary', label: 'Salario', sortable: true, align: 'right' },
  { key: 'status', label: 'Estado', sortable: true, align: 'center' },
]

const columnsWithActions: DataTableColumn[] = [
  ...baseColumns,
  { key: 'actions', label: 'Acciones', align: 'right', width: '120px' },
]

const selectedKeysRowSelection = ref<(string | number)[]>([1, 4])
const selectedKeysCombined = ref<(string | number)[]>([])

const officeFilter = ref('')
const statusFilter = ref('')

const filteredBySelectData = computed(() => {
  return sampleEmployees.value.filter((emp) => {
    const matchOffice = !officeFilter.value || emp.office === officeFilter.value
    const matchStatus = !statusFilter.value || emp.status === statusFilter.value
    return matchOffice && matchStatus
  })
})

const activeTab = ref('combined')

const tabs = [
  { id: 'combined', label: '1. Controles Combinados' },
  { id: 'default', label: '2. Por Defecto' },
  { id: 'selection', label: '3. Selección de Filas' },
  { id: 'filters', label: '4. Filtros de Tabla' },
  { id: 'column-filters', label: '5. Filtros por Columna' },
  { id: 'hidden-columns', label: '6. Columnas Ocultables' },
  { id: 'sticky', label: '7. Cabecera Fija' },
  { id: 'scrollable', label: '8. Scroll Vertical' },
  { id: 'actions', label: '9. Acciones & Ordenamiento' },
  { id: 'export', label: '10. Exportación' },
  { id: 'basic', label: '11. Básico' },
]

const formatMoney = (val: number) => {
  return new Intl.NumberFormat('es-PE', { style: 'currency', currency: 'USD', maximumFractionDigits: 0 }).format(val)
}

const statusVariant = (st: Employee['status']) => {
  switch (st) {
    case 'Activo':
      return 'success'
    case 'Remoto':
      return 'primary'
    case 'Permiso':
      return 'warning'
    case 'Inactivo':
      return 'danger'
    default:
      return 'neutral'
  }
}

const deleteSelected = (keys: (string | number)[]) => {
  sampleEmployees.value = sampleEmployees.value.filter((emp) => !keys.includes(emp.id))
  selectedKeysCombined.value = []
  selectedKeysRowSelection.value = []
}
</script>

<template>
  <div class="space-y-6 pb-12">
    <div class="border-b border-border overflow-x-auto">
      <nav class="flex space-x-2 py-1" aria-label="Tabs">
        <button
          v-for="t in tabs"
          :key="t.id"
          type="button"
          class="px-3 py-2 text-xs font-semibold rounded-lg whitespace-nowrap transition cursor-pointer"
          :class="activeTab === t.id ? 'bg-primary text-primary-foreground shadow-xs' : 'text-muted-foreground hover:text-foreground hover:bg-muted'"
          @click="activeTab = t.id"
        >
          {{ t.label }}
        </button>
      </nav>
    </div>

    <section v-if="activeTab === 'combined'" class="space-y-4">
      <Card>
        <CardHeader>
          <div>
            <CardTitle>Combined Controls DataTable</CardTitle>
            <CardDescription>
              Integra búsqueda global, selector de columnas visibles, exportación a CSV/Portapapeles, selección masiva de filas y paginación numerada.
            </CardDescription>
          </div>
        </CardHeader>
        <CardContent>
          <DataTable
            v-model:selected-keys="selectedKeysCombined"
            :columns="columnsWithActions"
            :data="sampleEmployees"
            selectable
            searchable
            show-column-visibility
            exportable
            export-filename="reporte_colaboradores"
            :page-size="5"
            :page-size-options="[5, 10, 20]"
          >
            <template #batch-actions="{ selectedKeys }">
              <Button size="xs" variant="danger" @click="deleteSelected(selectedKeys)">
                <template #prefix>
                  <IconTrash class="size-3.5" />
                </template>
                Eliminar ({{ selectedKeys.length }})
              </Button>
            </template>

            <template #cell-status="{ value }">
              <Badge :variant="statusVariant(value)" dot>
                {{ value }}
              </Badge>
            </template>

            <template #cell-salary="{ value }">
              <span class="font-semibold text-foreground">{{ formatMoney(value) }}</span>
            </template>

            <template #cell-actions="{ row }">
              <div class="flex items-center justify-end gap-1">
                <Button size="xs" variant="ghost" title="Editar registro">
                  <IconEdit class="size-3.5 text-muted-foreground" />
                </Button>
                <Button size="xs" variant="ghost" class="text-destructive hover:bg-destructive/10" title="Eliminar" @click="deleteSelected([row.id])">
                  <IconTrash class="size-3.5" />
                </Button>
              </div>
            </template>
          </DataTable>
        </CardContent>
      </Card>
    </section>

    <section v-else-if="activeTab === 'default'" class="space-y-4">
      <Card>
        <CardHeader>
          <div>
            <CardTitle>Default DataTable</CardTitle>
            <CardDescription>
              Barra de búsqueda superior, información de registros y paginación numerada interactiva.
            </CardDescription>
          </div>
        </CardHeader>
        <CardContent>
          <DataTable
            :columns="baseColumns"
            :data="sampleEmployees"
            searchable
            :page-size="5"
            :page-size-options="[5, 10, 20]"
          >
            <template #cell-status="{ value }">
              <Badge :variant="statusVariant(value)" dot>{{ value }}</Badge>
            </template>
            <template #cell-salary="{ value }">
              <span class="font-semibold text-foreground">{{ formatMoney(value) }}</span>
            </template>
          </DataTable>
        </CardContent>
      </Card>
    </section>

    <section v-else-if="activeTab === 'selection'" class="space-y-4">
      <Card>
        <CardHeader>
          <div>
            <CardTitle>Row Selection DataTable</CardTitle>
            <CardDescription>
              Casillas de selección de filas con checkbox "Seleccionar todo" con soporte de estado indeterminado.
            </CardDescription>
          </div>
        </CardHeader>
        <CardContent>
          <DataTable
            v-model:selected-keys="selectedKeysRowSelection"
            :columns="baseColumns"
            :data="sampleEmployees"
            selectable
            :page-size="5"
          >
            <template #batch-actions="{ selectedKeys }">
              <Button size="xs" variant="outline">
                <template #prefix>
                  <IconMail class="size-3.5" />
                </template>
                Enviar correo a {{ selectedKeys.length }} seleccionados
              </Button>
              <Button size="xs" variant="danger" @click="deleteSelected(selectedKeys)">
                <template #prefix>
                  <IconTrash class="size-3.5" />
                </template>
                Eliminar seleccionados
              </Button>
            </template>

            <template #cell-status="{ value }">
              <Badge :variant="statusVariant(value)" dot>{{ value }}</Badge>
            </template>
            <template #cell-salary="{ value }">
              <span class="font-semibold text-foreground">{{ formatMoney(value) }}</span>
            </template>
          </DataTable>
        </CardContent>
      </Card>
    </section>

    <section v-else-if="activeTab === 'filters'" class="space-y-4">
      <Card>
        <CardHeader>
          <div>
            <CardTitle>Table Filters (Categorías / Dropdown)</CardTitle>
            <CardDescription>
              Filtros externos para segmentar por sede y estado laboral antes del procesamiento.
            </CardDescription>
          </div>
        </CardHeader>
        <CardContent class="space-y-4">
          <div class="flex flex-wrap items-center gap-3 p-3 bg-muted/20 border border-border rounded-xl">
            <div class="flex items-center gap-2">
              <IconFilter class="size-4 text-muted-foreground" />
              <span class="text-xs font-semibold text-foreground">Filtrar por:</span>
            </div>

            <div class="flex items-center gap-2">
              <select
                v-model="officeFilter"
                class="bg-card border border-border rounded-lg px-2.5 py-1 text-xs text-foreground outline-none focus:border-primary"
              >
                <option value="">Todas las Sedes</option>
                <option value="Lima">Lima</option>
                <option value="Arequipa">Arequipa</option>
                <option value="Cusco">Cusco</option>
                <option value="Trujillo">Trujillo</option>
                <option value="Chiclayo">Chiclayo</option>
              </select>

              <select
                v-model="statusFilter"
                class="bg-card border border-border rounded-lg px-2.5 py-1 text-xs text-foreground outline-none focus:border-primary"
              >
                <option value="">Todos los Estados</option>
                <option value="Activo">Activo</option>
                <option value="Remoto">Remoto</option>
                <option value="Permiso">Permiso</option>
                <option value="Inactivo">Inactivo</option>
              </select>

              <Button
                v-if="officeFilter || statusFilter"
                size="xs"
                variant="ghost"
                @click="officeFilter = ''; statusFilter = ''"
              >
                Limpiar filtros
              </Button>
            </div>
          </div>

          <DataTable
            :columns="baseColumns"
            :data="filteredBySelectData"
            searchable
            :page-size="5"
          >
            <template #cell-status="{ value }">
              <Badge :variant="statusVariant(value)" dot>{{ value }}</Badge>
            </template>
            <template #cell-salary="{ value }">
              <span class="font-semibold text-foreground">{{ formatMoney(value) }}</span>
            </template>
          </DataTable>
        </CardContent>
      </Card>
    </section>

    <section v-else-if="activeTab === 'column-filters'" class="space-y-4">
      <Card>
        <CardHeader>
          <div>
            <CardTitle>Column Filters (Filtros por Columna)</CardTitle>
            <CardDescription>
              Fila secundaria de búsqueda bajo cada cabecera para filtrar específicamente por nombre, puesto o sede.
            </CardDescription>
          </div>
        </CardHeader>
        <CardContent>
          <DataTable
            :columns="baseColumns"
            :data="sampleEmployees"
            column-filters
            :searchable="false"
            :page-size="5"
          >
            <template #cell-status="{ value }">
              <Badge :variant="statusVariant(value)" dot>{{ value }}</Badge>
            </template>
            <template #cell-salary="{ value }">
              <span class="font-semibold text-foreground">{{ formatMoney(value) }}</span>
            </template>
          </DataTable>
        </CardContent>
      </Card>
    </section>

    <section v-else-if="activeTab === 'hidden-columns'" class="space-y-4">
      <Card>
        <CardHeader>
          <div>
            <CardTitle>Hidden Columns (Visibilidad de Columnas)</CardTitle>
            <CardDescription>
              Permite a los usuarios mostrar u ocultar columnas según sus preferencias desde el botón desplegable.
            </CardDescription>
          </div>
        </CardHeader>
        <CardContent>
          <DataTable
            :columns="baseColumns"
            :data="sampleEmployees"
            show-column-visibility
            searchable
            :page-size="5"
          >
            <template #cell-status="{ value }">
              <Badge :variant="statusVariant(value)" dot>{{ value }}</Badge>
            </template>
            <template #cell-salary="{ value }">
              <span class="font-semibold text-foreground">{{ formatMoney(value) }}</span>
            </template>
          </DataTable>
        </CardContent>
      </Card>
    </section>

    <section v-else-if="activeTab === 'sticky'" class="space-y-4">
      <Card>
        <CardHeader>
          <div>
            <CardTitle>Fixed Table Header (Sticky Header)</CardTitle>
            <CardDescription>
              Cabecera pegajosa (sticky) fija en la parte superior durante el scroll vertical continuo.
            </CardDescription>
          </div>
        </CardHeader>
        <CardContent>
          <DataTable
            :columns="baseColumns"
            :data="sampleEmployees"
            sticky-header
            max-height="280px"
            :paginated="false"
          >
            <template #cell-status="{ value }">
              <Badge :variant="statusVariant(value)" dot>{{ value }}</Badge>
            </template>
            <template #cell-salary="{ value }">
              <span class="font-semibold text-foreground">{{ formatMoney(value) }}</span>
            </template>
          </DataTable>
        </CardContent>
      </Card>
    </section>

    <section v-else-if="activeTab === 'scrollable'" class="space-y-4">
      <Card>
        <CardHeader>
          <div>
            <CardTitle>Scrollable Tbody DataTable</CardTitle>
            <CardDescription>
              Contenedor de tabla compacto con desplazamiento vertical fijo para datasets densos.
            </CardDescription>
          </div>
        </CardHeader>
        <CardContent>
          <DataTable
            :columns="baseColumns"
            :data="sampleEmployees"
            max-height="220px"
            :paginated="false"
            :searchable="false"
          >
            <template #cell-status="{ value }">
              <Badge :variant="statusVariant(value)" dot>{{ value }}</Badge>
            </template>
            <template #cell-salary="{ value }">
              <span class="font-semibold text-foreground">{{ formatMoney(value) }}</span>
            </template>
          </DataTable>
        </CardContent>
      </Card>
    </section>

    <section v-else-if="activeTab === 'actions'" class="space-y-4">
      <Card>
        <CardHeader>
          <div>
            <CardTitle>Column Actions & Sorting</CardTitle>
            <CardDescription>
              Ordenamiento bidireccional interactivo por columnas y menú de acciones por fila.
            </CardDescription>
          </div>
        </CardHeader>
        <CardContent>
          <DataTable
            :columns="columnsWithActions"
            :data="sampleEmployees"
            :page-size="5"
          >
            <template #cell-status="{ value }">
              <Badge :variant="statusVariant(value)" dot>{{ value }}</Badge>
            </template>
            <template #cell-salary="{ value }">
              <span class="font-semibold text-foreground">{{ formatMoney(value) }}</span>
            </template>
            <template #cell-actions>
              <div class="flex items-center justify-end gap-1">
                <Button size="xs" variant="outline">Ver</Button>
                <Button size="xs" variant="primary">Editar</Button>
              </div>
            </template>
          </DataTable>
        </CardContent>
      </Card>
    </section>

    <section v-else-if="activeTab === 'export'" class="space-y-4">
      <Card>
        <CardHeader>
          <div>
            <CardTitle>Export Actions</CardTitle>
            <CardDescription>
              Exporta los registros actuales o filtrados a CSV, copia datos tabulados al portapapeles o imprime la tabla.
            </CardDescription>
          </div>
        </CardHeader>
        <CardContent>
          <DataTable
            :columns="baseColumns"
            :data="sampleEmployees"
            exportable
            export-filename="empleados_crm"
            searchable
            :page-size="5"
          >
            <template #cell-status="{ value }">
              <Badge :variant="statusVariant(value)" dot>{{ value }}</Badge>
            </template>
            <template #cell-salary="{ value }">
              <span class="font-semibold text-foreground">{{ formatMoney(value) }}</span>
            </template>
          </DataTable>
        </CardContent>
      </Card>
    </section>

    <section v-else-if="activeTab === 'basic'" class="space-y-4">
      <Card>
        <CardHeader>
          <div>
            <CardTitle>Basic DataTable</CardTitle>
            <CardDescription>
              Tabla esencial con paginación simple y contador de registros sin barras de búsqueda adicionales.
            </CardDescription>
          </div>
        </CardHeader>
        <CardContent>
          <DataTable
            :columns="baseColumns"
            :data="sampleEmployees"
            :searchable="false"
            pagination-variant="simple"
            :page-size="5"
          >
            <template #cell-status="{ value }">
              <Badge :variant="statusVariant(value)" dot>{{ value }}</Badge>
            </template>
            <template #cell-salary="{ value }">
              <span class="font-semibold text-foreground">{{ formatMoney(value) }}</span>
            </template>
          </DataTable>
        </CardContent>
      </Card>
    </section>
  </div>
</template>
