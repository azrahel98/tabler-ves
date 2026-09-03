<script setup lang="ts">
import { ref } from 'vue'
import Button from '@/components/ui/button/Button.vue'
import Badge from '@/components/ui/badge/Badge.vue'
import Card from '@/components/ui/card/Card.vue'
import CardFooter from '@/components/ui/card/CardFooter.vue'
import CardImage from '@/components/ui/card/CardImage.vue'
import Input from '@/components/ui/form/Input.vue'
import Textarea from '@/components/ui/form/Textarea.vue'
import Select from '@/components/ui/form/Select.vue'
import Checkbox from '@/components/ui/form/Checkbox.vue'
import Switch from '@/components/ui/form/Switch.vue'
import Link from '@/components/ui/link/Link.vue'
import Table from '@/components/ui/table/Table.vue'
import TableHeader from '@/components/ui/table/TableHeader.vue'
import TableBody from '@/components/ui/table/TableBody.vue'
import TableRow from '@/components/ui/table/TableRow.vue'
import TableHead from '@/components/ui/table/TableHead.vue'
import TableCell from '@/components/ui/table/TableCell.vue'
import DataTable, { type DataTableColumn } from '@/components/ui/datatable/DataTable.vue'
import {
  IconSend,
  IconTrash,
  IconDownload,
  IconPlus,
  IconSearch,
} from '@tabler/icons-vue'

const buttonLoading = ref(false)

const formState = ref({
  fullName: '',
  email: '',
  plan: 'pro',
  notes: '',
  newsletter: true,
  notifications: true,
  terms: false,
})

const planOptions = [
  { label: 'Starter ($19/mes)', value: 'starter' },
  { label: 'Pro ($49/mes)', value: 'pro' },
  { label: 'Enterprise ($99/mes)', value: 'enterprise' },
]

const sampleTableData = [
  { id: 1, name: 'Acme Corp', contact: 'Juan Pérez', status: 'Activo', value: '$12,400' },
  { id: 2, name: 'Globex Ltd', contact: 'Elena Gómez', status: 'Pendiente', value: '$8,200' },
  { id: 3, name: 'Soylent Inc', contact: 'Carlos Ruiz', status: 'Inactivo', value: '$4,150' },
]

interface LeadItem {
  id: number
  client: string
  company: string
  email: string
  status: 'Ganado' | 'En Negociación' | 'Nuevo' | 'Perdido'
  budget: number
  city: string
}

const datatableColumns: DataTableColumn[] = [
  { key: 'client', label: 'Cliente', sortable: true },
  { key: 'company', label: 'Empresa', sortable: true },
  { key: 'email', label: 'Correo', sortable: true },
  { key: 'city', label: 'Ciudad', sortable: true },
  { key: 'status', label: 'Estado', sortable: true },
  { key: 'budget', label: 'Presupuesto', sortable: true, align: 'right' },
  { key: 'actions', label: 'Acciones', align: 'right' },
]

const datatableData = ref<LeadItem[]>([
  { id: 101, client: 'Martín Morales', company: 'Inversiones Alfa', email: 'martin@alfa.pe', status: 'Ganado', budget: 14500, city: 'Lima' },
  { id: 102, client: 'Sofía Valdivia', company: 'Soluciones Cloud', email: 'sofia@cloud.com', status: 'En Negociación', budget: 8200, city: 'Arequipa' },
  { id: 103, client: 'Diego Paredes', company: 'Andes Logistics', email: 'diego@andes.pe', status: 'Nuevo', budget: 4300, city: 'Cusco' },
  { id: 104, client: 'Valeria Mendoza', company: 'TechNova SAC', email: 'valeria@technova.pe', status: 'Ganado', budget: 22000, city: 'Lima' },
  { id: 105, client: 'Rodrigo Flores', company: 'Agro Export', email: 'rodrigo@agro.com', status: 'Perdido', budget: 3100, city: 'Trujillo' },
  { id: 106, client: 'Camila Rojas', company: 'Financiera Futuro', email: 'camila@futuro.pe', status: 'En Negociación', budget: 18900, city: 'Lima' },
  { id: 107, client: 'Mateo Castillo', company: 'Constructora Sur', email: 'mateo@sur.pe', status: 'Nuevo', budget: 9500, city: 'Arequipa' },
  { id: 108, client: 'Lucía Benítez', company: 'Innovar Perú', email: 'lucia@innovar.pe', status: 'Ganado', budget: 16400, city: 'Lima' },
])

const selectedLeadKeys = ref<(string | number)[]>([101])

const formatMoney = (val: number) => {
  return new Intl.NumberFormat('es-PE', { style: 'currency', currency: 'USD', maximumFractionDigits: 0 }).format(val)
}

const statusBadgeVariant = (status: LeadItem['status']) => {
  switch (status) {
    case 'Ganado':
      return 'success'
    case 'En Negociación':
      return 'warning'
    case 'Nuevo':
      return 'primary'
    case 'Perdido':
      return 'danger'
    default:
      return 'neutral'
  }
}

const toggleLoading = () => {
  buttonLoading.value = true
  setTimeout(() => {
    buttonLoading.value = false
  }, 2000)
}
</script>

<template>
  <div class="space-y-10 pb-12">
    <section class="space-y-4">
      <div class="border-b border-border pb-2">
        <h2 class="text-lg font-semibold text-foreground">1. Buttons</h2>
        <p class="text-xs text-muted-foreground">Variantes, tamaños, iconos y estados interactivos.</p>
      </div>

      <Card>
        <CardContent class="space-y-6">
          <div class="space-y-2">
            <span class="text-xs font-semibold text-muted-foreground uppercase tracking-wider block">Variantes de Color</span>
            <div class="flex flex-wrap gap-2.5 items-center">
              <Button variant="primary">Primary</Button>
              <Button variant="secondary">Secondary</Button>
              <Button variant="outline">Outline</Button>
              <Button variant="ghost">Ghost</Button>
              <Button variant="danger">Danger</Button>
              <Button variant="link">Link Style</Button>
            </div>
          </div>

          <div class="space-y-2">
            <span class="text-xs font-semibold text-muted-foreground uppercase tracking-wider block">Tamaños</span>
            <div class="flex flex-wrap gap-2.5 items-center">
              <Button size="xs">Extra Small (xs)</Button>
              <Button size="sm">Small (sm)</Button>
              <Button size="md">Medium (md)</Button>
              <Button size="lg">Large (lg)</Button>
            </div>
          </div>

          <div class="space-y-2">
            <span class="text-xs font-semibold text-muted-foreground uppercase tracking-wider block">Con Iconos y Estados</span>
            <div class="flex flex-wrap gap-2.5 items-center">
              <Button variant="primary">
                <template #prefix>
                  <IconPlus class="size-4" />
                </template>
                Crear Registro
              </Button>

              <Button variant="outline">
                Descargar
                <template #suffix>
                  <IconDownload class="size-4" />
                </template>
              </Button>

              <Button variant="danger">
                <template #prefix>
                  <IconTrash class="size-4" />
                </template>
                Eliminar
              </Button>

              <Button
                variant="primary"
                :loading="buttonLoading"
                @click="toggleLoading"
              >
                {{ buttonLoading ? 'Procesando...' : 'Probar Loading' }}
              </Button>

              <Button variant="secondary" disabled>
                Deshabilitado
              </Button>
            </div>
          </div>
        </CardContent>
      </Card>
    </section>

    <section class="space-y-4">
      <div class="border-b border-border pb-2">
        <h2 class="text-lg font-semibold text-foreground">2. Badges</h2>
        <p class="text-xs text-muted-foreground">Etiquetas de estado con soporte para dot indicator y tamaños.</p>
      </div>

      <Card>
        <CardContent class="space-y-4">
          <div class="space-y-2">
            <span class="text-xs font-semibold text-muted-foreground uppercase tracking-wider block">Variantes Semánticas</span>
            <div class="flex flex-wrap gap-2 items-center">
              <Badge variant="primary">Primary</Badge>
              <Badge variant="secondary">Secondary</Badge>
              <Badge variant="success">Success</Badge>
              <Badge variant="warning">Warning</Badge>
              <Badge variant="danger">Danger</Badge>
              <Badge variant="outline">Outline</Badge>
              <Badge variant="neutral">Neutral</Badge>
            </div>
          </div>

          <div class="space-y-2">
            <span class="text-xs font-semibold text-muted-foreground uppercase tracking-wider block">Con Indicador de Punto (Dot)</span>
            <div class="flex flex-wrap gap-2 items-center">
              <Badge variant="success" dot>Activo</Badge>
              <Badge variant="warning" dot>En Progreso</Badge>
              <Badge variant="danger" dot>Rechazado</Badge>
              <Badge variant="primary" dot>Nuevo Lead</Badge>
              <Badge variant="neutral" dot>Pausado</Badge>
            </div>
          </div>

          <div class="space-y-2">
            <span class="text-xs font-semibold text-muted-foreground uppercase tracking-wider block">Tamaños</span>
            <div class="flex flex-wrap gap-2 items-center">
              <Badge size="xs" variant="primary">Badge XS</Badge>
              <Badge size="sm" variant="primary">Badge SM</Badge>
              <Badge size="md" variant="primary">Badge MD</Badge>
            </div>
          </div>
        </CardContent>
      </Card>
    </section>

    <section class="space-y-4">
      <div class="border-b border-border pb-2">
        <h2 class="text-lg font-semibold text-foreground">3. Cards</h2>
        <p class="text-xs text-muted-foreground">Contenedores modulares con encabezado, descripción, contenido y pie.</p>
      </div>

      <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
        <Card hoverable top-border="primary">
          <CardHeader variant="surface">
            <div>
              <CardTitle>Reporte Comercial</CardTitle>
              <CardDescription>Resumen del último trimestre del CRM</CardDescription>
            </div>
            <template #action>
              <Badge variant="success" dot>Actualizado</Badge>
            </template>
          </CardHeader>
          <CardContent>
            <p class="text-sm text-muted-foreground">
              Composición modular de Preline con acento superior de color (topBorder="primary"), encabezado de superficie sutil y bordes adaptables a temas.
            </p>
          </CardContent>
          <CardFooter variant="surface">
            <span class="text-xs text-muted-foreground">Último cambio hace 5 min</span>
            <Button size="xs" variant="outline">Ver Detalles</Button>
          </CardFooter>
        </Card>

        <Card top-border="success">
          <CardHeader>
            <CardTitle>Métricas Rápidas</CardTitle>
            <CardDescription>Indicadores de prospección</CardDescription>
          </CardHeader>
          <CardContent class="grid grid-cols-2 gap-3">
            <div class="p-3 bg-muted/30 rounded-lg border border-border">
              <span class="text-xs text-muted-foreground block">Tasa de Cierre</span>
              <span class="text-xl font-bold text-foreground mt-1 block">68.4%</span>
            </div>
            <div class="p-3 bg-muted/30 rounded-lg border border-border">
              <span class="text-xs text-muted-foreground block">Tiempo Promedio</span>
              <span class="text-xl font-bold text-foreground mt-1 block">14 días</span>
            </div>
          </CardContent>
          <CardFooter>
            <Button size="xs" variant="primary" class="w-full">
              Explorar Pipeline
            </Button>
          </CardFooter>
        </Card>

        <Card>
          <CardImage
            src="https://images.unsplash.com/photo-1680868543815-b8666dba60f7?ixlib=rb-4.0.3&auto=format&fit=crop&w=400&q=80"
            alt="Preline Default Card"
            position="top"
          />
          <CardContent class="space-y-1.5">
            <CardTitle size="sm">Preline Default Card</CardTitle>
            <CardDescription>
              Card con imagen superior, texto estructurado y botón directo de acción.
            </CardDescription>
            <div class="pt-2">
              <Button size="xs" variant="primary">Ver Catálogo</Button>
            </div>
          </CardContent>
        </Card>
      </div>
    </section>

    <section class="space-y-4">
      <div class="border-b border-border pb-2">
        <h2 class="text-lg font-semibold text-foreground">4. Forms & Controls</h2>
        <p class="text-xs text-muted-foreground">Campos de formulario consistentes con enlace reactivo (v-model).</p>
      </div>

      <Card>
        <CardContent class="space-y-5">
          <div class="grid grid-cols-1 sm:grid-cols-2 gap-4">
            <Input
              v-model="formState.fullName"
              label="Nombre Completo"
              placeholder="Ej. Rodrigo Quispe"
              required
            >
              <template #prefix>
                <IconSearch class="size-4" />
              </template>
            </Input>

            <Input
              v-model="formState.email"
              type="email"
              label="Correo Electrónico"
              placeholder="rodrigo@empresa.pe"
              helper-text="Usaremos este email para enviar reportes."
            />
          </div>

          <div class="grid grid-cols-1 sm:grid-cols-2 gap-4">
            <Select
              v-model="formState.plan"
              label="Plan Comercial"
              :options="planOptions"
            />

            <Input
              label="Campo con Error (Demostración)"
              placeholder="Ingrese código..."
              error-message="Este código de cliente no existe en la base de datos."
            />
          </div>

          <Textarea
            v-model="formState.notes"
            label="Notas y Observaciones"
            placeholder="Añada notas relevantes para el equipo comercial..."
            :rows="3"
          />

          <div class="pt-2 border-t border-border space-y-3">
            <div class="flex flex-col sm:flex-row sm:items-center justify-between gap-3">
              <Checkbox
                v-model="formState.newsletter"
                label="Recibir resumen semanal de oportunidades"
                description="Se enviará los lunes a primera hora."
              />

              <Switch
                v-model="formState.notifications"
                label="Notificaciones Push"
                description="Alertas cuando un lead cambia de fase."
              />
            </div>

            <Checkbox
              v-model="formState.terms"
              label="Acepto los términos y políticas de tratamiento de datos"
              required
            />
          </div>
        </CardContent>
        <CardFooter>
          <span class="text-xs text-muted-foreground">Estado del formulario reactivo verificado</span>
          <Button variant="primary" size="sm">
            <template #prefix>
              <IconSend class="size-4" />
            </template>
            Guardar Formulario
          </Button>
        </CardFooter>
      </Card>
    </section>

    <section class="space-y-4">
      <div class="border-b border-border pb-2">
        <h2 class="text-lg font-semibold text-foreground">5. Links</h2>
        <p class="text-xs text-muted-foreground">Enlaces estilizados para navegación interna (RouterLink) o externa.</p>
      </div>

      <Card>
        <CardContent>
          <div class="flex flex-wrap gap-6 items-center text-sm">
            <Link to="/panel" variant="primary">
              Ir al Dashboard Principal
            </Link>

            <Link to="/pruebas" variant="subtle">
              Enlace Sutil
            </Link>

            <Link to="/pruebas" variant="muted">
              Enlace Muted
            </Link>

            <Link href="https://preline.co" target="_blank" variant="hover-underline">
              Preline UI Externo (Nueva Pestaña)
            </Link>

            <Link to="/pruebas" variant="danger">
              Enlace Destructivo
            </Link>
          </div>
        </CardContent>
      </Card>
    </section>

    <section class="space-y-4">
      <div class="border-b border-border pb-2">
        <h2 class="text-lg font-semibold text-foreground">6. Primitivas de Tabla (Table)</h2>
        <p class="text-xs text-muted-foreground">Componentes atómicos de tabla para composición manual.</p>
      </div>

      <div class="bg-card rounded-xl border border-border overflow-hidden shadow-xs">
        <Table>
          <TableHeader>
            <TableRow :hoverable="false">
              <TableHead>Empresa</TableHead>
              <TableHead>Contacto</TableHead>
              <TableHead>Estado</TableHead>
              <TableHead align="right">Valor</TableHead>
            </TableRow>
          </TableHeader>
          <TableBody>
            <TableRow v-for="item in sampleTableData" :key="item.id">
              <TableCell class="font-medium">{{ item.name }}</TableCell>
              <TableCell>{{ item.contact }}</TableCell>
              <TableCell>
                <Badge
                  :variant="item.status === 'Activo' ? 'success' : item.status === 'Pendiente' ? 'warning' : 'neutral'"
                  dot
                >
                  {{ item.status }}
                </Badge>
              </TableCell>
              <TableCell align="right" class="font-semibold">{{ item.value }}</TableCell>
            </TableRow>
          </TableBody>
        </Table>
      </div>
    </section>

    <section class="space-y-4">
      <div class="border-b border-border pb-2">
        <h2 class="text-lg font-semibold text-foreground">7. DataTable Avanzado</h2>
        <p class="text-xs text-muted-foreground">
          Búsqueda instantánea, ordenamiento multidireccional, selección con checkboxes y paginación integrada.
        </p>
      </div>

      <DataTable
        v-model:selected-keys="selectedLeadKeys"
        :columns="datatableColumns"
        :data="datatableData"
        :page-size="4"
        :page-size-options="[2, 4, 8, 10]"
        selectable
      >
        <template #toolbar>
          <span v-if="selectedLeadKeys.length" class="text-xs text-muted-foreground">
            {{ selectedLeadKeys.length }} seleccionados
          </span>
          <Button size="xs" variant="primary">
            <template #prefix>
              <IconPlus class="size-3.5" />
            </template>
            Nuevo Lead
          </Button>
        </template>

        <template #cell-status="{ value }">
          <Badge :variant="statusBadgeVariant(value)" dot>
            {{ value }}
          </Badge>
        </template>

        <template #cell-budget="{ value }">
          <span class="font-semibold text-foreground">{{ formatMoney(value) }}</span>
        </template>

        <template #cell-actions>
          <div class="flex items-center justify-end gap-1.5">
            <Button size="xs" variant="ghost">Editar</Button>
            <Button size="xs" variant="ghost" class="text-destructive hover:bg-destructive/10">
              <IconTrash class="size-3.5" />
            </Button>
          </div>
        </template>
      </DataTable>
    </section>
  </div>
</template>