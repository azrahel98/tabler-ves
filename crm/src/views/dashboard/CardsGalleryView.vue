<script setup lang="ts">
import { ref } from 'vue'
import Card from '@/components/ui/card/Card.vue'
import CardFooter from '@/components/ui/card/CardFooter.vue'
import CardImage from '@/components/ui/card/CardImage.vue'
import Button from '@/components/ui/button/Button.vue'
import Badge from '@/components/ui/badge/Badge.vue'
import {
  IconArrowRight,
  IconDotsVertical,
  IconSparkles,
  IconInbox,
  IconPlus,
  IconShare,
  IconBookmark,
} from '@tabler/icons-vue'

const activeTab = ref('all')

const tabs = [
  { id: 'all', label: 'Todos los Cards' },
  { id: 'default', label: 'Default & Simple' },
  { id: 'header-footer', label: 'Header & Footer' },
  { id: 'top-bordered', label: 'Top-Bordered' },
  { id: 'horizontal', label: 'Horizontal' },
  { id: 'images', label: 'Imágenes & Overlay' },
  { id: 'empty-scrollable', label: 'Empty State & Scrollable' },
]

const sampleArticles = [
  { id: 1, title: 'Optimización de Procesos Comerciales en 2026', author: 'Equipo CRM', time: 'Hace 10 min' },
  { id: 2, title: 'Nuevas Estrategias de Fidelización B2B', author: 'Ventas Corporativas', time: 'Hace 1 hora' },
  { id: 3, title: 'Automatización de Pipelines con Preline UI', author: 'Desarrollo Frontend', time: 'Hace 3 horas' },
  { id: 4, title: 'Métricas Clave para Equipos de Alto Rendimiento', author: 'Dirección Comercial', time: 'Ayer' },
  { id: 5, title: 'Mejores Prácticas en Retención de Clientes', author: 'Éxito del Cliente', time: 'Hace 2 días' },
]
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

    <div class="space-y-8">
      <section v-if="activeTab === 'all' || activeTab === 'default'" class="space-y-4">
        <div class="border-b border-border pb-2">
          <h2 class="text-base sm:text-lg font-semibold text-foreground">1. Default Card & Simple Card</h2>
          <p class="text-xs text-muted-foreground">Estructura base con imagen superior, cuerpo de texto y botón de acción.</p>
        </div>

        <div class="grid grid-cols-1 md:grid-cols-3 gap-5">
          <Card>
            <CardImage
              src="https://images.unsplash.com/photo-1680868543815-b8666dba60f7?ixlib=rb-4.0.3&auto=format&fit=crop&w=600&q=80"
              alt="Default Card"
              position="top"
            />
            
              <h3 class="font-semibold text-foreground tracking-tight text-base">Default Card</h3>
              <p class="text-xs text-muted-foreground">
                Card estándar de Preline con imagen en cabecera, texto explicativo y llamada a la acción principal.
              </p>
              <div class="pt-2">
                <Button size="sm" variant="primary">
                  Explorar
                  <template #suffix>
                    <IconArrowRight class="size-3.5" />
                  </template>
                </Button>
              </div>
            
          </Card>

          <Card>
            
              <span class="text-xs font-semibold text-primary uppercase tracking-wider block">Novedad</span>
              <h3 class="font-semibold text-foreground tracking-tight text-base">Simple Card</h3>
              <p class="text-xs text-muted-foreground">
                Card minimalista sin división de cabecera ni pie. Ideal para notas rápidas, citas o resúmenes de texto directo.
              </p>
              <p class="text-xs text-muted-foreground">
                Diseñado para integrarse en paneles con alta densidad de datos sin sobrecargar la interfaz.
              </p>
            
          </Card>

          <Card hoverable>
            
              <div class="flex items-center justify-between">
                <Badge variant="success" dot>Interactivo</Badge>
                <IconBookmark class="size-4 text-muted-foreground group-hover:text-primary transition" />
              </div>
              <h3 class="font-semibold text-foreground tracking-tight text-base">Interactive Hover Card</h3>
              <p class="text-xs text-muted-foreground">
                Card con respuesta interactiva al cursor. Eleva la sombra y resalta el borde suavemente al posar el mouse.
              </p>
              <div class="pt-2 flex items-center text-xs font-semibold text-primary group-hover:underline gap-1">
                <span>Ver detalles del elemento</span>
                <IconArrowRight class="size-3.5" />
              </div>
            
          </Card>
        </div>
      </section>

      <section v-if="activeTab === 'all' || activeTab === 'header-footer'" class="space-y-4">
        <div class="border-b border-border pb-2">
          <h2 class="text-base sm:text-lg font-semibold text-foreground">2. Header and Footer / Panel Actions</h2>
          <p class="text-xs text-muted-foreground">Encabezados con fondo diferenciado (surface) y acciones en la cabecera.</p>
        </div>

        <div class="grid grid-cols-1 md:grid-cols-2 gap-5">
          <Card>
            
              <h3 class="font-semibold text-foreground tracking-tight text-base">Reporte de Gestión</h3>
              <p class="text-xs text-muted-foreground">Período fiscal 2026</p>
              <template #action>
                <Badge variant="primary">Q3</Badge>
              </template>
            
            
              <p class="text-sm text-foreground">
                Card con cabecera en contraste sutil (variant="surface"), contenido principal y pie informativo con botón alineado.
              </p>
            
            <CardFooter variant="surface">
              <span class="text-xs text-muted-foreground">Última sincronización: Hoy</span>
              <Button size="xs" variant="outline">Descargar PDF</Button>
            </CardFooter>
          </Card>

          <Card>
            
              <div class="flex items-center gap-2">
                <IconSparkles class="size-4 text-primary" />
                <h3 class="font-semibold text-foreground tracking-tight text-base">Panel con Acciones</h3>
              </div>
              <template #action>
                <Button size="xs" variant="ghost" title="Compartir">
                  <IconShare class="size-3.5 text-muted-foreground" />
                </Button>
                <Button size="xs" variant="ghost" title="Más opciones">
                  <IconDotsVertical class="size-3.5 text-muted-foreground" />
                </Button>
              </template>
            
            
              <p class="text-sm text-foreground">
                Permite insertar botones de acción en la esquina superior derecha mediante el slot #action de CardHeader.
              </p>
            
            <CardFooter>
              <span class="text-xs text-muted-foreground">5 tareas pendientes</span>
              <Button size="xs" variant="primary">Gestionar</Button>
            </CardFooter>
          </Card>
        </div>
      </section>

      <section v-if="activeTab === 'all' || activeTab === 'top-bordered'" class="space-y-4">
        <div class="border-b border-border pb-2">
          <h2 class="text-base sm:text-lg font-semibold text-foreground">3. Top-Bordered Cards (Borde Superior de Color)</h2>
          <p class="text-xs text-muted-foreground">Acento superior para categorizar información y destacar estados clave.</p>
        </div>

        <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4">
          <Card top-border="primary">
            
              <span class="text-xs font-semibold text-primary uppercase">Primary</span>
              <h3 class="font-semibold text-foreground tracking-tight text-base">Oportunidad</h3>
              <p class="text-xs text-muted-foreground">Borde superior de color primario para nuevos prospectos o flujos estándar.</p>
            
          </Card>

          <Card top-border="success">
            
              <span class="text-xs font-semibold text-emerald-600 dark:text-emerald-400 uppercase">Success</span>
              <h3 class="font-semibold text-foreground tracking-tight text-base">Negocio Ganado</h3>
              <p class="text-xs text-muted-foreground">Acento verde para transacciones completadas satisfactoriamente.</p>
            
          </Card>

          <Card top-border="warning">
            
              <span class="text-xs font-semibold text-amber-500 uppercase">Warning</span>
              <h3 class="font-semibold text-foreground tracking-tight text-base">En Negociación</h3>
              <p class="text-xs text-muted-foreground">Acento ámbar para tratos que requieren seguimiento inmediato.</p>
            
          </Card>

          <Card top-border="danger">
            
              <span class="text-xs font-semibold text-rose-600 dark:text-rose-400 uppercase">Danger</span>
              <h3 class="font-semibold text-foreground tracking-tight text-base">Riesgo de Fuga</h3>
              <p class="text-xs text-muted-foreground">Acento carmesí para alertar sobre cuentas con baja actividad.</p>
            
          </Card>
        </div>
      </section>

      <section v-if="activeTab === 'all' || activeTab === 'horizontal'" class="space-y-4">
        <div class="border-b border-border pb-2">
          <h2 class="text-base sm:text-lg font-semibold text-foreground">4. Horizontal Card</h2>
          <p class="text-xs text-muted-foreground">Distribución horizontal para artículos, perfiles o tarjetas de catálogo.</p>
        </div>

        <div class="grid grid-cols-1 lg:grid-cols-2 gap-5">
          <Card horizontal>
            <CardImage
              src="https://images.unsplash.com/photo-1594909122845-11baa439b7bf?ixlib=rb-4.0.3&auto=format&fit=crop&w=600&q=80"
              alt="Horizontal Card"
              position="left"
            />
            <div class="flex flex-col justify-between flex-1 p-4 sm:p-5">
              <div class="space-y-2">
                <Badge variant="primary" size="xs">Casos de Éxito</Badge>
                <h3 class="font-semibold text-foreground tracking-tight text-base">Transformación Digital en Retail</h3>
                <p class="text-xs text-muted-foreground">
                  Cómo una empresa líder del sector incrementó su conversión comercial un 42% adoptando flujos automatizados.
                </p>
              </div>
              <div class="pt-4 flex items-center justify-between border-t border-border mt-3 text-xs text-muted-foreground">
                <span>Por Martín Morales</span>
                <Button size="xs" variant="outline">Leer Caso</Button>
              </div>
            </div>
          </Card>

          <Card horizontal hoverable>
            <CardImage
              src="https://images.unsplash.com/photo-1542744094-3a31f272c490?ixlib=rb-4.0.3&auto=format&fit=crop&w=600&q=80"
              alt="Horizontal Card 2"
              position="left"
              zoom-on-hover
            />
            <div class="flex flex-col justify-between flex-1 p-4 sm:p-5">
              <div class="space-y-2">
                <Badge variant="success" size="xs">Guía Práctica</Badge>
                <h3 class="font-semibold text-foreground tracking-tight text-base">Estrategias de Retención 2026</h3>
                <p class="text-xs text-muted-foreground">
                  Claves esenciales para mejorar la satisfacción del cliente y reducir la rotación con herramientas modernas.
                </p>
              </div>
              <div class="pt-4 flex items-center justify-between border-t border-border mt-3 text-xs text-muted-foreground">
                <span>5 min de lectura</span>
                <span class="text-primary font-semibold group-hover:underline">Ver reporte &rarr;</span>
              </div>
            </div>
          </Card>
        </div>
      </section>

      <section v-if="activeTab === 'all' || activeTab === 'images'" class="space-y-4">
        <div class="border-b border-border pb-2">
          <h2 class="text-base sm:text-lg font-semibold text-foreground">5. Image Scaling on Hover & Image Overlays</h2>
          <p class="text-xs text-muted-foreground">Efectos visuales atractivos para vitrinas y destacados.</p>
        </div>

        <div class="grid grid-cols-1 md:grid-cols-2 gap-5">
          <Card hoverable>
            <CardImage
              src="https://images.unsplash.com/photo-1680868543815-b8666dba60f7?ixlib=rb-4.0.3&auto=format&fit=crop&w=800&q=80"
              alt="Zoom Effect"
              position="top"
              zoom-on-hover
            />
            
              <Badge variant="primary" size="xs">Efecto Zoom</Badge>
              <h3 class="font-semibold text-foreground tracking-tight text-base">Imagen con Escala en Hover</h3>
              <p class="text-xs text-muted-foreground">
                La imagen se amplía suavemente con transición al pasar el cursor sobre la tarjeta.
              </p>
            
          </Card>

          <Card class="relative min-h-[260px]">
            <CardImage
              src="https://images.unsplash.com/photo-1542744094-3a31f272c490?ixlib=rb-4.0.3&auto=format&fit=crop&w=800&q=80"
              alt="Image Overlay"
              position="overlay"
            >
              <span class="text-xs font-semibold uppercase tracking-wider text-blue-300">Destacado del Mes</span>
              <h3 class="text-lg font-bold text-white mt-1">Conferencia Anual CRM 2026</h3>
              <p class="text-xs text-stone-200 mt-1 max-w-sm">
                Aprende de los mejores líderes de la industria sobre automatización, experiencia de usuario e inteligencia comercial.
              </p>
              <div class="mt-3">
                <Button size="xs" variant="primary">Inscribirme</Button>
              </div>
            </CardImage>
          </Card>
        </div>
      </section>

      <section v-if="activeTab === 'all' || activeTab === 'empty-scrollable'" class="space-y-4">
        <div class="border-b border-border pb-2">
          <h2 class="text-base sm:text-lg font-semibold text-foreground">6. Empty State & Scrollable Body</h2>
          <p class="text-xs text-muted-foreground">Estados vacíos centrados y cuerpos con desplazamiento vertical interno.</p>
        </div>

        <div class="grid grid-cols-1 md:grid-cols-2 gap-5">
          <Card>
            
              <div class="size-12 rounded-full bg-primary/10 text-primary flex items-center justify-center">
                <IconInbox class="size-6" />
              </div>
              <div class="space-y-1">
                <h3 class="font-semibold text-foreground tracking-tight text-base">Bandeja de Leads Vacía</h3>
                <p class="text-xs text-muted-foreground">
                  No hay prospectos asignados a este filtro en este momento. Puedes crear un nuevo lead cuando lo desees.
                </p>
              </div>
              <Button size="sm" variant="primary">
                <template #prefix>
                  <IconPlus class="size-4" />
                </template>
                Crear Nuevo Lead
              </Button>
            
          </Card>

          <Card>
            
              <h3 class="font-semibold text-foreground tracking-tight text-base">Actividad Reciente (Scrollable)</h3>
              <p class="text-xs text-muted-foreground">Desplazamiento vertical en cuerpo</p>
            
            
              <div
                v-for="item in sampleArticles"
                :key="item.id"
                class="py-2.5 first:pt-0 last:pb-0 flex items-center justify-between text-xs"
              >
                <div class="space-y-0.5">
                  <p class="font-medium text-foreground">{{ item.title }}</p>
                  <p class="text-muted-foreground">{{ item.author }}</p>
                </div>
                <span class="text-muted-foreground/70 shrink-0">{{ item.time }}</span>
              </div>
            
            <CardFooter>
              <span class="text-xs text-muted-foreground">{{ sampleArticles.length }} registros en cola</span>
              <Button size="xs" variant="outline">Ver Todo</Button>
            </CardFooter>
          </Card>
        </div>
      </section>
    </div>
  </div>
</template>
